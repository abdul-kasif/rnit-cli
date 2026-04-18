use crate::domain::fs::FileEntry;
use serde_json;

pub fn print_json(entries: &[FileEntry]) {
    match serde_json::to_string_pretty(entries) {
        Ok(json_string) => {
            println!("{}", json_string)
        }
        Err(e) => eprintln!("Failed to serilalize to JSON: {}", e),
    }
}
