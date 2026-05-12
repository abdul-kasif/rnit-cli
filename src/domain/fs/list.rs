use std::fs;

use crate::domain::fs::{FileEntry, FsError, build_file_entry};

pub fn list_current_dir(include_hidden: bool) -> Result<Vec<FileEntry>, FsError> {
    let entries = fs::read_dir(".")?;
    let mut file_list: Vec<FileEntry> = Vec::new();

    for entry_result in entries {
        let entry = entry_result?;

        let name = extract_dir_entry_name(&entry)?;

        if !include_hidden && is_hidden_filename(&name) {
            continue;
        }

        let metadata = entry.metadata()?;
        file_list.push(build_file_entry(name, &metadata));
    }

    Ok(file_list)
}

fn extract_dir_entry_name(entry: &fs::DirEntry) -> Result<String, FsError> {
    entry
        .file_name()
        .into_string()
        .map_err(|_| FsError::InvalidName {
            reason: "Filename contains invalid UTF-8".to_string(),
        })
}

fn is_hidden_filename(name: &str) -> bool {
    name.starts_with('.')
}
