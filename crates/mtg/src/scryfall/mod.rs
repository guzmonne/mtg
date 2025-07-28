use crate::prelude::*;
use prettytable::{Cell, Row};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod autocomplete;
pub mod random;
pub mod search;
pub mod sets;
pub mod smart;

#[derive(Debug, clap::Parser)]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug)]
enum QueryIssue {
    UnknownKeyword(String),
    #[allow(dead_code)]
    InvalidOperator(String),
    #[allow(dead_code)]
    MalformedExpression(String),
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Smart search that auto-detects what you're looking for (recommended for LLMs)
    Find {
        /// What to search for - can be card name, "set collector", Arena ID, or search query
        query: String,

        /// Display result in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,

        /// Page number for search results (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,
    },

    /// Search for Magic cards using Scryfall advanced search
    Search {
        /// Search query using Scryfall syntax (e.g., "c:red t:creature", "Lightning Bolt")
        query: String,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,

        /// Page number for pagination (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,

        /// Sort order (name, set, released, rarity, color, usd, tix, eur, cmc, power, toughness, edhrec, penny, artist, review)
        #[clap(long, default_value = "name")]
        order: String,

        /// Sort direction (auto, asc, desc)
        #[clap(long, default_value = "auto")]
        dir: String,

        /// Include extra cards (tokens, emblems, etc.)
        #[clap(long)]
        include_extras: bool,

        /// Include multilingual cards
        #[clap(long)]
        include_multilingual: bool,

        /// Include variations (different printings)
        #[clap(long)]
        include_variations: bool,

        /// Unique mode (cards, prints, art)
        #[clap(long, default_value = "cards")]
        unique: String,

        /// Return results in CSV format
        #[clap(long)]
        csv: bool,
    },

    /// Get a specific Magic card by exact name
    Named {
        /// Card name to fetch
        name: String,

        /// Display result in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,

        /// Set code to get specific printing (optional)
        #[clap(long, short)]
        set: Option<String>,
    },

