use globset::{Glob, GlobMatcher};
use std::io;

use crate::domain::fs::{FileEntry, list_current_dir};

pub fn find_current_dir(
    include_hidden: bool,
    name_filter: Option<&str>,
) -> Result<Vec<FileEntry>, io::Error> {
    let entries = list_current_dir(include_hidden)?;

    if let Some(pattern) = name_filter {
        let glob = Glob::new(pattern)
            .map_err(|err| io::Error::other(format!("Invalid pattern: {}", err)))?;

        let matcher: GlobMatcher = glob.compile_matcher();

        let filtered = entries
            .into_iter()
            .filter(|entry| matcher.is_match(&entry.name))
            .collect();

        Ok(filtered)
    } else {
        Ok(entries)
    }
}
