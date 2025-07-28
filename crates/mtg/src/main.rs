#![allow(clippy::large_enum_variant)]
#![allow(clippy::uninlined_format_args)]
use prettytable::{format, Cell, Row};
use std::str::FromStr;

use crate::prelude::*;
use clap::Parser;

mod api;
mod cache;
// Companion module is only available on macOS and Windows since MTG Arena
// is not available on Linux platforms
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod companion;
mod completions;
mod decks;
mod error;
mod gatherer;
mod mcp;
mod prelude;
mod scryfall;
mod sets;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = "Magic The Gathering API CLI")]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,

    #[clap(flatten)]
    global: Global,
}

#[derive(Debug, Clone, clap::Args)]
pub struct Global {
    /// MTG API Base URL
    #[clap(
        long,
        env = "MTG_API_BASE_URL",
        global = true,
        default_value = "https://api.magicthegathering.io/v1"
    )]
    pub api_base_url: String,

    /// Request timeout in seconds
    #[clap(long, env = "MTG_TIMEOUT", global = true, default_value = "30")]
    pub timeout: u64,

    /// Whether to display additional information
    #[clap(long, env = "MTG_VERBOSE", global = true, default_value = "false")]
    pub verbose: bool,
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

#[derive(Debug, clap::Parser)]
pub enum McpCommands {
    /// Start MCP server with STDIO transport (default)
    Stdio,

    /// Start MCP server with SSE transport (HTTP endpoints)
    Sse {
        /// Host to bind to
        #[clap(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[clap(long, default_value = "3000")]
        port: u16,
    },
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Access the MTG API directly
    Api {
        #[command(subcommand)]
        command: crate::api::ApiCommands,
    },

    /// Search cards using Wizards' Gatherer advanced search
    Gatherer(crate::gatherer::App),

    /// Search cards using Scryfall advanced search
    Scryfall(crate::scryfall::App),

    /// Manage and browse Magic sets
    Sets {
        #[command(subcommand)]
        command: SetCommands,
    },

    /// Generate shell completions
    Completions(crate::completions::App),

    /// Analyze Magic: The Gathering deck lists
    Decks(crate::decks::App),

    /// Start Model Context Protocol server for AI integration (defaults to STDIO)
    Mcp {
        #[command(subcommand)]
        command: Option<McpCommands>,
    },

    /// Track and analyze MTG Arena log files
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Companion(crate::companion::App),
}

async fn handle_sets_command(command: SetCommands, global: Global) -> Result<()> {
    use mtg_core::scryfall::sets::{SetListParams, SetType};

    match command {
        SetCommands::List {
            set_type,
            released_after,
            released_before,
            block,
            digital_only,
            pretty,
        } => {
            let set_type_enum: Option<SetType> = if let Some(type_str) = set_type {
                match SetType::from_str(&type_str) {
                    Ok(t) => Some(t),
                    Err(e) => {
                        aeprintln!("Error: {e}");
                        aeprintln!(
                            "Invalid set type: {type_str}. Use 'mtg sets types' to see available types."
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let app = App::parse();

    match app.command {
        SubCommands::Api { command } => command.run().await,
        SubCommands::Gatherer(sub_app) => crate::gatherer::run(sub_app, app.global).await,
        SubCommands::Scryfall(sub_app) => crate::scryfall::run(sub_app, app.global).await,
        SubCommands::Sets { command } => handle_sets_command(command, app.global).await,
        SubCommands::Completions(sub_app) => crate::completions::run(sub_app, app.global).await,
        SubCommands::Decks(sub_app) => crate::decks::run(sub_app, app.global).await,
        SubCommands::Mcp { command } => match command {
            Some(McpCommands::Stdio) | None => crate::mcp::run_mcp_server(app.global).await,
            Some(McpCommands::Sse { host, port }) => {
                crate::mcp::run_sse_server(app.global, host, port).await
            }
        },
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        SubCommands::Companion(sub_app) => crate::companion::run(sub_app, app.global).await,
    }
    .map_err(|err: color_eyre::eyre::Report| eyre!(err))
}
