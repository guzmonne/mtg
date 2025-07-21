use crate::prelude::*;
use std::collections::HashMap;
use prettytable::{Table, Row, Cell, format};
use serde::Serialize;
use clap_stdin::MaybeStdin;

#[derive(Debug, clap::Parser)]
#[command(name = "deck")]
#[command(about = "Analyze Magic: The Gathering deck lists")]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Analyze deck statistics from a deck list
    #[clap(name = "stats")]
    Stats {
        /// Deck list input (use '-' for stdin, provide deck list as string, or omit to read from stdin)
        #[clap(value_name = "DECK_LIST")]
        input: Option<MaybeStdin<String>>,
        
        /// Read deck list from file
        #[clap(short, long, value_name = "FILE")]
        file: Option<String>,
        
        /// Output format (pretty table or JSON)
        #[clap(long, default_value = "pretty")]
        format: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct DeckCard {
    pub quantity: u32,
    pub name: String,
    pub set_code: Option<String>,
    pub collector_number: Option<String>,
    pub card_details: Option<crate::scryfall::ScryfallCard>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeckList {
    pub main_deck: Vec<DeckCard>,
    pub sideboard: Vec<DeckCard>,
}

#[derive(Debug, Clone)]
pub struct DeckStats {
    pub total_cards: u32,
    pub main_deck_cards: u32,
    pub sideboard_cards: u32,
    pub unique_cards: u32,
    pub average_mana_value: f64,
    pub mana_curve: HashMap<u32, u32>,
    pub color_distribution: HashMap<String, u32>,
    pub type_distribution: HashMap<String, u32>,
    pub rarity_distribution: HashMap<String, u32>,
    pub format_legality: HashMap<String, bool>,
}

impl App {
    pub async fn run(self, global: crate::Global) -> Result<()> {
        match self.command {
            Commands::Stats { input, file, format } => {
                analyze_deck_stats(input, file, format, global).await
            }
        }
    }
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    app.run(global).await
}

async fn analyze_deck_stats(
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
        std::io::stdin().read_to_string(&mut buffer)
            .map_err(|e| eyre!("Failed to read from stdin: {}", e))?;
        buffer
    };

    // Check if content is empty
    if deck_content.trim().is_empty() {
        return Err(eyre!("Deck list is empty. Please provide a valid deck list."));
    }

    // Parse deck list
    let deck_list = parse_deck_list(&deck_content)?;
    
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

fn parse_deck_list(content: &str) -> Result<DeckList> {
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
    
    Ok(DeckList { main_deck, sideboard })
}

fn parse_card_line(line: &str) -> Result<Option<DeckCard>> {
    // This function should only be called for lines that start with a number
    // Pattern: "4 Lightning Bolt (M21) 162"
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(eyre!("Invalid card line format: '{}'. Expected format: 'QUANTITY CARD_NAME [SET_INFO]'", line));
    }
    
    let quantity = parts[0].parse::<u32>()
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
            return Err(eyre!("Missing card name before set info in line: '{}'", line));
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
            
            (name_part.to_string(), Some(set_code.to_string()), collector_number)
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

async fn fetch_card_details(mut deck_list: DeckList, global: crate::Global) -> Result<DeckList> {
    // Fetch details for main deck cards
    for card in &mut deck_list.main_deck {
        if let Ok(details) = fetch_single_card(&card.name, card.set_code.as_deref(), &global).await {
            card.card_details = Some(details);
        }
    }
    
    // Fetch details for sideboard cards
    for card in &mut deck_list.sideboard {
        if let Ok(details) = fetch_single_card(&card.name, card.set_code.as_deref(), &global).await {
            card.card_details = Some(details);
        }
    }
    
    Ok(deck_list)
}

async fn fetch_single_card(
    name: &str,
    set_code: Option<&str>,
    global: &crate::Global,
) -> Result<crate::scryfall::ScryfallCard> {
    let mut url = format!("https://api.scryfall.com/cards/named?exact={}", urlencoding::encode(name));
    
    if let Some(set) = set_code {
        url.push_str(&format!("&set={}", set.to_lowercase()));
    }
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;
    
    let response = client.get(&url).send().await?;
    let response_text = response.text().await?;
    
    let json_value: serde_json::Value = serde_json::from_str(&response_text)?;
    
    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            let error_msg = json_value.get("details").and_then(|v| v.as_str()).unwrap_or("Unknown error");
            return Err(eyre!("Card not found: {}", error_msg));
        }
    }
    
    let card: crate::scryfall::ScryfallCard = serde_json::from_value(json_value)?;
    Ok(card)
}

