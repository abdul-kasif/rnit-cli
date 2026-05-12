use std::{fs, io, path::Path};

use crate::domain::fs::{
    FileEntry, FsError, build_file_entry, ensure_path_exists, ensure_path_not_exists,
    extract_filename, validate_entry_name, validate_parent_exists,
};

pub fn rename_entry<P: AsRef<Path>>(source: P, destination: P) -> Result<FileEntry, FsError> {
    let src = source.as_ref();
    let dest = destination.as_ref();

    if src == dest {
        return Err(FsError::InvalidName {
            reason: "Source and destination paths are identical".into(),
        });
    }

    let src_metadata = ensure_path_exists(src, "Source")?;

    let dest_name = extract_filename(dest)?;

    validate_entry_name(&dest_name)?;

    validate_parent_exists(dest)?;

    ensure_path_not_exists(dest, "Destination")?;

    execute_rename(src, dest)?;

    Ok(build_file_entry(dest_name, &src_metadata))
}

fn execute_rename(src: &Path, dest: &Path) -> Result<(), FsError> {
    fs::rename(src, dest).map_err(|err| match err.kind() {
        io::ErrorKind::CrossesDevices => FsError::CrossesFilesystem {
            src: src.display().to_string(),
            dest: dest.display().to_string(),
        },
        _ => FsError::Io(err),
    })
}

