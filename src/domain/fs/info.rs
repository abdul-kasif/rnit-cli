use std::{fs, io, path::Path};

use crate::domain::fs::{FileEntry, build_file_entry, extract_filename};

pub fn get_file_info(path: &Path) -> Result<FileEntry, io::Error> {
    let name = extract_filename(path)?;

    let metadata = fs::metadata(path)?;

    Ok(build_file_entry(name, &metadata))
}
