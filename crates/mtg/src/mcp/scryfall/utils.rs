use prettytable::{Cell, Row};

use crate::prelude::*;

// Helper function to format Scryfall search results as pretty table
pub fn format_scryfall_search_results(
    response: &crate::scryfall::search::Response,
) -> Result<String> {
    let mut table = new_table();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Cost"),
        Cell::new("Type"),
        Cell::new("Set"),
        Cell::new("Rarity"),
        Cell::new("P/T/L"),
    ]));

    for card in &response.data {
        let mana_cost = card.mana_cost.as_deref().unwrap_or("");
        let pt_loyalty = if let Some(loyalty) = &card.loyalty {
            loyalty.clone()
        } else if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
            format!("{power}/{toughness}")
        } else {
            "-".to_string()
        };

        table.add_row(Row::new(vec![
            Cell::new(&card.name),
            Cell::new(mana_cost),
            Cell::new(&card.type_line),
            Cell::new(&card.set_name),
            Cell::new(&card.rarity),
            Cell::new(&pt_loyalty),
        ]));
    }

    let mut buffer = Vec::new();
    table
        .print(&mut buffer)
        .map_err(|e| eyre!("Failed to format table: {}", e))?;
    let mut output =
        String::from_utf8(buffer).map_err(|e| eyre!("Failed to convert table to string: {}", e))?;

    // Add summary information
    let total_cards = response.total_cards.unwrap_or(response.data.len() as u32) as usize;
    output.push_str(&format!("\nFound {total_cards} cards"));
    if response.data.len() < total_cards {
        output.push_str(&format!(" (showing {} on this page)", response.data.len()));
    }

    // Display warnings if any
    if let Some(warnings) = &response.warnings {
        output.push_str("\n\n⚠️  Warnings:\n");
        for warning in warnings {
            output.push_str(&format!("   • {warning}\n"));
        }
    }

    Ok(output)
}

// Helper function to format single card details as pretty table
pub fn format_single_card_details(card: &crate::scryfall::Card) -> Result<String> {
    let mut table = new_table();

    // Card name
    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(&card.name)]));

    // Mana cost
    if let Some(mana_cost) = &card.mana_cost {
        if !mana_cost.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Mana Cost"), Cell::new(mana_cost)]));
        }
    }

    // Mana value
    if card.cmc > 0.0 {
        table.add_row(Row::new(vec![
            Cell::new("Mana Value"),
            Cell::new(&card.cmc.to_string()),
        ]));
    }

    // Type line
    table.add_row(Row::new(vec![
        Cell::new("Type"),
        Cell::new(&card.type_line),
    ]));

    // Oracle text
    if let Some(oracle_text) = &card.oracle_text {
        if !oracle_text.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Oracle Text"),
                Cell::new(oracle_text),
            ]));
        }
    }

    // Power/Toughness
    if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
        table.add_row(Row::new(vec![
            Cell::new("Power/Toughness"),
            Cell::new(&format!("{power}/{toughness}")),
        ]));
    }

    // Loyalty
    if let Some(loyalty) = &card.loyalty {
        table.add_row(Row::new(vec![Cell::new("Loyalty"), Cell::new(loyalty)]));
    }

    // Set
    table.add_row(Row::new(vec![
        Cell::new("Set"),
        Cell::new(&format!("{} ({})", card.set_name, card.set.to_uppercase())),
    ]));

    // Rarity
    table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new(&card.rarity)]));

    // Artist
    if let Some(artist) = &card.artist {
        table.add_row(Row::new(vec![Cell::new("Artist"), Cell::new(artist)]));
    }

    // Flavor text
    if let Some(flavor_text) = &card.flavor_text {
        if !flavor_text.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Flavor Text"),
                Cell::new(flavor_text),
            ]));
        }
    }

    // Collector number
    table.add_row(Row::new(vec![
        Cell::new("Collector Number"),
        Cell::new(&card.collector_number),
    ]));

    // Legalities (show a few key formats)
    if let Some(legalities) = card.legalities.as_object() {
        let mut legal_formats = Vec::new();
        let key_formats = [
            "standard",
            "pioneer",
            "modern",
            "legacy",
            "vintage",
            "commander",
        ];

        for format in &key_formats {
            if let Some(status) = legalities.get(*format).and_then(|v| v.as_str()) {
                if status == "legal" {
                    legal_formats.push(format.to_string());
                }
            }
        }

        // Add legal formats to table
        if !legal_formats.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Legal In"),
                Cell::new(&legal_formats.join(", ")),
            ]));
        }
    }

    // Convert table to string and return
    let mut buffer = Vec::new();
    table
        .print(&mut buffer)
        .map_err(|e| eyre!("Failed to format table: {}", e))?;
    let output =
        String::from_utf8(buffer).map_err(|e| eyre!("Failed to convert table to string: {}", e))?;
    Ok(output)
}
