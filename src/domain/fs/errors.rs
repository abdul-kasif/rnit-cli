use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("{label} not found: {path}")]
    NotFound { label: String, path: String },

    #[error("{label} already exists: {path}")]
    AlreadyExists { label: String, path: String },

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

impl PartialEq for FsError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::NotFound {
                    label: l1,
                    path: p1,
                },
                Self::NotFound {
                    label: l2,
                    path: p2,
                },
            ) => l1 == l2 && p1 == p2,

            (
                Self::AlreadyExists {
                    label: l1,
                    path: p1,
                },
                Self::AlreadyExists {
                    label: l2,
                    path: p2,
                },
            ) => l1 == l2 && p1 == p2,

            (Self::InvalidName { reason: r1 }, Self::InvalidName { reason: r2 }) => r1 == r2,

            (Self::ParentNotFound { path: p1 }, Self::ParentNotFound { path: p2 }) => p1 == p2,

            (Self::TypeMismatch { message: m1 }, Self::TypeMismatch { message: m2 }) => m1 == m2,

            (Self::DirectoryNotEmpty { path: p1 }, Self::DirectoryNotEmpty { path: p2 }) => {
                p1 == p2
            }

            (
                Self::CrossesFilesystem { src: s1, dest: d1 },
                Self::CrossesFilesystem { src: s2, dest: d2 },
            ) => s1 == s2 && d1 == d2,

            (
                Self::InvalidGlob {
                    pattern: p1,
                    reason: r1,
                },
                Self::InvalidGlob {
                    pattern: p2,
                    reason: r2,
                },
            ) => p1 == p2 && r1 == r2,

            (Self::Io(e1), Self::Io(e2)) => e1.kind() == e2.kind(),
            _ => false,
        }
    }
}
