use crate::cache::{CacheStore, DiskCacheBuilder};
use crate::decks::{DeckCard, DeckList, ParsedDeck};
use crate::scryfall::{Card, ScryfallClient};
use color_eyre::{eyre::eyre, Result};
use futures::future::join_all;
use std::collections::{HashMap, HashSet};

/// Statistics calculated from a deck list
#[derive(Debug, Clone)]
pub struct DeckStats {
    pub total_cards: u32,
    pub main_deck_cards: u32,
    pub sideboard_cards: u32,
    pub unique_cards: u32,
    pub average_mana_value: f64,
    pub mana_curve: HashMap<u32, u32>,
    pub color_distribution: HashMap<String, u32>,
    pub type_distribution: HashMap<String, u32>,
    pub rarity_distribution: HashMap<String, u32>,
    pub format_legality: HashMap<String, bool>,
}

/// Calculate comprehensive statistics from a deck list
pub fn calculate_deck_stats(deck_list: &DeckList) -> Result<DeckStats> {
    let mut total_cards = 0;
    let mut main_deck_cards = 0;
    let mut sideboard_cards = 0;
    let mut unique_cards = 0;
    let mut total_mana_value = 0.0;
    let mut cards_with_mv = 0;
    let mut mana_curve = HashMap::new();
    let mut color_distribution = HashMap::new();
    let mut type_distribution = HashMap::new();
    let mut rarity_distribution = HashMap::new();
    let mut format_legality: HashMap<String, bool> = HashMap::new();

    // Process main deck
    for card in &deck_list.main_deck {
        main_deck_cards += card.quantity;
        total_cards += card.quantity;
        unique_cards += 1;

        if let Some(details) = &card.card_details {
            // Mana curve
            let mv = details.cmc as u32;
            *mana_curve.entry(mv).or_insert(0) += card.quantity;
            total_mana_value += details.cmc * card.quantity as f64;
            cards_with_mv += card.quantity;

            // Color distribution
            let colors = &details.color_identity;
            if colors.is_empty() {
                *color_distribution
                    .entry("Colorless".to_string())
                    .or_insert(0) += card.quantity;
            } else {
                for color in colors {
                    *color_distribution.entry(color.clone()).or_insert(0) += card.quantity;
                }
            }

            // Type distribution
            let type_line = &details.type_line;
            let primary_type = extract_primary_type(type_line);
            *type_distribution.entry(primary_type).or_insert(0) += card.quantity;

            // Rarity distribution
            *rarity_distribution
                .entry(details.rarity.clone())
                .or_insert(0) += card.quantity;

            // Format legality - collect all formats where this card is legal
            if let Some(legalities_obj) = details.legalities.as_object() {
                for (format, legality) in legalities_obj {
                    if let Some(legality_str) = legality.as_str() {
                        if legality_str == "legal" {
                            format_legality.insert(format.clone(), true);
                        }
                    }
                }
            }
        }
    }

    // Process sideboard (for total counts only, not for mana curve)
    for card in &deck_list.sideboard {
        sideboard_cards += card.quantity;
        total_cards += card.quantity;
        unique_cards += 1;

        if let Some(details) = &card.card_details {
            // Color distribution
            let colors = &details.color_identity;
            if colors.is_empty() {
                *color_distribution
                    .entry("Colorless".to_string())
                    .or_insert(0) += card.quantity;
            } else {
                for color in colors {
                    *color_distribution.entry(color.clone()).or_insert(0) += card.quantity;
                }
            }

            // Type distribution
            let type_line = &details.type_line;
            let primary_type = extract_primary_type(type_line);
            *type_distribution.entry(primary_type).or_insert(0) += card.quantity;

            // Rarity distribution
            *rarity_distribution
                .entry(details.rarity.clone())
                .or_insert(0) += card.quantity;

            // Format legality
            if let Some(legalities_obj) = details.legalities.as_object() {
                for (format, legality) in legalities_obj {
                    if let Some(legality_str) = legality.as_str() {
                        if legality_str == "legal" {
                            format_legality.insert(format.clone(), true);
                        }
                    }
                }
            }
        }
    }

    // Calculate average mana value
    let average_mana_value = if cards_with_mv > 0 {
        total_mana_value / cards_with_mv as f64
    } else {
        0.0
    };

    // Filter format legality to only include formats where ALL cards are legal
    let mut final_format_legality = HashMap::new();
    for format in format_legality.keys() {
        let mut all_legal = true;

        // Check main deck
        for card in &deck_list.main_deck {
            if let Some(details) = &card.card_details {
                if let Some(legalities_obj) = details.legalities.as_object() {
                    if let Some(legality) = legalities_obj.get(format) {
                        if let Some(legality_str) = legality.as_str() {
                            if legality_str != "legal" {
                                all_legal = false;
                                break;
                            }
                        } else {
                            all_legal = false;
                            break;
                        }
                    } else {
                        all_legal = false;
                        break;
                    }
                } else {
                    all_legal = false;
                    break;
                }
            }
        }

        // Check sideboard if main deck is all legal
        if all_legal {
            for card in &deck_list.sideboard {
                if let Some(details) = &card.card_details {
                    if let Some(legalities_obj) = details.legalities.as_object() {
                        if let Some(legality) = legalities_obj.get(format) {
                            if let Some(legality_str) = legality.as_str() {
                                if legality_str != "legal" {
                                    all_legal = false;
                                    break;
                                }
                            } else {
                                all_legal = false;
                                break;
                            }
                        } else {
                            all_legal = false;
                            break;
                        }
                    } else {
                        all_legal = false;
                        break;
                    }
                }
            }
        }

        final_format_legality.insert(format.clone(), all_legal);
    }

    Ok(DeckStats {
        total_cards,
        main_deck_cards,
        sideboard_cards,
        unique_cards,
        average_mana_value,
        mana_curve,
        color_distribution,
        type_distribution,
        rarity_distribution,
        format_legality: final_format_legality,
    })
}

