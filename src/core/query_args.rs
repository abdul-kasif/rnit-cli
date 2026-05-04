use clap::{Args, ValueEnum};

use crate::core::SortOrder;

#[derive(Args, Debug)]
pub struct QueryArgs<T: ValueEnum + Clone + Send + Sync + 'static> {
    #[arg(long, value_enum)]
    pub sort: Option<T>,

    #[arg(long, value_enum, default_value_t = SortOrder::Asc)]
    pub order: SortOrder,

    #[arg(long)]
    pub limit: Option<usize>,
}
