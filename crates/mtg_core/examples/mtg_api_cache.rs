use mtg_core::cache::{CachedHttpClient, Serializer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
struct ScryfallCard {
    id: String,
    name: String,
    mana_cost: Option<String>,
    type_line: String,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScryfallSearchResponse {
    object: String,
    total_cards: u32,
    data: Vec<ScryfallCard>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MTG API Caching Example");
    println!("=======================");

    // Create an HTTP client specifically for Scryfall API caching
    let scryfall_client = CachedHttpClient::builder()
        .cache_prefix("scryfall/api")
        .cache_serializer(Serializer::Json)
        .timeout(Duration::from_secs(10))
        .user_agent("mtg-core-cache-example/1.0")
        .default_ttl(Duration::from_secs(3600)) // Cache for 1 hour
        .build()?;

    // Search for Lightning Bolt cards
    let search_url = "https://api.scryfall.com/cards/search?q=name:\"Lightning Bolt\"";

    println!("Searching for Lightning Bolt cards (first request)...");
    let start = std::time::Instant::now();
    let response = scryfall_client.get(search_url).await?;
    let first_duration = start.elapsed();

    if response.status_code().is_success() {
        let search_result: ScryfallSearchResponse = response.json()?;
        println!("Found {} Lightning Bolt cards", search_result.total_cards);

        if let Some(first_card) = search_result.data.first() {
            println!(
                "First result: {} ({})",
                first_card.name, first_card.type_line
            );
            if let Some(mana_cost) = &first_card.mana_cost {
                println!("Mana cost: {}", mana_cost);
            }
        }
    }

    println!("First request took: {:?}", first_duration);

    // Make the same request again - should be cached
    println!("\nMaking the same request again (should be cached)...");
    let start = std::time::Instant::now();
    let _cached_response = scryfall_client.get(search_url).await?;
    let cached_duration = start.elapsed();

    println!("Cached request took: {:?}", cached_duration);
    println!(
        "Speed improvement: {:.1}x faster",
        first_duration.as_nanos() as f64 / cached_duration.as_nanos() as f64
    );

    // Search for a different card to show multiple cache entries
    let counterspell_url = "https://api.scryfall.com/cards/search?q=name:\"Counterspell\"";
    println!("\nSearching for Counterspell cards...");
    let counterspell_response = scryfall_client.get(counterspell_url).await?;

    if counterspell_response.status_code().is_success() {
        let search_result: ScryfallSearchResponse = counterspell_response.json()?;
        println!("Found {} Counterspell cards", search_result.total_cards);
    }

    // Show cache statistics
    let stats = scryfall_client.cache_stats().await?;
    println!("\nCache statistics:");
    println!("  Total cached responses: {}", stats.total_files);
    println!("  Total cache size: {} bytes", stats.total_size);
    println!("  Cache prefixes: {:?}", stats.prefixes);

    // Demonstrate cache management
    println!("\nCache management options:");

    // Show how to clean old entries (older than 30 minutes)
    let old_entries = scryfall_client
        .clean_older_than(Duration::from_secs(1800))
        .await?;
    println!(
        "  Cleaned {} old entries, freed {} bytes",
        old_entries.removed_count, old_entries.freed_bytes
    );

    // Show how to limit cache size (keep under 1MB)
    let size_limit = scryfall_client.clean_to_size_limit(1_048_576).await?;
    println!(
        "  Size limit cleanup: {} entries removed, {} bytes freed",
        size_limit.removed_count, size_limit.freed_bytes
    );

    println!("\n✓ MTG API caching example completed!");
    println!("This demonstrates how the cache can significantly speed up repeated API calls");
    println!("while respecting rate limits and reducing server load.");

    Ok(())
}
