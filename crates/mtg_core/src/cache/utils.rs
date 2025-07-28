use crate::cache::error::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Get the default cache directory path
pub fn default_cache_path() -> PathBuf {
    dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .or_else(|| {
            dirs::home_dir().map(|home| {
                #[cfg(windows)]
                return home.join("AppData").join("Local");

                #[cfg(not(windows))]
                return home.join(".local").join("share");
            })
        })
        .unwrap_or_else(|| {
            // Last resort fallback
            #[cfg(windows)]
            return PathBuf::from(r"C:\Users\Default\AppData\Local");

            #[cfg(not(windows))]
            return PathBuf::from("/tmp");
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache_path_cross_platform() {
        let cache_path = default_cache_path();

        // Should always end with mtg/cache
        assert!(cache_path.ends_with("mtg/cache") || cache_path.ends_with(r"mtg\cache"));

        // Should be an absolute path
        assert!(cache_path.is_absolute());

        // Should not contain Unix-specific paths on Windows or vice versa
        let path_str = cache_path.to_string_lossy();

        #[cfg(windows)]
        {
            // On Windows, should not contain Unix-style paths
            assert!(!path_str.contains("/.local/share"));
            assert!(!path_str.contains("/tmp"));
        }

        #[cfg(not(windows))]
        {
            // On Unix-like systems, should not contain Windows-style paths
            assert!(!path_str.contains(r"C:\"));
            assert!(!path_str.contains("AppData"));
        }
    }

    #[test]
    fn test_key_to_path_cross_platform() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let path1 = key_to_path(base_path, Some("test"), "my-key");
        let path2 = key_to_path(base_path, None, "my-key");

        // Both paths should be under the base path
        assert!(path1.starts_with(base_path));
        assert!(path2.starts_with(base_path));

        // Path with prefix should contain the prefix
        assert!(path1.to_string_lossy().contains("test"));

        // Both should end with .cache extension
        assert_eq!(path1.extension().unwrap(), "cache");
        assert_eq!(path2.extension().unwrap(), "cache");

        // Should use proper path separators for the platform
        let path_str = path1.to_string_lossy();
        #[cfg(windows)]
        {
            // On Windows, should use backslashes
            if path_str.len() > base_path.to_string_lossy().len() {
                assert!(path_str.contains('\\'));
            }
        }

        #[cfg(not(windows))]
        {
            // On Unix-like systems, should use forward slashes
            if path_str.len() > base_path.to_string_lossy().len() {
                assert!(path_str.contains('/'));
            }
        }
    }

    #[test]
    fn test_hash_key_consistency() {
        let key = "test-key-123";
        let hash1 = hash_key(key);
        let hash2 = hash_key(key);

        // Same key should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be hex string
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));

        // Different keys should produce different hashes
        let hash3 = hash_key("different-key");
        assert_ne!(hash1, hash3);
    }
}
