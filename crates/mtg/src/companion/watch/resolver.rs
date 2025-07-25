#![allow(dead_code)]

use super::types::CardInfo;
use crate::prelude::*;
use std::collections::HashMap;

pub struct CardResolver {
    cache: HashMap<u32, CardInfo>,
    http_client: reqwest::Client,
}

impl CardResolver {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            http_client: reqwest::Client::builder()
                .user_agent("mtg-cli/0.0.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn resolve_card(&mut self, grp_id: u32) -> Result<Option<CardInfo>> {
        // Check cache first
        if let Some(card_info) = self.cache.get(&grp_id) {
            return Ok(Some(card_info.clone()));
        }

        // Try to resolve via Scryfall API
        match self.resolve_from_scryfall(grp_id).await {
            Ok(card_info) => {
                self.cache.insert(grp_id, card_info.clone());
                Ok(Some(card_info))
            }
            Err(e) => {
                aeprintln!("Failed to resolve card with grpId {}: {}", grp_id, e);
                Ok(None)
            }
        }
    }

    async fn resolve_from_scryfall(&self, grp_id: u32) -> Result<CardInfo> {
        // Arena grpId to Scryfall ID mapping is complex
        // For now, we'll try a few approaches:

        // 1. Try searching by Arena ID (some cards have this)
        let search_url = format!("https://api.scryfall.com/cards/search?q=arena:{}", grp_id);
        if let Ok(response) = self.http_client.get(&search_url).send().await {
            if let Ok(search_result) = response
                .json::<crate::scryfall::List<crate::scryfall::Card>>()
                .await
            {
                if let Some(first_card) = search_result.data.first() {
                    return Ok(CardInfo {
                        name: first_card.name.clone(),
                        mana_cost: first_card.mana_cost.clone().unwrap_or_default(),
                        type_line: first_card.type_line.clone(),
                        oracle_text: first_card.oracle_text.clone().unwrap_or_default(),
                    });
                }
            }
        }

        // 2. If search fails, create a placeholder
        // Arena grpId mapping is complex and would require a separate database
        Ok(CardInfo {
            name: format!("Unknown Card (grpId: {})", grp_id),
            mana_cost: String::new(),
            type_line: "Unknown".to_string(),
            oracle_text: String::new(),
        })
    }

    pub async fn resolve_multiple(&mut self, grp_ids: &[u32]) -> Result<HashMap<u32, CardInfo>> {
        let mut results = HashMap::new();

        // Process in batches to avoid overwhelming the API
        for chunk in grp_ids.chunks(10) {
            let mut tasks = Vec::new();

            for &grp_id in chunk {
                if !self.cache.contains_key(&grp_id) {
                    tasks.push(grp_id);
                }
            }

            // Resolve uncached cards
            for grp_id in tasks {
                if let Ok(Some(card_info)) = self.resolve_card(grp_id).await {
                    results.insert(grp_id, card_info);
                }

                // Small delay to be respectful to the API
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }

            // Add cached cards to results
            for &grp_id in chunk {
                if let Some(card_info) = self.cache.get(&grp_id) {
                    results.insert(grp_id, card_info.clone());
                }
            }
        }

        Ok(results)
    }

    pub fn get_cached(&self, grp_id: u32) -> Option<&CardInfo> {
        self.cache.get(&grp_id)
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for CardResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_card_resolver_cache() {
        let mut resolver = CardResolver::new();

        // Mock a card info
        let card_info = CardInfo {
            name: "Lightning Bolt".to_string(),
            mana_cost: "R".to_string(),
            type_line: "Instant".to_string(),
            oracle_text: "Lightning Bolt deals 3 damage to any target.".to_string(),
        };

        resolver.cache.insert(12345, card_info.clone());

        // Should return cached result
        let result = resolver.get_cached(12345);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Lightning Bolt");

        // Should return None for uncached card
        let result = resolver.get_cached(99999);
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_operations() {
        let mut resolver = CardResolver::new();
        assert_eq!(resolver.cache_size(), 0);

        let card_info = CardInfo {
            name: "Test Card".to_string(),
            mana_cost: "1U".to_string(),
            type_line: "Creature".to_string(),
            oracle_text: "Test text".to_string(),
        };

        resolver.cache.insert(1, card_info);
        assert_eq!(resolver.cache_size(), 1);

        resolver.clear_cache();
        assert_eq!(resolver.cache_size(), 0);
    }
}
