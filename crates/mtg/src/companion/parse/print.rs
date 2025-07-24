use crate::prelude::*;
use prettytable::{Cell, Row};
use std::collections::HashMap;

use super::{get_format_abbreviation, CombinedDeckInfo, Deck, DeckSummary, InventoryInfo};

pub fn inventory(info: &InventoryInfo) {
    println!("=== Inventory Information ===\n");

    // Resources table
    let mut resources_table = new_table();
    resources_table.add_row(Row::new(vec![Cell::new("Resource"), Cell::new("Amount")]));

    if let Some(gems) = info.gems {
        resources_table.add_row(Row::new(vec![
            Cell::new("💎 Gems"),
            Cell::new(&gems.to_string()),
        ]));
    }

    if let Some(gold) = info.gold {
        resources_table.add_row(Row::new(vec![
            Cell::new("🪙 Gold"),
            Cell::new(&gold.to_string()),
        ]));
    }

    if let Some(vault) = info.total_vault_progress {
        resources_table.add_row(Row::new(vec![
            Cell::new("📦 Vault Progress"),
            Cell::new(&format!("{}%", vault)),
        ]));
    }

    resources_table.printstd();

    // Wildcards table
    println!("\n🃏 Wildcards:\n");
    let mut wildcards_table = new_table();
    wildcards_table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new("Count")]));

    if let Some(common) = info.wild_card_commons {
        wildcards_table.add_row(Row::new(vec![
            Cell::new("Common"),
            Cell::new(&common.to_string()),
        ]));
    }

    if let Some(uncommon) = info.wild_card_un_commons {
        wildcards_table.add_row(Row::new(vec![
            Cell::new("Uncommon"),
            Cell::new(&uncommon.to_string()),
        ]));
    }

    if let Some(rare) = info.wild_card_rares {
        wildcards_table.add_row(Row::new(vec![
            Cell::new("Rare"),
            Cell::new(&rare.to_string()),
        ]));
    }

    if let Some(mythic) = info.wild_card_mythics {
        wildcards_table.add_row(Row::new(vec![
            Cell::new("Mythic"),
            Cell::new(&mythic.to_string()),
        ]));
    }

    wildcards_table.printstd();

    // Custom tokens table
    if let Some(tokens) = &info.custom_tokens {
        if !tokens.is_empty() {
            println!("\n🎫 Custom Tokens:\n");
            let mut tokens_table = new_table();
            tokens_table.add_row(Row::new(vec![Cell::new("Token"), Cell::new("Count")]));

            for (name, count) in tokens {
                tokens_table.add_row(Row::new(vec![
                    Cell::new(name),
                    Cell::new(&count.to_string()),
                ]));
            }

            tokens_table.printstd();
        }
    }
}

pub fn combined_decks(
    summaries: &[DeckSummary],
    decks: &HashMap<String, Deck>,
) -> Vec<CombinedDeckInfo> {
    println!("\n=== Decks ===");
    println!(
        "Found {} deck summaries and {} deck definitions\n",
        summaries.len(),
        decks.len()
    );

    let mut table = new_table();
    let mut combined_decks = Vec::new();

    // Add header row
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("#"),
        Cell::new("Name"),
        Cell::new("Format"),
        Cell::new("Main"),
        Cell::new("Side"),
        Cell::new("Sample Cards"),
        Cell::new("Legal Formats"),
    ]));

    for (i, summary) in summaries.iter().enumerate() {
        let clean_name = clean_deck_name(&summary.name);

        // Get format from attributes
        let format = summary
            .attributes
            .as_ref()
            .and_then(|attrs| attrs.iter().find(|a| a.name == "Format"))
            .map(|a| a.value.as_str())
            .unwrap_or("Unknown");

        // Get deck content info
        let (main_count, side_count, sample_cards_str) =
            if let Some(deck) = decks.get(&summary.deck_id) {
                let main = deck.main_deck.len();
                let side = deck.sideboard.as_ref().map(|s| s.len()).unwrap_or(0);

                // Show first few cards
                let sample_cards: Vec<String> = deck
                    .main_deck
                    .iter()
                    .take(3)
                    .map(|card| format!("{}×{}", card.card_id, card.quantity))
                    .collect();

                let cards_display = if deck.main_deck.len() > 3 {
                    format!("{} ...", sample_cards.join(", "))
                } else {
                    sample_cards.join(", ")
                };

                (main, side, cards_display)
            } else {
                (0, 0, "N/A".to_string())
            };

        // Get legal format abbreviations
        let legal_formats = if let Some(legalities) = &summary.format_legalities {
            let formats: Vec<&str> = legalities
                .iter()
                .filter(|(_, &legal)| legal)
                .map(|(format, _)| get_format_abbreviation(format))
                .collect();

            if formats.len() > 10 {
                // If too many formats, show first 8 and count
                format!("{} ... (+{})", formats[..8].join(", "), formats.len() - 8)
            } else {
                formats.join(", ")
            }
        } else {
            String::from("None")
        };

        table.add_row(Row::new(vec![
            Cell::new(&summary.deck_id),
            Cell::new(&(i + 1).to_string()),
            Cell::new(&clean_name),
            Cell::new(format),
            Cell::new(&main_count.to_string()),
            Cell::new(&side_count.to_string()),
            Cell::new(&sample_cards_str),
            Cell::new(&legal_formats),
        ]));

        // Create combined deck info for caching
        if let Some(deck) = decks.get(&summary.deck_id) {
            combined_decks.push(CombinedDeckInfo {
                id: summary.deck_id.clone(),
                name: clean_name,
                format: format.to_string(),
                main_deck_count: main_count,
                sideboard_count: side_count,
                deck_content: deck.clone(),
                format_legalities: summary.format_legalities.clone().unwrap_or_default(),
            });
        }
    }

    table.printstd();
    combined_decks
}

fn clean_deck_name(name: &str) -> String {
    // Handle localization strings like "?=?Loc/Decks/Precon/CC_ANB_B"
    if name.starts_with("?=?Loc/") {
        // Extract the meaningful part after the last slash
        if let Some(last_part) = name.split('/').next_back() {
            // Convert underscores to spaces and format nicely
            return last_part.replace('_', " ");
        }
    }
    name.to_string()
}
