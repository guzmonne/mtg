/*!
Disable rate limiting example for the Scryfall client.

Run with: `cargo run --example no_rate_limiting`

WARNING: Use with caution! This can overwhelm the Scryfall API.
*/

use mtg_core::scryfall::ScryfallClient;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Client with no rate limiting (use with caution!)
    let client = ScryfallClient::builder().rate_limit_delay(None).build()?;

    println!("Making requests with NO rate limiting (use with caution!)...");

    let start = std::time::Instant::now();

    // Make a few requests as fast as possible
    for i in 1..=3 {
        let request_start = std::time::Instant::now();
        let _sets: serde_json::Value = client.get("sets").await?;
        let request_elapsed = request_start.elapsed();
        println!("Request {} took {:?}", i, request_elapsed);
    }

    let total_elapsed = start.elapsed();
    println!("Total time: {:?}", total_elapsed);
    println!("Note: Requests were made as fast as possible without artificial delays");

    Ok(())
}
