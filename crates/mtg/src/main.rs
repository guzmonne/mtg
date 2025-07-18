#![allow(unused)]

use crate::prelude::*;
use clap::Parser;

mod error;
mod prelude;
mod cards;
mod sets;
mod types;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = "Magic The Gathering API CLI")]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,

    #[clap(flatten)]
    global: Global,
}

#[derive(Debug, clap::Args)]
pub struct Global {
    /// MTG API Base URL
    #[clap(long, env = "MTG_API_BASE_URL", global = true, default_value = "https://api.magicthegathering.io/v1")]
    api_base_url: String,

    /// Request timeout in seconds
    #[clap(long, env = "MTG_TIMEOUT", global = true, default_value = "30")]
    timeout: u64,

    /// Whether to display additional information
    #[clap(long, env = "MTG_VERBOSE", global = true, default_value = "false")]
    verbose: bool,
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Search and retrieve Magic cards
    Cards(crate::cards::App),

    /// Browse Magic sets and generate booster packs
    Sets(crate::sets::App),

    /// Get card types, subtypes, supertypes, and formats
    Types(crate::types::App),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let app = App::parse();

    match app.command {
        SubCommands::Cards(sub_app) => crate::cards::run(sub_app, app.global).await,
        SubCommands::Sets(sub_app) => crate::sets::run(sub_app, app.global).await,
        SubCommands::Types(sub_app) => crate::types::run(sub_app, app.global).await,
    }
    .map_err(|err: color_eyre::eyre::Report| eyre!(err))
}
