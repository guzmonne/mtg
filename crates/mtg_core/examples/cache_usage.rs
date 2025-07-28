use mtg_core::cache::{CacheStore, DiskCache, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Card {
    name: String,
    mana_cost: String,
    card_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MTG Core Cache Example");
    println!("======================");

    // Create a cache with JSON serialization and a prefix
    let cache = DiskCache::builder()
        .prefix("example/cards")
        .with_serializer(Serializer::Json)
        .build()?;

    // Create some example cards
    let lightning_bolt = Card {
        name: "Lightning Bolt".to_string(),
        mana_cost: "R".to_string(),
        card_type: "Instant".to_string(),
    };

    let counterspell = Card {
        name: "Counterspell".to_string(),
        mana_cost: "UU".to_string(),
        card_type: "Instant".to_string(),
    };

    // Store cards in cache
    println!("Storing cards in cache...");
    cache
        .insert("lightning-bolt", lightning_bolt.clone())
        .await?;
    cache.insert("counterspell", counterspell.clone()).await?;

    // Retrieve cards from cache
    println!("Retrieving cards from cache...");
    let retrieved_bolt: Option<Card> = cache.get("lightning-bolt").await?;
    let retrieved_counter: Option<Card> = cache.get("counterspell").await?;

    println!("Lightning Bolt: {:?}", retrieved_bolt);
    println!("Counterspell: {:?}", retrieved_counter);

    // Check cache statistics
    let stats = cache.stats(Some("example/cards")).await?;
    println!(
        "Cache stats: {} files, {} bytes",
        stats.total_files, stats.total_size
    );

    // List all keys (note: these are hashes, not original keys)
    let keys = <DiskCache as CacheStore<&str, Card>>::keys(&cache).await?;
    println!("Cache keys: {:?}", keys);

    // Clean up - remove the example cache
    let report = cache.clean_prefix("example").await?;
    println!(
        "Cleaned up: {} files, {} bytes freed",
        report.removed_count, report.freed_bytes
    );

    Ok(())
}
