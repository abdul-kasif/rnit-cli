use std::{fs, io, path::Path};

use crate::domain::fs::{FileEntry, FsError};

pub fn build_file_entry(name: String, metadata: &fs::Metadata) -> FileEntry {
    FileEntry {
        name,
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    }
}

pub fn ensure_path_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<fs::Metadata, FsError> {
    let path = path.as_ref();
    fs::metadata(path).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            FsError::NotFound {
                path: path.display().to_string(),
                label: label.to_string(),
            }
        } else {
            FsError::Io(err)
        }
    })
}

pub fn ensure_path_not_exists<P: AsRef<Path>>(path: P, label: &str) -> Result<(), FsError> {
    let path = path.as_ref();

    match fs::metadata(path) {
        Ok(_) => Err(FsError::AlreadyExists {
            path: path.display().to_string(),
            label: label.to_string(),
        }),

        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),

        Err(err) => Err(FsError::Io(err)),
    }
}

pub fn extract_filename<P: AsRef<Path>>(path: P) -> Result<String, FsError> {
    let path_ref = path.as_ref();

    if path_ref
        .to_str()
        .is_some_and(|s| s.ends_with("/") || s.ends_with("\\"))
    {
        return Err(FsError::InvalidName {
            reason: "Path ends with a directory seperator".to_string(),
        });
    }

    path_ref
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| FsError::InvalidName {
            reason: "Path contains invalid UTF-8 or has no filename component".to_string(),
        })
}

pub fn validate_entry_name(name: &str) -> Result<(), FsError> {
    if name.trim().is_empty() {
        return Err(FsError::InvalidName {
            reason: "Entry name cannot be empty".to_string(),
        });
    }
    if name == "." || name == ".." {
        return Err(FsError::InvalidName {
            reason: format!("Entry name '{}' is reserved", name),
        });
    }
    if name.contains('\0') || name.contains('/') || name.contains('\\') {
        return Err(FsError::InvalidName {
            reason: "Entry name contains invalid characters (\\0, /, \\)".to_string(),
        });
    }
    if name.len() > 255 {
        return Err(FsError::InvalidName {
            reason: "Entry name exceeds maximum length (255 bytes)".to_string(),
        });
    }
    Ok(())
}

