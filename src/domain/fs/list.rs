// src/domain/fs/list.rs

use std::{fs, io};

use crate::domain::fs::{FileEntry, build_file_entry};

pub fn list_current_dir(include_hidden: bool) -> Result<Vec<FileEntry>, io::Error> {
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

fn extract_dir_entry_name(entry: &fs::DirEntry) -> Result<String, io::Error> {
    entry.file_name().into_string().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Filename contains invalid UTF-8",
        )
    })
}

fn is_hidden_filename(name: &str) -> bool {
    name.starts_with('.')
}
