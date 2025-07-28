/*!
Custom base URL example for the Scryfall client.

Run with: `cargo run --example custom_base_url`

Note: This example will fail unless you have a test server running,
but it demonstrates how to configure a custom base URL.
*/

use mtg_core::scryfall::ScryfallClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Client pointing to a test server
    let client = ScryfallClient::builder()
        .base_url("https://test-api.example.com")
        .build()?;

    println!("Client configured with custom base URL: https://test-api.example.com");

    // This will likely fail unless you have a test server running
    println!("Attempting to make request to custom base URL...");
    match client.get::<serde_json::Value>("sets").await {
        Ok(_response) => {
            println!("✓ Successfully connected to custom base URL");
        }
        Err(e) => {
            println!("✗ Expected failure connecting to test server: {e}");
            println!("This is normal - the example demonstrates configuration, not actual usage");
        }
    }

    // Show how to switch back to the real Scryfall API
    println!("\nSwitching to real Scryfall API...");
    let real_client = ScryfallClient::builder()
        .base_url("https://api.scryfall.com")
        .build()?;

    let _sets: serde_json::Value = real_client.get("sets").await?;
    println!("✓ Successfully connected to real Scryfall API");

    Ok(())
}
