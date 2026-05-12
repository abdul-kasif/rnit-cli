use crate::domain::fs::{FileEntry, FsError, list_current_dir};
use globset::GlobBuilder;

pub fn find_current_dir(
    include_hidden: bool,
    name_filter: Option<&str>,
) -> Result<Vec<FileEntry>, FsError> {
    let mut entries = list_current_dir(include_hidden)?;

    let Some(pattern) = name_filter else {
        return Ok(entries);
    };

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

