pub mod scryfall;

// Re-export the ScryfallClient for easy access from the binary
pub use scryfall::{ScryfallClient, ScryfallClientBuilder, ScryfallClientConfig};
