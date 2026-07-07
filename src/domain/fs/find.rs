use std::path::{Path, PathBuf};

use crate::domain::fs::{FileEntry, FsError, list_current_dir};
use globset::GlobBuilder;

pub fn find_current_dir(
    include_hidden: bool,
    pattern: &str,
    path: Option<PathBuf>,
) -> Result<Vec<FileEntry>, FsError> {
    let target_path = path.as_deref().unwrap_or(Path::new("."));

    let mut entries = list_current_dir(target_path, include_hidden)?;

    let matcher = GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .map_err(|e| FsError::InvalidGlob {
            pattern: pattern.to_string(),
            reason: e.to_string(),
        })?
        .compile_matcher();

    entries.retain(|e| matcher.is_match(&e.name));

    Ok(entries)
}
