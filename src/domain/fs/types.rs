use clap::ValueEnum;
use serde::Serialize;

use crate::core::traits::TableRender;

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

impl TableRender for FileEntry {
    fn headers() -> Vec<&'static str> {
        vec!["NAME", "SIZE", "TYPE"]
    }

    fn row(&self) -> Vec<String> {
        let size_str = if self.is_dir {
            "-".to_string()
        } else {
            format!("{}B", self.size)
        };

        let type_str = if self.is_dir { "DIR" } else { "FILE" };

        vec![self.name.clone(), size_str, type_str.to_string()]
    }
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Default)]
pub enum FsSortField {
    #[default]
    Name,
    Size,
}
