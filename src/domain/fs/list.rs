use std::{fs, io};

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

pub fn list_current_dir() -> Result<Vec<FileEntry>, io::Error> {
    let entries = fs::read_dir(".")?;

    let mut file_list: Vec<FileEntry> = Vec::new();

    for entry_result in entries {
        let entry = entry_result?;
        let metadata = entry.metadata()?;

        let name = entry
            .file_name()
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
