use super::client::ScryfallClient;
use super::types::*;
use color_eyre::Result;

impl ScryfallClient {
    /// Smart search that auto-detects query intent and routes to appropriate method
    pub async fn smart_search(&self, query: &str) -> Result<SmartSearchResult> {
        let query = query.trim();

        if let Some(intent) = self.detect_query_intent(query) {
            match intent {
                QueryIntent::ExactCardName(name) => {
                    let card = self.get_card_named(&name, None).await?;
                    Ok(SmartSearchResult::SingleCard(Box::new(card)))
                }
                QueryIntent::SetCollector(set, collector) => {
                    let card = self.get_card_by_collector(&set, &collector, None).await?;
                    Ok(SmartSearchResult::SingleCard(Box::new(card)))
                }
                QueryIntent::ArenaId(id) => {
                    let card = self.get_card_by_arena_id(id).await?;
                    Ok(SmartSearchResult::SingleCard(Box::new(card)))
                }
                QueryIntent::MtgoId(id) => {
                    let card = self.get_card_by_mtgo_id(id).await?;
                    Ok(SmartSearchResult::SingleCard(Box::new(card)))
                }
                QueryIntent::ScryfallId(id) => {
                    let card = self.get_card_by_id(&id).await?;
                    Ok(SmartSearchResult::SingleCard(Box::new(card)))
                }
                QueryIntent::SearchQuery(search_query) => {
                    let params = SearchParams {
                        q: search_query,
                        ..Default::default()
                    };
                    let response = self.search_cards(params).await?;
                    Ok(SmartSearchResult::SearchResults(response))
                }
            }
        } else {
            // Fallback to search if we can't detect intent
            let params = SearchParams {
                q: query.to_string(),
                ..Default::default()
            };
            let response = self.search_cards(params).await?;
            Ok(SmartSearchResult::SearchResults(response))
        }
    }

    /// Detect what kind of query this is based on patterns
    pub fn detect_query_intent(&self, query: &str) -> Option<QueryIntent> {
        let query = query.trim();

        // Check for Scryfall UUID (36 characters with hyphens)
        if query.len() == 36 && query.chars().filter(|&c| c == '-').count() == 4 {
            return Some(QueryIntent::ScryfallId(query.to_string()));
        }

        // Check for pure numbers (Arena/MTGO IDs)
        if let Ok(id) = query.parse::<u32>() {
            // Heuristic: Arena IDs are typically larger than MTGO IDs
            if id > 50000 {
                return Some(QueryIntent::ArenaId(id));
            } else {
                return Some(QueryIntent::MtgoId(id));
            }
        }

        // Check for "SET COLLECTOR" pattern (e.g., "ktk 96", "war 001")
        let parts: Vec<&str> = query.split_whitespace().collect();
        if parts.len() == 2 {
            let potential_set = parts[0].to_lowercase();
            let potential_collector = parts[1];

            // Check if first part looks like a set code (2-4 characters, mostly letters)
            if potential_set.len() >= 2
                && potential_set.len() <= 4
                && potential_set.chars().all(|c| c.is_alphanumeric())
            {
                // Check if second part looks like a collector number
                if potential_collector.chars().any(|c| c.is_ascii_digit()) {
                    return Some(QueryIntent::SetCollector(
                        potential_set,
                        potential_collector.to_string(),
                    ));
                }
            }
        }

        // Check if it contains Scryfall search syntax
        if query.contains(':')
            || query.contains(">=")
            || query.contains("<=")
            || query.contains("!=")
            || query.contains('>')
            || query.contains('<')
        {
            return Some(QueryIntent::SearchQuery(query.to_string()));
        }

        // Check for common search patterns
        let lower_query = query.to_lowercase();
        if lower_query.contains(" creature")
            || lower_query.contains(" instant")
            || lower_query.contains(" sorcery")
            || lower_query.contains(" artifact")
            || lower_query.contains(" enchantment")
            || lower_query.contains(" planeswalker")
        {
            return Some(QueryIntent::SearchQuery(query.to_string()));
        }

        // If it's a simple phrase without special characters, treat as exact card name
        if !query.contains('"') && !query.contains('(') && !query.contains('[') {
            return Some(QueryIntent::ExactCardName(query.to_string()));
        }

        // Default to search query
        Some(QueryIntent::SearchQuery(query.to_string()))
    }

