use crate::prelude::*;
use crate::cache::CacheManager;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Complete Scryfall Set object with all API fields
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScryfallSet {
    /// Always "set"
    pub object: String,
    /// UUID for this set
    pub id: String,
    /// 3-6 letter set code
    pub code: String,
    /// MTGO-specific code
    pub mtgo_code: Option<String>,
    /// Arena-specific code
    pub arena_code: Option<String>,
    /// TCGPlayer group ID
    pub tcgplayer_id: Option<u32>,
    /// English set name
    pub name: String,
    /// Set type classification
    pub set_type: String,
    /// Release date
    pub released_at: Option<String>,
    /// Block code
    pub block_code: Option<String>,
    /// Block name
    pub block: Option<String>,
    /// Parent set code for promos/tokens
    pub parent_set_code: Option<String>,
    /// Number of cards in this set
    pub card_count: u32,
    /// Collector number denominator
    pub printed_size: Option<u32>,
    /// Video game only
    pub digital: bool,
    /// Only foil cards
    pub foil_only: bool,
    /// Only nonfoil cards
    pub nonfoil_only: bool,
    /// Scryfall website link
    pub scryfall_uri: String,
    /// API link
    pub uri: String,
    /// Set icon SVG
    pub icon_svg_uri: String,
    /// Cards in set search URI
    pub search_uri: String,
}

/// Set type enumeration with all documented types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    /// Yearly Magic core sets
    Core,
    /// Rotational expansion sets
    Expansion,
    /// Reprint sets with no new cards
    Masters,
    /// Arena Alchemy sets
    Alchemy,
    /// Masterpiece Series premium foil cards
    Masterpiece,
    /// Commander-oriented gift sets
    Arsenal,
    /// From the Vault gift sets
    FromTheVault,
    /// Spellbook series gift sets
    Spellbook,
    /// Premium Deck Series decks
    PremiumDeck,
    /// Duel Decks
    DuelDeck,
    /// Special draft sets (Conspiracy, Battlebond)
    DraftInnovation,
    /// Magic Online treasure chest prize sets
    TreasureChest,
    /// Commander preconstructed decks
    Commander,
    /// Planechase sets
    Planechase,
    /// Archenemy sets
    Archenemy,
    /// Vanguard card sets
    Vanguard,
    /// Un-sets and funny promos
    Funny,
    /// Starter/introductory sets
    Starter,
    /// Gift box sets
    Box,
    /// Promotional cards
    Promo,
    /// Token sets
    Token,
    /// Non-legal special cards
    Memorabilia,
    /// Minigame card inserts
    Minigame,
}

impl SetType {
    /// Get all set types as a vector
    pub fn all() -> Vec<SetType> {
        vec![
            SetType::Core,
            SetType::Expansion,
            SetType::Masters,
            SetType::Alchemy,
            SetType::Masterpiece,
            SetType::Arsenal,
            SetType::FromTheVault,
            SetType::Spellbook,
            SetType::PremiumDeck,
            SetType::DuelDeck,
            SetType::DraftInnovation,
            SetType::TreasureChest,
            SetType::Commander,
            SetType::Planechase,
            SetType::Archenemy,
            SetType::Vanguard,
            SetType::Funny,
            SetType::Starter,
            SetType::Box,
            SetType::Promo,
            SetType::Token,
            SetType::Memorabilia,
            SetType::Minigame,
        ]
    }

