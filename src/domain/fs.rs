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
    /// List files in the current directory
    List {
        #[arg(index = 1)]
        path: Option<PathBuf>,
        #[arg(short, long, default_value_t = false)]
        all: bool,
        #[command(flatten)]
        query: QueryArgs<FsSortField>,
        #[command(flatten)]
        output: OutputArgs,
    },
    /// Get information about File/Folder
    Info {
        #[arg(index = 1)]
        path: PathBuf,
        #[command(flatten)]
        output: OutputArgs,
    },
    /// Find files in the current directory
    Find {
        #[arg(long)]
        name: Option<String>,
        #[arg(short, long, default_value_t = false)]
        all: bool,
        #[command(flatten)]
        query: QueryArgs<FsSortField>,
        #[command(flatten)]
        output: OutputArgs,
    },
    /// Create a file/directory entry
    Create {
        #[arg(index = 1)]
        name: String,
        #[arg(long)]
        dir: bool,
        #[arg(long)]
        dry_run: bool,
        #[command(flatten)]
        output: OutputArgs,
    },
    /// Delete a file/directory
    Delete {
        #[arg(index = 1)]
        path: PathBuf,
        #[arg(long)]
        dir: bool,
        #[arg(long)]
        dry_run: bool,
        #[command(flatten)]
        output: OutputArgs,
    },
    /// Rename a file/directory
    Rename {
        #[arg(index = 1)]
        source: PathBuf,
        #[arg(index = 2)]
        destination: PathBuf,
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
            name,
            all,
            query,
            output,
        } => {
            let mut entries = find_current_dir(all, name.as_deref())?;

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
