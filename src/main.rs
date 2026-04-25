use std::path;

use clap::{Parser, Subcommand};
mod core;
mod domain;
mod output;

use crate::{
    core::{OutputArgs, QueryArgs, apply_limit, apply_sort},
    domain::fs::{FileEntry, FsSortField, get_file_info, list_current_dir},
    output::print_output,
};

#[derive(Parser)]
#[command(name = "Rnit", version, about = "Rnit CLI Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// FileSystem related operations
    Fs {
        #[command(subcommand)]
        action: FsCommands,
    },
}

#[derive(Subcommand)]
enum FsCommands {
    /// List files in the current directory
    List {
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
        path: path::PathBuf,

        #[command(flatten)]
        output: OutputArgs,
    },
}
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), std::io::Error> {
    match cli.command {
        Commands::Fs { action } => match action {
            FsCommands::List { all, query, output } => {
                let mut entries = list_current_dir(all)?;

                let sort_fn = query.sort.map(|field| match field {
                    FsSortField::Name => |a: &FileEntry, b: &FileEntry| a.name.cmp(&b.name),
                    FsSortField::Size => |a: &FileEntry, b: &FileEntry| b.size.cmp(&a.size),
                });

                apply_sort(&mut entries, sort_fn);
                apply_limit(&mut entries, query.limit);
                print_output(&entries, &output.format);
            }

            FsCommands::Info { path, output } => {
                let entry = get_file_info(&path)?;

                print_output(std::slice::from_ref(&entry), &output.format);
            }
        },
    }

    Ok(())
}
