use clap_stdin::MaybeStdin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod compare;
mod mcp;
mod ranked;
mod stats;
mod utils;

pub use mcp::analyze_deck_list_mcp;
pub use utils::{generate_short_hash, parse_deck_list};

use crate::prelude::*;

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
        /// Deck list input (use '-' for stdin, provide deck list as string, deck ID, or omit to read from stdin)
        #[clap(value_name = "DECK_LIST_OR_ID")]
        input: Option<MaybeStdin<String>>,

        /// Read deck list from file
        #[clap(short, long, value_name = "FILE")]
        file: Option<String>,

        /// Output format (pretty table or JSON)
        #[clap(long, default_value = "pretty")]
        format: String,
    },
    /// Access ranked deck lists from tournaments
    #[clap(name = "ranked")]
    Ranked {
        #[command(subcommand)]
        command: ranked::Commands,
    },
    /// Compare two deck lists
    #[clap(name = "compare")]
    Compare(compare::CompareArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckCard {
    pub quantity: u32,
    pub name: String,
    pub set_code: Option<String>,
    pub collector_number: Option<String>,
    pub card_details: Option<crate::scryfall::ScryfallCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckList {
    pub main_deck: Vec<DeckCard>,
    pub sideboard: Vec<DeckCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDeck {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub event_date: Option<String>,
    pub event_name: Option<String>,
    pub format: Option<String>,
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
            Commands::Stats {
                input,
                file,
                format,
            } => stats::run(input, file, format, global).await,
            Commands::Ranked { command } => ranked::run(command, global).await,
            Commands::Compare(args) => args.run().await,
        }
    }
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    app.run(global).await
}
