use crate::prelude::*;

use super::utils::{calculate_deck_stats, fetch_card_details};

// Function for MCP integration
pub async fn analyze_deck_list_mcp(deck_content: &str, global: crate::Global) -> Result<String> {
    // Check if content is empty
    if deck_content.trim().is_empty() {
        return Err(eyre!(
            "Deck list is empty. Please provide a valid deck list."
        ));
    }

    let deck_list = super::parse_deck_list(deck_content)?;
    let deck_with_details = fetch_card_details(deck_list, global).await?;
    let stats = calculate_deck_stats(&deck_with_details)?;

    // Format as pretty output for MCP
    let mut output = String::new();
    output.push_str("=== DECK ANALYSIS ===\n\n");

    // Basic stats
    output.push_str(&format!("Total Cards: {}\n", stats.total_cards));
    output.push_str(&format!("Main Deck: {}\n", stats.main_deck_cards));
    output.push_str(&format!("Sideboard: {}\n", stats.sideboard_cards));
    output.push_str(&format!("Unique Cards: {}\n", stats.unique_cards));
    output.push_str(&format!(
        "Average Mana Value: {:.2}\n\n",
        stats.average_mana_value
    ));

    // Mana curve
    if !stats.mana_curve.is_empty() {
        output.push_str("Mana Curve:\n");
        let mut sorted_curve: Vec<_> = stats.mana_curve.iter().collect();
        sorted_curve.sort_by_key(|(mv, _)| *mv);

        for (mv, count) in sorted_curve {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            output.push_str(&format!("  {mv}: {count} cards ({percentage:.1}%)\n"));
        }
        output.push('\n');
    }

    // Type distribution
    if !stats.type_distribution.is_empty() {
        output.push_str("Card Types:\n");
        let mut sorted_types: Vec<_> = stats.type_distribution.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));

        for (card_type, count) in sorted_types {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            output.push_str(&format!(
                "  {card_type}: {count} cards ({percentage:.1}%)\n",
            ));
        }
        output.push('\n');
    }

    // Format legality
    if !stats.format_legality.is_empty() {
        output.push_str("Format Legality:\n");
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
                output.push_str(&format!(
                    "  {}: {}\n",
                    format.to_uppercase(),
                    if *is_legal { "Legal" } else { "Not Legal" }
                ));
            }
        }
        output.push('\n');
    }

    // Main deck list
    if !deck_with_details.main_deck.is_empty() {
        output.push_str(&format!("Main Deck ({} cards):\n", stats.main_deck_cards));
        for card in &deck_with_details.main_deck {
            let mana_cost = card
                .card_details
                .as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            output.push_str(&format!(
                "  {}x {} {}\n",
                card.quantity, card.name, mana_cost
            ));
        }
        output.push('\n');
    }

    // Sideboard
    if !deck_with_details.sideboard.is_empty() {
        output.push_str(&format!("Sideboard ({} cards):\n", stats.sideboard_cards));
        for card in &deck_with_details.sideboard {
            let mana_cost = card
                .card_details
                .as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            output.push_str(&format!(
                "  {}x {} {}\n",
                card.quantity, card.name, mana_cost
            ));
        }
    }

    Ok(output)
}
