use std::{fs, io, path::Path};

use crate::domain::fs::{FileEntry, FsError};

pub fn build_file_entry(name: String, metadata: &fs::Metadata) -> FileEntry {
    FileEntry {
        name,
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    }
}

pub fn ensure_path_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<fs::Metadata, FsError> {
    let path = path.as_ref();
    fs::metadata(path).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            FsError::NotFound {
                path: path.display().to_string(),
                label: label.to_string(),
            }
        } else {
            FsError::Io(err)
        }
    })
}

pub fn ensure_path_not_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<(), FsError> {
    let path = path.as_ref();

    match fs::metadata(path) {
        Ok(_) => Err(FsError::AlreadyExists {
            path: path.display().to_string(),
            label: label.to_string(),
        }),

        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),

        Err(err) => Err(FsError::Io(err)),
    }
}

pub fn extract_filename<P: AsRef<Path>>(path: P) -> Result<String, FsError> {
    let path_ref = path.as_ref();

    if path_ref
        .to_str()
        .is_some_and(|s| s.ends_with("/") || s.ends_with("\\"))
    {
        return Err(FsError::InvalidName {
            reason: "Path ends with a directory seperator".to_string(),
        });
    }

    path_ref
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| FsError::InvalidName {
            reason: "Path contains invalid UTF-8 or has no filename component".to_string(),
        })
}

pub fn validate_entry_name(name: &str) -> Result<(), FsError> {
    if name.trim().is_empty() {
        return Err(FsError::InvalidName {
            reason: "Entry name cannot be empty".to_string(),
        });
    }
    if name == "." || name == ".." {
        return Err(FsError::InvalidName {
            reason: format!("Entry name '{}' is reserved", name),
        });
    }
    if name.contains('\0') || name.contains('/') || name.contains('\\') {
        return Err(FsError::InvalidName {
            reason: "Entry name contains invalid characters (\\0, /, \\)".to_string(),
        });
    }
    if name.len() > 255 {
        return Err(FsError::InvalidName {
            reason: "Entry name exceeds maximum length (255 bytes)".to_string(),
        });
    }
    Ok(())
}

pub fn validate_parent_exists<P: AsRef<Path>>(path: P) -> Result<(), FsError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        return Err(FsError::ParentNotFound {
            path: parent.display().to_string(),
        });
    }
    Ok(())
}
