pub mod cache;
pub mod gatherer;
pub mod scryfall;

// Re-export the ScryfallClient for easy access from the binary
pub use scryfall::{ScryfallClient, ScryfallClientBuilder, ScryfallClientConfig};

// Re-export the GathererClient for easy access from the binary
pub use gatherer::{
    Card as GathererCard, GathererClient, SearchParams as GathererSearchParams,
    SearchResponse as GathererSearchResponse,
};

// Re-export cache types for easy access
pub use cache::{CacheStore, DiskCache, DiskCacheBuilder};
