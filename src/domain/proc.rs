use crate::{
    core::{OutputArgs, QueryArgs, apply_limit},
    output::print_output,
};

mod list;
pub mod types;

use list::get_all_processes;
pub use types::{ProcSortField, ProcessInfo};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProcCommands {
    List {
        #[command(flatten)]
        query: QueryArgs<ProcSortField>,

        #[command(flatten)]
        output: OutputArgs,
    },
}

pub fn run(action: ProcCommands) {
    match action {
        ProcCommands::List { query, output } => {
            let mut processes = get_all_processes();

            apply_limit(&mut processes, query.limit);
            print_output(&processes, &output.format);
        }
    }
}