    /// Get the string representation for API queries
    pub fn as_str(&self) -> &'static str {
        match self {
            SetType::Core => "core",
            SetType::Expansion => "expansion",
            SetType::Masters => "masters",
            SetType::Alchemy => "alchemy",
            SetType::Masterpiece => "masterpiece",
            SetType::Arsenal => "arsenal",
            SetType::FromTheVault => "from_the_vault",
            SetType::Spellbook => "spellbook",
            SetType::PremiumDeck => "premium_deck",
            SetType::DuelDeck => "duel_deck",
            SetType::DraftInnovation => "draft_innovation",
            SetType::TreasureChest => "treasure_chest",
            SetType::Commander => "commander",
            SetType::Planechase => "planechase",
            SetType::Archenemy => "archenemy",
            SetType::Vanguard => "vanguard",
            SetType::Funny => "funny",
            SetType::Starter => "starter",
            SetType::Box => "box",
            SetType::Promo => "promo",
            SetType::Token => "token",
            SetType::Memorabilia => "memorabilia",
            SetType::Minigame => "minigame",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<SetType> {
        match s {
            "core" => Some(SetType::Core),
            "expansion" => Some(SetType::Expansion),
            "masters" => Some(SetType::Masters),
            "alchemy" => Some(SetType::Alchemy),
            "masterpiece" => Some(SetType::Masterpiece),
            "arsenal" => Some(SetType::Arsenal),
            "from_the_vault" => Some(SetType::FromTheVault),
            "spellbook" => Some(SetType::Spellbook),
            "premium_deck" => Some(SetType::PremiumDeck),
            "duel_deck" => Some(SetType::DuelDeck),
            "draft_innovation" => Some(SetType::DraftInnovation),
            "treasure_chest" => Some(SetType::TreasureChest),
            "commander" => Some(SetType::Commander),
            "planechase" => Some(SetType::Planechase),
            "archenemy" => Some(SetType::Archenemy),
            "vanguard" => Some(SetType::Vanguard),
            "funny" => Some(SetType::Funny),
            "starter" => Some(SetType::Starter),
            "box" => Some(SetType::Box),
            "promo" => Some(SetType::Promo),
            "token" => Some(SetType::Token),
            "memorabilia" => Some(SetType::Memorabilia),
            "minigame" => Some(SetType::Minigame),
            _ => None,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            SetType::Core => "Yearly Magic core sets",
            SetType::Expansion => "Rotational expansion sets in blocks",
            SetType::Masters => "Reprint sets with no new cards",
            SetType::Alchemy => "Arena sets designed for Alchemy",
            SetType::Masterpiece => "Masterpiece Series premium foil cards",
            SetType::Arsenal => "Commander-oriented gift sets",
            SetType::FromTheVault => "From the Vault gift sets",
            SetType::Spellbook => "Spellbook series gift sets",
            SetType::PremiumDeck => "Premium Deck Series decks",
            SetType::DuelDeck => "Duel Decks",
            SetType::DraftInnovation => "Special draft sets like Conspiracy",
            SetType::TreasureChest => "Magic Online treasure chest prize sets",
            SetType::Commander => "Commander preconstructed decks",
            SetType::Planechase => "Planechase sets",
            SetType::Archenemy => "Archenemy sets",
            SetType::Vanguard => "Vanguard card sets",
            SetType::Funny => "Un-sets and funny promos",
            SetType::Starter => "Starter/introductory sets",
            SetType::Box => "Gift box sets",
            SetType::Promo => "Promotional cards",
            SetType::Token => "Token and emblem sets",
            SetType::Memorabilia => "Non-legal special cards",
            SetType::Minigame => "Minigame card inserts",
        }
    }
}

/// Type alias for set lists
pub type ScryfallSetList = crate::scryfall::ScryfallList<ScryfallSet>;

/// Parameters for set listing
#[derive(Debug, Clone)]
pub struct SetListParams {
    pub set_type: Option<SetType>,
    pub released_after: Option<String>,
    pub released_before: Option<String>,
    pub block: Option<String>,
    pub digital_only: Option<bool>,
}

/// Get all sets from Scryfall API
pub async fn list_sets(params: SetListParams, global: crate::Global) -> Result<ScryfallSetList> {
    let cache_manager = CacheManager::new()?;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = "https://api.scryfall.com/sets";
    
    // Generate cache key based on parameters
    let cache_key = CacheManager::hash_request(&(&url, &params.set_type, &params.released_after, &params.released_before));

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let mut response: ScryfallSetList = serde_json::from_value(cached_response.data)?;
        
        // Apply client-side filtering if needed
        apply_set_filters(&mut response, &params);
        
        return Ok(response);
    }

    if global.verbose {
        println!("Cache miss, fetching sets from API");
        println!("Request URL: {}", url);
    }

    let response = client
        .get(url)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;
    
    // Parse the response
    let mut set_response = parse_scryfall_set_list_response(&response_text)?;
    
