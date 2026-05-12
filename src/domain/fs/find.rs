use globset::GlobBuilder;
use std::io;

use crate::domain::fs::{FileEntry, list_current_dir};

pub fn find_current_dir(
    include_hidden: bool,
    name_filter: Option<&str>,
) -> Result<Vec<FileEntry>, io::Error> {
    let mut entries = list_current_dir(include_hidden)?;

    let Some(pattern) = name_filter else {
        return Ok(entries);
    };

    let matcher = GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid glob pattern: {}", e),
            )
        })?
        .compile_matcher();

    entries.retain(|e| matcher.is_match(&e.name));

    Ok(entries)
}

