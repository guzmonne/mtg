pub mod cards;
pub mod sets;
pub mod types;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ApiCommands {
    /// Search for cards
    Cards(cards::CardsCommand),
    /// Search for sets
    Sets(sets::SetsCommand),
    /// Search for types
    Types(types::TypesCommand),
}

impl ApiCommands {
    pub async fn run(self) -> crate::Result<()> {
        match self {
            Self::Cards(cmd) => cmd.run().await,
            Self::Sets(cmd) => cmd.run().await,
            Self::Types(cmd) => cmd.run().await,
        }
    }
}
