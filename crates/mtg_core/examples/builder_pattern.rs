/*!
Builder pattern example for the Scryfall client.

Run with: `cargo run --example builder_pattern`
*/

use mtg_core::scryfall::ScryfallClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Create a customized client
    let client = ScryfallClient::builder()
        .timeout_secs(60) // 60 second timeout
        .user_agent("my-app/1.0") // Custom user agent
        .verbose(true) // Enable verbose logging
        .rate_limit_delay_ms(Some(150)) // 150ms between requests
        .header("X-API-Key", "my-key")? // Custom header
        .build()?;

    // Use the client
    let sets: serde_json::Value = client.get("sets").await?;
    println!("Retrieved sets with custom client configuration");

    if let Some(data) = sets.get("data") {
        if let Some(array) = data.as_array() {
            println!("Found {} sets", array.len());
        }
    }

    Ok(())
}