pub fn validate_parent_exists<P: AsRef<Path>>(path: P) -> Result<(), FsError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        return Err(FsError::ParentNotFound {
            path: parent.display().to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::path::PathBuf;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_build_file_entry_regular_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let metadata = fs::metadata(temp_file.path()).unwrap();
        let entry = build_file_entry("test.txt".to_string(), &metadata);

        assert_eq!(entry.name, "test.txt");
        assert_eq!(entry.size, 0);
        assert!(!entry.is_dir);
    }

    #[test]
    fn test_build_file_entry_directory() {
        let temp_dir = TempDir::new().unwrap();
        let metadata = fs::metadata(temp_dir.path()).unwrap();
        let entry = build_file_entry("my_dir".to_string(), &metadata);

        assert_eq!(entry.name, "my_dir");
        assert!(entry.is_dir);
    }

    #[test]
    fn test_build_file_entry_preserves_custom_metadata() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), b"hello world").unwrap();
        let metadata = fs::metadata(temp_file.path()).unwrap();
        let entry = build_file_entry("data.bin".to_string(), &metadata);

        assert_eq!(entry.name, "data.bin");
        assert_eq!(entry.size, 11);
        assert!(!entry.is_dir);
    }

    #[test]
    fn test_ensure_path_exists_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let result = ensure_path_exists(temp_file.path(), "existing file");
        assert!(result.is_ok());
    }

    #[test]
    fn test_ensure_path_exists_not_found() {
        let path = PathBuf::from("/nonexistent/deep/path/file.txt");
        let err = ensure_path_exists(&path, "missing file").unwrap_err();

        assert_eq!(
            err,
            FsError::NotFound {
                label: "missing file".to_string(),
                path: path.display().to_string()
            }
        );
    }

    #[test]
    fn test_ensure_path_not_exists_success() {
        let path = PathBuf::from("/nonexistent/deep/path/file.txt");
        assert!(ensure_path_not_exists(&path, "new file").is_ok());
    }

    #[test]
    fn test_ensure_path_not_exists_already_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let err = ensure_path_not_exists(temp_file.path(), "duplicate").unwrap_err();

        assert_eq!(
            err,
            FsError::AlreadyExists {
                label: "duplicate".to_string(),
                path: temp_file.path().display().to_string()
            }
        );
    }

    #[test]
    fn test_extract_filename_success_absolute_and_relative() {
        assert_eq!(extract_filename("/usr/local/bin/tool").unwrap(), "tool");
        assert_eq!(
            extract_filename("./relative/path/config.yml").unwrap(),
            "config.yml"
        );
    }

    #[test]
    fn test_extract_filename_no_filename_component() {
        assert!(extract_filename("/path/to/directory/").is_err());
        assert!(extract_filename("/").is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_extract_filename_invalid_utf8() {
        let invalid_bytes = vec![0x66, 0x69, 0x6c, 0x65, 0xff];
        let os_str = OsString::from_vec(invalid_bytes);
        let path = PathBuf::from(os_str);

        let err = extract_filename(&path).unwrap_err();
        assert_eq!(
            err,
            FsError::InvalidName {
                reason: "Path contains invalid UTF-8 or has no filename component".to_string()
            }
        );
    }

    #[test]
    fn test_validate_entry_name_valid() {
        assert!(validate_entry_name("normal_name").is_ok());
        assert!(validate_entry_name("archive.tar.gz").is_ok());
        assert!(validate_entry_name(&"a".repeat(255)).is_ok()); // Max length
    }

    #[test]
    fn test_validate_entry_name_empty_or_whitespace() {
        let err_empty = validate_entry_name("").unwrap_err();
        let err_ws = validate_entry_name("   \t\n").unwrap_err();

        let expected = FsError::InvalidName {
            reason: "Entry name cannot be empty".to_string(),
        };
        assert_eq!(err_empty, expected);
        assert_eq!(err_ws, expected);
    }

    #[test]
    fn test_validate_entry_name_reserved() {
        let err_dot = validate_entry_name(".").unwrap_err();
        let err_dotdot = validate_entry_name("..").unwrap_err();

        assert!(err_dot.to_string().contains("'.' is reserved"));
        assert!(err_dotdot.to_string().contains("'..' is reserved"));
    }

    #[test]
    fn test_validate_entry_name_invalid_characters() {
        for (char, desc) in [('\0', "null"), ('/', "slash"), ('\\', "backslash")] {
            let name = format!("bad{}name", char);
            let err = validate_entry_name(&name).unwrap_err();
            assert!(
                err.to_string().contains("invalid characters"),
                "Failed for char {}",
                desc
            );
        }
    }

    #[test]
    fn test_validate_entry_name_exceeds_max_length() {
        let long_name = "a".repeat(256);
        let err = validate_entry_name(&long_name).unwrap_err();
        assert!(err.to_string().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_parent_exists_success_when_parent_exists() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("new_file.txt");
        assert!(validate_parent_exists(&path).is_ok());
    }

    #[test]
    fn test_validate_parent_exists_fails_when_parent_missing() {
        let temp_dir = TempDir::new().unwrap();
        let missing_parent = temp_dir.path().join("nonexistent_subdir");
        let path = missing_parent.join("file.txt");

        let err = validate_parent_exists(&path).unwrap_err();
        assert_eq!(
            err,
            FsError::ParentNotFound {
                path: missing_parent.display().to_string()
            }
        );
    }

    #[test]
    fn test_validate_parent_exists_skips_check_for_empty_parent() {
        assert!(validate_parent_exists("standalone.txt").is_ok());
    }

    #[test]
    fn test_validate_parent_exists_skips_check_for_root() {
        assert!(validate_parent_exists("/").is_ok());
    }

    #[test]
    fn test_validate_parent_exists_current_dir_reference() {
        assert!(validate_parent_exists(".").is_ok());
    }
}
