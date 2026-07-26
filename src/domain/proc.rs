use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crate::{
    core::{OutputArgs, QueryArgs, apply_limit, apply_sort},
    output::print_output,
};

mod list;
pub mod types;

use list::{calculate_process_info, capture_current_state};
pub use types::{ProcSortField, ProcessInfo};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProcCommands {
    List {
        #[arg(short, long, default_value_t = false)]
        watch: bool,

        #[command(flatten)]
        query: QueryArgs<ProcSortField>,

        #[command(flatten)]
        output: OutputArgs,
    },
}

pub fn run(action: ProcCommands) {
    match action {
        ProcCommands::List {
            watch,
            query,
            output,
        } => {
            let sort_fn: Option<fn(&ProcessInfo, &ProcessInfo) -> std::cmp::Ordering> =
                query.sort.map(|field| match field {
                    ProcSortField::Pid => |a: &ProcessInfo, b: &ProcessInfo| a.pid.cmp(&b.pid),
                    ProcSortField::Name => |a: &ProcessInfo, b: &ProcessInfo| a.name.cmp(&b.name),
                    ProcSortField::State => {
                        |a: &ProcessInfo, b: &ProcessInfo| a.state.cmp(&b.state)
                    }
                    ProcSortField::Memory => |a: &ProcessInfo, b: &ProcessInfo| a.rss.cmp(&b.rss),
                    ProcSortField::Cpu => {
                        |a: &ProcessInfo, b: &ProcessInfo| a.cpu.total_cmp(&b.cpu)
                    }
                });

            // Initial capture for Delta A
            let mut prev_state = capture_current_state();

            if !watch {
                // Single-run mode: quick 200ms sample window
                sleep(Duration::from_millis(200));
            }

            loop {
                if watch {
                    // Sleep for watch interval before capturing Snapshot B
                    sleep(Duration::from_millis(500));
                    print!("\x1B[2J\x1B[1;1H");
                    let _ = io::stdout().flush();
                }

                let curr_state = capture_current_state();
                let mut processes = calculate_process_info(&prev_state, &curr_state);

                apply_sort(&mut processes, sort_fn, query.order);
                apply_limit(&mut processes, query.limit);
                print_output(&processes, &output.format);

                if !watch {
                    break;
                }

                // Reuse current snapshot as previous for next tick
                prev_state = curr_state;
            }
        }
    }
}
