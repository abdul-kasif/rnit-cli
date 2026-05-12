use std::{fs, path::Path};

use crate::domain::fs::{
    FileEntry, FsError, build_file_entry, ensure_path_not_exists, extract_filename,
    validate_entry_name, validate_parent_exists,
};

pub fn create_entry<P: AsRef<Path>>(path: P, is_dir: bool) -> Result<FileEntry, FsError> {
    let target = path.as_ref();

    let name = extract_filename(target)?;

    validate_entry_name(&name)?;

    validate_parent_exists(target)?;

    let label = if is_dir { "Directory" } else { "File" };

    ensure_path_not_exists(target, label)?;

    execute_creation(target, label, is_dir)?;

    let metadata = fs::metadata(target)?;

    Ok(build_file_entry(name, &metadata))
}

fn execute_creation(path: &Path, label: &str, is_dir: bool) -> Result<(), FsError> {
    let err_map = |e: std::io::Error| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            FsError::AlreadyExists {
                path: path.display().to_string(),
                label: label.to_string(),
            }
        } else {
            FsError::Io(e)
        }
    };

    if is_dir {
        fs::create_dir(path).map_err(err_map)?;
    } else {
        fs::File::create_new(path).map_err(err_map)?;
    }

    Ok(())
}
