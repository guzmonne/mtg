use mtg_core::cache::{CachedHttpClient, Serializer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
struct JsonPlaceholderPost {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MTG Core HTTP Cache Example");
    println!("============================");

    // Create an HTTP client with caching
    let client = CachedHttpClient::builder()
        .cache_prefix("http_example")
        .cache_serializer(Serializer::Json)
        .timeout(Duration::from_secs(10))
        .user_agent("mtg-core-example/1.0")
        .default_ttl(Duration::from_secs(300)) // Cache for 5 minutes
        .build()?;

    // Test URL - JSONPlaceholder is a free testing API
    let test_url = "https://jsonplaceholder.typicode.com/posts/1";

    println!("Making first request (will hit the API)...");
    let start = std::time::Instant::now();
    let response1 = client.get(test_url).await?;
    let duration1 = start.elapsed();

    println!("Status: {}", response1.status_code());
    println!("Response time: {:?}", duration1);

    // Parse the JSON response
    let post: JsonPlaceholderPost = response1.json()?;
    println!("Post title: {}", post.title);

    println!("\nMaking second request (should be cached)...");
    let start = std::time::Instant::now();
    let response2 = client.get(test_url).await?;
    let duration2 = start.elapsed();

    println!("Status: {}", response2.status_code());
    println!("Response time: {:?}", duration2);
    println!(
        "Cached response is much faster: {:?} vs {:?}",
        duration2, duration1
    );

    // Verify the cached response has the same content
    let post2: JsonPlaceholderPost = response2.json()?;
    assert_eq!(post.title, post2.title);
    println!("✓ Cached response matches original");

    // Check cache statistics
    let stats = client.cache_stats().await?;
    println!("\nCache statistics:");
    println!("  Files: {}", stats.total_files);
    println!("  Size: {} bytes", stats.total_size);

    // Test with different URLs to show cache key generation
    println!("\nTesting different URLs...");
    let url2 = "https://jsonplaceholder.typicode.com/posts/2";
    let response3 = client.get(url2).await?;
    println!("Second URL status: {}", response3.status_code());

    let stats_after = client.cache_stats().await?;
    println!("Cache files after second URL: {}", stats_after.total_files);

    // Test with custom headers
    println!("\nTesting with custom headers...");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-Custom-Header", "test-value".parse()?);

    let response4 = client.get_with_headers(test_url, Some(headers)).await?;
    println!("Request with headers status: {}", response4.status_code());

    // Clean up the cache
    println!("\nCleaning up cache...");
    client.clear_cache().await?;

    let final_stats = client.cache_stats().await?;
    println!("Cache files after cleanup: {}", final_stats.total_files);

    println!("\n✓ HTTP caching example completed successfully!");

    Ok(())
}
