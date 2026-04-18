use std::{
    fs::{self, FileType},
    io,
};

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub file_type: FileType,
}

pub fn list_current_dir() -> Result<Vec<FileEntry>, io::Error> {
    let entries = fs::read_dir(".")?;

    let mut file_list: Vec<FileEntry> = entries
        .flatten()
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let name = entry.file_name().into_string().ok()?;
            Some(FileEntry {
                name,
                size: metadata.len(),
                file_type: metadata.file_type(),
            })
        })
        .collect();

    file_list.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(file_list)
}
