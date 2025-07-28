pub mod cache;
pub mod scryfall;

// Re-export the ScryfallClient for easy access from the binary
pub use scryfall::{ScryfallClient, ScryfallClientBuilder, ScryfallClientConfig};

// Re-export cache types for easy access
pub use cache::{CacheStore, DiskCache, DiskCacheBuilder};