    // Apply client-side filtering
    apply_set_filters(&mut set_response, &params);
    
    // Cache the response
    cache_manager.set(&cache_key, serde_json::to_value(&set_response)?).await?;
    
    if global.verbose {
        println!("Response cached");
    }
    
    Ok(set_response)
}

/// Get a specific set by code
pub async fn get_set_by_code(code: &str, global: crate::Global) -> Result<ScryfallSet> {
    let cache_manager = CacheManager::new()?;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/sets/{}", urlencoding::encode(code));
    
    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let set: ScryfallSet = serde_json::from_value(cached_response.data)?;
        return Ok(set);
    }

    if global.verbose {
        println!("Cache miss, fetching set from API");
        println!("Request URL: {}", url);
    }

    let response = client
        .get(&url)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;
    
    // Parse the response
    let set = parse_scryfall_set_response(&response_text)?;
    
    // Cache the response
    cache_manager.set(&cache_key, serde_json::to_value(&set)?).await?;
    
    if global.verbose {
        println!("Response cached");
    }
    
    Ok(set)
}

/// Parse Scryfall set list response
fn parse_scryfall_set_list_response(response_text: &str) -> Result<ScryfallSetList> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;
    
    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response
            let scryfall_error: crate::error::ScryfallError = serde_json::from_str(response_text)?;
            let api_error = crate::error::ScryfallApiError::from_scryfall_error(scryfall_error);
            return Err(eyre!("Scryfall API error: {}", api_error));
        }
    }
    
    // Parse as set list response
    let set_list: ScryfallSetList = serde_json::from_str(response_text)?;
    Ok(set_list)
}

/// Parse Scryfall single set response
fn parse_scryfall_set_response(response_text: &str) -> Result<ScryfallSet> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;
    
    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response
            let scryfall_error: crate::error::ScryfallError = serde_json::from_str(response_text)?;
            let api_error = crate::error::ScryfallApiError::from_scryfall_error(scryfall_error);
            return Err(eyre!("Scryfall API error: {}", api_error));
        }
    }
    
    // Parse as set response
    let set: ScryfallSet = serde_json::from_str(response_text)?;
    Ok(set)
}

/// Apply client-side filters to set list
fn apply_set_filters(response: &mut ScryfallSetList, params: &SetListParams) {
    if params.set_type.is_none() && params.released_after.is_none() && 
       params.released_before.is_none() && params.block.is_none() && 
       params.digital_only.is_none() {
        return; // No filters to apply
    }

    response.data.retain(|set| {
        // Filter by set type
        if let Some(ref filter_type) = params.set_type {
            if set.set_type != filter_type.as_str() {
                return false;
            }
        }

        // Filter by release date
        if let Some(ref after) = params.released_after {
            if let Some(ref released) = set.released_at {
                if released < after {
                    return false;
                }
            }
        }

        if let Some(ref before) = params.released_before {
            if let Some(ref released) = set.released_at {
                if released > before {
                    return false;
                }
            }
        }

        // Filter by block
        if let Some(ref filter_block) = params.block {
            if let Some(ref set_block) = set.block {
                if !set_block.to_lowercase().contains(&filter_block.to_lowercase()) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Filter by digital only
        if let Some(digital_filter) = params.digital_only {
            if set.digital != digital_filter {
                return false;
            }
        }

        true
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_type_parsing() {
        assert_eq!(SetType::from_str("core"), Some(SetType::Core));
        assert_eq!(SetType::from_str("expansion"), Some(SetType::Expansion));
        assert_eq!(SetType::from_str("invalid"), None);
    }

    #[test]
    fn test_set_type_string_conversion() {
        assert_eq!(SetType::Core.as_str(), "core");
        assert_eq!(SetType::Expansion.as_str(), "expansion");
        assert_eq!(SetType::Masters.as_str(), "masters");
    }

    #[test]
    fn test_set_type_all() {
        let all_types = SetType::all();
        assert!(all_types.len() >= 23); // At least 23 types documented
        assert!(all_types.contains(&SetType::Core));
        assert!(all_types.contains(&SetType::Expansion));
    }
}