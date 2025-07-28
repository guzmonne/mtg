use crate::cache::CacheManager;
use crate::prelude::*;
use mtg_core::scryfall::sets::{ScryfallSet, ScryfallSetList, SetListParams, SetsClient};

/// Get all sets from Scryfall API with caching
pub async fn list_sets(params: SetListParams, global: crate::Global) -> Result<ScryfallSetList> {
    let cache_manager = CacheManager::new()?;

    // Generate cache key based on parameters
    let cache_key = CacheManager::hash_request(&(
        "https://api.scryfall.com/sets",
        &params.set_type,
        &params.released_after,
        &params.released_before,
    ));

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let response: ScryfallSetList = serde_json::from_value(cached_response.data)?;
        return Ok(response);
    }

    if global.verbose {
        println!("Cache miss, fetching sets from API");
    }

    // Create API client using the global Scryfall client
    let scryfall_client = global.create_scryfall_client()?;
    let client = SetsClient::new(scryfall_client);

    // Fetch from API
    let set_response = client.list_sets(params).await?;

    // Cache the response
    cache_manager
        .set(&cache_key, serde_json::to_value(&set_response)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    Ok(set_response)
}

/// Get a specific set by code with caching
pub async fn get_set_by_code(code: &str, global: crate::Global) -> Result<ScryfallSet> {
    let cache_manager = CacheManager::new()?;

    let url = format!(
        "https://api.scryfall.com/sets/{}",
        urlencoding::encode(code)
    );

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let set: ScryfallSet = serde_json::from_value(cached_response.data)?;
        return Ok(set);
    }

    if global.verbose {
        println!("Cache miss, fetching set from API");
    }

    // Create API client using the global Scryfall client
    let scryfall_client = global.create_scryfall_client()?;
    let client = SetsClient::new(scryfall_client);

    // Fetch from API
    let set = client.get_set_by_code(code).await?;

    // Cache the response
    cache_manager
        .set(&cache_key, serde_json::to_value(&set)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    Ok(set)
}
