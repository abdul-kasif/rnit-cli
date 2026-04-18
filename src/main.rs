use clap::{Parser, Subcommand};

mod domain;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// FileSystem related operations
    Fs {
        #[command(subcommand)]
        fs_command: FsCommand,
    },
}

#[derive(Subcommand)]
enum FsCommand {
    /// List files in the current directory
    List,
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Fs { fs_command } => match fs_command {
            FsCommand::List => {
                println!("FS LIST triggered");
            }
        },
    }
}
