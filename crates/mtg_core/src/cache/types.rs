use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Cache entry wrapper that includes metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub value: T,
    pub created_at: SystemTime,
    pub metadata: HashMap<String, String>,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            created_at: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(value: T, metadata: HashMap<String, String>) -> Self {
        Self {
            value,
            created_at: SystemTime::now(),
            metadata,
        }
    }
}

/// Report generated after cleaning cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanReport {
    pub prefix: String,
    pub removed_count: usize,
    pub freed_bytes: u64,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_files: usize,
    pub total_size: u64,
    pub prefixes: Vec<String>,
}

/// Metadata about a cache entry for cleaning operations
#[derive(Debug, Clone)]
pub struct CacheEntryMetadata {
    pub key: String,
    pub created_at: SystemTime,
    pub size_bytes: u64,
    pub metadata: HashMap<String, String>,
}
