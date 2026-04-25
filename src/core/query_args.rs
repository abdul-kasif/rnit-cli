use clap::{Args, ValueEnum};
#[derive(Args, Debug)]
pub struct QueryArgs<T: ValueEnum + Clone + Send + Sync + 'static> {
    #[arg(long, value_enum)]
    pub sort: Option<T>,

    #[arg(long)]
    pub limit: Option<usize>,
}
