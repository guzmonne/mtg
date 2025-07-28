use crate::cache::error::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Get the default cache directory path
pub fn default_cache_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("mtg")
        .join("cache")
}

/// Hash a key to create a filename
pub fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Create a file path from a key with optional prefix and subdirectory structure
pub fn key_to_path(base_path: &Path, prefix: Option<&str>, key: &str) -> PathBuf {
    let hash = hash_key(key);
    let mut path = base_path.to_path_buf();

    // Add prefix if provided
    if let Some(prefix) = prefix {
        path = path.join(prefix);
    }

    // Use first 2 chars of hash for subdirectory (git-like structure)
    path = path.join(&hash[..2]);
    path.join(format!("{}.cache", &hash[2..]))
}

/// Ensure a directory exists, creating it if necessary
pub async fn ensure_dir_exists(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

/// Calculate the total size of a directory recursively
pub async fn calculate_dir_size(path: &Path) -> Result<u64> {
    if !path.exists() {
        return Ok(0);
    }

    let mut total_size = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(current_path) = stack.pop() {
        let mut entries = fs::read_dir(&current_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                stack.push(entry_path);
            } else {
                total_size += metadata.len();
            }
        }
    }

    Ok(total_size)
}

/// Count cache files in a directory recursively
pub async fn count_cache_files(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }

    let mut count = 0;
    let mut stack = vec![path.to_path_buf()];

    while let Some(current_path) = stack.pop() {
        let mut entries = fs::read_dir(&current_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                stack.push(entry_path);
            } else if entry_path.extension().and_then(|s| s.to_str()) == Some("cache") {
                count += 1;
            }
        }
    }

    Ok(count)
}
