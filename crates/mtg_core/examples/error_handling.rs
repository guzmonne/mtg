/*!
Error handling example for the Scryfall client.

Run with: `cargo run --example error_handling`
*/

use mtg_core::scryfall::{sets::ScryfallSet, ScryfallClient};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = ScryfallClient::new()?;

    // Handle API errors gracefully
    println!("Attempting to fetch a nonexistent set...");
    match client.get::<ScryfallSet>("sets/nonexistent").await {
        Ok(set) => println!("Found set: {}", set.name),
        Err(e) => {
            if e.to_string().contains("Scryfall API error") {
                println!("✓ API returned an error as expected: {e}");
            } else {
                println!("✓ Network or parsing error as expected: {e}");
            }
        }
    }

    // Now try a valid request
    println!("\nAttempting to fetch a valid set...");
    match client.get::<ScryfallSet>("sets/lea").await {
        Ok(set) => println!("✓ Successfully found set: {} ({})", set.name, set.code),
        Err(e) => println!("✗ Unexpected error: {e}"),
    }

    Ok(())
}
