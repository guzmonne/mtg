/*!
Search with parameters example for the Scryfall client.

Run with: `cargo run --example search_with_params`
*/

use mtg_core::scryfall::ScryfallClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = ScryfallClient::new()?;

    // Search for cards with parameters
    let params = vec![
        ("q".to_string(), "Lightning Bolt".to_string()),
        ("unique".to_string(), "cards".to_string()),
        ("order".to_string(), "name".to_string()),
    ];

    let results: serde_json::Value = client.get_with_params("cards/search", params).await?;

    println!("Search results for 'Lightning Bolt':");

    if let Some(data) = results.get("data") {
        if let Some(cards) = data.as_array() {
            println!("Found {} cards", cards.len());

            // Print first few results
            for (i, card) in cards.iter().take(3).enumerate() {
                if let Some(name) = card.get("name").and_then(|n| n.as_str()) {
                    if let Some(set_name) = card.get("set_name").and_then(|s| s.as_str()) {
                        println!("{}. {} ({})", i + 1, name, set_name);
                    }
                }
            }
        }
    }

    Ok(())
}
