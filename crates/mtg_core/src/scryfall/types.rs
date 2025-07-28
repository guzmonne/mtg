use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A Magic: The Gathering card from Scryfall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub object: String,
    pub id: String,
    pub oracle_id: Option<String>,
    pub multiverse_ids: Option<Vec<u32>>,
    pub mtgo_id: Option<u32>,
    pub arena_id: Option<u32>,
    pub tcgplayer_id: Option<u32>,
    pub cardmarket_id: Option<u32>,
    pub name: String,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: String,
    pub highres_image: bool,
    pub image_status: String,
    pub image_uris: Option<Value>,
    pub mana_cost: Option<String>,
    pub cmc: f64,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub legalities: Value,
    pub games: Vec<String>,
    pub reserved: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: String,
    pub set_search_uri: String,
    pub scryfall_set_uri: String,
    pub rulings_uri: String,
    pub prints_search_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    pub flavor_text: Option<String>,
    pub card_back_id: Option<String>,
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<String>>,
    pub illustration_id: Option<String>,
    pub border_color: String,
    pub frame: String,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: Option<u32>,
    pub penny_rank: Option<u32>,
    pub prices: Option<Value>,
    pub related_uris: Option<Value>,
    pub purchase_uris: Option<Value>,
}

/// Search response from Scryfall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub object: String,
    pub total_cards: Option<u32>,
    pub has_more: bool,
    pub next_page: Option<String>,
    pub data: Vec<Card>,
    pub warnings: Option<Vec<String>>,
}

/// Parameters for card search
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub q: String,
    pub unique: Option<String>,
    pub order: Option<String>,
    pub dir: Option<String>,
    pub include_extras: Option<bool>,
    pub include_multilingual: Option<bool>,
    pub include_variations: Option<bool>,
    pub page: Option<u32>,
}

/// Parameters for advanced search
#[derive(Debug, Clone, Default)]
pub struct AdvancedSearchParams {
    pub name: Option<String>,
    pub oracle: Option<String>,
    pub card_type: Option<String>,
    pub colors: Option<String>,
    pub identity: Option<String>,
    pub mana: Option<String>,
    pub mv: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub set: Option<String>,
    pub rarity: Option<String>,
    pub artist: Option<String>,
    pub flavor: Option<String>,
    pub format: Option<String>,
    pub language: Option<String>,
    pub page: Option<u32>,
    pub order: Option<String>,
    pub dir: Option<String>,
    pub include_extras: Option<bool>,
    pub include_multilingual: Option<bool>,
    pub include_variations: Option<bool>,
    pub unique: Option<String>,
}

/// Autocomplete response from Scryfall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteResponse {
    pub object: String,
    pub total_values: u32,
    pub data: Vec<String>,
}

/// Result type for smart search operations
#[derive(Debug, Clone)]
pub enum SmartSearchResult {
    SingleCard(Box<Card>),
    SearchResults(SearchResponse),
}

/// Query validation issues
#[derive(Debug, Clone)]
pub enum QueryIssue {
    UnknownKeyword(String),
    InvalidOperator(String),
    MalformedExpression(String),
}

/// Query intent detection for smart search
#[derive(Debug, Clone)]
pub enum QueryIntent {
    ExactCardName(String),
    SetCollector(String, String),
    ArenaId(u32),
    MtgoId(u32),
    ScryfallId(String),
    SearchQuery(String),
}

/// Error types for Scryfall operations
#[derive(thiserror::Error, Debug)]
pub enum ScryfallError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cache error: {0}")]
    Cache(#[from] crate::cache::error::CacheError),

    #[error("Card not found: {0}")]
    CardNotFound(String),

    #[error("Invalid search query: {0}")]
    InvalidQuery(String),

    #[error("API error: {0}")]
    ApiError(String),
}
