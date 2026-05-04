use crate::core::{OutputFormat, SortOrder};
use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub struct QueryArgs<T: ValueEnum + Clone + Send + Sync + 'static> {
    #[arg(long, value_enum)]
    pub sort: Option<T>,

    #[arg(long, value_enum, default_value_t = SortOrder::Asc)]
    pub order: SortOrder,

    #[arg(long)]
    pub limit: Option<usize>,
}

#[derive(Args, Clone, Debug)]
pub struct OutputArgs {
    /// Output format: table (human-readable) or json (machine-readable)
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
