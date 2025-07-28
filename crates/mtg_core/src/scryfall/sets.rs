use color_eyre::Result;
use serde::{Deserialize, Serialize};

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

// Implement std::str::FromStr for SetType.
impl std::str::FromStr for SetType {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "core" => Ok(SetType::Core),
            "expansion" => Ok(SetType::Expansion),
            "masters" => Ok(SetType::Masters),
            "alchemy" => Ok(SetType::Alchemy),
            "masterpiece" => Ok(SetType::Masterpiece),
            "arsenal" => Ok(SetType::Arsenal),
            "from_the_vault" => Ok(SetType::FromTheVault),
            "spellbook" => Ok(SetType::Spellbook),
            "premium_deck" => Ok(SetType::PremiumDeck),
            "duel_deck" => Ok(SetType::DuelDeck),
            "draft_innovation" => Ok(SetType::DraftInnovation),
            "treasure_chest" => Ok(SetType::TreasureChest),
            "commander" => Ok(SetType::Commander),
            "planechase" => Ok(SetType::Planechase),
            "archenemy" => Ok(SetType::Archenemy),
            "vanguard" => Ok(SetType::Vanguard),
            "funny" => Ok(SetType::Funny),
            "starter" => Ok(SetType::Starter),
            "box" => Ok(SetType::Box),
            "promo" => Ok(SetType::Promo),
            "token" => Ok(SetType::Token),
            "memorabilia" => Ok(SetType::Memorabilia),
            "minigame" => Ok(SetType::Minigame),
            _ => Err(color_eyre::eyre::eyre!("Unknown set type: {}", s)),
        }
    }
}

/// Type alias for set lists
pub type ScryfallSetList = super::List<ScryfallSet>;

/// Parameters for set listing
#[derive(Debug, Clone)]
pub struct SetListParams {
    pub set_type: Option<SetType>,
    pub released_after: Option<String>,
    pub released_before: Option<String>,
    pub block: Option<String>,
    pub digital_only: Option<bool>,
}

/// Client for interacting with Scryfall sets API
pub struct SetsClient {
    client: super::ScryfallClient,
}

impl SetsClient {
    /// Create a new sets client with a custom Scryfall client
    pub fn new(client: super::ScryfallClient) -> Self {
        Self { client }
    }

    /// Get all sets from Scryfall API
    pub async fn list_sets(&self, params: SetListParams) -> Result<ScryfallSetList> {
        // Use the generic client to fetch sets
        let mut set_response: ScryfallSetList = self.client.get("sets").await?;

        // Apply client-side filtering
        apply_set_filters(&mut set_response, &params);

        Ok(set_response)
    }

    /// Get a specific set by code
    pub async fn get_set_by_code(&self, code: &str) -> Result<ScryfallSet> {
        let endpoint = format!("sets/{}", urlencoding::encode(code));
        let set: ScryfallSet = self.client.get(&endpoint).await?;
        Ok(set)
    }
}

/// Apply client-side filters to set list
fn apply_set_filters(response: &mut ScryfallSetList, params: &SetListParams) {
    if params.set_type.is_none()
        && params.released_after.is_none()
        && params.released_before.is_none()
        && params.block.is_none()
        && params.digital_only.is_none()
    {
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
                if !set_block
                    .to_lowercase()
                    .contains(&filter_block.to_lowercase())
                {
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
    use std::str::FromStr;

    #[test]
    fn test_set_type_parsing() -> Result<()> {
        assert_eq!(SetType::from_str("core")?, SetType::Core);
        assert_eq!(SetType::from_str("expansion")?, SetType::Expansion);
        Ok(())
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
