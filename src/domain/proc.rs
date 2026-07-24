use crate::{
    core::{OutputArgs, QueryArgs, apply_limit, apply_sort},
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

            let sort_fn = query.sort.map(|field| match field {
                ProcSortField::Pid => |a: &ProcessInfo, b: &ProcessInfo| a.pid.cmp(&b.pid),
                ProcSortField::Name => |a: &ProcessInfo, b: &ProcessInfo| a.name.cmp(&b.name),
                ProcSortField::State => |a: &ProcessInfo, b: &ProcessInfo| a.state.cmp(&b.state),
                ProcSortField::Memory => |a: &ProcessInfo, b: &ProcessInfo| a.rss.cmp(&b.rss),
            });

            apply_sort(&mut processes, sort_fn, query.order);
            apply_limit(&mut processes, query.limit);
            print_output(&processes, &output.format);
        }
    }
}
