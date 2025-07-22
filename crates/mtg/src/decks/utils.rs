use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::prelude::*;

use super::{DeckCard, DeckList, DeckStats};

pub fn parse_deck_list(content: &str) -> Result<DeckList> {
    let mut main_deck = Vec::new();
    let mut sideboard = Vec::new();
    let mut current_section = &mut main_deck;
    let mut parsed_any_cards = false;

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Only process lines that start with "Deck", "Sideboard", or a number
        let first_char = line.chars().next().unwrap_or(' ');
        let line_lower = line.to_lowercase();

        if line_lower == "deck" {
            current_section = &mut main_deck;
            continue;
        } else if line_lower == "sideboard" {
            current_section = &mut sideboard;
            continue;
        } else if line_lower.starts_with("deck") {
            // Handle variations like "Deck:" or "Deck List"
            current_section = &mut main_deck;
            continue;
        } else if line_lower.starts_with("sideboard") {
            // Handle variations like "Sideboard:" or "Sideboard Cards"
            current_section = &mut sideboard;
            continue;
        } else if first_char.is_ascii_digit() {
            // Parse card line: "4 Lightning Bolt (M21) 162"
            match parse_card_line(line) {
                Ok(Some(card)) => {
                    current_section.push(card);
                    parsed_any_cards = true;
                }
                Ok(None) => {
                    // Line starts with number but couldn't be parsed as card - this is an error
                    return Err(eyre!("Failed to parse card line: '{}'", line));
                }
                Err(e) => return Err(e),
            }
        }
        // Ignore all other lines (comments, metadata, etc.)
    }

    if !parsed_any_cards {
        return Err(eyre!("No valid card lines found. Make sure lines with cards start with a number (e.g., '4 Lightning Bolt')."));
    }

    Ok(super::DeckList {
        main_deck,
        sideboard,
    })
}

fn parse_card_line(line: &str) -> Result<Option<DeckCard>> {
    // This function should only be called for lines that start with a number
    // Pattern: "4 Lightning Bolt (M21) 162"
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(eyre!(
            "Invalid card line format: '{}'. Expected format: 'QUANTITY CARD_NAME [SET_INFO]'",
            line
        ));
    }

    let quantity = parts[0]
        .parse::<u32>()
        .map_err(|_| eyre!("Invalid quantity '{}' in line: '{}'", parts[0], line))?;

    if quantity == 0 {
        return Err(eyre!("Card quantity cannot be zero in line: '{}'", line));
    }

    let rest = parts[1].trim();
    if rest.is_empty() {
        return Err(eyre!("Missing card name in line: '{}'", line));
    }

    // Try to extract set code and collector number
    let (name, set_code, collector_number) = if let Some(set_start) = rest.rfind(" (") {
        let name_part = rest[..set_start].trim();
        if name_part.is_empty() {
            return Err(eyre!(
                "Missing card name before set info in line: '{}'",
                line
            ));
        }

        let set_part = &rest[set_start + 2..];

        if let Some(set_end) = set_part.find(')') {
            let set_code = set_part[..set_end].trim();
            let remaining = set_part[set_end + 1..].trim();

            let collector_number = if !remaining.is_empty() {
                Some(remaining.to_string())
            } else {
                None
            };

            (
                name_part.to_string(),
                Some(set_code.to_string()),
                collector_number,
            )
        } else {
            // Malformed set info - treat the whole thing as card name
            (rest.to_string(), None, None)
        }
    } else {
        (rest.to_string(), None, None)
    };

    Ok(Some(super::DeckCard {
        quantity,
        name,
        set_code,
        collector_number,
        card_details: None,
    }))
}

/// Generate a 16-character hash from a serializable value
pub fn generate_short_hash<T: Serialize>(value: &T) -> String {
    let serialized = serde_json::to_string(value).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let result = hasher.finalize();
    // Convert first 8 bytes to hex string (16 characters)
    result
        .iter()
        .take(8)
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

pub async fn fetch_card_details(
    mut deck_list: DeckList,
    global: crate::Global,
) -> Result<DeckList> {
    // Fetch details for main deck cards
    for card in &mut deck_list.main_deck {
        if let Ok(details) = fetch_single_card(&card.name, card.set_code.as_deref(), &global).await
        {
            card.card_details = Some(details);
        }
    }

    // Fetch details for sideboard cards
    for card in &mut deck_list.sideboard {
        if let Ok(details) = fetch_single_card(&card.name, card.set_code.as_deref(), &global).await
        {
            card.card_details = Some(details);
        }
    }

    Ok(deck_list)
}

async fn fetch_single_card(
    name: &str,
    set_code: Option<&str>,
    global: &crate::Global,
) -> Result<crate::scryfall::ScryfallCard> {
    let mut url = format!(
        "https://api.scryfall.com/cards/named?exact={}",
        urlencoding::encode(name)
    );

    if let Some(set) = set_code {
        url.push_str(&format!("&set={}", set.to_lowercase()));
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let response = client.get(&url).send().await?;
    let response_text = response.text().await?;

    let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            let error_msg = json_value
                .get("details")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(eyre!("Card not found: {}", error_msg));
        }
    }

    let card: crate::scryfall::ScryfallCard = serde_json::from_value(json_value)?;
    Ok(card)
}

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
            if type_line.contains("Land") {
                *type_distribution.entry("Land".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Creature") {
                *type_distribution.entry("Creature".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Instant") {
                *type_distribution.entry("Instant".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Sorcery") {
                *type_distribution.entry("Sorcery".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Artifact") {
                *type_distribution.entry("Artifact".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Enchantment") {
                *type_distribution
                    .entry("Enchantment".to_string())
                    .or_insert(0) += card.quantity;
            } else if type_line.contains("Planeswalker") {
                *type_distribution
                    .entry("Planeswalker".to_string())
                    .or_insert(0) += card.quantity;
            } else {
                *type_distribution.entry("Other".to_string()).or_insert(0) += card.quantity;
            }

            // Rarity distribution
            *rarity_distribution
                .entry(details.rarity.clone())
                .or_insert(0) += card.quantity;

            // Format legality (check if all cards are legal in each format)
            if let Some(legalities) = details.legalities.as_object() {
                for (format, status) in legalities {
                    let is_legal = status.as_str() == Some("legal");
                    let current_status = format_legality.get(format).copied().unwrap_or(true);
                    format_legality.insert(format.clone(), current_status && is_legal);
                }
            }
        }
    }

    // Process sideboard
    for card in &deck_list.sideboard {
        sideboard_cards += card.quantity;
        total_cards += card.quantity;
        unique_cards += 1;
    }

    let average_mana_value = if cards_with_mv > 0 {
        total_mana_value / cards_with_mv as f64
    } else {
        0.0
    };

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
        format_legality,
    })
}
