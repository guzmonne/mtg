use crate::decks::DeckList;
use color_eyre::Result;
use std::collections::HashMap;

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
    use std::collections::HashMap;

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
