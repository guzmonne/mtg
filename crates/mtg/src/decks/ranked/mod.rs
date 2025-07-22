use crate::prelude::*;

mod list;
mod show;

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// List tournament deck lists
    #[clap(name = "list")]
    List {
        /// Filter by format (e.g., alchemy, standard, modern)
        #[clap(short, long)]
        format: Option<String>,

        /// Number of results to fetch (default: 20)
        #[clap(short, long, default_value = "20")]
        limit: u32,

        /// Number of results to skip (default: 0)
        #[clap(short, long, default_value = "0", conflicts_with = "page")]
        skip: u32,

        /// Page number (1-based, automatically calculates skip based on limit)
        #[clap(short, long, conflicts_with = "skip")]
        page: Option<u32>,

        /// Output format (pretty table or JSON)
        #[clap(long, default_value = "pretty")]
        output: String,
    },
    /// Show deck lists from a specific article
    #[clap(name = "show")]
    Show {
        /// ID (from list command) or URL of the deck list article
        #[clap(value_name = "ID_OR_URL")]
        identifier: String,

        /// Output format (pretty table or JSON)
        #[clap(long, default_value = "pretty")]
        output: String,
    },
}

pub async fn run(command: Commands, global: crate::Global) -> Result<()> {
    match command {
        Commands::List {
            format,
            limit,
            skip,
            page,
            output,
        } => {
            // Calculate skip from page if provided
            let actual_skip = if let Some(p) = page {
                if p == 0 {
                    return Err(eyre!("Page number must be 1 or greater"));
                }
                (p - 1) * limit
            } else {
                skip
            };
            list::run(format, limit, actual_skip, output, global).await
        }
        Commands::Show { identifier, output } => show::run(identifier, output, global).await,
    }
}
