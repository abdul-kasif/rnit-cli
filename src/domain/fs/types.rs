use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Default)]
pub enum FsSortField {
    #[default]
    Name,
    Size,
}