    /// Validate query syntax and provide suggestions
    pub fn validate_query(&self, query: &str) -> Result<String, String> {
        let query = query.trim();

        // Check for empty query
        if query.is_empty() {
            return Err("Empty query provided".to_string());
        }

        // Check for common mistakes
        let issues = self.find_query_issues(query);
        if !issues.is_empty() {
            let suggestions: Vec<String> = issues
                .iter()
                .map(|issue| match issue {
                    QueryIssue::UnknownKeyword(keyword) => {
                        if let Some(suggestion) = self.suggest_keyword_correction(keyword) {
                            format!(
                                "Unknown keyword '{keyword}'. Did you mean '{suggestion}'?"
                            )
                        } else {
                            format!(
                                "Unknown keyword '{keyword}'. Check documentation for valid keywords."
                            )
                        }
                    }
                    QueryIssue::InvalidOperator(op) => {
                        format!("Invalid operator '{op}'. Use: =, !=, <, <=, >, >=")
                    }
                    QueryIssue::MalformedExpression(expr) => {
                        format!("Malformed expression '{expr}'. Check syntax.")
                    }
                })
                .collect();

            return Err(format!(
                "Query validation issues:\n  {}",
                suggestions.join("\n  ")
            ));
        }

        Ok(query.to_string())
    }

    fn find_query_issues(&self, query: &str) -> Vec<QueryIssue> {
        let mut issues = Vec::new();

        // Valid Scryfall keywords
        let valid_keywords = [
            "c",
            "color",
            "colors",
            "id",
            "identity",
            "m",
            "mana",
            "mv",
            "cmc",
            "t",
            "type",
            "o",
            "oracle",
            "pow",
            "power",
            "tou",
            "toughness",
            "loy",
            "loyalty",
            "r",
            "rarity",
            "s",
            "set",
            "f",
            "format",
            "a",
            "artist",
            "ft",
            "flavor",
            "is",
            "not",
            "cn",
            "number",
            "lang",
            "language",
            "year",
            "frame",
            "border",
            "game",
            "legal",
            "banned",
            "restricted",
            "new",
            "old",
            "reprint",
            "firstprint",
            "unique",
            "art",
            "prints",
            "usd",
            "eur",
            "tix",
            "penny",
        ];

        // Check for unknown keywords
        for part in query.split_whitespace() {
            if part.contains(':') {
                let keyword = part.split(':').next().unwrap_or("");
                if !keyword.is_empty() && !valid_keywords.contains(&keyword.to_lowercase().as_str())
                {
                    issues.push(QueryIssue::UnknownKeyword(keyword.to_string()));
                }
            }
        }

        issues
    }

