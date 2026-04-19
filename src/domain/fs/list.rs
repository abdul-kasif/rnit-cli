use serde::Serialize;
use std::{fs, io};

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

pub fn list_current_dir(include_hidden: bool) -> Result<Vec<FileEntry>, io::Error> {
    let entries = fs::read_dir(".")?;
    let mut file_list: Vec<FileEntry> = Vec::new();

    for entry_result in entries {
        let entry = entry_result?;

        let os_name = entry.file_name();

        let is_hidden = os_name
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false);

        if !include_hidden && is_hidden {
            continue;
        }

        let metadata = entry.metadata()?;

        let name = os_name
            .into_string()
            .map_err(|_| io::Error::other("Invalid filename format, expected UTF-8"))?;

        file_list.push(FileEntry {
            name,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        });
    }

    file_list.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(file_list)
}
