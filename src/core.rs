use clap::{Args, ValueEnum};

pub mod limit;

#[derive(Args, Debug)]
pub struct QueryArgs<T: ValueEnum + Clone + Send + Sync + 'static> {
    #[arg(long, value_enum)]
    pub sort: Option<T>,

    #[arg(long)]
    pub limit: Option<usize>,
}

pub use limit::apply_limit;
