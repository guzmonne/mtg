#![allow(clippy::large_enum_variant)]
#![allow(unused)]

use crate::prelude::*;
use clap::Parser;

mod api;
mod cache;
mod completions;
mod error;
mod gatherer;
mod mcp;
mod prelude;
mod scryfall;

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

    /// Generate shell completions
    Completions(crate::completions::App),

    /// Start Model Context Protocol server for AI integration (defaults to STDIO)
    Mcp {
        #[command(subcommand)]
        command: Option<McpCommands>,
    },
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
        SubCommands::Completions(sub_app) => crate::completions::run(sub_app, app.global).await,
        SubCommands::Mcp { command } => match command {
            Some(McpCommands::Stdio) | None => crate::mcp::run_mcp_server(app.global).await,
            Some(McpCommands::Sse { host, port }) => crate::mcp::run_sse_server(app.global, host, port).await,
        },
    }
    .map_err(|err: color_eyre::eyre::Report| eyre!(err))
}
