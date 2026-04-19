use clap::{Parser, Subcommand};
mod domain;
mod output;

use crate::{
    domain::fs::list_current_dir,
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
            FsCommands::List { json } => {
                let format = if json {
                    OutputFormat::Json
                } else {
                    OutputFormat::Table
                };

                let entries = list_current_dir()?;
                print_output(&entries, &format);
            }
        },
    }

    Ok(())
}
