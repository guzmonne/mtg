use crate::cache::{
    error::{CacheError, Result},
    serializer::Serializer,
    types::{CacheEntry, CacheStats, CleanReport},
    utils::{calculate_dir_size, count_cache_files, ensure_dir_exists, key_to_path},
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};
use tokio::fs;

/// HashMap-like cache trait
#[allow(async_fn_in_trait)]
pub trait CacheStore<K, V>: Send + Sync
where
    K: AsRef<str>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    async fn get(&self, key: K) -> Result<Option<V>>;
    async fn insert(&self, key: K, value: V) -> Result<Option<V>>;
    async fn remove(&self, key: K) -> Result<Option<V>>;
    async fn contains_key(&self, key: K) -> Result<bool>;
    async fn clear(&self) -> Result<()>;
    async fn len(&self) -> Result<usize>;
    async fn is_empty(&self) -> Result<bool>;
    async fn keys(&self) -> Result<Vec<String>>;
}

/// Disk-based cache implementation
#[derive(Debug, Clone)]
pub struct DiskCache {
    base_path: PathBuf,
    prefix: Option<String>,
    serializer: Serializer,
}

impl DiskCache {
    /// Create a new DiskCache with the given configuration
    pub fn new(base_path: PathBuf, prefix: Option<String>, serializer: Serializer) -> Result<Self> {
        let cache = Self {
            base_path,
            prefix,
            serializer,
        };

        // Ensure base directory exists synchronously
        std::fs::create_dir_all(&cache.base_path)?;

        Ok(cache)
    }

    /// Get the file path for a given key
    fn get_file_path(&self, key: &str) -> PathBuf {
        key_to_path(&self.base_path, self.prefix.as_deref(), key)
    }

    /// Read a cache entry from disk
    async fn read_entry<V>(&self, key: &str) -> Result<Option<CacheEntry<V>>>
    where
        V: for<'de> Deserialize<'de>,
    {
        let file_path = self.get_file_path(key);

        if !file_path.exists() {
            return Ok(None);
        }

        let data = fs::read(&file_path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                return CacheError::KeyNotFound(key.to_string());
            }
            CacheError::Io(e)
        })?;

        let entry = self
            .serializer
            .deserialize::<CacheEntry<V>>(&data)
            .map_err(|_| CacheError::Corrupted {
                path: file_path.clone(),
                reason: "Failed to deserialize cache entry".to_string(),
            })?;

        Ok(Some(entry))
    }

    /// Write a cache entry to disk
    async fn write_entry<V>(&self, key: &str, entry: &CacheEntry<V>) -> Result<()>
    where
        V: Serialize,
    {
        let file_path = self.get_file_path(key);

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            ensure_dir_exists(&parent.to_path_buf()).await?;
        }

        let data = self.serializer.serialize(entry)?;
        fs::write(&file_path, data).await?;

        Ok(())
    }

    /// Clean cache by prefix
    pub async fn clean_prefix(&self, prefix: &str) -> Result<CleanReport> {
        let prefix_path = self.base_path.join(prefix);
        let mut removed_count = 0;
        let mut freed_bytes = 0u64;

        if prefix_path.exists() {
            freed_bytes = calculate_dir_size(&prefix_path).await?;
            removed_count = count_cache_files(&prefix_path).await?;
            fs::remove_dir_all(&prefix_path).await?;
        }

        Ok(CleanReport {
            prefix: prefix.to_string(),
            removed_count,
            freed_bytes,
        })
    }

    /// Clean entire cache
    pub async fn clean_all(&self) -> Result<CleanReport> {
        let mut total_removed = 0;
        let mut total_freed = 0u64;

        // Get all prefixes
        let prefixes = self.list_prefixes().await?;

        for prefix in prefixes {
            let report = self.clean_prefix(&prefix).await?;
            total_removed += report.removed_count;
            total_freed += report.freed_bytes;
        }

        Ok(CleanReport {
            prefix: "all".to_string(),
            removed_count: total_removed,
            freed_bytes: total_freed,
        })
    }

    /// List all prefixes in use
    pub async fn list_prefixes(&self) -> Result<Vec<String>> {
        let mut prefixes = Vec::new();

        if !self.base_path.exists() {
            return Ok(prefixes);
        }

        let mut entries = fs::read_dir(&self.base_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    prefixes.push(name.to_string());
                }
            }
        }

        Ok(prefixes)
    }

    /// Get cache statistics
    pub async fn stats(&self, prefix: Option<&str>) -> Result<CacheStats> {
        let path = match prefix {
            Some(p) => self.base_path.join(p),
            None => self.base_path.clone(),
        };

        Ok(CacheStats {
            total_files: count_cache_files(&path).await?,
            total_size: calculate_dir_size(&path).await?,
            prefixes: self.list_prefixes().await?,
        })
    }

    /// Clean entries older than the specified duration
    pub async fn clean_older_than(
        &self,
        age: Duration,
        prefix: Option<&str>,
    ) -> Result<CleanReport> {
        let cutoff = std::time::SystemTime::now() - age;
        let mut removed_count = 0;
        let mut freed_bytes = 0u64;

        let path = match prefix {
            Some(p) => self.base_path.join(p),
            None => self.base_path.clone(),
        };

        if !path.exists() {
            return Ok(CleanReport {
                prefix: prefix.unwrap_or("all").to_string(),
                removed_count: 0,
                freed_bytes: 0,
            });
        }

        // Walk directory and remove old files
        let mut stack = vec![path];

        while let Some(current_path) = stack.pop() {
            let mut entries = fs::read_dir(&current_path).await?;

            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    stack.push(entry_path);
                } else if entry_path.extension().and_then(|s| s.to_str()) == Some("cache") {
                    if let Ok(created) = metadata.created() {
                        if created < cutoff {
                            freed_bytes += metadata.len();
                            removed_count += 1;
                            fs::remove_file(&entry_path).await?;
                        }
                    }
                }
            }
        }

        Ok(CleanReport {
            prefix: prefix.unwrap_or("all").to_string(),
            removed_count,
            freed_bytes,
        })
    }

    /// Clean cache to stay under size limit
    pub async fn clean_to_size_limit(
        &self,
        max_bytes: u64,
        prefix: Option<&str>,
    ) -> Result<CleanReport> {
        let path = match prefix {
            Some(p) => self.base_path.join(p),
            None => self.base_path.clone(),
        };

        let current_size = calculate_dir_size(&path).await?;
        if current_size <= max_bytes {
            return Ok(CleanReport {
                prefix: prefix.unwrap_or("all").to_string(),
                removed_count: 0,
                freed_bytes: 0,
            });
        }

        // Collect all cache files with their metadata
        let mut files = Vec::new();
        let mut stack = vec![path];

        while let Some(current_path) = stack.pop() {
            let mut entries = fs::read_dir(&current_path).await?;

            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    stack.push(entry_path);
                } else if entry_path.extension().and_then(|s| s.to_str()) == Some("cache") {
                    if let Ok(created) = metadata.created() {
                        files.push((entry_path, created, metadata.len()));
                    }
                }
            }
        }

        // Sort by creation time (oldest first)
        files.sort_by_key(|(_, created, _)| *created);

        let mut removed_count = 0;
        let mut freed_bytes = 0u64;
        let mut remaining_size = current_size;

        // Remove oldest files until under limit
        for (file_path, _, size) in files {
            if remaining_size <= max_bytes {
                break;
            }

            fs::remove_file(&file_path).await?;
            removed_count += 1;
            freed_bytes += size;
            remaining_size -= size;
        }

        Ok(CleanReport {
            prefix: prefix.unwrap_or("all").to_string(),
            removed_count,
            freed_bytes,
        })
    }
}

