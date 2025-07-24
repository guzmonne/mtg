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
    },

    /// Parse and analyze existing log files
    Parse {
        /// Path to the log file to parse (defaults to newest log file)
        #[clap(default_value = "latest")]
        file: String,

        /// Type of analysis to perform (inventory, decks, full)
        #[clap(long)]
        analyze: Option<String>,

        /// Display output in pretty format instead of JSON
        #[clap(long)]
        pretty: bool,
    },
}

pub async fn run(app: App, _global: crate::Global) -> Result<()> {
    match app.command {
        CompanionCommands::Watch {
            log_path,
            filter,
            format,
        } => {
            watch::run(watch::Params {
                log_path,
                filter,
                format,
            })
            .await
        }
        CompanionCommands::Parse {
            file,
            analyze,
            pretty,
        } => {
            parse::run(parse::Params {
                file,
                analyze,
                pretty,
            })
            .await
        }
    }
}
