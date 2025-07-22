use crate::cache::CacheManager;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Scryfall autocomplete response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoComplete {
    /// Always "catalog"
    pub object: String,
    /// Array of card name suggestions
    pub data: Vec<String>,
    /// Total number of suggestions
    pub total_values: u32,
}

pub async fn run(query: &str, include_extras: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL with query parameters
    let url = "https://api.scryfall.com/cards/autocomplete".to_string();
    let mut query_params = vec![("q", query.to_string())];

    if include_extras {
        query_params.push(("include_extras", "true".to_string()));
    }

    // Generate cache key
    let cache_key = CacheManager::hash_request(&(&url, &query_params));

    // Check cache first
    if let Ok(Some(cached_response)) = cache_manager.get(&cache_key).await {
        if global.verbose {
            println!("Cache hit");
        }

        let autocomplete: AutoComplete = serde_json::from_value(cached_response.data)?;
        for suggestion in &autocomplete.data {
            println!("{suggestion}");
        }

        if global.verbose {
            aeprintln!("Found {} suggestions", autocomplete.total_values);
        }

        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
    }

    // Build query string
    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let full_url = format!("{url}?{query_string}");

    if global.verbose {
        println!("Request URL: {full_url}");
    }

    let response = client.get(&full_url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let autocomplete: AutoComplete = serde_json::from_str(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&autocomplete)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    for suggestion in &autocomplete.data {
        println!("{suggestion}");
    }

    if global.verbose {
        aeprintln!("Found {} suggestions", autocomplete.total_values);
    }

    Ok(())
}
