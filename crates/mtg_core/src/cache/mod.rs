//! Disk-based cache module with HashMap-like interface
//!
//! This module provides a persistent cache that stores data on disk with
//! configurable serialization, prefix-based organization, and manual cleaning.
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use mtg_core::cache::{DiskCache, CacheStore};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let cache = DiskCache::builder()
//!     .prefix("scryfall/cards")
//!     .build()?;
//!
//! // Store and retrieve data
//! cache.insert("lightning-bolt", "card data".to_string()).await?;
//! let data: Option<String> = cache.get("lightning-bolt").await?;
//! # Ok(())
//! # }
//! ```
//!
//! With custom configuration:
//! ```rust
//! use mtg_core::cache::{DiskCache, Serializer};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let temp_dir = tempfile::tempdir()?;
//! let cache = DiskCache::builder()
//!     .base_path(temp_dir.path())
//!     .prefix("gatherer/search")
//!     .with_serializer(Serializer::Bincode)
//!     .build()?;
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod disk;
pub mod error;
pub mod http;
pub mod serializer;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export main types for convenience
pub use builder::DiskCacheBuilder;
pub use disk::{CacheStore, DiskCache};
pub use error::{CacheError, Result};
pub use http::{CachedHttpClient, CachedHttpClientBuilder, CachedResponse};
pub use serializer::Serializer;
pub use types::{CacheEntry, CacheStats, CleanReport};

impl DiskCache {
    /// Create a new builder for configuring the cache
    pub fn builder() -> DiskCacheBuilder {
        DiskCacheBuilder::new()
    }
}
