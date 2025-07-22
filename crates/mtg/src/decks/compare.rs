use crate::prelude::*;
use clap::Args;
use color_eyre::owo_colors::OwoColorize;
use prettytable::row;
use std::collections::HashMap;

use super::ParsedDeck;

#[derive(Args, Debug)]
pub struct CompareArgs {
    #[arg(help = "First deck ID or article ID")]
    deck1: String,

    #[arg(help = "Second deck ID or article ID")]
    deck2: String,
}

#[derive(Debug, Clone)]
struct CardEntry {
    main_count: u32,
    side_count: u32,
}

impl CardEntry {
    fn total(&self) -> u32 {
        self.main_count + self.side_count
    }
}

#[derive(Debug)]
struct DeckComparison {
    deck1_name: String,
    deck2_name: String,
    shared_cards: HashMap<String, (CardEntry, CardEntry)>,
    deck1_unique: HashMap<String, CardEntry>,
    deck2_unique: HashMap<String, CardEntry>,
}

async fn load_deck_from_id_or_url(identifier: &str) -> Result<ParsedDeck> {
    let cache_manager = crate::cache::CacheManager::new()?;

    // First, try to load as a deck ID
    if let Ok(Some(cached_item)) = cache_manager.get(identifier).await {
        // Check if it's a deck (has main_deck field)
        if cached_item.data.get("main_deck").is_some() {
            // It's a deck, deserialize it
            let deck: ParsedDeck = serde_json::from_value(cached_item.data)?;
            return Ok(deck);
        }

        // It's an article, fetch and parse it
        if let Some(url) = cached_item.data.get("link").and_then(|v| v.as_str()) {
            let parsed_decks = fetch_and_parse_decks(url).await?;
            if let Some(first_deck) = parsed_decks.into_iter().next() {
                return Ok(first_deck);
            } else {
                return Err(eyre!("No decks found in article"));
            }
        }
    }

    // If not found in cache and it's a URL, fetch it directly
    if identifier.starts_with("http://") || identifier.starts_with("https://") {
        let parsed_decks = fetch_and_parse_decks(identifier).await?;
        if let Some(first_deck) = parsed_decks.into_iter().next() {
            return Ok(first_deck);
        } else {
            return Err(eyre!("No decks found at URL"));
        }
    }

    Err(eyre!("Deck or article not found with ID: {}. If this is an old cached deck, try re-fetching it with 'mtg decks ranked show'", identifier))
}

async fn fetch_and_parse_decks(url: &str) -> Result<Vec<ParsedDeck>> {
    use scraper::{Html, Selector};

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .build()?;

    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(eyre!("Failed to fetch page: HTTP {}", response.status()));
    }

    let html_content = response.text().await?;
    let document = Html::parse_document(&html_content);
    let deck_list_selector = Selector::parse("deck-list").unwrap();
    let mut parsed_decks = Vec::new();

    for deck_element in document.select(&deck_list_selector) {
        let deck_title = deck_element.value().attr("deck-title").map(String::from);
        let subtitle = deck_element.value().attr("subtitle").map(String::from);
        let event_date = deck_element.value().attr("event-date").map(String::from);
        let event_name = deck_element.value().attr("event-name").map(String::from);
        let format = deck_element.value().attr("format").map(String::from);

        let main_deck_selector = Selector::parse("main-deck").unwrap();
        let mut main_deck = Vec::new();

        if let Some(main_deck_element) = deck_element.select(&main_deck_selector).next() {
            let deck_content = main_deck_element.text().collect::<String>();
            if let Ok(parsed_list) = crate::decks::parse_deck_list(&deck_content) {
                main_deck = parsed_list.main_deck;
            }
        }

        let sideboard_selector = Selector::parse("side-board").unwrap();
        let mut sideboard = Vec::new();

        if let Some(sideboard_element) = deck_element.select(&sideboard_selector).next() {
            let deck_content = sideboard_element.text().collect::<String>();
            if let Ok(parsed_list) = crate::decks::parse_deck_list(&deck_content) {
                sideboard = parsed_list.main_deck;
            }
        }

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

    Ok(parsed_decks)
}

impl CompareArgs {
    pub async fn run(&self) -> Result<()> {
        let deck1 = load_deck_from_id_or_url(&self.deck1).await?;
        let deck2 = load_deck_from_id_or_url(&self.deck2).await?;

        let comparison = compare_decks(&deck1, &deck2);

        display_comparison(&comparison);

        Ok(())
    }
}

