use crate::cache::{
    disk::DiskCache,
    error::{CacheError, Result},
    serializer::{default_serializer, Serializer},
    utils::default_cache_path,
};
use std::path::PathBuf;

/// Builder for configuring and creating a DiskCache
#[derive(Debug)]
pub struct DiskCacheBuilder {
    base_path: Option<PathBuf>,
    prefix: Option<String>,
    serializer: Option<Serializer>,
}

impl Default for DiskCacheBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DiskCacheBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            base_path: None,
            prefix: None,
            serializer: None,
        }
    }

    /// Set the base path for the cache directory
    pub fn base_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.base_path = Some(path.into());
        self
    }

    /// Set the prefix for organizing cache files
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set a custom serializer
    pub fn with_serializer(mut self, serializer: Serializer) -> Self {
        self.serializer = Some(serializer);
        self
    }

    /// Build the DiskCache with the configured settings
    pub fn build(self) -> Result<DiskCache> {
        let base_path = self.base_path.unwrap_or_else(default_cache_path);
        let serializer = self.serializer.unwrap_or_else(default_serializer);

        // Validate configuration
        if let Some(ref prefix) = self.prefix {
            if prefix.is_empty() {
                return Err(CacheError::InvalidConfiguration(
                    "Prefix cannot be empty".to_string(),
                ));
            }

            // Check for invalid characters in prefix
            if prefix.contains("..") || prefix.contains('\0') {
                return Err(CacheError::InvalidConfiguration(
                    "Prefix contains invalid characters".to_string(),
                ));
            }
        }

        DiskCache::new(base_path, self.prefix, serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::serializer::Serializer;
    use tempfile::TempDir;

    #[test]
    fn test_builder_default() {
        let cache = DiskCacheBuilder::new().build();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_builder_with_custom_path() {
        let temp_dir = TempDir::new().unwrap();
        let cache = DiskCacheBuilder::new().base_path(temp_dir.path()).build();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_builder_with_prefix() {
        let temp_dir = TempDir::new().unwrap();
        let cache = DiskCacheBuilder::new()
            .base_path(temp_dir.path())
            .prefix("test/prefix")
            .build();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_builder_with_serializer() {
        let temp_dir = TempDir::new().unwrap();
        let cache = DiskCacheBuilder::new()
            .base_path(temp_dir.path())
            .with_serializer(Serializer::Bincode)
            .build();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_builder_invalid_prefix() {
        let temp_dir = TempDir::new().unwrap();
        let result = DiskCacheBuilder::new()
            .base_path(temp_dir.path())
            .prefix("")
            .build();
        assert!(result.is_err());

        let result = DiskCacheBuilder::new()
            .base_path(temp_dir.path())
            .prefix("../invalid")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_chaining() {
        let temp_dir = TempDir::new().unwrap();
        let cache = DiskCacheBuilder::new()
            .base_path(temp_dir.path())
            .prefix("scryfall/cards")
            .with_serializer(Serializer::Json)
            .build();
        assert!(cache.is_ok());
    }
}