fn calculate_deck_stats(deck_list: &DeckList) -> Result<DeckStats> {
    let mut total_cards = 0;
    let mut main_deck_cards = 0;
    let mut sideboard_cards = 0;
    let mut unique_cards = 0;
    let mut total_mana_value = 0.0;
    let mut cards_with_mv = 0;
    let mut mana_curve = HashMap::new();
    let mut color_distribution = HashMap::new();
    let mut type_distribution = HashMap::new();
    let mut rarity_distribution = HashMap::new();
    let mut format_legality: HashMap<String, bool> = HashMap::new();
    
    // Process main deck
    for card in &deck_list.main_deck {
        main_deck_cards += card.quantity;
        total_cards += card.quantity;
        unique_cards += 1;
        
        if let Some(details) = &card.card_details {
            // Mana curve
            let mv = details.cmc as u32;
            *mana_curve.entry(mv).or_insert(0) += card.quantity;
            total_mana_value += details.cmc * card.quantity as f64;
            cards_with_mv += card.quantity;
            
            // Color distribution
            let colors = &details.color_identity;
            if colors.is_empty() {
                *color_distribution.entry("Colorless".to_string()).or_insert(0) += card.quantity;
            } else {
                for color in colors {
                    *color_distribution.entry(color.clone()).or_insert(0) += card.quantity;
                }
            }
            
            // Type distribution
            let type_line = &details.type_line;
            if type_line.contains("Land") {
                *type_distribution.entry("Land".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Creature") {
                *type_distribution.entry("Creature".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Instant") {
                *type_distribution.entry("Instant".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Sorcery") {
                *type_distribution.entry("Sorcery".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Artifact") {
                *type_distribution.entry("Artifact".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Enchantment") {
                *type_distribution.entry("Enchantment".to_string()).or_insert(0) += card.quantity;
            } else if type_line.contains("Planeswalker") {
                *type_distribution.entry("Planeswalker".to_string()).or_insert(0) += card.quantity;
            } else {
                *type_distribution.entry("Other".to_string()).or_insert(0) += card.quantity;
            }
            
            // Rarity distribution
            *rarity_distribution.entry(details.rarity.clone()).or_insert(0) += card.quantity;
            
            // Format legality (check if all cards are legal in each format)
            if let Some(legalities) = details.legalities.as_object() {
                for (format, status) in legalities {
                    let is_legal = status.as_str() == Some("legal");
                    let current_status = format_legality.get(format).copied().unwrap_or(true);
                    format_legality.insert(format.clone(), current_status && is_legal);
                }
            }
        }
    }
    
    // Process sideboard
    for card in &deck_list.sideboard {
        sideboard_cards += card.quantity;
        total_cards += card.quantity;
        unique_cards += 1;
    }
    
    let average_mana_value = if cards_with_mv > 0 {
        total_mana_value / cards_with_mv as f64
    } else {
        0.0
    };
    
    Ok(DeckStats {
        total_cards,
        main_deck_cards,
        sideboard_cards,
        unique_cards,
        average_mana_value,
        mana_curve,
        color_distribution,
        type_distribution,
        rarity_distribution,
        format_legality,
    })
}

fn output_pretty(deck_list: &DeckList, stats: &DeckStats) -> Result<()> {
    println!("=== DECK ANALYSIS ===\n");
    
    // Basic stats
    let mut basic_table = Table::new();
    basic_table.set_format(*format::consts::FORMAT_CLEAN);
    basic_table.add_row(Row::new(vec![
        Cell::new("Metric"),
        Cell::new("Value"),
    ]));
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
        let mut curve_table = Table::new();
        curve_table.set_format(*format::consts::FORMAT_CLEAN);
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
                Cell::new(&format!("{:.1}%", percentage)),
            ]));
        }
        
        curve_table.printstd();
        println!();
    }
    
    // Type distribution
    if !stats.type_distribution.is_empty() {
        println!("Card Types:");
        let mut type_table = Table::new();
        type_table.set_format(*format::consts::FORMAT_CLEAN);
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
                Cell::new(&format!("{:.1}%", percentage)),
            ]));
        }
        
        type_table.printstd();
        println!();
    }
    
    // Color distribution
    if !stats.color_distribution.is_empty() {
        println!("Color Distribution:");
        let mut color_table = Table::new();
        color_table.set_format(*format::consts::FORMAT_CLEAN);
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
                Cell::new(&format!("{:.1}%", percentage)),
            ]));
        }
        
        color_table.printstd();
        println!();
    }
    
    // Format legality
    if !stats.format_legality.is_empty() {
        println!("Format Legality:");
        let mut format_table = Table::new();
        format_table.set_format(*format::consts::FORMAT_CLEAN);
        format_table.add_row(Row::new(vec![
            Cell::new("Format"),
            Cell::new("Legal"),
        ]));
        
        let key_formats = ["standard", "pioneer", "modern", "legacy", "vintage", "commander"];
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
    
    // Card list
    if !deck_list.main_deck.is_empty() {
        println!("Main Deck ({} cards):", stats.main_deck_cards);
        let mut deck_table = Table::new();
        deck_table.set_format(*format::consts::FORMAT_CLEAN);
        deck_table.add_row(Row::new(vec![
            Cell::new("Qty"),
            Cell::new("Name"),
            Cell::new("Mana Cost"),
            Cell::new("Type"),
            Cell::new("Set"),
        ]));
        
        for card in &deck_list.main_deck {
            let mana_cost = card.card_details.as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            let type_line = card.card_details.as_ref()
                .map(|d| d.type_line.as_str())
                .unwrap_or("");
            let set_info = if let Some(set_code) = &card.set_code {
                set_code.clone()
            } else {
                card.card_details.as_ref()
                    .map(|d| d.set.clone())
                    .unwrap_or_default()
            };
            
            deck_table.add_row(Row::new(vec![
                Cell::new(&card.quantity.to_string()),
                Cell::new(&card.name),
                Cell::new(mana_cost),
                Cell::new(type_line),
                Cell::new(&set_info),
            ]));
        }
        
        deck_table.printstd();
        println!();
    }
    
    if !deck_list.sideboard.is_empty() {
        println!("Sideboard ({} cards):", stats.sideboard_cards);
        let mut sb_table = Table::new();
        sb_table.set_format(*format::consts::FORMAT_CLEAN);
        sb_table.add_row(Row::new(vec![
            Cell::new("Qty"),
            Cell::new("Name"),
            Cell::new("Mana Cost"),
            Cell::new("Type"),
            Cell::new("Set"),
        ]));
        
        for card in &deck_list.sideboard {
            let mana_cost = card.card_details.as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            let type_line = card.card_details.as_ref()
                .map(|d| d.type_line.as_str())
                .unwrap_or("");
            let set_info = if let Some(set_code) = &card.set_code {
                set_code.clone()
            } else {
                card.card_details.as_ref()
                    .map(|d| d.set.clone())
                    .unwrap_or_default()
            };
            
            sb_table.add_row(Row::new(vec![
                Cell::new(&card.quantity.to_string()),
                Cell::new(&card.name),
                Cell::new(mana_cost),
                Cell::new(type_line),
                Cell::new(&set_info),
            ]));
        }
        
        sb_table.printstd();
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

// Function for MCP integration
pub async fn analyze_deck_list_mcp(deck_content: &str, global: crate::Global) -> Result<String> {
    // Check if content is empty
    if deck_content.trim().is_empty() {
        return Err(eyre!("Deck list is empty. Please provide a valid deck list."));
    }
    
    let deck_list = parse_deck_list(deck_content)?;
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
    output.push_str(&format!("Average Mana Value: {:.2}\n\n", stats.average_mana_value));
    
    // Mana curve
    if !stats.mana_curve.is_empty() {
        output.push_str("Mana Curve:\n");
        let mut sorted_curve: Vec<_> = stats.mana_curve.iter().collect();
        sorted_curve.sort_by_key(|(mv, _)| *mv);
        
        for (mv, count) in sorted_curve {
            let percentage = (*count as f64 / stats.main_deck_cards as f64) * 100.0;
            output.push_str(&format!("  {}: {} cards ({:.1}%)\n", mv, count, percentage));
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
            output.push_str(&format!("  {}: {} cards ({:.1}%)\n", card_type, count, percentage));
        }
        output.push('\n');
    }
    
    // Format legality
    if !stats.format_legality.is_empty() {
        output.push_str("Format Legality:\n");
        let key_formats = ["standard", "pioneer", "modern", "legacy", "vintage", "commander"];
        for format in &key_formats {
            if let Some(is_legal) = stats.format_legality.get(*format) {
                output.push_str(&format!("  {}: {}\n", 
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
            let mana_cost = card.card_details.as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            output.push_str(&format!("  {}x {} {}\n", card.quantity, card.name, mana_cost));
        }
        output.push('\n');
    }
    
    // Sideboard
    if !deck_with_details.sideboard.is_empty() {
        output.push_str(&format!("Sideboard ({} cards):\n", stats.sideboard_cards));
        for card in &deck_with_details.sideboard {
            let mana_cost = card.card_details.as_ref()
                .and_then(|d| d.mana_cost.as_deref())
                .unwrap_or("");
            output.push_str(&format!("  {}x {} {}\n", card.quantity, card.name, mana_cost));
        }
    }
    
    Ok(output)
}