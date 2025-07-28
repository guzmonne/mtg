pub mod client;
pub mod search;
pub mod sets;
pub mod smart;
pub mod types;

use serde::{Deserialize, Serialize};

// Re-export the client for convenience
pub use client::{ScryfallClient, ScryfallClientBuilder, ScryfallClientConfig};

// Re-export types for convenience
pub use types::{
    AdvancedSearchParams, AutocompleteResponse, Card, QueryIntent, QueryIssue, ScryfallError,
    SearchParams, SearchResponse, SmartSearchResult,
};

/// Generic list object for Scryfall API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
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
