use std::{fs, path::Path};

use crate::domain::fs::{FileEntry, FsError, build_file_entry, extract_filename};

pub fn get_file_info(path: &Path) -> Result<FileEntry, FsError> {
    let name = extract_filename(path)?;

    let metadata = fs::metadata(path)?;

    Ok(build_file_entry(name, &metadata))
}
