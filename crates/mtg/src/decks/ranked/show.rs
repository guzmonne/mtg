use crate::prelude::*;
use mtg_core::cache::{CacheStore, CachedHttpClient, DiskCache, DiskCacheBuilder};
use mtg_core::{ParsedDeck, ParsedDecksResponse, RankedDecksClient};

/// Try to fetch a single deck by its ID from cache
async fn try_fetch_single_deck(deck_id: &str, cache: &DiskCache) -> Result<ParsedDeck> {
    let cache_key = format!("parsed_deck_{}", deck_id);
    let cached_result: Result<Option<serde_json::Value>, _> = cache.get(&cache_key).await;

    if let Ok(Some(cached_deck)) = cached_result {
        // Parse the cached deck data
        let main_deck = cached_deck
            .get("main_deck")
            .and_then(|v| v.as_array())
            .ok_or_else(|| eyre!("Invalid deck data: missing main_deck"))?;

        let sideboard = cached_deck
            .get("sideboard")
            .and_then(|v| v.as_array())
            .ok_or_else(|| eyre!("Invalid deck data: missing sideboard"))?;

        // Convert to mtg_core types
        let main_deck_cards: Vec<mtg_core::DeckCard> = main_deck
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

                Some(mtg_core::DeckCard {
                    quantity,
                    name,
                    set_code,
                    collector_number,
                    card_details: None,
                })
            })
            .collect();

        let sideboard_cards: Vec<mtg_core::DeckCard> = sideboard
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

                Some(mtg_core::DeckCard {
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
    } else {
        Err(eyre!("Deck ID not found in cache"))
    }
}

pub async fn run(identifier: String, output: String, global: crate::Global) -> Result<()> {
    // Create cache and HTTP client - use same prefixes as list command
    let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

    let http_client = CachedHttpClient::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .cache_prefix("ranked_list_http")
        .build()?;

    // Create ranked decks client
    let client = RankedDecksClient::new(http_client, cache.clone());

    // Check if this might be a deck ID first
    let response = if let Ok(deck) = try_fetch_single_deck(&identifier, &cache).await {
        // It's a deck ID, create a response with just this deck
        mtg_core::ParsedDecksResponse {
            url: format!("Deck ID: {}", identifier),
            decks: vec![deck],
        }
    } else {
        // Try as article ID or URL
        match client.fetch_decks_response(&identifier).await {
            Ok(response) => response,
            Err(e) => {
                // Check if it's a cache miss error and provide helpful message
                if e.to_string().contains("No cached item found with ID") {
                    return Err(eyre!(
                        "ID '{}' not found in cache. Please run 'mtg decks ranked list' first to generate IDs, or provide a URL directly.",
                        identifier
                    ));
                }
                return Err(e);
            }
        }
    };

    if response.decks.is_empty() {
        return Err(eyre!("No deck lists found on the page"));
    }

    // Output results
    match output.as_str() {
        "json" => output_parsed_decks_json(&response)?,
        "pretty" => output_parsed_decks_pretty(&response)?,
        _ => output_parsed_decks_pretty(&response)?,
    }

    Ok(())
}

fn output_parsed_decks_json(response: &ParsedDecksResponse) -> Result<()> {
    aprintln!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn output_parsed_decks_pretty(response: &ParsedDecksResponse) -> Result<()> {
    aprintln!("=== DECK LISTS FROM {} ===\n", response.url);
    aprintln!("Found {} deck(s)\n", response.decks.len());

    for (idx, deck) in response.decks.iter().enumerate() {
        aprintln!("Deck {} - ID: {}", idx + 1, deck.id);
        aprintln!("{}", "=".repeat(60));

        if let Some(title) = &deck.title {
            aprintln!("Title: {}", title);
        }
        if let Some(subtitle) = &deck.subtitle {
            aprintln!("Subtitle: {}", subtitle);
        }
        if let Some(event_name) = &deck.event_name {
            aprintln!("Event: {}", event_name);
        }
        if let Some(event_date) = &deck.event_date {
            aprintln!("Date: {}", event_date);
        }
        if let Some(format) = &deck.format {
            aprintln!("Format: {}", format);
        }

        aprintln!();

        // Main deck
        if !deck.main_deck.is_empty() {
            let total_main: u32 = deck.main_deck.iter().map(|c| c.quantity).sum();
            aprintln!("Deck ({} cards):", total_main);
            for card in &deck.main_deck {
                aprintln!("{} {}", card.quantity, card.name);
            }
        }

        // Sideboard
        if !deck.sideboard.is_empty() {
            aprintln!();
            let total_side: u32 = deck.sideboard.iter().map(|c| c.quantity).sum();
            aprintln!("Sideboard ({} cards):", total_side);
            for card in &deck.sideboard {
                aprintln!("{} {}", card.quantity, card.name);
            }
        }

        aprintln!();
    }

    Ok(())
}

/// Fetch and parse decks from an article identifier (ID or URL)
pub async fn fetch_decks_from_article(
    identifier: &str,
    global: &crate::Global,
) -> Result<Vec<mtg_core::ParsedDeck>> {
    // Create cache and HTTP client - use same prefixes as list command
    let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

    let http_client = CachedHttpClient::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .cache_prefix("ranked_list_http")
        .build()?;

    // Create ranked decks client
    let client = RankedDecksClient::new(http_client, cache);

    // Fetch decks using mtg_core
    match client.fetch_decks_from_article(identifier).await {
        Ok(decks) => Ok(decks),
        Err(e) => {
            // Check if it's a cache miss error and provide helpful message
            if e.to_string().contains("No cached item found with ID") {
                return Err(eyre!(
                    "ID '{}' not found in cache. Please run 'mtg decks ranked list' first to generate IDs, or provide a URL directly.",
                    identifier
                ));
            }
            Err(e)
        }
    }
}
