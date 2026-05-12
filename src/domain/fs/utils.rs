use std::{fs, io, path::Path};

use crate::domain::fs::FileEntry;

pub fn build_file_entry(name: String, metadata: &fs::Metadata) -> FileEntry {
    FileEntry {
        name,
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    }
}

pub fn ensure_path_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<fs::Metadata, io::Error> {
    let path = path.as_ref();
    fs::metadata(path).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} not found: {}", label, path.display()),
            )
        } else {
            err
        }
    })
}

pub fn ensure_path_not_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<(), io::Error> {
    let path = path.as_ref();

    match fs::metadata(path) {
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{} already exists: {}", label, path.display()),
        )),

        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),

        Err(err) => Err(err),
    }
}

pub fn extract_filename<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    path.as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path contains invalid UTF-8"))
}

pub fn validate_entry_name(name: &str) -> Result<(), io::Error> {
    if name.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name cannot be empty",
        ));
    }

    if name == "." || name == ".." {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Entry name '{}' is reserved", name),
        ));
    }

    if name.contains('\0') || name.contains('/') || name.contains('\\') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name contains invalid characters (\\0, /, \\)",
        ));
    }

    if name.len() > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Entry name exceeds maximum length (255 bytes)",
        ));
    }

    Ok(())
}

pub fn validate_parent_exists<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let path = path.as_ref();
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
