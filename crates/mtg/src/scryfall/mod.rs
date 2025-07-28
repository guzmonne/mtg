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

/// Convert mtg_core Card to CLI Card
pub fn convert_core_card_to_cli(core_card: &mtg_core::scryfall::Card) -> Card {
    Card {
        object: core_card.object.clone(),
        id: core_card.id.clone(),
        oracle_id: core_card.oracle_id.clone(),
        multiverse_ids: core_card.multiverse_ids.clone(),
        mtgo_id: core_card.mtgo_id,
        arena_id: core_card.arena_id,
        tcgplayer_id: core_card.tcgplayer_id,
        cardmarket_id: core_card.cardmarket_id,
        name: core_card.name.clone(),
        lang: core_card.lang.clone(),
        released_at: core_card.released_at.clone(),
        uri: core_card.uri.clone(),
        scryfall_uri: core_card.scryfall_uri.clone(),
        layout: core_card.layout.clone(),
        highres_image: core_card.highres_image,
        image_status: core_card.image_status.clone(),
        image_uris: core_card.image_uris.clone(),
        mana_cost: core_card.mana_cost.clone(),
        cmc: core_card.cmc,
        type_line: core_card.type_line.clone(),
        oracle_text: core_card.oracle_text.clone(),
        power: core_card.power.clone(),
        toughness: core_card.toughness.clone(),
        loyalty: core_card.loyalty.clone(),
        colors: core_card.colors.clone(),
        color_identity: core_card.color_identity.clone(),
        keywords: core_card.keywords.clone(),
        legalities: core_card.legalities.clone(),
        games: core_card.games.clone(),
        reserved: core_card.reserved,
        foil: core_card.foil,
        nonfoil: core_card.nonfoil,
        finishes: core_card.finishes.clone(),
        oversized: core_card.oversized,
        promo: core_card.promo,
        reprint: core_card.reprint,
        variation: core_card.variation,
        set_id: core_card.set_id.clone(),
        set: core_card.set.clone(),
        set_name: core_card.set_name.clone(),
        set_type: core_card.set_type.clone(),
        set_uri: core_card.set_uri.clone(),
        set_search_uri: core_card.set_search_uri.clone(),
        scryfall_set_uri: core_card.scryfall_set_uri.clone(),
        rulings_uri: core_card.rulings_uri.clone(),
        prints_search_uri: core_card.prints_search_uri.clone(),
        collector_number: core_card.collector_number.clone(),
        digital: core_card.digital,
        rarity: core_card.rarity.clone(),
        flavor_text: core_card.flavor_text.clone(),
        card_back_id: core_card.card_back_id.clone(),
        artist: core_card.artist.clone(),
        artist_ids: core_card.artist_ids.clone(),
        illustration_id: core_card.illustration_id.clone(),
        border_color: core_card.border_color.clone(),
        frame: core_card.frame.clone(),
        security_stamp: core_card.security_stamp.clone(),
        full_art: core_card.full_art,
        textless: core_card.textless,
        booster: core_card.booster,
        story_spotlight: core_card.story_spotlight,
        edhrec_rank: core_card.edhrec_rank,
        penny_rank: core_card.penny_rank,
        prices: core_card.prices.clone(),
        related_uris: core_card.related_uris.clone(),
        purchase_uris: core_card.purchase_uris.clone(),
    }
}

/// Convert mtg_core SearchResponse to CLI Response  
pub fn convert_core_response_to_cli(
    core_response: &mtg_core::scryfall::SearchResponse,
) -> search::Response {
    List {
        object: core_response.object.clone(),
        data: core_response
            .data
            .iter()
            .map(convert_core_card_to_cli)
            .collect(),
        has_more: core_response.has_more,
        next_page: core_response.next_page.clone(),
        total_cards: core_response.total_cards,
        warnings: core_response.warnings.clone(),
    }
}
