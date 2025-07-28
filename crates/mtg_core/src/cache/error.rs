use std::path::PathBuf;

/// Cache-related errors
#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Cache corrupted at path {path}: {reason}")]
    Corrupted { path: PathBuf, reason: String },

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Invalid cache configuration: {0}")]
    InvalidConfiguration(String),
}

pub type Result<T> = std::result::Result<T, CacheError>;
