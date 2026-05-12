use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("Entry not found: {path}")]
    NotFound { path: String },

    #[error("Entry already exists: {path}")]
    AlreadyExists { path: String },

    #[error("Invalid entry name: {reason}")]
    InvalidName { reason: String },

    #[error("Parent directory does not exist: {path}")]
    ParentNotFound { path: String },

    #[error("Type mismatch: {message}")]
    TypeMismatch { message: String },

    #[error("Directory not empty: {path}")]
    DirectoryNotEmpty { path: String },

    #[error("Cannot rename across filesystems: {src} → {dest}")]
    CrossesFilesystem { src: String, dest: String },

    #[error("Invalid glob pattern: {pattern} — {reason}")]
    InvalidGlob { pattern: String, reason: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
