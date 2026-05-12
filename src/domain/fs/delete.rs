use crate::domain::fs::{
    FileEntry, FsError, build_file_entry, ensure_path_exists, extract_filename, validate_entry_name,
};
use std::{fs, path::Path};

pub fn delete_entry<P: AsRef<Path>>(path: P, expect_dir: bool) -> Result<FileEntry, FsError> {
    let target = path.as_ref();

    let name = extract_filename(target)?;

    validate_entry_name(&name)?;

    let metadata = validate_deletion_target(target, expect_dir)?;

    execute_deletion(target, metadata.is_dir())?;

    Ok(build_file_entry(name, &metadata))
}

fn validate_deletion_target(target: &Path, expect_dir: bool) -> Result<fs::Metadata, FsError> {
    let metadata = ensure_path_exists(target, "Target")?;
    let is_dir = metadata.is_dir();

    match (expect_dir, is_dir) {
        (true, false) => Err(FsError::TypeMismatch {
            message: format!(
                "Target is a file. Remove `--dir` to delete files: {}",
                target.display()
            ),
        }),
        (false, true) => Err(FsError::TypeMismatch {
            message: format!(
                "Target is a directory. Use `--dir` to delete directories: {}",
                target.display()
            ),
        }),
        _ => Ok(metadata),
    }
}

fn execute_deletion(target: &Path, is_dir: bool) -> Result<(), FsError> {
    if is_dir {
        fs::remove_dir(target).map_err(|e| {
            if e.kind() == std::io::ErrorKind::DirectoryNotEmpty {
                FsError::DirectoryNotEmpty {
                    path: target.display().to_string(),
                }
            } else {
                FsError::Io(e)
            }
        })?;
    } else {
        fs::remove_file(target)?;
    }
    Ok(())
}

