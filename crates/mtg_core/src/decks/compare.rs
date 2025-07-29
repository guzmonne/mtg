use crate::cache::{CacheStore, DiskCache};
use crate::decks::{ranked::RankedDecksClient, ParsedDeck};
use color_eyre::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CardEntry {
    pub main_count: u32,
    pub side_count: u32,
}

impl CardEntry {
    pub fn total(&self) -> u32 {
        self.main_count + self.side_count
    }
}

#[derive(Debug)]
pub struct DeckComparison {
    pub deck1_name: String,
    pub deck2_name: String,
    pub shared_cards: HashMap<String, (CardEntry, CardEntry)>,
    pub deck1_unique: HashMap<String, CardEntry>,
    pub deck2_unique: HashMap<String, CardEntry>,
}

/// Load a deck from ID or URL using the new caching system
pub async fn load_deck_from_id_or_url(
    identifier: &str,
    ranked_client: &RankedDecksClient,
    cache: &DiskCache,
) -> Result<ParsedDeck> {
    // First, try to load as a deck ID from the new cache system
    let cache_key = format!("parsed_deck_{identifier}");
    let cached_result: Result<Option<serde_json::Value>, _> = cache.get(&cache_key).await;

    if let Ok(Some(cached_deck)) = cached_result {
        return parse_cached_deck_to_parsed_deck(&cached_deck, identifier);
    }

    // Try as article ID
    let article_cache_key = format!("ranked_article_{identifier}");
    let article_cached_result: Result<Option<serde_json::Value>, _> =
        cache.get(&article_cache_key).await;

    if let Ok(Some(cached_article)) = article_cached_result {
        // Extract URL from cached article and fetch decks
        if let Some(url) = cached_article.get("link").and_then(|v| v.as_str()) {
            let decks = ranked_client.fetch_decks_from_article(url).await?;
            if let Some(first_deck) = decks.into_iter().next() {
                return Ok(first_deck);
            } else {
                return Err(color_eyre::eyre::eyre!("No decks found in article"));
            }
        }
    }

    // If not found in cache and it's a URL, fetch it directly
    if identifier.starts_with("http://") || identifier.starts_with("https://") {
        let decks = ranked_client.fetch_decks_from_article(identifier).await?;
        if let Some(first_deck) = decks.into_iter().next() {
            return Ok(first_deck);
        } else {
            return Err(color_eyre::eyre::eyre!("No decks found at URL"));
        }
    }

    Err(color_eyre::eyre::eyre!(
        "Deck or article not found with ID: {}. Please run 'mtg decks ranked list' or 'mtg decks ranked show' first to cache the data.",
        identifier
    ))
}

/// Parse cached deck data to ParsedDeck
fn parse_cached_deck_to_parsed_deck(
    cached_deck: &serde_json::Value,
    deck_id: &str,
) -> Result<ParsedDeck> {
    let main_deck = cached_deck
        .get("main_deck")
        .and_then(|v| v.as_array())
        .ok_or_else(|| color_eyre::eyre::eyre!("Invalid deck data: missing main_deck"))?;

    let sideboard = cached_deck
        .get("sideboard")
        .and_then(|v| v.as_array())
        .ok_or_else(|| color_eyre::eyre::eyre!("Invalid deck data: missing sideboard"))?;

    // Convert to mtg_core types
    let main_deck_cards: Vec<crate::DeckCard> = main_deck
        .iter()
        .filter_map(|card| {
            let quantity = card.get("quantity")?.as_u64()? as u32;
            let name = card.get("name")?.as_str()?.to_string();
            let set_code = card
                .get("set_code")
                .and_then(|v| v.as_str())
                .map(String::from);
            let collector_number = card
                .get("collector_number")
                .and_then(|v| v.as_str())
                .map(String::from);

            Some(crate::DeckCard {
                quantity,
                name,
                set_code,
                collector_number,
                card_details: None,
            })
        })
        .collect();

    let sideboard_cards: Vec<crate::DeckCard> = sideboard
        .iter()
        .filter_map(|card| {
            let quantity = card.get("quantity")?.as_u64()? as u32;
            let name = card.get("name")?.as_str()?.to_string();
            let set_code = card
                .get("set_code")
                .and_then(|v| v.as_str())
                .map(String::from);
            let collector_number = card
                .get("collector_number")
                .and_then(|v| v.as_str())
                .map(String::from);

            Some(crate::DeckCard {
                quantity,
                name,
                set_code,
                collector_number,
                card_details: None,
            })
        })
        .collect();

    // Extract other deck metadata
    let title = cached_deck
        .get("title")
        .and_then(|v| v.as_str())
        .map(String::from);
    let subtitle = cached_deck
        .get("subtitle")
        .and_then(|v| v.as_str())
        .map(String::from);
    let event_date = cached_deck
        .get("event_date")
        .and_then(|v| v.as_str())
        .map(String::from);
    let event_name = cached_deck
        .get("event_name")
        .and_then(|v| v.as_str())
        .map(String::from);
    let format = cached_deck
        .get("format")
        .and_then(|v| v.as_str())
        .map(String::from);

    Ok(ParsedDeck {
        id: deck_id.to_string(),
        title,
        subtitle,
        event_date,
        event_name,
        format,
        main_deck: main_deck_cards,
        sideboard: sideboard_cards,
    })
}

