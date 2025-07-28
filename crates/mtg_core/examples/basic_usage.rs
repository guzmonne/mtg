/*!
Basic usage example for the Scryfall client.

Run with: `cargo run --example basic_usage`
*/

use mtg_core::scryfall::{sets::ScryfallSetList, ScryfallClient};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Create a client with default settings
    let client = ScryfallClient::new()?;

    // Get all sets
    let sets: ScryfallSetList = client.get("sets").await?;
    println!("Found {} sets", sets.data.len());

    // Print first few sets as examples
    for (i, set) in sets.data.iter().take(5).enumerate() {
        println!("{}. {} ({})", i + 1, set.name, set.code);
    }

    Ok(())
}
