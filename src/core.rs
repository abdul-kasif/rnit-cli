use clap::{Args, ValueEnum};

pub mod limit;
pub mod sort;
pub mod traits;

#[derive(Args, Debug)]
pub struct QueryArgs<T: ValueEnum + Clone + Send + Sync + 'static> {
    #[arg(long, value_enum)]
    pub sort: Option<T>,

    #[arg(long)]
    pub limit: Option<usize>,
}

pub use limit::apply_limit;
pub use sort::apply_sort;
pub use traits::TableRender;

