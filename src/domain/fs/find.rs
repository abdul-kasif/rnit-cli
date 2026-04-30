use std::io;

use crate::domain::fs::{FileEntry, list_current_dir};

pub fn find_current_dir(
    include_hidden: bool,
    name_filter: Option<&str>,
) -> Result<Vec<FileEntry>, io::Error> {
    let entries = list_current_dir(include_hidden)?;

    if let Some(pattern) = name_filter {
        let filtered = entries
            .into_iter()
            .filter(|e| e.name.to_lowercase().contains(pattern))
            .collect();

        Ok(filtered)
    } else {
        Ok(entries)
    }
}
