use std::{fs, io, path::Path};

use crate::domain::fs::FileEntry;

pub fn create_entry<P: AsRef<Path>>(name: P, is_dir: bool) -> Result<FileEntry, io::Error> {
    let path = name.as_ref();
    let name_str = path.to_string_lossy().to_string();

    if is_dir {
        fs::create_dir_all(path).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                io::Error::new(e.kind(), format!("Directory already exists: {}/", name_str))
            } else {
                e
            }
        })?;
    } else {
        fs::File::create_new(path).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                io::Error::new(e.kind(), format!("File already exists: {}", name_str))
            } else {
                e
            }
        })?;
    }

    Ok(FileEntry {
        name: name_str,
        size: 0,
        is_dir,
    })
}
