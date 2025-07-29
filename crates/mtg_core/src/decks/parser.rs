use crate::decks::{DeckCard, DeckList};
use color_eyre::{eyre::eyre, Result};

/// Parse a deck list from text content
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

    Ok(DeckList {
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

    Ok(Some(DeckCard {
        quantity,
        name,
        set_code,
        collector_number,
        card_details: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_deck() {
        let content = r#"
4 Lightning Bolt
2 Counterspell
Sideboard
1 Negate
"#;
        let deck = parse_deck_list(content).unwrap();
        assert_eq!(deck.main_deck.len(), 2);
        assert_eq!(deck.sideboard.len(), 1);
        assert_eq!(deck.main_deck[0].name, "Lightning Bolt");
        assert_eq!(deck.main_deck[0].quantity, 4);
    }

    #[test]
    fn test_parse_with_set_info() {
        let content = "4 Lightning Bolt (M21) 162";
        let deck = parse_deck_list(content).unwrap();
        assert_eq!(deck.main_deck.len(), 1);
        assert_eq!(deck.main_deck[0].name, "Lightning Bolt");
        assert_eq!(deck.main_deck[0].set_code, Some("M21".to_string()));
        assert_eq!(deck.main_deck[0].collector_number, Some("162".to_string()));
    }

    #[test]
    fn test_parse_empty_content() {
        let content = "";
        assert!(parse_deck_list(content).is_err());
    }

    #[test]
    fn test_parse_invalid_quantity() {
        let content = "0 Lightning Bolt";
        assert!(parse_deck_list(content).is_err());
    }
}
