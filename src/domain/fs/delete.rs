use std::{fs, io, path::Path};

use crate::domain::fs::FileEntry;

pub fn delete_entry<P: AsRef<Path>>(path: P, expect_dir: bool) -> Result<FileEntry, io::Error> {
    let target = path.as_ref();

    let entry_name = target
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Path contains invalid UTF-8")
        })?;

    let metadata = validate_deletion_target(target, expect_dir)?;

    execute_deletion(target, metadata.is_dir())?;

    Ok(FileEntry {
        name: entry_name.to_string(),
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    })
}

fn validate_deletion_target(target: &Path, expect_dir: bool) -> Result<fs::Metadata, io::Error> {
    let metadata = fs::metadata(target).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Target not found: {}", target.display()),
            )
        } else {
            err
        }
    })?;

    let is_dir = metadata.is_dir();

    if expect_dir && !is_dir {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Target is a file. Remove `--dir` to delete files: {}",
                target.display()
            ),
        ));
    }

    if !expect_dir && is_dir {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Target is a directory. Use `--dir` to delete directories: {}",
                target.display()
            ),
        ));
    }

    Ok(metadata)
}

fn execute_deletion(target: &Path, is_dir: bool) -> Result<(), io::Error> {
    if is_dir {
        fs::remove_dir(target).map_err(|e| {
            if e.kind() == io::ErrorKind::DirectoryNotEmpty {
                io::Error::new(
                    io::ErrorKind::DirectoryNotEmpty,
                    format!("Directory not empty: {}", target.display()),
                )
            } else {
                e
            }
        })?;
    } else {
        fs::remove_file(target)?;
    }

    Ok(())
}