impl<K, V> CacheStore<K, V> for DiskCache
where
    K: AsRef<str>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    async fn get(&self, key: K) -> Result<Option<V>> {
        let entry = self.read_entry::<V>(key.as_ref()).await?;
        Ok(entry.map(|e| e.value))
    }

    async fn insert(&self, key: K, value: V) -> Result<Option<V>> {
        let key_str = key.as_ref();

        // Check if key already exists
        let old_value = self.get(key_str).await?;

        // Create new entry
        let entry = CacheEntry::new(value);
        self.write_entry(key_str, &entry).await?;

        Ok(old_value)
    }

    async fn remove(&self, key: K) -> Result<Option<V>> {
        let key_str = key.as_ref();
        let old_value = self.get(key_str).await?;

        if old_value.is_some() {
            let file_path = self.get_file_path(key_str);
            if file_path.exists() {
                fs::remove_file(&file_path).await?;
            }
        }

        Ok(old_value)
    }

    async fn contains_key(&self, key: K) -> Result<bool> {
        let file_path = self.get_file_path(key.as_ref());
        Ok(file_path.exists())
    }

    async fn clear(&self) -> Result<()> {
        let path = match &self.prefix {
            Some(prefix) => self.base_path.join(prefix),
            None => self.base_path.clone(),
        };

        if path.exists() {
            fs::remove_dir_all(&path).await?;
            ensure_dir_exists(&path).await?;
        }

        Ok(())
    }

    async fn len(&self) -> Result<usize> {
        let path = match &self.prefix {
            Some(prefix) => self.base_path.join(prefix),
            None => self.base_path.clone(),
        };

        count_cache_files(&path).await
    }

    async fn is_empty(&self) -> Result<bool> {
        Ok(<Self as CacheStore<K, V>>::len(self).await? == 0)
    }

    async fn keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        let path = match &self.prefix {
            Some(prefix) => self.base_path.join(prefix),
            None => self.base_path.clone(),
        };

        if !path.exists() {
            return Ok(keys);
        }

        let mut stack = vec![path];

        while let Some(current_path) = stack.pop() {
            let mut entries = fs::read_dir(&current_path).await?;

            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    stack.push(entry_path);
                } else if entry_path.extension().and_then(|s| s.to_str()) == Some("cache") {
                    if let Some(filename) = entry_path.file_stem().and_then(|s| s.to_str()) {
                        // Note: This returns the hash, not the original key
                        // In a real implementation, you might want to store a key->hash mapping
                        keys.push(filename.to_string());
                    }
                }
            }
        }

        Ok(keys)
    }
}