/// Compare two decks and return the comparison result
pub fn compare_decks(deck1: &ParsedDeck, deck2: &ParsedDeck) -> DeckComparison {
    let mut deck1_cards: HashMap<String, CardEntry> = HashMap::new();
    let mut deck2_cards: HashMap<String, CardEntry> = HashMap::new();

    // Build deck1 card map
    for card in &deck1.main_deck {
        deck1_cards.insert(
            card.name.clone(),
            CardEntry {
                main_count: card.quantity,
                side_count: 0,
            },
        );
    }

    for card in &deck1.sideboard {
        deck1_cards
            .entry(card.name.clone())
            .and_modify(|e| e.side_count = card.quantity)
            .or_insert(CardEntry {
                main_count: 0,
                side_count: card.quantity,
            });
    }

    // Build deck2 card map
    for card in &deck2.main_deck {
        deck2_cards.insert(
            card.name.clone(),
            CardEntry {
                main_count: card.quantity,
                side_count: 0,
            },
        );
    }

    for card in &deck2.sideboard {
        deck2_cards
            .entry(card.name.clone())
            .and_modify(|e| e.side_count = card.quantity)
            .or_insert(CardEntry {
                main_count: 0,
                side_count: card.quantity,
            });
    }

    // Find shared and unique cards
    let mut shared_cards = HashMap::new();
    let mut deck1_unique = HashMap::new();

    for (card_name, entry1) in deck1_cards {
        if let Some(entry2) = deck2_cards.get(&card_name) {
            shared_cards.insert(card_name, (entry1, entry2.clone()));
        } else {
            deck1_unique.insert(card_name, entry1);
        }
    }

    let mut deck2_unique = HashMap::new();
    for (card_name, entry2) in deck2_cards {
        if !shared_cards.contains_key(&card_name) {
            deck2_unique.insert(card_name, entry2);
        }
    }

    DeckComparison {
        deck1_name: deck1.title.clone().unwrap_or_else(|| "Deck 1".to_string()),
        deck2_name: deck2.title.clone().unwrap_or_else(|| "Deck 2".to_string()),
        shared_cards,
        deck1_unique,
        deck2_unique,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DeckCard;

    fn create_test_deck(
        name: &str,
        main_cards: Vec<(&str, u32)>,
        side_cards: Vec<(&str, u32)>,
    ) -> ParsedDeck {
        ParsedDeck {
            id: "test".to_string(),
            title: Some(name.to_string()),
            subtitle: None,
            event_date: None,
            event_name: None,
            format: None,
            main_deck: main_cards
                .into_iter()
                .map(|(name, qty)| DeckCard {
                    quantity: qty,
                    name: name.to_string(),
                    set_code: None,
                    collector_number: None,
                    card_details: None,
                })
                .collect(),
            sideboard: side_cards
                .into_iter()
                .map(|(name, qty)| DeckCard {
                    quantity: qty,
                    name: name.to_string(),
                    set_code: None,
                    collector_number: None,
                    card_details: None,
                })
                .collect(),
        }
    }

    #[test]
    fn test_compare_identical_decks() {
        let deck1 = create_test_deck(
            "Deck 1",
            vec![("Lightning Bolt", 4)],
            vec![("Counterspell", 2)],
        );
        let deck2 = create_test_deck(
            "Deck 2",
            vec![("Lightning Bolt", 4)],
            vec![("Counterspell", 2)],
        );

        let comparison = compare_decks(&deck1, &deck2);

        assert_eq!(comparison.shared_cards.len(), 2);
        assert_eq!(comparison.deck1_unique.len(), 0);
        assert_eq!(comparison.deck2_unique.len(), 0);
    }

    #[test]
    fn test_compare_different_decks() {
        let deck1 = create_test_deck("Deck 1", vec![("Lightning Bolt", 4)], vec![]);
        let deck2 = create_test_deck("Deck 2", vec![("Counterspell", 4)], vec![]);

        let comparison = compare_decks(&deck1, &deck2);

        assert_eq!(comparison.shared_cards.len(), 0);
        assert_eq!(comparison.deck1_unique.len(), 1);
        assert_eq!(comparison.deck2_unique.len(), 1);
    }

    #[test]
    fn test_compare_partially_overlapping_decks() {
        let deck1 = create_test_deck("Deck 1", vec![("Lightning Bolt", 4), ("Shock", 2)], vec![]);
        let deck2 = create_test_deck(
            "Deck 2",
            vec![("Lightning Bolt", 3), ("Counterspell", 4)],
            vec![],
        );

        let comparison = compare_decks(&deck1, &deck2);

        assert_eq!(comparison.shared_cards.len(), 1);
        assert_eq!(comparison.deck1_unique.len(), 1);
        assert_eq!(comparison.deck2_unique.len(), 1);

        // Check that Lightning Bolt is shared with different quantities
        let lightning_bolt = comparison.shared_cards.get("Lightning Bolt").unwrap();
        assert_eq!(lightning_bolt.0.main_count, 4);
        assert_eq!(lightning_bolt.1.main_count, 3);
    }
}
