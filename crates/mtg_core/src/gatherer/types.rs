use serde::{Deserialize, Serialize};

/// Parameters for searching cards on Gatherer
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub name: Option<String>,
    pub rules: Option<String>,
    pub card_type: Option<String>,
    pub subtype: Option<String>,
    pub supertype: Option<String>,
    pub mana_cost: Option<String>,
    pub set: Option<String>,
    pub rarity: Option<String>,
    pub artist: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub flavor: Option<String>,
    pub colors: Option<String>,
    pub format: Option<String>,
    pub language: Option<String>,
    pub page: u32,
}

/// Internal request structure for Gatherer API
#[derive(Debug, Serialize)]
pub(crate) struct SearchRequest {
    #[serde(rename = "searchTerm")]
    pub search_term: String,
    #[serde(rename = "cardName")]
    pub card_name: String,
    pub rules: String,
    #[serde(rename = "instanceSuperType")]
    pub instance_super_type: String,
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    #[serde(rename = "instanceSubtype")]
    pub instance_subtype: String,
    pub colors: String,
    #[serde(rename = "commanderColor")]
    pub commander_color: String,
    #[serde(rename = "manaCost")]
    pub mana_cost: String,
    #[serde(rename = "formatLegalities")]
    pub format_legalities: String,
    #[serde(rename = "setName")]
    pub set_name: String,
    #[serde(rename = "rarityName")]
    pub rarity_name: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    pub power: String,
    pub toughness: String,
    pub loyalty: String,
    #[serde(rename = "flavorText")]
    pub flavor_text: String,
    pub language: String,
    #[serde(rename = "cardPrints")]
    pub card_prints: String,
    #[serde(rename = "extraCards")]
    pub extra_cards: String,
    pub page: String,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            search_term: "$undefined".to_string(),
            card_name: "$undefined".to_string(),
            rules: "$undefined".to_string(),
            instance_super_type: "$undefined".to_string(),
            instance_type: "$undefined".to_string(),
            instance_subtype: "$undefined".to_string(),
            colors: "$undefined".to_string(),
            commander_color: "$undefined".to_string(),
            mana_cost: "$undefined".to_string(),
            format_legalities: "$undefined".to_string(),
            set_name: "$undefined".to_string(),
            rarity_name: "$undefined".to_string(),
            artist_name: "$undefined".to_string(),
            power: "$undefined".to_string(),
            toughness: "$undefined".to_string(),
            loyalty: "$undefined".to_string(),
            flavor_text: "$undefined".to_string(),
            language: "eq~English~en-us".to_string(),
            card_prints: "$undefined".to_string(),
            extra_cards: "$undefined".to_string(),
            page: "$undefined".to_string(),
        }
    }
}

/// Response from Gatherer search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "totalPages")]
    pub total_pages: Option<u64>,
    #[serde(rename = "pageIndex")]
    pub page_index: Option<u64>,
    #[serde(rename = "totalItems")]
    pub total_items: Option<u64>,
    #[serde(rename = "currentItemCount")]
    pub current_item_count: Option<u64>,
    pub items: Option<Vec<Card>>,
}

/// A Magic card from Gatherer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "instanceName")]
    pub name: Option<String>,
    #[serde(rename = "instanceTypeLine")]
    pub type_line: Option<String>,
    #[serde(rename = "instanceManaText")]
    pub mana_cost: Option<String>,
    #[serde(rename = "instanceText")]
    pub oracle_text: Option<String>,
    #[serde(rename = "oraclePower")]
    pub power: Option<String>,
    #[serde(rename = "oracleToughness")]
    pub toughness: Option<String>,
    #[serde(rename = "calculatedLoyalty")]
    pub loyalty: Option<u64>,
    #[serde(rename = "setName")]
    pub set_name: Option<String>,
    #[serde(rename = "rarityName")]
    pub rarity: Option<String>,
    #[serde(rename = "artistName")]
    pub artist: Option<String>,
    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,
}

/// Error types for Gatherer operations
#[derive(thiserror::Error, Debug)]
pub enum GathererError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cache error: {0}")]
    Cache(#[from] crate::cache::error::CacheError),

    #[error("Card not found: {0}")]
    CardNotFound(String),

    #[error("Invalid response format")]
    InvalidResponse,
}

pub type Result<T> = std::result::Result<T, GathererError>;
