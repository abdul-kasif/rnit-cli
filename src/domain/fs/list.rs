use std::{fs, io};

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
}

pub fn list_current_dir() -> Result<Vec<FileEntry>, io::Error> {
    let entries = fs::read_dir(".")?;

    let mut file_list: Vec<FileEntry> = entries
        .flatten()
        .filter_map(|entry| {
            entry
                .file_name()
                .into_string()
                .ok()
                .map(|name| FileEntry { name })
        })
        .collect();

    file_list.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(file_list)
}
