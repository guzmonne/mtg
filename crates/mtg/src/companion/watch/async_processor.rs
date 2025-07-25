use crate::prelude::*;
use crate::scryfall::search::get_card_by_arena_id;
use serde_json::Value;
use tokio::sync::mpsc;

pub enum AsyncTask {
    FetchDeckCards { deck_data: Value },
    FetchCardByGrpId { grp_id: u32, context: String },
}

pub struct AsyncProcessor {
    receiver: mpsc::Receiver<AsyncTask>,
}

impl AsyncProcessor {
    pub fn new(receiver: mpsc::Receiver<AsyncTask>) -> Self {
        Self { receiver }
    }

    pub async fn run(mut self) -> Result<()> {
        while let Some(task) = self.receiver.recv().await {
            match task {
                AsyncTask::FetchDeckCards { deck_data } => {
                    if let Err(e) = self.display_deck_with_cards(&deck_data).await {
                        eprintln!("Error displaying deck: {}", e);
                    }
                }
                AsyncTask::FetchCardByGrpId { grp_id, context } => {
                    if let Err(e) = self.display_card_by_grp_id(grp_id, &context).await {
                        eprintln!("Error fetching card {}: {}", grp_id, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn display_deck_with_cards(&self, deck_data: &Value) -> Result<()> {
        println!("\n🃏 Deck Event (with card details):");

        // Debug: print the structure to understand what we're getting
        if std::env::var("MTG_DEBUG").is_ok() {
            println!(
                "DEBUG: Deck data structure: {}",
                serde_json::to_string_pretty(deck_data)?
            );
        }

        // Extract deck summary info
        if let Some(summary) = deck_data.get("Summary") {
            if let Some(name) = summary.get("Name").and_then(|v| v.as_str()) {
                println!("  📝 Name: {}", name);
            }

            if let Some(deck_id) = summary.get("DeckId").and_then(|v| v.as_str()) {
                println!("  🆔 ID: {}", deck_id);
            }

            // Extract format from attributes
            if let Some(attrs) = summary.get("Attributes").and_then(|v| v.as_array()) {
                for attr in attrs {
                    if let (Some(name), Some(value)) = (
                        attr.get("name").and_then(|v| v.as_str()),
                        attr.get("value").and_then(|v| v.as_str()),
                    ) {
                        match name {
                            "Format" => {
                                println!("  🎮 Format: {}", value);
                            }
                            "Companion" => {
                                println!("  🤝 Companion: {}", value);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Extract and display main deck with card names - check both at root level and inside "Deck" field
        let main_deck = deck_data
            .get("MainDeck")
            .and_then(|v| v.as_array())
            .or_else(|| {
                deck_data
                    .get("Deck")
                    .and_then(|d| d.get("MainDeck"))
                    .and_then(|v| v.as_array())
            });

        if let Some(main_deck) = main_deck {
            if main_deck.is_empty() {
                println!("\n  📋 Main Deck: Empty or not loaded");
                return Ok(());
            }

            let total_cards: u32 = main_deck
                .iter()
                .filter_map(|card| card.get("quantity").and_then(|v| v.as_u64()))
                .map(|q| q as u32)
                .sum();

            println!("\n  📋 Main Deck ({} cards):", total_cards);

            // Group cards by arena_id and sum quantities
            let mut card_entries: Vec<(u32, u32)> = Vec::new();
            for card in main_deck {
                if let (Some(arena_id), Some(quantity)) = (
                    card.get("cardId").and_then(|v| v.as_u64()),
                    card.get("quantity").and_then(|v| v.as_u64()),
                ) {
                    card_entries.push((arena_id as u32, quantity as u32));
                }
            }

            // Fetch card data and display
            println!(
                "\n  {:<3} {:<40} {:<20} {:<15} {:<6}",
                "Qty", "Name", "Type", "Mana Cost", "Draw %"
            );
            println!("  {}", "-".repeat(90));

            let mut mana_curve: std::collections::HashMap<u32, u32> =
                std::collections::HashMap::new();

            for (arena_id, quantity) in card_entries {
                match get_card_by_arena_id(arena_id).await {
                    Ok(card) => {
                        let draw_prob = calculate_draw_probability(quantity, total_cards);
                        let type_line = &card.type_line;
                        let mana_cost = card.mana_cost.as_deref().unwrap_or("");

                        println!(
                            "  {:<3} {:<40} {:<20} {:<15} {:<6.1}%",
                            quantity,
                            card.name,
                            truncate_string(type_line, 20),
                            mana_cost,
                            draw_prob
                        );

                        // Update mana curve
                        let cmc_int = card.cmc as u32;
                        *mana_curve.entry(cmc_int.min(7)).or_insert(0) += quantity;
                    }
                    Err(_) => {
                        println!(
                            "  {:<3} Card #{:<35} {:<20} {:<15} {:<6.1}%",
                            quantity,
                            arena_id,
                            "Unknown",
                            "?",
                            calculate_draw_probability(quantity, total_cards)
                        );
                    }
                }
            }

            // Display mana curve
            if !mana_curve.is_empty() {
                println!("\n  📊 Mana Curve:");
                for cmc in 0..=7 {
                    if let Some(&count) = mana_curve.get(&cmc) {
                        let label = if cmc == 7 {
                            "7+".to_string()
                        } else {
                            cmc.to_string()
                        };
                        let bar = "█".repeat((count as f32 / 4.0).ceil() as usize);
                        println!("  {} CMC: {} ({})", label, bar, count);
                    }
                }
            }
        }

        // Extract and display sideboard - check both at root level and inside "Deck" field
        let sideboard = deck_data
            .get("Sideboard")
            .and_then(|v| v.as_array())
            .or_else(|| {
                deck_data
                    .get("Deck")
                    .and_then(|d| d.get("Sideboard"))
                    .and_then(|v| v.as_array())
            });

        if let Some(sideboard) = sideboard {
            if !sideboard.is_empty() {
                let total_cards: u32 = sideboard
                    .iter()
                    .filter_map(|card| card.get("quantity").and_then(|v| v.as_u64()))
                    .map(|q| q as u32)
                    .sum();

                println!("\n  📋 Sideboard ({} cards):", total_cards);

                let mut card_entries: Vec<(u32, u32)> = Vec::new();
                for card in sideboard {
                    if let (Some(arena_id), Some(quantity)) = (
                        card.get("cardId").and_then(|v| v.as_u64()),
                        card.get("quantity").and_then(|v| v.as_u64()),
                    ) {
                        card_entries.push((arena_id as u32, quantity as u32));
                    }
                }

                for (arena_id, quantity) in card_entries {
                    match get_card_by_arena_id(arena_id).await {
                        Ok(card) => {
                            let type_line = &card.type_line;
                            let mana_cost = card.mana_cost.as_deref().unwrap_or("");

                            println!(
                                "  {:<3} {:<40} {:<20} {:<15}",
                                quantity,
                                card.name,
                                truncate_string(type_line, 20),
                                mana_cost
                            );
                        }
                        Err(_) => {
                            println!("  {:<3} Card #{:<35}", quantity, arena_id);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn calculate_draw_probability(quantity: u32, deck_size: u32) -> f32 {
    // Probability of drawing at least one copy in opening hand (7 cards)
    let prob_not_draw = 1.0 - (quantity as f32 / deck_size as f32);
    let prob_not_in_seven = prob_not_draw.powi(7);
    (1.0 - prob_not_in_seven) * 100.0
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

impl AsyncProcessor {
    async fn display_card_by_grp_id(&self, grp_id: u32, context: &str) -> Result<()> {
        // First try to get card by Arena ID (GRP ID)
        match get_card_by_arena_id(grp_id).await {
            Ok(card) => {
                println!("     📋 {}", card.name);
                println!("        Type: {}", card.type_line);

                if let Some(mana_cost) = &card.mana_cost {
                    if card.cmc == 0.0 && mana_cost.is_empty() {
                        println!("        Cost: Free");
                    } else {
                        println!("        Cost: {} (CMC: {})", mana_cost, card.cmc as u32);
                    }
                } else if card.cmc > 0.0 {
                    println!("        Cost: CMC {}", card.cmc as u32);
                }

                if let Some(oracle_text) = &card.oracle_text {
                    // Display oracle text, truncated if too long
                    let text = oracle_text.replace('\n', " ");
                    if text.len() > 80 {
                        println!("        Text: {}...", &text[..77]);
                    } else if !text.is_empty() {
                        println!("        Text: {}", text);
                    }
                }

                if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
                    println!("        P/T: {}/{}", power, toughness);
                } else if let Some(loyalty) = &card.loyalty {
                    println!("        Loyalty: {}", loyalty);
                }

                println!("        Context: {}", context);
            }
            Err(_) => {
                // If we can't find the card, just note the GRP ID
                println!("     🃏 Unknown Card #{} - {}", grp_id, context);
            }
        }

        Ok(())
    }
}
