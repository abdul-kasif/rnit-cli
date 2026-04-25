use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

#[derive(clap::Args, Clone, Debug)]
pub struct OutputArgs {
    /// Output format: table (human-readable) or json (machine-readable)
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}
