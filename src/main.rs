use clap::{Parser, Subcommand};
use std::error::Error;

mod core;
mod domain;
mod output;

use domain::fs::FsCommands;

#[derive(Parser)]
#[command(
    name = "rnit",
    version,
    about = "Rnit CLI Tool - A consistent DSL for system operations"
)]
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

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::Fs { action } => {
            domain::fs::run(action)?;
        }
    }

    Ok(())
}

