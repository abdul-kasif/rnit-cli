use crate::{core::OutputArgs, output::print_output};

mod list;
pub mod types;

use list::get_all_processes;
pub use types::ProcessInfo;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProcCommands {
    List {
        #[command(flatten)]
        output: OutputArgs,
    },
}

pub fn run(action: ProcCommands) {
    match action {
        ProcCommands::List { output } => {
            let processes = get_all_processes();
            print_output(&processes, &output.format);
        }
    }
}
