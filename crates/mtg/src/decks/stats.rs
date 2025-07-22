use clap_stdin::MaybeStdin;
use prettytable::{Cell, Row};

use super::{
    utils::{calculate_deck_stats, fetch_card_details, parse_deck_list},
    DeckCard, DeckList, DeckStats,
};
use crate::prelude::*;

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

    // Check if the input is a deck ID (16 character hex string)
    let deck_list = if is_deck_id(&deck_content) {
        // Try to fetch deck from cache
        match fetch_deck_from_cache(&deck_content).await {
            Ok(deck) => deck,
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
                DeckList {
                    main_deck: decks[0].main_deck.clone(),
                    sideboard: decks[0].sideboard.clone(),
                }
            }
        }
    } else {
        // Parse deck list normally
        parse_deck_list(&deck_content)?
    };

    // Fetch card details
    let deck_with_details = fetch_card_details(deck_list, global).await?;

    // Calculate statistics
    let stats = calculate_deck_stats(&deck_with_details)?;

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

async fn fetch_deck_from_cache(deck_id: &str) -> Result<DeckList> {
    let cache_manager = crate::cache::CacheManager::new()?;

    // First try to get it as a deck ID
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

fn output_pretty(deck_list: &DeckList, stats: &DeckStats) -> Result<()> {
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

        for card in &deck_list.main_deck {
            println!();
            println!("{}x {}", card.quantity, card.name);

            if let Some(details) = &card.card_details {
                // Mana cost and type line
                if let Some(mana_cost) = &details.mana_cost {
                    if !mana_cost.is_empty() {
                        println!("Mana Cost: {mana_cost}");
                    }
                }
                println!("Type: {}", details.type_line);

                // Power/Toughness for creatures
                if let (Some(power), Some(toughness)) = (&details.power, &details.toughness) {
                    println!("Power/Toughness: {power}/{toughness}");
                }

                // Loyalty for planeswalkers
                if let Some(loyalty) = &details.loyalty {
                    println!("Loyalty: {loyalty}");
                }

                // Oracle text
                if let Some(oracle_text) = &details.oracle_text {
                    if !oracle_text.is_empty() {
                        println!("Oracle Text:");
                        // Format oracle text with proper line breaks
                        for line in oracle_text.lines() {
                            println!("  {line}");
                        }
                    }
                }

                // Flavor text
                if let Some(flavor_text) = &details.flavor_text {
                    if !flavor_text.is_empty() {
                        println!("Flavor Text:");
                        for line in flavor_text.lines() {
                            println!("  \"{line}\"");
                        }
                    }
                }

                // Set information
                println!("Set: {} ({})", details.set_name, details.set);

                // Rarity
                println!("Rarity: {}", details.rarity);
            } else {
                println!("(Card details not available)");
            }

            println!("{}", "-".repeat(40));
        }

        println!();
    }

    if !deck_list.sideboard.is_empty() {
        println!("Sideboard ({} cards):", stats.sideboard_cards);
        println!("{}", "=".repeat(80));

        for card in &deck_list.sideboard {
            println!();
            println!("{}x {}", card.quantity, card.name);

            if let Some(details) = &card.card_details {
                // Mana cost and type line
                if let Some(mana_cost) = &details.mana_cost {
                    if !mana_cost.is_empty() {
                        println!("Mana Cost: {mana_cost}");
                    }
                }
                println!("Type: {}", details.type_line);

                // Power/Toughness for creatures
                if let (Some(power), Some(toughness)) = (&details.power, &details.toughness) {
                    println!("Power/Toughness: {power}/{toughness}");
                }

                // Loyalty for planeswalkers
                if let Some(loyalty) = &details.loyalty {
                    println!("Loyalty: {loyalty}");
                }

                // Oracle text
                if let Some(oracle_text) = &details.oracle_text {
                    if !oracle_text.is_empty() {
                        println!("Oracle Text:");
                        // Format oracle text with proper line breaks
                        for line in oracle_text.lines() {
                            println!("  {line}");
                        }
                    }
                }

                // Flavor text
                if let Some(flavor_text) = &details.flavor_text {
                    if !flavor_text.is_empty() {
                        println!("Flavor Text:");
                        for line in flavor_text.lines() {
                            println!("  \"{line}\"");
                        }
                    }
                }

                // Set information
                println!("Set: {} ({})", details.set_name, details.set);

                // Rarity
                println!("Rarity: {}", details.rarity);
            } else {
                println!("(Card details not available)");
            }

            println!("{}", "-".repeat(40));
        }
    }

    Ok(())
}

fn output_json(deck_list: &DeckList, stats: &DeckStats) -> Result<()> {
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