    fn suggest_keyword_correction(&self, keyword: &str) -> Option<String> {
        let keyword_lower = keyword.to_lowercase();

        let corrections = [
            ("colour", "c"),
            ("color", "c"),
            ("type", "t"),
            ("oracle", "o"),
            ("manavalue", "mv"),
            ("manacost", "m"),
            ("power", "pow"),
            ("toughness", "tou"),
            ("loyalty", "loy"),
            ("rarity", "r"),
            ("set", "s"),
            ("format", "f"),
            ("artist", "a"),
            ("flavor", "ft"),
            ("identity", "id"),
            ("cmc", "mv"),
        ];

        for (wrong, right) in &corrections {
            if keyword_lower == *wrong {
                return Some(right.to_string());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_query_intent() {
        let client = ScryfallClient::new().expect("Failed to create client");

        // Test Scryfall UUID
        let uuid = "12345678-1234-1234-1234-123456789012";
        assert!(matches!(
            client.detect_query_intent(uuid),
            Some(QueryIntent::ScryfallId(_))
        ));

        // Test Arena ID (large number)
        assert!(matches!(
            client.detect_query_intent("75000"),
            Some(QueryIntent::ArenaId(75000))
        ));

        // Test MTGO ID (small number)
        assert!(matches!(
            client.detect_query_intent("1000"),
            Some(QueryIntent::MtgoId(1000))
        ));

        // Test set/collector pattern
        assert!(matches!(
            client.detect_query_intent("ktk 96"),
            Some(QueryIntent::SetCollector(_, _))
        ));

        // Test search query with syntax
        assert!(matches!(
            client.detect_query_intent("c:red t:creature"),
            Some(QueryIntent::SearchQuery(_))
        ));

        // Test exact card name
        assert!(matches!(
            client.detect_query_intent("Lightning Bolt"),
            Some(QueryIntent::ExactCardName(_))
        ));
    }

    #[test]
    fn test_validate_query() {
        let client = ScryfallClient::new().expect("Failed to create client");

        // Valid query
        assert!(client.validate_query("c:red t:creature").is_ok());

        // Empty query
        assert!(client.validate_query("").is_err());
        assert!(client.validate_query("   ").is_err());

        // Valid query with known keywords
        assert!(client.validate_query("color:red type:creature").is_ok());
    }

    #[test]
    fn test_suggest_keyword_correction() {
        let client = ScryfallClient::new().expect("Failed to create client");

        assert_eq!(
            client.suggest_keyword_correction("color"),
            Some("c".to_string())
        );
        assert_eq!(
            client.suggest_keyword_correction("type"),
            Some("t".to_string())
        );
        assert_eq!(
            client.suggest_keyword_correction("oracle"),
            Some("o".to_string())
        );
        assert_eq!(client.suggest_keyword_correction("unknown"), None);
    }

    #[test]
    fn test_query_issue_detection() {
        let client = ScryfallClient::new().expect("Failed to create client");

        // Valid query should have no issues
        let issues = client.find_query_issues("c:red t:creature");
        assert!(issues.is_empty());

        // Query with unknown keyword should have issues
        let issues = client.find_query_issues("badkeyword:value");
        assert_eq!(issues.len(), 1);
        match &issues[0] {
            QueryIssue::UnknownKeyword(keyword) => assert_eq!(keyword, "badkeyword"),
            _ => panic!("Expected UnknownKeyword issue"),
        }
    }

    #[test]
    fn test_smart_search_intent_patterns() {
        let client = ScryfallClient::new().expect("Failed to create client");

        // Test various patterns
        let test_cases = vec![
            (
                "Lightning Bolt",
                QueryIntent::ExactCardName("Lightning Bolt".to_string()),
            ),
            (
                "ktk 96",
                QueryIntent::SetCollector("ktk".to_string(), "96".to_string()),
            ),
            ("75000", QueryIntent::ArenaId(75000)),
            ("1000", QueryIntent::MtgoId(1000)),
            (
                "c:red t:creature",
                QueryIntent::SearchQuery("c:red t:creature".to_string()),
            ),
            (
                "red creature",
                QueryIntent::SearchQuery("red creature".to_string()),
            ),
        ];

        for (query, expected) in test_cases {
            let result = client.detect_query_intent(query);
            assert!(result.is_some(), "Failed to detect intent for: {query}");

            let intent = result.unwrap();
            match (&intent, &expected) {
                (QueryIntent::ExactCardName(a), QueryIntent::ExactCardName(b)) => assert_eq!(a, b),
                (QueryIntent::SetCollector(a1, a2), QueryIntent::SetCollector(b1, b2)) => {
                    assert_eq!(a1, b1);
                    assert_eq!(a2, b2);
                }
                (QueryIntent::ArenaId(a), QueryIntent::ArenaId(b)) => assert_eq!(a, b),
                (QueryIntent::MtgoId(a), QueryIntent::MtgoId(b)) => assert_eq!(a, b),
                (QueryIntent::SearchQuery(a), QueryIntent::SearchQuery(b)) => assert_eq!(a, b),
                _ => panic!(
                    "Intent mismatch for query '{query}': expected {expected:?}, got {intent:?}"
                ),
            }
        }
    }
}