    /// Get a card by Scryfall UUID
    Id {
        /// Scryfall UUID of the card
        id: String,

        /// Display result in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Get a card by set code and collector number
    Collector {
        /// Set code (e.g., "ktk", "war", "m21")
        set_code: String,

        /// Collector number (e.g., "96", "001", "★")
        collector_number: String,

        /// Language code (optional, e.g., "en", "ja", "de")
        #[clap(long, short)]
        lang: Option<String>,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a card by Arena ID
    Arena {
        /// Arena ID number
        arena_id: u32,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a card by MTGO ID
    Mtgo {
        /// MTGO ID number
        mtgo_id: u32,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a card by Multiverse ID
    Multiverse {
        /// Multiverse ID number
        multiverse_id: u32,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a card by TCGPlayer ID
    Tcgplayer {
        /// TCGPlayer product ID
        tcgplayer_id: u32,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a card by Cardmarket ID
    Cardmarket {
        /// Cardmarket product ID
        cardmarket_id: u32,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get a random card
    Random {
        /// Search query to filter random results (optional)
        #[clap(long, short)]
        query: Option<String>,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Search for cards with advanced filters
    Advanced {
        /// Card name (partial matching allowed)
        #[clap(long, short)]
        name: Option<String>,

        /// Oracle text to search for
        #[clap(long, short)]
        oracle: Option<String>,

        /// Card type (e.g., "creature", "instant")
        #[clap(long, short = 't')]
        card_type: Option<String>,

        /// Colors (e.g., "w", "wu", "wubrg")
        #[clap(long, short)]
        colors: Option<String>,

        /// Color identity for commander (e.g., "w", "wu")
        #[clap(long)]
        identity: Option<String>,

        /// Mana cost (e.g., "{2}{U}", "3")
        #[clap(long, short = 'm')]
        mana: Option<String>,

        /// Mana value/CMC (e.g., "3", ">=4", "<2")
        #[clap(long)]
        mv: Option<String>,

        /// Power (e.g., "2", ">=3", "*")
        #[clap(long, short)]
        power: Option<String>,

        /// Toughness (e.g., "2", ">=3", "*")
        #[clap(long)]
        toughness: Option<String>,

        /// Loyalty (e.g., "3", ">=4")
        #[clap(long)]
        loyalty: Option<String>,

        /// Set code (e.g., "ktk", "war")
        #[clap(long, short)]
        set: Option<String>,

        /// Rarity (common, uncommon, rare, mythic)
        #[clap(long, short)]
        rarity: Option<String>,

        /// Artist name
        #[clap(long, short)]
        artist: Option<String>,

        /// Flavor text to search for
        #[clap(long)]
        flavor: Option<String>,

        /// Format legality (e.g., "standard", "modern", "legacy")
        #[clap(long, short = 'f')]
        format: Option<String>,

        /// Language (e.g., "en", "ja", "de")
        #[clap(long, short = 'l')]
        language: Option<String>,

        /// Display results in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Page number for pagination (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,

        /// Sort order
        #[clap(long, default_value = "name")]
        order: String,

        /// Sort direction (auto, asc, desc)
        #[clap(long, default_value = "auto")]
        dir: String,

        /// Include extra cards (tokens, emblems, etc.)
        #[clap(long)]
        include_extras: bool,

        /// Include multilingual cards
        #[clap(long)]
        include_multilingual: bool,

        /// Include variations (different printings)
        #[clap(long)]
        include_variations: bool,

        /// Unique mode (cards, prints, art)
        #[clap(long, default_value = "cards")]
        unique: String,
    },

    /// Get autocomplete suggestions for card names
    Autocomplete {
        /// Partial card name to get suggestions for
        query: String,

        /// Include extra cards in suggestions
        #[clap(long)]
        include_extras: bool,
    },

    /// Search for creatures with optional filters (convenience command)
    Creatures {
        /// Color filter (e.g., "red", "wu", "bant")
        #[clap(long, short)]
        color: Option<String>,

        /// Minimum power (e.g., "3", ">=5")
        #[clap(long)]
        power: Option<String>,

        /// Minimum toughness (e.g., "3", ">=5")
        #[clap(long)]
        toughness: Option<String>,

        /// Mana value filter (e.g., "3", "<=4", ">=2")
        #[clap(long)]
        mana_value: Option<String>,

        /// Format legality (e.g., "standard", "modern")
        #[clap(long, short)]
        format: Option<String>,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Search for instants with optional filters (convenience command)
    Instants {
        /// Color filter (e.g., "blue", "wu", "jeskai")
        #[clap(long, short)]
        color: Option<String>,

        /// Mana value filter (e.g., "1", "<=3", ">=2")
        #[clap(long)]
        mana_value: Option<String>,

        /// Format legality (e.g., "standard", "modern")
        #[clap(long, short)]
        format: Option<String>,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Search for sorceries with optional filters (convenience command)
    Sorceries {
        /// Color filter (e.g., "black", "bg", "golgari")
        #[clap(long, short)]
        color: Option<String>,

        /// Mana value filter (e.g., "4", "<=6", ">=3")
        #[clap(long)]
        mana_value: Option<String>,

        /// Format legality (e.g., "standard", "modern")
        #[clap(long, short)]
        format: Option<String>,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Search for planeswalkers with optional filters (convenience command)
    Planeswalkers {
        /// Color filter (e.g., "white", "uw", "azorius")
        #[clap(long, short)]
        color: Option<String>,

        /// Loyalty filter (e.g., "3", ">=4")
        #[clap(long)]
        loyalty: Option<String>,

        /// Format legality (e.g., "standard", "modern")
        #[clap(long, short)]
        format: Option<String>,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Search for commanders (legendary creatures) with optional filters
    Commanders {
        /// Color identity (e.g., "bant", "wu", "esper")
        #[clap(long, short)]
        identity: Option<String>,

        /// Mana value filter (e.g., "4", "<=6", ">=3")
        #[clap(long)]
        mana_value: Option<String>,

        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Build a search query interactively with prompts
    Build {
        /// Display results in a formatted table (default: true)
        #[clap(long, default_value = "true")]
        pretty: bool,

        /// Force JSON output instead of pretty table
        #[clap(long)]
        json: bool,
    },

    /// Manage and browse Magic sets
    Sets {
        #[command(subcommand)]
        command: SetCommands,
    },
}

#[derive(Debug, clap::Parser)]
pub enum SetCommands {
    /// List all Magic sets
    List {
        /// Filter by set type (core, expansion, masters, etc.)
        #[clap(long)]
        set_type: Option<String>,

        /// Filter sets released after this date (YYYY-MM-DD)
        #[clap(long)]
        released_after: Option<String>,

        /// Filter sets released before this date (YYYY-MM-DD)
        #[clap(long)]
        released_before: Option<String>,

        /// Filter by block name
        #[clap(long)]
        block: Option<String>,

        /// Filter digital-only sets
        #[clap(long)]
        digital_only: Option<bool>,

        /// Display results in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// Get information about a specific set
    Get {
        /// Set code (e.g., "ktk", "war", "m21")
        set_code: String,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },

    /// List available set types
    Types,
}

/// Generic list object for Scryfall API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    /// Always "list"
    pub object: String,
    /// Array of requested objects
    pub data: Vec<T>,
    /// True if this List is paginated and there is a page beyond the current page
    pub has_more: bool,
    /// URI to next page if available
    pub next_page: Option<String>,
    /// Total number of cards found across all pages (for card lists)
    pub total_cards: Option<u32>,
    /// Non-fatal warnings from the API
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    object: String,
    id: String,
    oracle_id: Option<String>,
    multiverse_ids: Option<Vec<u32>>,
    mtgo_id: Option<u32>,
    arena_id: Option<u32>,
    tcgplayer_id: Option<u32>,
    cardmarket_id: Option<u32>,
    pub name: String,
    lang: String,
    released_at: String,
    uri: String,
    scryfall_uri: String,
    layout: String,
    highres_image: bool,
    image_status: String,
    image_uris: Option<Value>,
    pub mana_cost: Option<String>,
    pub cmc: f64,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    colors: Option<Vec<String>>,
    pub color_identity: Vec<String>,
    keywords: Option<Vec<String>>,
    pub legalities: Value,
    games: Vec<String>,
    reserved: bool,
    foil: bool,
    nonfoil: bool,
    finishes: Vec<String>,
    oversized: bool,
    promo: bool,
    reprint: bool,
    variation: bool,
    set_id: String,
    pub set: String,
    pub set_name: String,
    set_type: String,
    set_uri: String,
    set_search_uri: String,
    scryfall_set_uri: String,
    rulings_uri: String,
    prints_search_uri: String,
    pub collector_number: String,
    digital: bool,
    pub rarity: String,
    pub flavor_text: Option<String>,
    card_back_id: Option<String>,
    pub artist: Option<String>,
    artist_ids: Option<Vec<String>>,
    illustration_id: Option<String>,
    border_color: String,
    frame: String,
    security_stamp: Option<String>,
    full_art: bool,
    textless: bool,
    booster: bool,
    story_spotlight: bool,
    edhrec_rank: Option<u32>,
    penny_rank: Option<u32>,
    prices: Option<Value>,
    related_uris: Option<Value>,
    purchase_uris: Option<Value>,
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    match app.command {
        SubCommands::Search {
            query,
            pretty,
            json,
            page,
            order,
            dir,
            include_extras,
            include_multilingual,
            include_variations,
            unique,
            csv,
        } => {
            search::run(
                search::Params {
                    query,
                    pretty: !json && pretty,
                    page,
                    order,
                    dir,
                    include_extras,
                    include_multilingual,
                    include_variations,
                    unique,
                    csv,
                },
                global,
            )
            .await
        }
        SubCommands::Named {
            name,
            pretty,
            json,
            set,
        } => search::by_name(&name, !json && pretty, set.as_deref(), global).await,
        SubCommands::Id { id, pretty, json } => search::by_id(&id, !json && pretty, global).await,
        SubCommands::Collector {
            set_code,
            collector_number,
            lang,
            pretty,
        } => {
            search::by_collector(
                &set_code,
                &collector_number,
                lang.as_deref(),
                pretty,
                global,
            )
            .await
        }
        SubCommands::Arena { arena_id, pretty } => {
            search::by_arena_id(arena_id, pretty, global).await
        }
        SubCommands::Mtgo { mtgo_id, pretty } => search::by_mtgo_id(mtgo_id, pretty, global).await,
        SubCommands::Multiverse {
            multiverse_id,
            pretty,
        } => search::by_multiverse_id(multiverse_id, pretty, global).await,
        SubCommands::Tcgplayer {
            tcgplayer_id,
            pretty,
        } => search::by_tcgplayer_id(tcgplayer_id, pretty, global).await,
        SubCommands::Cardmarket {
            cardmarket_id,
            pretty,
        } => search::by_cardmarket_id(cardmarket_id, pretty, global).await,
        SubCommands::Random { query, pretty } => {
            random::run(query.as_deref(), pretty, global).await
        }
        SubCommands::Advanced {
            name,
            oracle,
            card_type,
            colors,
            identity,
            mana,
            mv,
            power,
            toughness,
            loyalty,
            set,
            rarity,
            artist,
            flavor,
            format,
            language,
            pretty,
            page,
            order,
            dir,
            include_extras,
            include_multilingual,
            include_variations,
            unique,
        } => {
            search::advanced(
                search::AdvancedParams {
                    name,
                    oracle,
                    card_type,
                    colors,
                    identity,
                    mana,
                    mv,
                    power,
                    toughness,
                    loyalty,
                    set,
                    rarity,
                    artist,
                    flavor,
                    format,
                    language,
                    pretty,
                    page,
                    order,
                    dir,
                    include_extras,
                    include_multilingual,
                    include_variations,
                    unique,
                },
                global,
            )
            .await
        }
        SubCommands::Autocomplete {
            query,
            include_extras,
        } => autocomplete::run(&query, include_extras, global).await,
        SubCommands::Find {
            query,
            pretty,
            json,
            page,
        } => smart::run(&query, !json && pretty, page, global).await,
        SubCommands::Creatures {
            color,
            power,
            toughness,
            mana_value,
            format,
            pretty,
            json,
        } => {
            search::creatures(
                color,
                power,
                toughness,
                mana_value,
                format,
                !json && pretty,
                global,
            )
            .await
        }
        SubCommands::Instants {
            color,
            mana_value,
            format,
            pretty,
            json,
        } => search::instants(color, mana_value, format, !json && pretty, global).await,
        SubCommands::Sorceries {
            color,
            mana_value,
            format,
            pretty,
            json,
        } => search::sorceries(color, mana_value, format, !json && pretty, global).await,
        SubCommands::Planeswalkers {
            color,
            loyalty,
            format,
            pretty,
            json,
        } => search::planeswalkers(color, loyalty, format, !json && pretty, global).await,
        SubCommands::Commanders {
            identity,
            mana_value,
            pretty,
            json,
        } => search::commanders(identity, mana_value, !json && pretty, global).await,
        SubCommands::Build { pretty, json } => {
            build_query_interactive(!json && pretty, global).await
        }
        SubCommands::Sets { command } => handle_sets_command(command, global).await,
    }
}

async fn handle_sets_command(command: SetCommands, global: crate::Global) -> Result<()> {
    use mtg_core::scryfall::sets::{SetListParams, SetType};
    use std::str::FromStr;

    match command {
        SetCommands::List {
            set_type,
            released_after,
            released_before,
            block,
            digital_only,
            pretty,
        } => {
            let set_type_enum = if let Some(type_str) = set_type {
                match SetType::from_str(&type_str) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        aeprintln!(
                            "Invalid set type: {type_str}. Use 'mtg scryfall sets types' to see available types."
                        );
                        return Ok(());
                    }
                }
            } else {
                None
            };

            let params = SetListParams {
                set_type: set_type_enum,
                released_after,
                released_before,
                block,
                digital_only,
            };

            let sets_list = sets::list_sets(params, global).await?;

            if pretty {
                display_sets_table(&sets_list)?;
            } else {
                println!("{}", serde_json::to_string_pretty(&sets_list)?);
            }
        }
        SetCommands::Get { set_code, pretty } => {
            let set = sets::get_set_by_code(&set_code, global).await?;

            if pretty {
                display_set_details(&set)?;
            } else {
                println!("{}", serde_json::to_string_pretty(&set)?);
            }
        }
        SetCommands::Types => {
            println!("Available set types:");
            println!();
            for set_type in SetType::all() {
                println!("  {:<20} - {}", set_type.as_str(), set_type.description());
            }
        }
    }
    Ok(())
}

fn display_sets_table(sets_list: &mtg_core::scryfall::sets::ScryfallSetList) -> Result<()> {
    let mut table = new_table();
    table.add_row(Row::new(vec![
        Cell::new("Code"),
        Cell::new("Name"),
        Cell::new("Type"),
        Cell::new("Released"),
        Cell::new("Cards"),
        Cell::new("Digital"),
    ]));

    for set in &sets_list.data {
        table.add_row(Row::new(vec![
            Cell::new(&set.code.to_uppercase()),
            Cell::new(&set.name),
            Cell::new(&set.set_type),
            Cell::new(set.released_at.as_deref().unwrap_or("Unknown")),
            Cell::new(&set.card_count.to_string()),
            Cell::new(if set.digital { "Yes" } else { "No" }),
        ]));
    }

    table.printstd();

    // Display warnings if any
    if let Some(warnings) = &sets_list.warnings {
        aeprintln!();
        aeprintln!("⚠️  Warnings:");
        for warning in warnings {
            aeprintln!("   • {warning}");
        }
    }

    aeprintln!();
    aeprintln!("Found {} sets", sets_list.data.len());

    Ok(())
}

fn display_set_details(set: &mtg_core::scryfall::sets::ScryfallSet) -> Result<()> {
    use prettytable::format;

    let mut table = new_table();
    table.set_format(*format::consts::FORMAT_CLEAN);

    table.add_row(Row::new(vec![Cell::new("Property"), Cell::new("Value")]));
    table.add_row(Row::new(vec![
        Cell::new("Code"),
        Cell::new(&set.code.to_uppercase()),
    ]));
    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(&set.name)]));
    table.add_row(Row::new(vec![Cell::new("Type"), Cell::new(&set.set_type)]));

    if let Some(released) = &set.released_at {
        table.add_row(Row::new(vec![Cell::new("Released"), Cell::new(released)]));
    }

    table.add_row(Row::new(vec![
        Cell::new("Card Count"),
        Cell::new(&set.card_count.to_string()),
    ]));

    if let Some(printed_size) = set.printed_size {
        table.add_row(Row::new(vec![
            Cell::new("Printed Size"),
            Cell::new(&printed_size.to_string()),
        ]));
    }

    if let Some(block) = &set.block {
        table.add_row(Row::new(vec![Cell::new("Block"), Cell::new(block)]));
    }

    table.add_row(Row::new(vec![
        Cell::new("Digital Only"),
        Cell::new(if set.digital { "Yes" } else { "No" }),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Foil Only"),
        Cell::new(if set.foil_only { "Yes" } else { "No" }),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Nonfoil Only"),
        Cell::new(if set.nonfoil_only { "Yes" } else { "No" }),
    ]));

    if let Some(mtgo_code) = &set.mtgo_code {
        table.add_row(Row::new(vec![Cell::new("MTGO Code"), Cell::new(mtgo_code)]));
    }

    if let Some(arena_code) = &set.arena_code {
        table.add_row(Row::new(vec![
            Cell::new("Arena Code"),
            Cell::new(arena_code),
        ]));
    }

    table.printstd();
    Ok(())
}

#[derive(Debug)]
enum QueryIntent {
    ExactCardName(String),
    SetCollector(String, String),
    ArenaId(u32),
    MtgoId(u32),
    ScryfallId(String),
    SearchQuery(String),
}

fn detect_query_intent(query: &str) -> Option<QueryIntent> {
    let query = query.trim();

    // Check for Scryfall UUID (36 characters with hyphens)
    if query.len() == 36 && query.chars().filter(|&c| c == '-').count() == 4 {
        return Some(QueryIntent::ScryfallId(query.to_string()));
    }

    // Check for pure numbers (Arena/MTGO IDs)
    if let Ok(id) = query.parse::<u32>() {
        // Heuristic: Arena IDs are typically larger than MTGO IDs
        if id > 50000 {
            return Some(QueryIntent::ArenaId(id));
        } else {
            return Some(QueryIntent::MtgoId(id));
        }
    }

    // Check for "SET COLLECTOR" pattern (e.g., "ktk 96", "war 001")
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() == 2 {
        let potential_set = parts[0].to_lowercase();
        let potential_collector = parts[1];

        // Check if first part looks like a set code (2-4 characters, mostly letters)
        if potential_set.len() >= 2
            && potential_set.len() <= 4
            && potential_set.chars().all(|c| c.is_alphanumeric())
        {
            // Check if second part looks like a collector number
            if potential_collector.chars().any(|c| c.is_ascii_digit()) {
                return Some(QueryIntent::SetCollector(
                    potential_set,
                    potential_collector.to_string(),
                ));
            }
        }
    }

    // Check if it contains Scryfall search syntax
    if query.contains(':')
        || query.contains(">=")
        || query.contains("<=")
        || query.contains("!=")
        || query.contains('>')
        || query.contains('<')
    {
        return Some(QueryIntent::SearchQuery(query.to_string()));
    }

    // Check for common search patterns
    let lower_query = query.to_lowercase();
    if lower_query.contains(" creature")
        || lower_query.contains(" instant")
        || lower_query.contains(" sorcery")
        || lower_query.contains(" artifact")
        || lower_query.contains(" enchantment")
        || lower_query.contains(" planeswalker")
    {
        return Some(QueryIntent::SearchQuery(query.to_string()));
    }

    // If it's a simple phrase without special characters, treat as exact card name
    if !query.contains('"') && !query.contains('(') && !query.contains('[') {
        return Some(QueryIntent::ExactCardName(query.to_string()));
    }

    // Default to search query
    Some(QueryIntent::SearchQuery(query.to_string()))
}

// Interactive query builder
async fn build_query_interactive(pretty: bool, global: crate::Global) -> Result<()> {
    use std::io::{self, Write};

    println!("🔍 Interactive Query Builder");
    println!("Press Enter to skip any option, or type 'help' for examples\n");

    let mut query_parts = Vec::new();

    // Card type
    print!("Card type (creature, instant, sorcery, etc.): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let card_type = input.trim();
    if !card_type.is_empty() && card_type != "help" {
        query_parts.push(format!("t:{card_type}"));
    } else if card_type == "help" {
        println!("Examples: creature, instant, sorcery, artifact, enchantment, planeswalker, land");
    }

    // Colors
    print!("Colors (w/u/b/r/g, or names like 'red', 'azorius'): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let colors = input.trim();
    if !colors.is_empty() && colors != "help" {
        query_parts.push(format_color_query(colors));
    } else if colors == "help" {
        println!("Examples: red, wu, bant, esper, wubrg, colorless");
    }

    // Mana value
    print!("Mana value (3, <=4, >=2, etc.): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let mana_value = input.trim();
    if !mana_value.is_empty() && mana_value != "help" {
        query_parts.push(format!("mv{}", format_comparison(mana_value)));
    } else if mana_value == "help" {
        println!("Examples: 3, <=4, >=2, <5, >1");
    }

    // Format
    print!("Format (standard, modern, legacy, etc.): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let format = input.trim();
    if !format.is_empty() && format != "help" {
        query_parts.push(format!("f:{format}"));
    } else if format == "help" {
        println!("Examples: standard, modern, legacy, vintage, commander, pioneer");
    }

    // Oracle text
    print!("Oracle text contains: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let oracle = input.trim();
    if !oracle.is_empty() && oracle != "help" {
        query_parts.push(format!("o:{oracle}"));
    } else if oracle == "help" {
        println!("Examples: flying, \"draw a card\", counter, destroy");
    }

    if query_parts.is_empty() {
        println!("No search criteria specified. Showing random card instead.");
        return random::run(None, pretty, global).await;
    }

    let query = query_parts.join(" ");
    println!("\n🔍 Generated query: {query}");
    println!("Searching...\n");

    search::run(
        search::Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
}

// Helper function to format color queries
fn format_color_query(color: &str) -> String {
    let color = color.to_lowercase();

    // Handle common color names and guild/shard names
    match color.as_str() {
        "white" => "c:w".to_string(),
        "blue" => "c:u".to_string(),
        "black" => "c:b".to_string(),
        "red" => "c:r".to_string(),
        "green" => "c:g".to_string(),
        "colorless" => "c:colorless".to_string(),
        "multicolor" => "c:m".to_string(),
        // Guild names
        "azorius" => "c:wu".to_string(),
        "dimir" => "c:ub".to_string(),
        "rakdos" => "c:br".to_string(),
        "gruul" => "c:rg".to_string(),
        "selesnya" => "c:gw".to_string(),
        "orzhov" => "c:wb".to_string(),
        "izzet" => "c:ur".to_string(),
        "golgari" => "c:bg".to_string(),
        "boros" => "c:rw".to_string(),
        "simic" => "c:gu".to_string(),
        // Shard names
        "bant" => "c:gwu".to_string(),
        "esper" => "c:wub".to_string(),
        "grixis" => "c:ubr".to_string(),
        "jund" => "c:brg".to_string(),
        "naya" => "c:rgw".to_string(),
        // Wedge names
        "abzan" => "c:wbg".to_string(),
        "jeskai" => "c:urw".to_string(),
        "sultai" => "c:bgu".to_string(),
        "mardu" => "c:rwb".to_string(),
        "temur" => "c:gur".to_string(),
        _ => {
            // If it's already in the right format or single letters, use as-is
            if color.starts_with("c:") {
                color
            } else {
                format!("c:{color}")
            }
        }
    }
}

// Helper function to format color identity queries
fn format_color_identity_query(identity: &str) -> String {
    let identity = identity.to_lowercase();

    // Handle common identity names
    match identity.as_str() {
        "white" => "id:w".to_string(),
        "blue" => "id:u".to_string(),
        "black" => "id:b".to_string(),
        "red" => "id:r".to_string(),
        "green" => "id:g".to_string(),
        "colorless" => "id:colorless".to_string(),
        // Guild names
        "azorius" => "id:wu".to_string(),
        "dimir" => "id:ub".to_string(),
        "rakdos" => "id:br".to_string(),
        "gruul" => "id:rg".to_string(),
        "selesnya" => "id:gw".to_string(),
        "orzhov" => "id:wb".to_string(),
        "izzet" => "id:ur".to_string(),
        "golgari" => "id:bg".to_string(),
        "boros" => "id:rw".to_string(),
        "simic" => "id:gu".to_string(),
        // Shard names
        "bant" => "id:gwu".to_string(),
        "esper" => "id:wub".to_string(),
        "grixis" => "id:ubr".to_string(),
        "jund" => "id:brg".to_string(),
        "naya" => "id:rgw".to_string(),
        // Wedge names
        "abzan" => "id:wbg".to_string(),
        "jeskai" => "id:urw".to_string(),
        "sultai" => "id:bgu".to_string(),
        "mardu" => "id:rwb".to_string(),
        "temur" => "id:gur".to_string(),
        _ => {
            if identity.starts_with("id:") {
                identity
            } else {
                format!("id:{identity}")
            }
        }
    }
}

// Helper function to format comparison operators
fn format_comparison(value: &str) -> String {
    if value.starts_with(">=")
        || value.starts_with("<=")
        || value.starts_with("!=")
        || value.starts_with('>')
        || value.starts_with('<')
    {
        value.to_string()
    } else {
        format!("={value}")
    }
}

// Enhanced error handling with suggestions
pub(crate) fn enhance_error_message(error: &str, query: &str) -> String {
    let error_lower = error.to_lowercase();
    let query_lower = query.to_lowercase();

    // Common error patterns and suggestions
    if error_lower.contains("no card found") || error_lower.contains("not found") {
        let mut suggestions = Vec::new();

        // Check for common misspellings
        if let Some(suggestion) = suggest_card_name_correction(&query_lower) {
            suggestions.push(format!(
                "Did you mean '{}'? Try: mtg scryfall find \"{}\"",
                suggestion, suggestion
            ));
        }

        // Suggest using search instead of exact name
        if !query.contains(':') && !query.contains(' ') {
            suggestions.push(format!(
                "Try searching instead: mtg scryfall find \"{}\"",
                query
            ));
        }

        // Suggest autocomplete
        suggestions.push(format!(
            "Get suggestions: mtg scryfall autocomplete \"{}\"",
            query
        ));

        if suggestions.is_empty() {
            format!("{}\n💡 Try: mtg scryfall help", error)
        } else {
            format!("{}\n💡 Suggestions:\n  {}", error, suggestions.join("\n  "))
        }
    } else if error_lower.contains("invalid search syntax") || error_lower.contains("syntax") {
        let mut suggestions = Vec::new();

        // Check for common syntax errors
        if let Some(correction) = suggest_query_correction(query) {
            suggestions.push(format!("Try: mtg scryfall search \"{}\"", correction));
        }

        suggestions.push("Use the query builder: mtg scryfall build".to_string());
        suggestions.push("See examples: mtg scryfall help".to_string());

        format!("{}\n💡 Suggestions:\n  {}", error, suggestions.join("\n  "))
    } else if error_lower.contains("rate limit") {
        format!(
            "{}\n💡 Wait a moment and try again. The CLI uses caching to reduce API calls.",
            error
        )
    } else if error_lower.contains("network") || error_lower.contains("connection") {
        format!(
            "{}\n💡 Check your internet connection and try again.",
            error
        )
    } else {
        // Generic enhancement
        format!("{}\n💡 Need help? Try: mtg scryfall help", error)
    }
}

// Suggest corrections for common card name misspellings
fn suggest_card_name_correction(query: &str) -> Option<String> {
    let common_corrections = [
        ("lightning bold", "Lightning Bolt"),
        ("lightning bolt", "Lightning Bolt"),
        ("counterspel", "Counterspell"),
        ("sol ring", "Sol Ring"),
        ("black lotus", "Black Lotus"),
        ("time walk", "Time Walk"),
        ("ancestral recall", "Ancestral Recall"),
        ("mox sapphire", "Mox Sapphire"),
        ("mox ruby", "Mox Ruby"),
        ("mox pearl", "Mox Pearl"),
        ("mox emerald", "Mox Emerald"),
        ("mox jet", "Mox Jet"),
        ("force of will", "Force of Will"),
        ("brainstorm", "Brainstorm"),
        ("swords to plowshare", "Swords to Plowshares"),
        ("path to exile", "Path to Exile"),
        ("birds of paradise", "Birds of Paradise"),
        ("llanowar elfs", "Llanowar Elves"),
        ("dark ritual", "Dark Ritual"),
        ("giant growth", "Giant Growth"),
    ];

    for (misspelling, correction) in &common_corrections {
        if query.contains(misspelling) {
            return Some(correction.to_string());
        }
    }

    None
}

// Suggest corrections for common query syntax errors
fn suggest_query_correction(query: &str) -> Option<String> {
    let mut corrected = query.to_string();
    let mut changed = false;

    // Common syntax corrections
    let corrections = [
        ("colour:", "c:"),
        ("color:", "c:"),
        ("type:", "t:"),
        ("oracle:", "o:"),
        ("manavalue:", "mv:"),
        ("mana_value:", "mv:"),
        ("manacost:", "m:"),
        ("mana_cost:", "m:"),
        ("power:", "pow:"),
        ("toughness:", "tou:"),
        ("loyalty:", "loy:"),
        ("rarity:", "r:"),
        ("set:", "s:"),
        ("format:", "f:"),
        ("artist:", "a:"),
        ("flavor:", "ft:"),
        ("identity:", "id:"),
        ("cmc:", "mv:"),
        ("converted mana cost:", "mv:"),
        ("creature type", "t:creature"),
        ("instant spell", "t:instant"),
        ("sorcery spell", "t:sorcery"),
        ("artifact card", "t:artifact"),
        ("enchantment card", "t:enchantment"),
        ("planeswalker card", "t:planeswalker"),
        ("land card", "t:land"),
        ("red card", "c:red"),
        ("blue card", "c:blue"),
        ("white card", "c:white"),
        ("black card", "c:black"),
        ("green card", "c:green"),
        ("multicolor card", "c:m"),
        ("colorless card", "c:colorless"),
    ];

    for (wrong, right) in &corrections {
        if corrected.to_lowercase().contains(&wrong.to_lowercase()) {
            corrected = corrected
                .to_lowercase()
                .replace(&wrong.to_lowercase(), right);
            changed = true;
        }
    }

    // Fix comparison operators
    if corrected.contains("equal to") {
        corrected = corrected.replace("equal to", "=");
        changed = true;
    }
    if corrected.contains("greater than or equal") {
        corrected = corrected.replace("greater than or equal", ">=");
        changed = true;
    }
    if corrected.contains("less than or equal") {
        corrected = corrected.replace("less than or equal", "<=");
        changed = true;
    }
    if corrected.contains("greater than") {
        corrected = corrected.replace("greater than", ">");
        changed = true;
    }
    if corrected.contains("less than") {
        corrected = corrected.replace("less than", "<");
        changed = true;
    }

    if changed {
        Some(corrected)
    } else {
        None
    }
}

// Validate query syntax and provide suggestions
fn validate_and_suggest_query(query: &str) -> Result<String, String> {
    let query = query.trim();

    // Check for empty query
    if query.is_empty() {
        return Err("Empty query. Try: mtg scryfall help".to_string());
    }

    // Check for common mistakes
    let issues = find_query_issues(query);
    if !issues.is_empty() {
        let suggestions: Vec<String> = issues
            .iter()
            .map(|issue| match issue {
                QueryIssue::UnknownKeyword(keyword) => {
                    if let Some(suggestion) = suggest_keyword_correction(keyword) {
                        format!(
                            "Unknown keyword '{}'. Did you mean '{}'?",
                            keyword, suggestion
                        )
                    } else {
                        format!(
                            "Unknown keyword '{}'. See 'mtg scryfall help' for valid keywords.",
                            keyword
                        )
                    }
                }
                QueryIssue::InvalidOperator(op) => {
                    format!("Invalid operator '{}'. Use: =, !=, <, <=, >, >=", op)
                }
                QueryIssue::MalformedExpression(expr) => {
                    format!("Malformed expression '{}'. Check syntax.", expr)
                }
            })
            .collect();

        return Err(format!(
            "Query validation issues:\n  {}",
            suggestions.join("\n  ")
        ));
    }

    Ok(query.to_string())
}

fn find_query_issues(query: &str) -> Vec<QueryIssue> {
    let mut issues = Vec::new();

    // Valid Scryfall keywords
    let valid_keywords = [
        "c",
        "color",
        "colors",
        "id",
        "identity",
        "m",
        "mana",
        "mv",
        "cmc",
        "t",
        "type",
        "o",
        "oracle",
        "pow",
        "power",
        "tou",
        "toughness",
        "loy",
        "loyalty",
        "r",
        "rarity",
        "s",
        "set",
        "f",
        "format",
        "a",
        "artist",
        "ft",
        "flavor",
        "is",
        "not",
        "cn",
        "number",
        "lang",
        "language",
        "year",
        "frame",
        "border",
        "game",
        "legal",
        "banned",
        "restricted",
        "new",
        "old",
        "reprint",
        "firstprint",
        "unique",
        "art",
        "prints",
        "usd",
        "eur",
        "tix",
        "penny",
    ];

    // Check for unknown keywords
    for part in query.split_whitespace() {
        if part.contains(':') {
            let keyword = part.split(':').next().unwrap_or("");
            if !keyword.is_empty() && !valid_keywords.contains(&keyword.to_lowercase().as_str()) {
                issues.push(QueryIssue::UnknownKeyword(keyword.to_string()));
            }
        }
    }

    issues
}

fn suggest_keyword_correction(keyword: &str) -> Option<String> {
    let keyword_lower = keyword.to_lowercase();

    let corrections = [
        ("colour", "c"),
        ("color", "c"),
        ("type", "t"),
        ("oracle", "o"),
        ("manavalue", "mv"),
        ("manacost", "m"),
        ("power", "pow"),
        ("toughness", "tou"),
        ("loyalty", "loy"),
        ("rarity", "r"),
        ("set", "s"),
        ("format", "f"),
        ("artist", "a"),
        ("flavor", "ft"),
        ("identity", "id"),
        ("cmc", "mv"),
    ];

    for (wrong, right) in &corrections {
        if keyword_lower == *wrong {
            return Some(right.to_string());
        }
    }

    None
}

fn parse_scryfall_response(response_text: &str) -> Result<search::Response> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;

    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response using the new comprehensive error structure
            let scryfall_error: crate::error::ScryfallError = serde_json::from_str(response_text)?;
            let api_error = crate::error::ScryfallApiError::from_scryfall_error(scryfall_error);
            let enhanced_error = enhance_error_message(&api_error.to_string(), "");
            return Err(eyre!("{}", enhanced_error));
        }
    }

    // Parse as search response
    let search_response: search::Response = serde_json::from_str(response_text)?;
    Ok(search_response)
}

fn parse_scryfall_card_response(response_text: &str) -> Result<Card> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;

    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response using the new comprehensive error structure
            let scryfall_error: crate::error::ScryfallError = serde_json::from_str(response_text)?;
            let api_error = crate::error::ScryfallApiError::from_scryfall_error(scryfall_error);
            let enhanced_error = enhance_error_message(&api_error.to_string(), "");
            return Err(eyre!("{}", enhanced_error));
        }
    }

    // Parse as card response
    let card: Card = serde_json::from_str(response_text)?;
    Ok(card)
}

fn display_single_card_details(card: &Card) -> Result<()> {
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

        if !legal_formats.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Legal In"),
                Cell::new(&legal_formats.join(", ")),
            ]));
        }
    }

    table.printstd();
    Ok(())
}
