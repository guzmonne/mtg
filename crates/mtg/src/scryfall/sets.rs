use crate::prelude::*;
use mtg_core::scryfall::sets::{ScryfallSet, ScryfallSetList, SetListParams, SetsClient};

/// Get all sets from Scryfall API with caching
pub async fn list_sets(params: SetListParams, global: crate::Global) -> Result<ScryfallSetList> {
    // Create API client using the global Scryfall client (which includes caching)
    let scryfall_client = global.create_scryfall_client()?;
    let client = SetsClient::new(scryfall_client);

    // Fetch from API (caching is handled automatically by mtg_core)
    let set_response = client.list_sets(params).await?;

    Ok(set_response)
}

/// Get a specific set by code with caching
pub async fn get_set_by_code(code: &str, global: crate::Global) -> Result<ScryfallSet> {
    // Create API client using the global Scryfall client (which includes caching)
    let scryfall_client = global.create_scryfall_client()?;
    let client = SetsClient::new(scryfall_client);

    // Fetch from API (caching is handled automatically by mtg_core)
    let set = client.get_set_by_code(code).await?;

    Ok(set)
}
