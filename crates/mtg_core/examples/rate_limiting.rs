/*!
Rate limiting example for the Scryfall client.

Run with: `cargo run --example rate_limiting`
*/

use mtg_core::scryfall::ScryfallClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Client with custom rate limiting
    let client = ScryfallClient::builder()
        .rate_limit_delay(Some(Duration::from_millis(200)))
        .build()?;

    println!("Making 5 requests with 200ms rate limiting...");

    // Make multiple requests - they will be automatically rate limited
    for i in 1..=5 {
        let start = std::time::Instant::now();
        let _sets: serde_json::Value = client.get("sets").await?;
        let elapsed = start.elapsed();
        println!("Request {} took {:?}", i, elapsed);
    }

    println!("\nTotal time should be approximately 1 second (5 requests × 200ms delay)");

    Ok(())
}
