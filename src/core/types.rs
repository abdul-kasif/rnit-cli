use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Copy, Default)]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}
