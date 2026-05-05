use std::{env, fs, io};

use crate::domain::fs::FileEntry;

pub fn create_entry(name: &str, is_dir: bool) -> Result<FileEntry, io::Error> {
    let target_path = env::current_dir()?.join(name);

    if is_dir {
        fs::create_dir(&target_path)?;
        Ok(FileEntry {
            name: name.to_string(),
            size: 0,
            is_dir: true,
        })
    } else {
        fs::File::create_new(&target_path)?;
        Ok(FileEntry {
            name: name.to_string(),
            size: 0,
            is_dir: false,
        })
    }
}
