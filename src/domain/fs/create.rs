use std::{fs, io, path::Path};

use crate::domain::fs::{
    FileEntry, build_file_entry, ensure_path_not_exists, extract_filename, validate_entry_name,
    validate_parent_exists,
};

pub fn create_entry<P: AsRef<Path>>(path: P, is_dir: bool) -> Result<FileEntry, io::Error> {
    let target = path.as_ref();

    let name = extract_filename(target)?;

    validate_entry_name(&name)?;

    validate_parent_exists(target)?;

    let label = if is_dir { "Directory" } else { "File" };

    ensure_path_not_exists(target, label)?;

    execute_creation(target, is_dir)?;

    let metadata = fs::metadata(target)?;

    Ok(build_file_entry(name, &metadata))
}

fn execute_creation(path: &Path, is_dir: bool) -> Result<(), io::Error> {
    if is_dir {
        fs::create_dir(path)?;
    } else {
        fs::File::create_new(path)?;
    }

    Ok(())
}
