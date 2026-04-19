pub mod json;
pub mod table;

use crate::domain::fs::FileEntry;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Table,
}

pub fn print_output(entries: &[FileEntry], format: &OutputFormat) {
    match format {
        OutputFormat::Json => json::print_json(entries),
        OutputFormat::Table => table::print_table(entries),
    }
}
