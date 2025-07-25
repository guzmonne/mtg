use crate::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Cache-related errors
#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Cache serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Cache IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct CacheManager {
    cache_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedResponse {
    pub data: serde_json::Value,
    pub timestamp: u64,
}

impl CacheManager {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;
        Ok(Self { cache_dir })
    }

    fn get_cache_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| {
            crate::error::Error::Generic("Could not find home directory".to_string())
        })?;

        let cache_dir = home.join(".local").join("share").join("mtg").join("cache");

        Self::ensure_cache_dir(&cache_dir)?;
        Ok(cache_dir)
    }

    fn ensure_cache_dir(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn hash_request<T: serde::Serialize>(request: &T) -> String {
        let serialized = serde_json::to_string(request).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn hash_gatherer_search_request(
        url: &str,
        payload: &serde_json::Value,
        headers: &[(String, String)],
    ) -> String {
        let mut hasher = Sha256::new();

        // Hash the URL
        hasher.update(url.as_bytes());

        // Hash the payload
        hasher.update(payload.to_string().as_bytes());

        // Hash relevant headers (excluding dynamic ones like timestamps)
        let relevant_headers: Vec<_> = headers
            .iter()
            .filter(|(k, _)| !matches!(k.as_str(), "date" | "x-request-id" | "x-trace-id"))
            .collect();

        for (key, value) in relevant_headers {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }

    pub fn get_cache_path(&self, hash: &str) -> PathBuf {
        self.cache_dir.join(format!("{hash}.json"))
    }

    pub async fn get(&self, hash: &str) -> Result<Option<CachedResponse>> {
        let cache_path = self.get_cache_path(hash);

        if !cache_path.exists() {
            return Ok(None);
        }

        let contents = tokio::fs::read_to_string(&cache_path).await?;
        let cached: CachedResponse = serde_json::from_str(&contents)?;

        Ok(Some(cached))
    }

    pub async fn set(&self, hash: &str, data: serde_json::Value) -> Result<()> {
        let cache_path = self.get_cache_path(hash);

        let cached = CachedResponse {
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let contents = serde_json::to_string_pretty(&cached)?;
        tokio::fs::write(&cache_path, contents).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    impl CacheManager {
        pub fn with_dir(cache_dir: PathBuf) -> Result<Self> {
            Self::ensure_cache_dir(&cache_dir)?;
            Ok(Self { cache_dir })
        }

        pub async fn clear(&self) -> Result<()> {
            let entries = fs::read_dir(&self.cache_dir)?;

            for entry in entries {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    tokio::fs::remove_file(entry.path()).await?;
                }
            }

            Ok(())
        }

        pub fn list_cached_files(&self) -> Result<Vec<String>> {
            let mut files = Vec::new();
            let entries = fs::read_dir(&self.cache_dir)?;

            for entry in entries {
                let entry = entry?;
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        files.push(name.to_string());
                    }
                }
            }

            Ok(files)
        }
    }

    #[tokio::test]
    async fn test_cache_manager() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cache = CacheManager::with_dir(temp_dir.path().to_path_buf())?;

        let test_data = serde_json::json!({
            "test": "data",
            "number": 42
        });

        let hash = "test_hash";

        // Test set
        cache.set(hash, test_data.clone()).await?;

        // Test get
        let retrieved = cache.get(hash).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, test_data);

        // Test list
        let files = cache.list_cached_files()?;
        assert_eq!(files.len(), 1);
        assert!(files[0].contains("test_hash"));

        // Test clear
        cache.clear().await?;
        let files = cache.list_cached_files()?;
        assert_eq!(files.len(), 0);

        Ok(())
    }
}