fn compare_decks(deck1: &ParsedDeck, deck2: &ParsedDeck) -> DeckComparison {
    let mut deck1_cards: HashMap<String, CardEntry> = HashMap::new();
    let mut deck2_cards: HashMap<String, CardEntry> = HashMap::new();

    // Build deck1 card map
    for card in &deck1.main_deck {
        deck1_cards.insert(
            card.name.clone(),
            CardEntry {
                main_count: card.quantity,
                side_count: 0,
            },
        );
    }

    for card in &deck1.sideboard {
        deck1_cards
            .entry(card.name.clone())
            .and_modify(|e| e.side_count = card.quantity)
            .or_insert(CardEntry {
                main_count: 0,
                side_count: card.quantity,
            });
    }

    // Build deck2 card map
    for card in &deck2.main_deck {
        deck2_cards.insert(
            card.name.clone(),
            CardEntry {
                main_count: card.quantity,
                side_count: 0,
            },
        );
    }

    for card in &deck2.sideboard {
        deck2_cards
            .entry(card.name.clone())
            .and_modify(|e| e.side_count = card.quantity)
            .or_insert(CardEntry {
                main_count: 0,
                side_count: card.quantity,
            });
    }

    // Find shared and unique cards
    let mut shared_cards = HashMap::new();
    let mut deck1_unique = HashMap::new();

    for (card_name, entry1) in deck1_cards {
        if let Some(entry2) = deck2_cards.get(&card_name) {
            shared_cards.insert(card_name, (entry1, entry2.clone()));
        } else {
            deck1_unique.insert(card_name, entry1);
        }
    }

    let mut deck2_unique = HashMap::new();
    for (card_name, entry2) in deck2_cards {
        if !shared_cards.contains_key(&card_name) {
            deck2_unique.insert(card_name, entry2);
        }
    }

    DeckComparison {
        deck1_name: deck1.title.clone().unwrap_or_else(|| "Deck 1".to_string()),
        deck2_name: deck2.title.clone().unwrap_or_else(|| "Deck 2".to_string()),
        shared_cards,
        deck1_unique,
        deck2_unique,
    }
}

fn display_comparison(comparison: &DeckComparison) {
    println!("\n{}", "Deck Comparison".bold().underline());
    println!("{}: {}", "Deck 1".cyan(), comparison.deck1_name);
    println!("{}: {}", "Deck 2".cyan(), comparison.deck2_name);

    // Summary statistics
    let total_shared = comparison.shared_cards.len();
    let total_unique_1 = comparison.deck1_unique.len();
    let total_unique_2 = comparison.deck2_unique.len();

    println!("\n{}", "Summary".bold());
    println!("Shared cards: {}", total_shared.to_string().green());
    println!("Unique to Deck 1: {}", total_unique_1.to_string().yellow());
    println!("Unique to Deck 2: {}", total_unique_2.to_string().yellow());

    // Shared cards table
    if !comparison.shared_cards.is_empty() {
        println!("\n{}", "Shared Cards".bold().green());
        let mut table = new_table();
        table.add_row(row![
            "Card Name",
            "Deck 1 (Main/Side)",
            "Deck 2 (Main/Side)",
            "Difference"
        ]);

        let mut shared_sorted: Vec<_> = comparison.shared_cards.iter().collect();
        shared_sorted.sort_by_key(|(name, _)| name.as_str());

        for (card_name, (entry1, entry2)) in shared_sorted {
            let deck1_str = format!("{}/{}", entry1.main_count, entry1.side_count);
            let deck2_str = format!("{}/{}", entry2.main_count, entry2.side_count);
            let diff = (entry1.total() as i32 - entry2.total() as i32).abs();
            let diff_str = if diff > 0 {
                format!("±{}", diff).yellow().to_string()
            } else {
                "=".green().to_string()
            };

            table.add_row(row![card_name, deck1_str, deck2_str, diff_str]);
        }

        table.printstd();
    }

    // Unique to deck 1
    if !comparison.deck1_unique.is_empty() {
        println!(
            "\n{}",
            format!("Unique to {}", comparison.deck1_name)
                .bold()
                .yellow()
        );
        let mut table = new_table();
        table.add_row(row!["Card Name", "Main", "Side", "Total"]);

        let mut unique1_sorted: Vec<_> = comparison.deck1_unique.iter().collect();
        unique1_sorted.sort_by_key(|(name, _)| name.as_str());

        for (card_name, entry) in unique1_sorted {
            table.add_row(row![
                card_name,
                entry.main_count,
                entry.side_count,
                entry.total()
            ]);
        }

        table.printstd();
    }

    // Unique to deck 2
    if !comparison.deck2_unique.is_empty() {
        println!(
            "\n{}",
            format!("Unique to {}", comparison.deck2_name)
                .bold()
                .yellow()
        );
        let mut table = new_table();
        table.add_row(row!["Card Name", "Main", "Side", "Total"]);

        let mut unique2_sorted: Vec<_> = comparison.deck2_unique.iter().collect();
        unique2_sorted.sort_by_key(|(name, _)| name.as_str());

        for (card_name, entry) in unique2_sorted {
            table.add_row(row![
                card_name,
                entry.main_count,
                entry.side_count,
                entry.total()
            ]);
        }

        table.printstd();
    }
}
