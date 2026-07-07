use clap::Subcommand;
use std::path::{Path, PathBuf};

pub mod create;
pub mod delete;
pub mod errors;
pub mod find;
pub mod info;
pub mod list;
pub mod rename;
pub mod types;
pub mod utils;

use crate::core::{OutputArgs, QueryArgs, apply_limit, apply_sort};
use crate::output::print_output;
pub use create::create_entry;
pub use delete::delete_entry;
pub use errors::FsError;
pub use find::find_current_dir;
pub use info::get_file_info;
pub use list::list_current_dir;
pub use rename::rename_entry;
pub use types::{FileEntry, FsSortField};
pub use utils::{
    build_file_entry, ensure_path_exists, ensure_path_not_exists, extract_filename,
    validate_entry_name, validate_parent_exists,
};

#[derive(Subcommand)]
pub enum FsCommands {
    /// List files and directories
    ///
    /// Displays the contents of the specified directory. By default, it reads the current
    /// directory and hides system/hidden files. Use modifiers to sort, filter, and limit the output.
    List {
        /// The target directory to list (defaults to the current directory ".")
        #[arg(index = 1)]
        path: Option<PathBuf>,

        /// Include hidden files and directories (those starting with a dot '.')
        #[arg(short, long, default_value_t = false)]
        all: bool,

        #[command(flatten)]
        query: QueryArgs<FsSortField>,

        #[command(flatten)]
        output: OutputArgs,
    },

    /// Get detailed information about a specific file or folder
    ///
    /// Retrieves standard metadata for the target path, including its exact size,
    /// type, and permissions. Useful for programmatic inspection via JSON output.
    Info {
        /// The exact path to the file or directory you want to inspect
        #[arg(index = 1)]
        path: PathBuf,

        #[command(flatten)]
        output: OutputArgs,
    },

    /// Find files matching a specific pattern
    ///
    /// Performs a glob-based search (e.g., "*.rs", "config_*.json") in the specified directory.
    /// The search is case-insensitive by default.
    Find {
        /// The search pattern to match against (e.g., "*.txt")
        #[arg(index = 1)]
        pattern: String,

        /// Optional directory to search within (defaults to the current directory ".")
        #[arg(index = 2)]
        path: Option<PathBuf>,

        /// Include hidden files and directories in the search results
        #[arg(short, long, default_value_t = false)]
        all: bool,

        #[command(flatten)]
        query: QueryArgs<FsSortField>,

        #[command(flatten)]
        output: OutputArgs,
    },

    /// Create a new file or directory
    ///
    /// Creates an empty file by default. Use the --dir flag to create a directory instead.
    /// Will fail if the target already exists to prevent accidental overwrites.
    Create {
        /// The name or path of the new entry to create
        #[arg(index = 1)]
        name: String,

        /// Create a directory instead of a standard file
        #[arg(short, long)]
        dir: bool,

        /// Preview what would be created without actually modifying the filesystem
        #[arg(long)]
        dry_run: bool,

        #[command(flatten)]
        output: OutputArgs,
    },

    /// Permanently delete a file or directory
    ///
    /// Removes the specified target from the filesystem. By default, it expects a file.
    /// You must explicitly provide the --dir flag to delete a directory.
    Delete {
        /// The path to the file or directory you want to remove
        #[arg(index = 1)]
        path: PathBuf,

        /// Confirm that the target is a directory (required for deleting folders)
        #[arg(short, long)]
        dir: bool,

        /// Preview what would be deleted without actually modifying the filesystem
        #[arg(long)]
        dry_run: bool,

        #[command(flatten)]
        output: OutputArgs,
    },

    /// Rename or move a file or directory
    ///
    /// Safely changes the name of a target or moves it to a new path.
    /// Will fail if a file already exists at the destination path.
    Rename {
        /// The current path of the file or directory
        #[arg(index = 1)]
        source: PathBuf,

        /// The new name or path for the target
        #[arg(index = 2)]
        destination: PathBuf,

        /// Preview the rename operation without actually modifying the filesystem
        #[arg(long)]
        dry_run: bool,

        #[command(flatten)]
        output: OutputArgs,
    },
}

/// The dedicated dispatcher for the `fs` domain
pub fn run(action: FsCommands) -> Result<(), FsError> {
    match action {
        FsCommands::List {
            path,
            all,
            query,
            output,
        } => {
            let target_path = path.as_deref().unwrap_or(Path::new("."));
            let mut entries = list_current_dir(target_path, all)?;

            let sort_fn = query.sort.map(|field| match field {
                FsSortField::Name => |a: &FileEntry, b: &FileEntry| a.name.cmp(&b.name),
                FsSortField::Size => |a: &FileEntry, b: &FileEntry| a.size.cmp(&b.size),
            });

            apply_sort(&mut entries, sort_fn, query.order);
            apply_limit(&mut entries, query.limit);
            print_output(&entries, &output.format);
        }
        FsCommands::Info { path, output } => {
            let entry = get_file_info(&path)?;
            print_output(std::slice::from_ref(&entry), &output.format);
        }
        FsCommands::Find {
            pattern,
            path,
            all,
            query,
            output,
        } => {
            let mut entries = find_current_dir(all, &pattern, path)?;

            let sort_fn = query.sort.map(|field| match field {
                FsSortField::Name => |a: &FileEntry, b: &FileEntry| a.name.cmp(&b.name),
                FsSortField::Size => |a: &FileEntry, b: &FileEntry| a.size.cmp(&b.size),
            });

            apply_sort(&mut entries, sort_fn, query.order);
            apply_limit(&mut entries, query.limit);
            print_output(&entries, &output.format);
        }
        FsCommands::Create {
            name,
            dir,
            dry_run,
            output,
        } => {
            if dry_run {
                let entry_type = if dir { "directory" } else { "file" };
                println!("[DRY-RUN] Would create {}: {}", entry_type, name);
                return Ok(());
            }
            let entry = create_entry(&name, dir)?;
            print_output(std::slice::from_ref(&entry), &output.format);
        }
        FsCommands::Delete {
            path,
            dir,
            dry_run,
            output,
        } => {
            if dry_run {
                let target_type = if dir { "directory" } else { "file" };
                println!("[DRY-RUN] Would delete {}: {}", target_type, path.display());
                return Ok(());
            }
            let entry = delete_entry(&path, dir)?;
            print_output(std::slice::from_ref(&entry), &output.format);
        }
        FsCommands::Rename {
            source,
            destination,
            dry_run,
            output,
        } => {
            if dry_run {
                println!(
                    "[DRY-RUN] Would rename: \n  from: {} \n  to: {}",
                    source.display(),
                    destination.display()
                );
                return Ok(());
            }
            let entry = rename_entry(&source, &destination)?;
            print_output(std::slice::from_ref(&entry), &output.format);
        }
    }
    Ok(())
}
