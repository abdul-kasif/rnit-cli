use clap::{Parser, Subcommand};
mod core;
mod domain;
mod output;

use crate::{
    core::{QueryArgs, apply_limit, apply_sort},
    domain::fs::{FileEntry, FsSortField, list_current_dir},
    output::{OutputFormat, print_output},
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
        #[arg(long)]
        json: bool,

        #[arg(short, long, default_value_t = false)]
        all: bool,

        #[command(flatten)]
        query: QueryArgs<FsSortField>,
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
            FsCommands::List { json, all, query } => {
                let format = if json {
                    OutputFormat::Json
                } else {
                    OutputFormat::Table
                };

                let mut entries = list_current_dir(all)?;

                let sort_fn = query.sort.map(|field| match field {
                    FsSortField::Name => |a: &FileEntry, b: &FileEntry| a.name.cmp(&b.name),
                    FsSortField::Size => |a: &FileEntry, b: &FileEntry| b.size.cmp(&a.size),
                });

                apply_sort(&mut entries, sort_fn);
                apply_limit(&mut entries, query.limit);
                print_output(&entries, &format);
            }
        },
    }

    Ok(())
}
