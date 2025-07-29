use clap_stdin::MaybeStdin;
use futures::future::join_all;
use prettytable::{Cell, Row};

use super::utils::{
    calculate_deck_stats_cli, convert_core_deck_list_to_cli, convert_parsed_deck_to_cli_deck_list,
    fetch_card_details_with_global,
};
use super::{DeckCard, DeckList};
use crate::cache::CacheManager;
use crate::prelude::*;
use mtg_core::cache::{CacheStore, DiskCacheBuilder};
use mtg_core::parse_deck_list;

pub async fn run(
    input: Option<MaybeStdin<String>>,
    file: Option<String>,
    format: String,
    global: crate::Global,
) -> Result<()> {
    // Get deck list content
    let deck_content = if let Some(file_path) = file {
        std::fs::read_to_string(&file_path)
            .map_err(|e| eyre!("Failed to read file '{}': {}", file_path, e))?
    } else if let Some(input_maybe_stdin) = input {
        input_maybe_stdin.to_string()
    } else {
        // If no input provided, read from stdin
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| eyre!("Failed to read from stdin: {}", e))?;
        buffer
    };

    // Check if content is empty
    if deck_content.trim().is_empty() {
        return Err(eyre!(
            "Deck list is empty. Please provide a valid deck list."
        ));
    }

    // Check if the input is an Arena deck ID (UUID format)
    let (deck_list, is_arena_deck) = if is_arena_deck_id(&deck_content) {
        aeprintln!("Detected MTG Arena deck ID: {}", deck_content.trim());

        // Try to fetch Arena deck from cache
        match fetch_arena_deck_from_cache(deck_content.trim()).await {
            Ok((arena_deck, deck_name)) => {
                aeprintln!("Found Arena deck: {}", deck_name);

                // Convert Arena card IDs to actual card names
                let converted_deck =
                    convert_arena_deck_to_named(arena_deck, &deck_name, &global).await?;
                (converted_deck, true)
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else if is_deck_id(&deck_content) {
        // Try to fetch deck from cache (regular deck ID)
        match fetch_deck_from_cache(&deck_content).await {
            Ok(deck) => (deck, false),
            Err(_) => {
                // If not found as deck, try as article ID
                let decks = crate::decks::ranked::fetch_decks_from_article(&deck_content, &global)
                    .await
                    .map_err(|_| {
                        eyre!(
                            "ID '{}' not found as deck or article ID",
                            deck_content.trim()
                        )
                    })?;

                if decks.is_empty() {
                    return Err(eyre!("No decks found in article"));
                }

                // If multiple decks, inform user and use the first one
                if decks.len() > 1 {
                    eprintln!(
                        "Note: Article contains {} decks. Analyzing the first deck (ID: {})",
                        decks.len(),
                        decks[0].id
                    );
                    eprintln!("To analyze other decks, use their specific IDs:");
                    for (i, deck) in decks.iter().enumerate().skip(1) {
                        let title = deck.title.as_deref().unwrap_or("Untitled");
                        eprintln!("  {} - {} ({})", i + 1, deck.id, title);
                    }
                    eprintln!();
                }

                // Convert ParsedDeck to DeckList
                (convert_parsed_deck_to_cli_deck_list(&decks[0]), false)
            }
        }
    } else {
        // Parse deck list normally
        let core_deck_list = parse_deck_list(&deck_content)?;
        (convert_core_deck_list_to_cli(&core_deck_list), false)
    };

    // For Arena decks, we already have card details from the conversion
    let deck_with_details = if is_arena_deck {
        deck_list
    } else {
        // Check if we already have card details cached
        let deck_has_details = deck_list
            .main_deck
            .iter()
            .any(|card| card.card_details.is_some())
            || deck_list
                .sideboard
                .iter()
                .any(|card| card.card_details.is_some());

        if deck_has_details {
            // We already have card details, use them
            deck_list
        } else {
            // Fetch card details for non-Arena decks
            let deck_with_fetched_details =
                fetch_card_details_with_global(deck_list, &global).await?;

            // Cache the deck with card details for future use
            if let Ok(deck_id) = extract_deck_id_from_input(&deck_content) {
                if let Err(e) = cache_deck_with_details(&deck_id, &deck_with_fetched_details).await
                {
                    // Log error but don't fail the command
                    eprintln!("Warning: Failed to cache deck with details: {}", e);
                }
            }

            deck_with_fetched_details
        }
    };

    // Calculate statistics
    let stats = calculate_deck_stats_cli(&deck_with_details)?;

    // Output results
    match format.as_str() {
        "json" => output_json(&deck_with_details, &stats)?,
        "pretty" => output_pretty(&deck_with_details, &stats)?,
        _ => output_pretty(&deck_with_details, &stats)?,
    }

    Ok(())
}

fn is_deck_id(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.len() == 16 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_arena_deck_id(input: &str) -> bool {
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

async fn fetch_deck_from_cache(deck_id: &str) -> Result<DeckList> {
    // First try the new mtg_core cache system for parsed decks
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

    // Fallback to legacy cache manager for backward compatibility
    let cache_manager = crate::cache::CacheManager::new()?;

    // Try to get it as a deck ID from legacy cache
    if let Ok(Some(cached_deck)) = cache_manager.get(deck_id.trim()).await {
        // Check if this is a deck (has main_deck field)
        if cached_deck.data.get("main_deck").is_some() {
            // Extract deck data from cached JSON
            let main_deck = cached_deck
                .data
                .get("main_deck")
                .and_then(|v| v.as_array())
                .ok_or_else(|| eyre!("Invalid deck data: missing main_deck"))?;

            let sideboard = cached_deck
                .data
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

                    Some(DeckCard {
                        quantity,
                        name,
                        set_code,
                        collector_number,
                        card_details: None,
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

                    Some(DeckCard {
                        quantity,
                        name,
                        set_code,
                        collector_number,
                        card_details: None,
                    })
                })
                .collect();

            return Ok(DeckList {
                main_deck: main_deck_cards,
                sideboard: sideboard_cards,
            });
        }
    }

    // If not found as a deck, try as an article ID
    Err(eyre!(
        "ID '{}' not found in cache. It might be an article ID - fetching from article...",
        deck_id
    ))
}

async fn fetch_arena_deck_from_cache(deck_id: &str) -> Result<(DeckList, String)> {
    let cache_manager = crate::cache::CacheManager::new()?;

    // Try to get the combined arena decks cache
    if let Ok(Some(cached_data)) = cache_manager.get("arena_decks_combined").await {
        // Look for decks array
        if let Some(decks) = cached_data.data.get("decks").and_then(|v| v.as_array()) {
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

/// Safe padding calculation to prevent underflow
fn safe_padding(total_width: usize, used_width: usize) -> String {
    if used_width >= total_width {
        String::new()
    } else {
        " ".repeat(total_width - used_width)
    }
}

/// Format a single card in a card-like display
fn format_card_display(card: &super::DeckCard, quantity: u32) -> String {
    if let Some(details) = &card.card_details {
        let mut output = String::new();

        // Card border (top)
        output.push_str(&format!("{}\n", " ".repeat(79)));

        // Name and mana cost line
        let name_line = if let Some(mana_cost) = &details.mana_cost {
            if !mana_cost.is_empty() {
                let used_width = 3 + quantity.to_string().len() + card.name.len() + mana_cost.len();
                format!(
                    " {}x {}{}{}",
                    quantity,
                    card.name,
                    safe_padding(79, used_width),
                    mana_cost
                )
            } else {
                let used_width = 3 + quantity.to_string().len() + card.name.len();
                format!(
                    " {}x {}{}",
                    quantity,
                    card.name,
                    safe_padding(79, used_width)
                )
            }
        } else {
            let used_width = 3 + quantity.to_string().len() + card.name.len();
            format!(
                " {}x {}{}",
                quantity,
                card.name,
                safe_padding(79, used_width)
            )
        };
        output.push_str(&name_line);
        output.push('\n');

        // Separator line
        output.push_str(&format!("{}\n", " ".repeat(79)));

        // Type line
        let used_width = 1 + details.type_line.len();
        let type_line = format!(" {}{}", details.type_line, safe_padding(79, used_width));
        output.push_str(&type_line);
        output.push('\n');

        // Another separator if we have oracle text
        if let Some(oracle_text) = &details.oracle_text {
            if !oracle_text.is_empty() {
                output.push_str(&format!("{}\n", " ".repeat(79)));

                // Oracle text (wrapped to fit in card)
                for line in oracle_text.lines() {
                    let wrapped_lines = wrap_text_to_width(line, 77);
                    for wrapped_line in wrapped_lines {
                        let used_width = 1 + wrapped_line.len();
                        let oracle_line =
                            format!(" {}{}", wrapped_line, safe_padding(79, used_width));
                        output.push_str(&oracle_line);
                        output.push('\n');
                    }
                }
            }
        }

        // Flavor text
        if let Some(flavor_text) = &details.flavor_text {
            if !flavor_text.is_empty() {
                output.push_str(&format!("{}\n", " ".repeat(79)));
                for line in flavor_text.lines() {
                    let wrapped_lines = wrap_text_to_width(line, 75);
                    for wrapped_line in wrapped_lines {
                        let used_width = 3 + wrapped_line.len();
                        let flavor_line =
                            format!(" \"{}\"{}", wrapped_line, safe_padding(79, used_width));
                        output.push_str(&flavor_line);
                        output.push('\n');
                    }
                }
            }
        }

        // Bottom section with P/T, Loyalty, Set info
        output.push_str(&format!("{}\n", " ".repeat(79)));

        // Power/Toughness or Loyalty in bottom right, Set info in bottom left
        let set_info = format!("{} ({})", details.set_name, details.set);
        let bottom_right =
            if let (Some(power), Some(toughness)) = (&details.power, &details.toughness) {
                format!("{}/{}", power, toughness)
            } else if let Some(loyalty) = &details.loyalty {
                loyalty.clone()
            } else {
                String::new()
            };

        let bottom_line = if !bottom_right.is_empty() {
            let used_width = 4 + set_info.len() + details.rarity.len() + bottom_right.len();
            format!(
                " {} • {}{}{}",
                set_info,
                details.rarity,
                safe_padding(79, used_width),
                bottom_right
            )
        } else {
            let used_width = 4 + set_info.len() + details.rarity.len();
            format!(
                " {} • {}{}",
                set_info,
                details.rarity,
                safe_padding(79, used_width)
            )
        };
        output.push_str(&bottom_line);
        output.push('\n');

        // Card border (bottom)
        output.push_str(&" ".repeat(79));

        output
    } else {
        // Simple format for cards without details
        let name_used_width = 3 + quantity.to_string().len() + card.name.len();
        let details_used_width = 31;
        format!(
            "{}\n {}x {}{}\n (Card details not available){}\n{}",
            " ".repeat(79),
            quantity,
            card.name,
            safe_padding(79, name_used_width),
            safe_padding(79, details_used_width),
            " ".repeat(79)
        )
    }
}

/// Wrap text to fit within a specified width
fn wrap_text_to_width(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn output_pretty(deck_list: &super::DeckList, stats: &mtg_core::DeckStats) -> Result<()> {
    println!("=== DECK ANALYSIS ===\n");

    // Basic stats
    let mut basic_table = new_table();
    basic_table.add_row(Row::new(vec![Cell::new("Metric"), Cell::new("Value")]));
    basic_table.add_row(Row::new(vec![
        Cell::new("Total Cards"),
        Cell::new(&stats.total_cards.to_string()),
    ]));
    basic_table.add_row(Row::new(vec![
        Cell::new("Main Deck"),
        Cell::new(&stats.main_deck_cards.to_string()),
    ]));
    basic_table.add_row(Row::new(vec![
        Cell::new("Sideboard"),
        Cell::new(&stats.sideboard_cards.to_string()),
    ]));
    basic_table.add_row(Row::new(vec![
        Cell::new("Unique Cards"),
        Cell::new(&stats.unique_cards.to_string()),
    ]));
    basic_table.add_row(Row::new(vec![
        Cell::new("Average Mana Value"),
        Cell::new(&format!("{:.2}", stats.average_mana_value)),
    ]));

    println!("Basic Statistics:");
    basic_table.printstd();
    println!();

    // Mana curve
    if !stats.mana_curve.is_empty() {
        println!("Mana Curve:");
        let mut curve_table = new_table();
        curve_table.add_row(Row::new(vec![
            Cell::new("Mana Value"),
            Cell::new("Cards"),
            Cell::new("Percentage"),
        ]));

        let mut sorted_curve: Vec<_> = stats.mana_curve.iter().collect();
        sorted_curve.sort_by_key(|(mv, _)| *mv);

        for (mv, count) in sorted_curve {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            curve_table.add_row(Row::new(vec![
                Cell::new(&mv.to_string()),
                Cell::new(&count.to_string()),
                Cell::new(&format!("{percentage:.1}%")),
            ]));
        }

        curve_table.printstd();
        println!();
    }

    // Type distribution
    if !stats.type_distribution.is_empty() {
        println!("Card Types:");
        let mut type_table = new_table();
        type_table.add_row(Row::new(vec![
            Cell::new("Type"),
            Cell::new("Cards"),
            Cell::new("Percentage"),
        ]));

        let mut sorted_types: Vec<_> = stats.type_distribution.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));

        for (card_type, count) in sorted_types {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            type_table.add_row(Row::new(vec![
                Cell::new(card_type),
                Cell::new(&count.to_string()),
                Cell::new(&format!("{percentage:.1}%")),
            ]));
        }

        type_table.printstd();
        println!();
    }

    // Color distribution
    if !stats.color_distribution.is_empty() {
        println!("Color Distribution:");
        let mut color_table = new_table();
        color_table.add_row(Row::new(vec![
            Cell::new("Color"),
            Cell::new("Cards"),
            Cell::new("Percentage"),
        ]));

        let mut sorted_colors: Vec<_> = stats.color_distribution.iter().collect();
        sorted_colors.sort_by(|a, b| b.1.cmp(a.1));

        for (color, count) in sorted_colors {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            color_table.add_row(Row::new(vec![
                Cell::new(color),
                Cell::new(&count.to_string()),
                Cell::new(&format!("{percentage:.1}%")),
            ]));
        }

        color_table.printstd();
        println!();
    }

    // Format legality
    if !stats.format_legality.is_empty() {
        println!("Format Legality:");
        let mut format_table = new_table();
        format_table.add_row(Row::new(vec![Cell::new("Format"), Cell::new("Legal")]));

        let key_formats = [
            "standard",
            "pioneer",
            "modern",
            "legacy",
            "vintage",
            "commander",
        ];
        for format in &key_formats {
            if let Some(is_legal) = stats.format_legality.get(*format) {
                format_table.add_row(Row::new(vec![
                    Cell::new(&format.to_uppercase()),
                    Cell::new(if *is_legal { "✓" } else { "✗" }),
                ]));
            }
        }

        format_table.printstd();
        println!();
    }

    // Card list with detailed descriptions
    if !deck_list.main_deck.is_empty() {
        println!("Main Deck ({} cards):", stats.main_deck_cards);
        println!("{}", "=".repeat(80));
        println!();

        for (i, card) in deck_list.main_deck.iter().enumerate() {
            if i > 0 {
                println!("{}", "-".repeat(79));
            }
            println!("{}", format_card_display(card, card.quantity));
            println!();
        }
    }

    if !deck_list.sideboard.is_empty() {
        println!("Sideboard ({} cards):", stats.sideboard_cards);
        println!("{}", "=".repeat(80));
        println!();

        for (i, card) in deck_list.sideboard.iter().enumerate() {
            if i > 0 {
                println!("{}", "-".repeat(79));
            }
            println!("{}", format_card_display(card, card.quantity));
            println!();
        }
    }

    Ok(())
}

fn output_json(deck_list: &super::DeckList, stats: &mtg_core::DeckStats) -> Result<()> {
    let output = serde_json::json!({
        "deck_list": deck_list,
        "statistics": {
            "total_cards": stats.total_cards,
            "main_deck_cards": stats.main_deck_cards,
            "sideboard_cards": stats.sideboard_cards,
            "unique_cards": stats.unique_cards,
            "average_mana_value": stats.average_mana_value,
            "mana_curve": stats.mana_curve,
            "color_distribution": stats.color_distribution,
            "type_distribution": stats.type_distribution,
            "rarity_distribution": stats.rarity_distribution,
            "format_legality": stats.format_legality,
        }
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

async fn fetch_card_by_arena_id(
    arena_id: u32,
    global: &crate::Global,
) -> Result<crate::scryfall::Card> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/arena/{arena_id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let card: crate::scryfall::Card = serde_json::from_value(cached_response.data)?;
        return Ok(card);
    }

    // Fetch from API
    let response = client.get(&url).send().await?;
    let response_text = response.text().await?;

    // Parse as JSON to check for errors
    let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            return Err(eyre!("Card not found for Arena ID: {}", arena_id));
        }
    }

    // Parse as card
    let card: crate::scryfall::Card = serde_json::from_value(json_value.clone())?;

    // Cache the successful response
    cache_manager.set(&cache_key, json_value).await?;

    Ok(card)
}

/// Extract deck ID from input string if it's a deck ID
fn extract_deck_id_from_input(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if is_deck_id(trimmed) {
        Ok(trimmed.to_string())
    } else {
        Err(eyre!("Input is not a deck ID"))
    }
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

/// Cache a deck with card details for faster future access
async fn cache_deck_with_details(deck_id: &str, deck_list: &DeckList) -> Result<()> {
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

    let cache_key = format!("parsed_deck_with_details_{}", deck_id);
    cache.insert(&cache_key, deck_json).await?;

    Ok(())
}

async fn convert_arena_deck_to_named(
    deck_list: DeckList,
    deck_name: &str,
    global: &crate::Global,
) -> Result<DeckList> {
    aeprintln!(
        "Converting Arena deck '{}' card IDs to card names...",
        deck_name
    );

    // Collect all unique Arena IDs
    let mut arena_ids = std::collections::HashSet::new();
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

    aeprintln!("Fetching {} unique cards from Scryfall...", arena_ids.len());

    // Fetch all cards in parallel
    let fetch_futures: Vec<_> = arena_ids
        .iter()
        .map(|&id| fetch_card_by_arena_id(id, global))
        .collect();

    let results = join_all(fetch_futures).await;

    // Build a map of Arena ID to card details
    let mut card_map = std::collections::HashMap::new();
    let mut fetch_errors = 0;

    for (id, result) in arena_ids.iter().zip(results) {
        match result {
            Ok(card) => {
                card_map.insert(*id, card);
            }
            Err(e) => {
                aeprintln!("Warning: Failed to fetch card {}: {}", id, e);
                fetch_errors += 1;
            }
        }
    }

    if fetch_errors > 0 {
        aeprintln!("⚠️  Failed to fetch {} cards", fetch_errors);
    } else {
        aeprintln!("✅ Successfully fetched all cards");
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

                    // Create card details
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

                    // Create card details
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
