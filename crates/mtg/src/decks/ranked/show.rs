use scraper::{Html, Selector};
use serde::Serialize;

use crate::decks::ParsedDeck;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct ParsedDecksResponse {
    pub url: String,
    pub decks: Vec<ParsedDeck>,
}

pub async fn run(identifier: String, output: String, global: crate::Global) -> Result<()> {
    // Determine if identifier is a URL or ID
    let url = if identifier.starts_with("http://") || identifier.starts_with("https://") {
        identifier.clone()
    } else {
        // It's an ID, fetch from cache
        let cache_manager = crate::cache::CacheManager::new()?;
        let cached_item = cache_manager
            .get(&identifier)
            .await?
            .ok_or_else(|| eyre!("No cached item found with ID: {}", identifier))?;

        // Extract the link from the cached item
        cached_item
            .data
            .get("link")
            .and_then(|v| v.as_str())
            .ok_or_else(|| eyre!("No link found in cached item"))?
            .to_string()
    };

    // Fetch the page
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .build()?;

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(eyre!("Failed to fetch page: HTTP {}", response.status()));
    }

    let html_content = response.text().await?;

    // Parse the HTML
    let document = Html::parse_document(&html_content);

    // Find all deck-list web components
    let deck_list_selector = Selector::parse("deck-list").unwrap();
    let mut parsed_decks = Vec::new();

    for deck_element in document.select(&deck_list_selector) {
        // Extract attributes from deck-list element
        let deck_title = deck_element.value().attr("deck-title").map(String::from);
        let subtitle = deck_element.value().attr("subtitle").map(String::from);
        let event_date = deck_element.value().attr("event-date").map(String::from);
        let event_name = deck_element.value().attr("event-name").map(String::from);
        let format = deck_element.value().attr("format").map(String::from);

        // Parse main deck
        let main_deck_selector = Selector::parse("main-deck").unwrap();
        let mut main_deck = Vec::new();

        if let Some(main_deck_element) = deck_element.select(&main_deck_selector).next() {
            let deck_content = main_deck_element.text().collect::<String>();
            if let Ok(parsed_list) = crate::decks::parse_deck_list(&deck_content) {
                main_deck = parsed_list.main_deck;
            }
        }

        // Parse sideboard
        let sideboard_selector = Selector::parse("side-board").unwrap();
        let mut sideboard = Vec::new();

        if let Some(sideboard_element) = deck_element.select(&sideboard_selector).next() {
            let deck_content = sideboard_element.text().collect::<String>();
            if let Ok(parsed_list) = crate::decks::parse_deck_list(&deck_content) {
                sideboard = parsed_list.main_deck; // Use main_deck since we're parsing just the sideboard content
            }
        }

        // Generate hash for this deck
        let deck_data = serde_json::json!({
            "title": deck_title,
            "subtitle": subtitle,
            "event_date": event_date,
            "event_name": event_name,
            "format": format,
            "main_deck": &main_deck,
            "sideboard": &sideboard,
        });
        let deck_hash = crate::decks::generate_short_hash(&deck_data);

        // Cache the parsed deck
        let cache_manager = crate::cache::CacheManager::new()?;
        cache_manager.set(&deck_hash, deck_data.clone()).await?;

        parsed_decks.push(ParsedDeck {
            id: deck_hash,
            title: deck_title,
            subtitle,
            event_date,
            event_name,
            format,
            main_deck,
            sideboard,
        });
    }

    if parsed_decks.is_empty() {
        return Err(eyre!("No deck lists found on the page"));
    }

    // Output results
    let response = ParsedDecksResponse {
        url: url.clone(),
        decks: parsed_decks,
    };

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
