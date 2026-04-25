pub mod json;
pub mod table;

use crate::core::OutputFormat;
use crate::core::traits::TableRender;
use serde::Serialize;

pub fn print_output<T: Serialize + TableRender>(entries: &[T], format: &OutputFormat) {
    match format {
        OutputFormat::Json => json::print_json(entries),
        OutputFormat::Table => table::print_table(entries),
    }
}
