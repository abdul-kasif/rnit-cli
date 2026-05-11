use std::{fs, io, path::Path};

use crate::domain::fs::{FileEntry, validate_entry_name};

pub fn rename_entry<P: AsRef<Path>>(source: P, destination: P) -> Result<FileEntry, io::Error> {
    let src = source.as_ref();
    let dest = destination.as_ref();

    if src == dest {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and destination paths are identical",
        ));
    }

    let src_metadata = validate_rename_source(src)?;

    let dest_name = dest.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination path contains invalid UTF-8",
        )
    })?;

    validate_entry_name(dest_name)?;

    validate_rename_destination(dest)?;

    execute_rename(src, dest)?;

    Ok(FileEntry {
        name: dest_name.to_string(),
        size: src_metadata.len(),
        is_dir: src_metadata.is_dir(),
    })
}

fn validate_rename_source(src: &Path) -> Result<fs::Metadata, io::Error> {
    let metadata = fs::metadata(src).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source not found: {}", src.display()),
            )
        } else {
            err
        }
    })?;

    Ok(metadata)
}

fn validate_rename_destination(dest: &Path) -> Result<(), io::Error> {
    if let Some(parent) = dest.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Destination parent directory does not exist: {}",
                parent.display()
            ),
        ));
    }

    if dest.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination already exists: {}", dest.display()),
        ));
    }

    Ok(())
}

fn execute_rename(src: &Path, dest: &Path) -> Result<(), io::Error> {
    fs::rename(src, dest).map_err(|err| match err.kind() {
        io::ErrorKind::CrossesDevices => io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot rename across filesystems (use copy+delete instead)",
        ),
        io::ErrorKind::PermissionDenied => io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Permission denied to rename: {}", dest.display()),
        ),
        _ => err,
    })?;
    Ok(())
}