fn extract_primary_type(type_line: &str) -> String {
    // Extract the primary type from a type line like "Creature — Human Wizard"
    if let Some(dash_pos) = type_line.find(" — ") {
        type_line[..dash_pos].trim().to_string()
    } else if let Some(dash_pos) = type_line.find(" - ") {
        type_line[..dash_pos].trim().to_string()
    } else {
        // No subtype separator, take the first word
        type_line
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decks::DeckCard;
    use crate::scryfall::types::Card;

    fn create_test_card(name: &str, cmc: f64, type_line: &str, rarity: &str) -> DeckCard {
        use serde_json::json;

        let legalities = json!({
            "standard": "legal",
            "modern": "legal"
        });

        DeckCard {
            quantity: 1,
            name: name.to_string(),
            set_code: None,
            collector_number: None,
            card_details: Some(Card {
                object: "card".to_string(),
                id: "test".to_string(),
                oracle_id: None,
                multiverse_ids: None,
                mtgo_id: None,
                arena_id: None,
                tcgplayer_id: None,
                cardmarket_id: None,
                name: name.to_string(),
                lang: "en".to_string(),
                released_at: "2021-01-01".to_string(),
                uri: "".to_string(),
                scryfall_uri: "".to_string(),
                layout: "normal".to_string(),
                highres_image: false,
                image_status: "".to_string(),
                image_uris: None,
                mana_cost: None,
                cmc,
                type_line: type_line.to_string(),
                oracle_text: None,
                power: None,
                toughness: None,
                loyalty: None,
                colors: None,
                color_identity: vec!["R".to_string()],
                keywords: None,
                legalities,
                games: vec![],
                reserved: false,
                foil: false,
                nonfoil: true,
                finishes: vec![],
                oversized: false,
                promo: false,
                reprint: false,
                variation: false,
                set_id: "".to_string(),
                set: "".to_string(),
                set_name: "".to_string(),
                set_type: "".to_string(),
                set_uri: "".to_string(),
                set_search_uri: "".to_string(),
                scryfall_set_uri: "".to_string(),
                rulings_uri: "".to_string(),
                prints_search_uri: "".to_string(),
                collector_number: "1".to_string(),
                digital: false,
                rarity: rarity.to_string(),
                flavor_text: None,
                card_back_id: None,
                artist: None,
                artist_ids: None,
                illustration_id: None,
                border_color: "black".to_string(),
                frame: "2015".to_string(),
                security_stamp: None,
                full_art: false,
                textless: false,
                booster: true,
                story_spotlight: false,
                edhrec_rank: None,
                penny_rank: None,
                prices: None,
                related_uris: None,
                purchase_uris: None,
            }),
        }
    }

    #[test]
    fn test_calculate_basic_stats() {
        let deck_list = DeckList {
            main_deck: vec![
                create_test_card("Lightning Bolt", 1.0, "Instant", "common"),
                create_test_card("Grizzly Bears", 2.0, "Creature — Bear", "common"),
            ],
            sideboard: vec![create_test_card("Negate", 2.0, "Instant", "common")],
        };

        let stats = calculate_deck_stats(&deck_list).unwrap();
        assert_eq!(stats.total_cards, 3);
        assert_eq!(stats.main_deck_cards, 2);
        assert_eq!(stats.sideboard_cards, 1);
        assert_eq!(stats.unique_cards, 3);
        assert_eq!(stats.average_mana_value, 1.5); // (1 + 2) / 2
    }

    #[test]
    fn test_extract_primary_type() {
        assert_eq!(extract_primary_type("Creature — Human Wizard"), "Creature");
        assert_eq!(extract_primary_type("Instant"), "Instant");
        assert_eq!(extract_primary_type("Artifact - Equipment"), "Artifact");
        assert_eq!(
            extract_primary_type("Legendary Creature — Dragon"),
            "Legendary Creature"
        );
    }
}

/// Check if input is a deck ID (16 hex characters)
pub fn is_deck_id(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.len() == 16 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if input is an Arena deck ID (UUID format)
pub fn is_arena_deck_id(input: &str) -> bool {
    let trimmed = input.trim();
    // Arena deck IDs are UUIDs: 8-4-4-4-12 format with hyphens
    if trimmed.len() == 36 {
        let parts: Vec<&str> = trimmed.split('-').collect();
        if parts.len() == 5 {
            return parts[0].len() == 8
                && parts[1].len() == 4
                && parts[2].len() == 4
                && parts[3].len() == 4
                && parts[4].len() == 12
                && trimmed.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        }
    }
    false
}

/// Load deck from cache by ID
pub async fn load_deck_from_cache(deck_id: &str) -> Result<DeckList> {
    let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

    // First try to get deck with card details (faster)
    let cache_key_with_details = format!("parsed_deck_with_details_{}", deck_id.trim());
    let cached_result_with_details: Result<Option<serde_json::Value>, _> =
        cache.get(&cache_key_with_details).await;
    if let Ok(Some(cached_deck)) = cached_result_with_details {
        return parse_cached_deck_data(&cached_deck);
    }

    // Fallback to deck without details
    let cache_key = format!("parsed_deck_{}", deck_id.trim());
    let cached_result: Result<Option<serde_json::Value>, _> = cache.get(&cache_key).await;
    if let Ok(Some(cached_deck)) = cached_result {
        return parse_cached_deck_data(&cached_deck);
    }

    Err(eyre!("Deck ID '{}' not found in cache", deck_id))
}

/// Load Arena deck from cache by ID
pub async fn load_arena_deck_from_cache(deck_id: &str) -> Result<(DeckList, String)> {
    let cache = DiskCacheBuilder::new().prefix("companion").build()?;

    // Try to get the combined arena decks cache
    let cached_result: Result<Option<serde_json::Value>, _> =
        cache.get("arena_decks_combined").await;
    if let Ok(Some(cached_data)) = cached_result {
        // Look for decks array
        if let Some(decks) = cached_data.get("decks").and_then(|v| v.as_array()) {
            // Find the deck with matching ID
            for deck in decks {
                if let Some(id) = deck.get("id").and_then(|v| v.as_str()) {
                    if id == deck_id {
                        // Extract deck content
                        let deck_content = deck
                            .get("deck_content")
                            .ok_or_else(|| eyre!("Deck content not found"))?;

                        let main_deck = deck_content
                            .get("MainDeck")
                            .and_then(|v| v.as_array())
                            .ok_or_else(|| eyre!("Invalid deck data: missing MainDeck"))?;

                        let empty_vec = Vec::new();
                        let sideboard = deck_content
                            .get("Sideboard")
                            .and_then(|v| v.as_array())
                            .unwrap_or(&empty_vec);

                        // Get deck name
                        let deck_name = deck
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown Deck")
                            .to_string();

                        // Convert Arena card IDs to DeckCards (temporarily with ID as name)
                        let main_deck_cards: Vec<DeckCard> = main_deck
                            .iter()
                            .filter_map(|card| {
                                let card_id = card.get("cardId")?.as_u64()? as u32;
                                let quantity = card.get("quantity")?.as_u64()? as u32;

                                Some(DeckCard {
                                    quantity,
                                    name: card_id.to_string(), // Temporarily use ID as name
                                    set_code: None,
                                    collector_number: None,
                                    card_details: None,
                                })
                            })
                            .collect();

                        let sideboard_cards: Vec<DeckCard> = sideboard
                            .iter()
                            .filter_map(|card| {
                                let card_id = card.get("cardId")?.as_u64()? as u32;
                                let quantity = card.get("quantity")?.as_u64()? as u32;

                                Some(DeckCard {
                                    quantity,
                                    name: card_id.to_string(), // Temporarily use ID as name
                                    set_code: None,
                                    collector_number: None,
                                    card_details: None,
                                })
                            })
                            .collect();

                        return Ok((
                            DeckList {
                                main_deck: main_deck_cards,
                                sideboard: sideboard_cards,
                            },
                            deck_name,
                        ));
                    }
                }
            }
        }
    }

    Err(eyre!(
        "Arena deck ID '{}' not found in cache. Please run 'mtg companion parse' first to cache Arena decks.",
        deck_id
    ))
}

/// Convert Arena deck with card IDs to named cards
pub async fn convert_arena_deck_to_named(
    deck_list: DeckList,
    _deck_name: &str,
    scryfall_client: &ScryfallClient,
) -> Result<DeckList> {
    // Collect all unique Arena IDs
    let mut arena_ids = HashSet::new();
    for card in &deck_list.main_deck {
        if let Ok(id) = card.name.parse::<u32>() {
            arena_ids.insert(id);
        }
    }
    for card in &deck_list.sideboard {
        if let Ok(id) = card.name.parse::<u32>() {
            arena_ids.insert(id);
        }
    }

    // Fetch all cards in parallel
    let fetch_futures: Vec<_> = arena_ids
        .iter()
        .map(|&id| fetch_card_by_arena_id(id, scryfall_client))
        .collect();

    let results = join_all(fetch_futures).await;

    // Build a map of Arena ID to card details
    let mut card_map = HashMap::new();
    let mut _fetch_errors = 0;

    for (id, result) in arena_ids.iter().zip(results) {
        match result {
            Ok(card) => {
                card_map.insert(*id, card);
            }
            Err(_) => {
                _fetch_errors += 1;
            }
        }
    }

    // Convert the deck list
    let main_deck: Vec<DeckCard> = deck_list
        .main_deck
        .into_iter()
        .map(|mut card| {
            if let Ok(id) = card.name.parse::<u32>() {
                if let Some(scryfall_card) = card_map.get(&id) {
                    card.name = scryfall_card.name.clone();
                    card.set_code = Some(scryfall_card.set.clone());
                    card.collector_number = Some(scryfall_card.collector_number.clone());
                    card.card_details = Some(scryfall_card.clone());
                }
            }
            card
        })
        .collect();

    let sideboard: Vec<DeckCard> = deck_list
        .sideboard
        .into_iter()
        .map(|mut card| {
            if let Ok(id) = card.name.parse::<u32>() {
                if let Some(scryfall_card) = card_map.get(&id) {
                    card.name = scryfall_card.name.clone();
                    card.set_code = Some(scryfall_card.set.clone());
                    card.collector_number = Some(scryfall_card.collector_number.clone());
                    card.card_details = Some(scryfall_card.clone());
                }
            }
            card
        })
        .collect();

    Ok(DeckList {
        main_deck,
        sideboard,
    })
}

/// Fetch card by Arena ID from Scryfall
async fn fetch_card_by_arena_id(arena_id: u32, scryfall_client: &ScryfallClient) -> Result<Card> {
    let url = format!("https://api.scryfall.com/cards/arena/{arena_id}");

    // Use the scryfall client's get method which handles caching
    let card: Card = scryfall_client.get(&url).await?;
    Ok(card)
}

/// Cache a deck with card details for faster future access
pub async fn cache_deck_with_details(deck_id: &str, deck_list: &DeckList) -> Result<()> {
    let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

    // Create a JSON representation of the deck with card details
    let deck_json = serde_json::json!({
        "id": deck_id,
        "main_deck": deck_list.main_deck.iter().map(|card| {
            serde_json::json!({
                "quantity": card.quantity,
                "name": card.name,
                "set_code": card.set_code,
                "collector_number": card.collector_number,
                "card_details": card.card_details
            })
        }).collect::<Vec<_>>(),
        "sideboard": deck_list.sideboard.iter().map(|card| {
            serde_json::json!({
                "quantity": card.quantity,
                "name": card.name,
                "set_code": card.set_code,
                "collector_number": card.collector_number,
                "card_details": card.card_details
            })
        }).collect::<Vec<_>>()
    });

    let cache_key = format!("parsed_deck_with_details_{deck_id}");
    cache.insert(&cache_key, deck_json).await?;

    Ok(())
}

/// Parse cached deck data from JSON
fn parse_cached_deck_data(cached_deck: &serde_json::Value) -> Result<DeckList> {
    // Extract deck data from cached JSON
    let main_deck = cached_deck
        .get("main_deck")
        .and_then(|v| v.as_array())
        .ok_or_else(|| eyre!("Invalid deck data: missing main_deck"))?;

    let sideboard = cached_deck
        .get("sideboard")
        .and_then(|v| v.as_array())
        .ok_or_else(|| eyre!("Invalid deck data: missing sideboard"))?;

    // Convert JSON arrays to DeckCard vectors
    let main_deck_cards: Vec<DeckCard> = main_deck
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

            // Try to parse card details if present
            let card_details = card
                .get("card_details")
                .and_then(|v| serde_json::from_value(v.clone()).ok());

            Some(DeckCard {
                quantity,
                name,
                set_code,
                collector_number,
                card_details,
            })
        })
        .collect();

    let sideboard_cards: Vec<DeckCard> = sideboard
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

            // Try to parse card details if present
            let card_details = card
                .get("card_details")
                .and_then(|v| serde_json::from_value(v.clone()).ok());

            Some(DeckCard {
                quantity,
                name,
                set_code,
                collector_number,
                card_details,
            })
        })
        .collect();

    Ok(DeckList {
        main_deck: main_deck_cards,
        sideboard: sideboard_cards,
    })
}

/// Convert ParsedDeck to DeckList
pub fn convert_parsed_deck_to_deck_list(parsed_deck: &ParsedDeck) -> DeckList {
    DeckList {
        main_deck: parsed_deck.main_deck.clone(),
        sideboard: parsed_deck.sideboard.clone(),
    }
}
