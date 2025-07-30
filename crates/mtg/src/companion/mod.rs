#![allow(unused_variables)]
use crate::prelude::*;
use clap::{Parser, Subcommand};

mod parse;
mod watch;

#[derive(Debug, Parser)]
#[command(about = "Track and analyze MTG Arena log files")]
pub struct App {
    #[command(subcommand)]
    pub command: CompanionCommands,
}

#[derive(Debug, Subcommand)]
pub enum CompanionCommands {
    /// Start watching MTG Arena log files for real-time updates
    Watch {
        /// Path to the MTG Arena log directory
        #[clap(long, env = "MTGA_LOG_PATH")]
        log_path: Option<String>,

        /// Watch for specific events only (e.g., "match", "draft", "collection")
        #[clap(long)]
        filter: Option<Vec<String>>,

        /// Output format (json, pretty, csv)
        #[clap(long, default_value = "pretty")]
        format: String,

        /// Start watching from the beginning of the file instead of the end
        #[clap(long)]
        from_beginning: bool,

        /// Show verbose debug output
        #[clap(long)]
        verbose: bool,
    },

    /// Parse and analyze existing log files
    Parse {
        /// Path to the log file to parse (defaults to newest log file)
        #[clap(default_value = "latest")]
        file: String,

        /// Type of analysis to perform (inventory, decks, full, events)
        #[clap(long)]
        analyze: Option<String>,

        /// Display output in pretty format instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Parse for specific events only (e.g., "match", "draft", "life", "actions")
        #[clap(long)]
        filter: Option<Vec<String>>,

        /// Include Player.log parsing for additional event details
        #[clap(long)]
        include_player_log: bool,

        /// Number of recent events to show (default: 50)
        #[clap(long, default_value = "50")]
        limit: usize,

        /// Show verbose debug output
        #[clap(long)]
        verbose: bool,
    },
}

pub async fn run(app: App, _global: crate::Global) -> Result<()> {
    match app.command {
        CompanionCommands::Watch {
            log_path,
            filter,
            format,
            from_beginning,
            verbose,
        } => {
            watch::run(watch::Params {
                log_path,
                filter,
                format,
                from_beginning,
                verbose,
            })
            .await
        }
        CompanionCommands::Parse {
            file,
            analyze,
            pretty,
            filter,
            include_player_log,
            limit,
            verbose,
        } => {
            parse::run(parse::Params {
                file,
                analyze,
                pretty,
                filter,
                include_player_log,
                limit,
                verbose,
            })
            .await
        }
    }
}
