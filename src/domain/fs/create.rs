use std::{fs, io, path::Path};

use crate::domain::fs::FileEntry;

pub fn create_entry<P: AsRef<Path>>(path: P, is_dir: bool) -> Result<FileEntry, io::Error> {
    let target = path.as_ref();

    validate_creation_target(target)?;

    execute_creation(target, is_dir)?;

    Ok(build_file_entry(target, is_dir))
}

fn validate_creation_target(path: &Path) -> Result<(), io::Error> {
    if path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Entry already exists: {}", path.display()),
        ));
    }

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Parent directory does not exist: {}", parent.display()),
        ));
    }

    Ok(())
}

fn execute_creation(path: &Path, is_dir: bool) -> Result<(), io::Error> {
    if is_dir {
        fs::create_dir(path)?;
    } else {
        fs::File::create_new(path)?;
    }

    Ok(())
}

fn build_file_entry(path: &Path, is_dir: bool) -> FileEntry {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("unknown"))
        .to_string();

    FileEntry {
        name,
        size: 0,
        is_dir,
    }
}
