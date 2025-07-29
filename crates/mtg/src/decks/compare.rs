use crate::prelude::*;
use clap::Args;
use color_eyre::owo_colors::OwoColorize;
use mtg_core::cache::{CachedHttpClient, DiskCacheBuilder};
use mtg_core::decks::{compare_decks, load_deck_from_id_or_url};
use mtg_core::RankedDecksClient;
use prettytable::row;

#[derive(Args, Debug)]
pub struct CompareArgs {
    #[arg(help = "First deck ID or article ID")]
    deck1: String,

    #[arg(help = "Second deck ID or article ID")]
    deck2: String,
}

impl CompareArgs {
    pub async fn run(&self, global: &crate::Global) -> Result<()> {
        // Create cache and HTTP client - use same prefixes as other commands
        let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

        let http_client = CachedHttpClient::builder()
            .timeout(std::time::Duration::from_secs(global.timeout))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
            .cache_prefix("ranked_list_http")
            .build()?;

        // Create ranked decks client
        let ranked_client = RankedDecksClient::new(http_client, cache.clone());

        // Load both decks using mtg_core business logic
        let deck1 = load_deck_from_id_or_url(&self.deck1, &ranked_client, &cache).await?;
        let deck2 = load_deck_from_id_or_url(&self.deck2, &ranked_client, &cache).await?;

        // Compare decks using mtg_core business logic
        let comparison = compare_decks(&deck1, &deck2);

        // Display results (presentation layer)
        display_comparison(&comparison);

        Ok(())
    }
}

fn display_comparison(comparison: &mtg_core::decks::DeckComparison) {
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
