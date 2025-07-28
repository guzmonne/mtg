/*!
Raw response handling example for the Scryfall client.

Run with: `cargo run --example raw_response`
*/

use mtg_core::scryfall::ScryfallClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let client = ScryfallClient::new()?;

    // Get raw response text (useful for CSV or custom parsing)
    let raw_response = client.get_raw("sets").await?;
    println!("Raw response length: {} characters", raw_response.len());
    println!("First 200 characters:");
    println!("{}", &raw_response[..200.min(raw_response.len())]);

    println!("\n{}", "=".repeat(50));

    // Example with search parameters
    let params = vec![
        ("q".to_string(), "Lightning Bolt".to_string()),
        ("unique".to_string(), "cards".to_string()),
    ];

    let search_response = client.get_raw_with_params("cards/search", params).await?;

    println!(
        "Search response length: {} characters",
        search_response.len()
    );
    println!("First 200 characters of search:");
    println!("{}", &search_response[..200.min(search_response.len())]);

    Ok(())
}
