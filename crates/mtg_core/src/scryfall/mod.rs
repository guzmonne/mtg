pub mod client;
pub mod sets;

use serde::{Deserialize, Serialize};

// Re-export the client for convenience
pub use client::{ScryfallClient, ScryfallClientBuilder, ScryfallClientConfig};

/// Generic list object for Scryfall API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    /// Always "list"
    pub object: String,
    /// Array of requested objects
    pub data: Vec<T>,
    /// True if this List is paginated and there is a page beyond the current page
    pub has_more: bool,
    /// URI to next page if available
    pub next_page: Option<String>,
    /// Total number of cards found across all pages (for card lists)
    pub total_cards: Option<u32>,
    /// Non-fatal warnings from the API
    pub warnings: Option<Vec<String>>,
}
