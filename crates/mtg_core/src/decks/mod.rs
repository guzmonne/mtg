use serde::{Deserialize, Serialize};

pub mod compare;
pub mod parser;
pub mod ranked;
pub mod stats;
pub mod utils;

pub use compare::{compare_decks, load_deck_from_id_or_url, CardEntry, DeckComparison};
pub use parser::parse_deck_list;
pub use stats::{calculate_deck_stats, DeckStats};
pub use utils::generate_short_hash;

/// Represents a single card in a deck with quantity and optional details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckCard {
    pub quantity: u32,
    pub name: String,
    pub set_code: Option<String>,
    pub collector_number: Option<String>,
    pub card_details: Option<crate::scryfall::types::Card>,
}

/// Represents a complete deck list with main deck and sideboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckList {
    pub main_deck: Vec<DeckCard>,
    pub sideboard: Vec<DeckCard>,
}

/// Represents a parsed deck with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDeck {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub event_date: Option<String>,
    pub event_name: Option<String>,
    pub format: Option<String>,
    pub main_deck: Vec<DeckCard>,
    pub sideboard: Vec<DeckCard>,
}
