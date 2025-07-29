use crate::decks::DeckList;
use crate::scryfall::ScryfallClient;
use color_eyre::Result;
use serde::Serialize;
use sha2::{Digest, Sha256};

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
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

/// Fetch card details for all cards in a deck list using Scryfall API
pub async fn fetch_card_details(
    mut deck_list: DeckList,
    scryfall_client: &ScryfallClient,
) -> Result<DeckList> {
    // Fetch details for main deck cards
    for card in &mut deck_list.main_deck {
        if let Ok(details) =
            fetch_single_card(&card.name, card.set_code.as_deref(), scryfall_client).await
        {
            card.card_details = Some(details);
        }
    }

    // Fetch details for sideboard cards
    for card in &mut deck_list.sideboard {
        if let Ok(details) =
            fetch_single_card(&card.name, card.set_code.as_deref(), scryfall_client).await
        {
            card.card_details = Some(details);
        }
    }

    Ok(deck_list)
}

async fn fetch_single_card(
    name: &str,
    set_code: Option<&str>,
    scryfall_client: &ScryfallClient,
) -> Result<crate::scryfall::types::Card> {
    // Use the Scryfall client's get_card_named method
    let card = scryfall_client.get_card_named(name, set_code).await?;
    Ok(card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_hash() {
        let test_data = "test string";
        let hash = generate_short_hash(&test_data);
        assert_eq!(hash.len(), 16);

        // Same input should produce same hash
        let hash2 = generate_short_hash(&test_data);
        assert_eq!(hash, hash2);

        // Different input should produce different hash
        let hash3 = generate_short_hash(&"different string");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_generate_short_hash_with_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            name: String,
            value: u32,
        }

        let test_struct = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let hash = generate_short_hash(&test_struct);
        assert_eq!(hash.len(), 16);
    }
}
