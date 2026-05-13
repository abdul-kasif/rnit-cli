use std::{fs, path::Path};

use crate::domain::fs::{FileEntry, FsError, build_file_entry};

pub fn list_current_dir<P: AsRef<Path>>(
    path: P,
    include_hidden: bool,
) -> Result<Vec<FileEntry>, FsError> {
    let entries = fs::read_dir(path)?;
    let mut file_list: Vec<FileEntry> = Vec::new();

    for entry_result in entries {
        let entry = entry_result?;

        let name = extract_dir_entry_name(&entry)?;

        if !include_hidden && is_hidden_filename(&name) {
            continue;
        }

        let metadata = entry.metadata()?;
        file_list.push(build_file_entry(name, &metadata));
    }

    Ok(file_list)
}

fn extract_dir_entry_name(entry: &fs::DirEntry) -> Result<String, FsError> {
    entry
        .file_name()
        .into_string()
        .map_err(|_| FsError::InvalidName {
            reason: "Filename contains invalid UTF-8".to_string(),
        })
}

fn is_hidden_filename(name: &str) -> bool {
    name.starts_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use tempfile::TempDir;

    fn entry_names(entries: &[FileEntry]) -> HashSet<&str> {
        entries.iter().map(|e| e.name.as_str()).collect()
    }

    #[test]
    fn test_list_empty_directory() {
        let dir = TempDir::new().unwrap();
        let entries = list_current_dir(dir.path(), false).unwrap();
        assert!(
            entries.is_empty(),
            "Empty directory should yield zero entries"
        );
    }

    #[test]
    fn test_list_excludes_hidden_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("visible.txt"), "data").unwrap();
        fs::write(dir.path().join(".hidden"), "data").unwrap();
        fs::create_dir(dir.path().join(".hidden_dir")).unwrap();

        let entries = list_current_dir(dir.path(), false).unwrap();
        let names = entry_names(&entries);

        assert!(names.contains("visible.txt"));
        assert!(!names.contains(".hidden"));
        assert!(!names.contains(".hidden_dir"));
        assert_eq!(entries.len(), 1, "Should only return non-hidden entries");
    }

    #[test]
    fn test_list_includes_hidden_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("visible.txt"), "data").unwrap();
        fs::write(dir.path().join(".config.yml"), "cfg").unwrap();
        fs::create_dir(dir.path().join(".git")).unwrap();

        let entries = list_current_dir(dir.path(), true).unwrap();
        let names = entry_names(&entries);

        assert!(names.contains("visible.txt"));
        assert!(names.contains(".config.yml"));
        assert!(names.contains(".git"));
        assert_eq!(
            entries.len(),
            3,
            "Should return all entries when flag is true"
        );
    }

    #[test]
    fn test_list_nonexistent_path_maps_to_not_found() {
        let err = list_current_dir("/tmp/nonexistent_dir_test_123", false).unwrap_err();
        assert!(
            matches!(err, FsError::Io(ref e) if e.kind() == std::io::ErrorKind::NotFound),
            "Expected NotFound Io error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_list_file_path_maps_to_not_a_directory() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("not_a_dir.txt");
        fs::write(&file_path, "content").unwrap();

        let err = list_current_dir(&file_path, false).unwrap_err();
        assert!(
            matches!(err, FsError::Io(ref e) if e.kind() == std::io::ErrorKind::NotADirectory),
            "Expected NotADirectory Io error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_list_preserves_file_type_and_metadata() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("file.log"), "12345").unwrap();
        fs::create_dir(dir.path().join("logs")).unwrap();

        let entries = list_current_dir(dir.path(), true).unwrap();
        let file = entries.iter().find(|e| e.name == "file.log").unwrap();
        let dir_entry = entries.iter().find(|e| e.name == "logs").unwrap();

        assert_eq!(file.size, 5);
        assert!(!file.is_dir);
        assert!(dir_entry.is_dir);
    }

    #[test]
    fn test_is_hidden_filename_comprehensive() {
        assert!(is_hidden_filename(".git"));
        assert!(is_hidden_filename("."));
        assert!(is_hidden_filename(".."));
        assert!(is_hidden_filename("..."));
        assert!(is_hidden_filename(".env.local"));
        assert!(is_hidden_filename(".hidden"));

        assert!(!is_hidden_filename("readme.md"));
        assert!(!is_hidden_filename("file.txt"));
        assert!(!is_hidden_filename("a.b.c"));
        assert!(!is_hidden_filename("normal"));
        assert!(!is_hidden_filename(""));
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_dir_entry_name_handles_invalid_utf8() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let dir = TempDir::new().unwrap();
        let invalid_name = OsString::from_vec(vec![0x66, 0x69, 0x6c, 0x65, 0xff]);
        let invalid_path = dir.path().join(invalid_name);
        fs::File::create(&invalid_path).unwrap();

        let err = list_current_dir(dir.path(), false).unwrap_err();
        assert!(
            matches!(err, FsError::InvalidName { ref reason } if reason == "Filename contains invalid UTF-8"),
            "Expected InvalidName for non-UTF-8 filenames, got: {:?}",
            err
        );
    }
}
