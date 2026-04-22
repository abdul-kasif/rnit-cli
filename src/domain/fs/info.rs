use std::{fs, io, path::Path};

use crate::domain::fs::FileEntry;

pub fn get_file_info(path: &Path) -> Result<FileEntry, io::Error> {
    let metadata = fs::metadata(path)?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());

    Ok(FileEntry {
        name,
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    })
}
