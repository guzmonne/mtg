use super::types::*;
use crate::cache::{CacheStore, DiskCache};
use serde_json::Value;
use std::time::Duration;

/// Client for interacting with the Gatherer API
#[derive(Debug, Clone)]
pub struct GathererClient {
    http_client: reqwest::Client,
    cache: Option<DiskCache>,
    verbose: bool,
}

impl GathererClient {
    /// Create a new GathererClient with default settings
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Create a builder for configuring the client
    pub fn builder() -> GathererClientBuilder {
        GathererClientBuilder::new()
    }

    /// Search for cards using the provided parameters
    pub async fn search(&self, params: &SearchParams) -> Result<SearchResponse> {
        let cache_key = self.generate_search_cache_key(params);

        if self.verbose {
            println!("Gatherer search cache key: {cache_key}");
        }

        // Check cache first if available
        if let Some(ref cache) = self.cache {
            match cache.get(&cache_key).await {
                Ok(Some(cached_response)) => {
                    if self.verbose {
                        println!("Using cached Gatherer search response");
                    }
                    return Ok(cached_response);
                }
                Ok(None) => {
                    if self.verbose {
                        println!("Cache miss for Gatherer search");
                    }
                }
                Err(e) => {
                    if self.verbose {
                        eprintln!("Cache error (continuing without cache): {e}");
                    }
                }
            }
        }

        // Build the request
        let request = self.build_search_request(params);
        let payload = serde_json::json!(["$undefined", request, {}, true, params.page]);

        if self.verbose {
            println!(
                "Gatherer request payload: {}",
                serde_json::to_string_pretty(&payload)?
            );
        }

        // Make the HTTP request
        let response = self.http_client
            .post("https://gatherer.wizards.com/advanced-search")
            .header("accept", "text/x-component")
            .header("accept-language", "en-US,en;q=0.9")
            .header("cache-control", "no-cache")
            .header("content-type", "text/plain;charset=UTF-8")
            .header("dnt", "1")
            .header("next-action", "7fdc558e6830828dfb95ae6a9638513cb4727e9b52")
            .header("next-router-state-tree", "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22en%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22advanced-search%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Fadvanced-search%22%2C%22refresh%22%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D%7D%2Cnull%2Cnull%5D")
            .header("origin", "https://gatherer.wizards.com")
            .header("pragma", "no-cache")
            .header("priority", "u=1, i")
            .header("referer", "https://gatherer.wizards.com/advanced-search")
            .header("sec-ch-ua", "\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\"")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", "\"macOS\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
            .json(&payload)
            .send()
            .await?;

        let response_text = response.text().await?;

        if self.verbose {
            println!(
                "Gatherer response length: {} characters",
                response_text.len()
            );
        }

        // Parse the Next.js server action response
        let parsed_data = self.parse_server_action_response(&response_text)?;
        let search_response: SearchResponse = serde_json::from_value(parsed_data)?;

        // Cache the response if cache is available
        if let Some(ref cache) = self.cache {
            if let Err(e) = cache.insert(cache_key, search_response.clone()).await {
                if self.verbose {
                    eprintln!("Failed to cache Gatherer search response: {e}");
                }
            } else if self.verbose {
                println!("Gatherer search response cached");
            }
        }

        Ok(search_response)
    }

    /// Get a specific card by name
    pub async fn get_card(&self, name: &str) -> Result<Card> {
        let params = SearchParams {
            name: Some(name.to_string()),
            page: 1,
            ..Default::default()
        };

        let response = self.search(&params).await?;

        if let Some(items) = response.items {
            if items.is_empty() {
                return Err(GathererError::CardNotFound(name.to_string()));
            }

            // Look for exact name match first
            for item in &items {
                if let Some(card_name) = &item.name {
                    if card_name.eq_ignore_ascii_case(name) {
                        return Ok(item.clone());
                    }
                }
            }

            // Return first match if no exact match found
            Ok(items[0].clone())
        } else {
            Err(GathererError::CardNotFound(name.to_string()))
        }
    }

    /// Check if verbose mode is enabled
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    fn build_search_request(&self, params: &SearchParams) -> SearchRequest {
        let mut request = SearchRequest::default();

        // Map parameters to request fields
        if let Some(ref name) = params.name {
            request.card_name = name.clone();
        }
        if let Some(ref rules) = params.rules {
            request.rules = rules.clone();
        }
        if let Some(ref supertype) = params.supertype {
            request.instance_super_type = self.format_query_with_operators(supertype);
        }
        if let Some(ref card_type) = params.card_type {
            request.instance_type = self.format_query_with_operators(card_type);
        }
        if let Some(ref subtype) = params.subtype {
            request.instance_subtype = self.format_query_with_operators(subtype);
        }
        if let Some(ref mana_cost) = params.mana_cost {
            request.mana_cost = mana_cost.replace(' ', "_");
        }
        if let Some(ref set) = params.set {
            let escaped_set = set.replace([' ', ':'], "_");
            request.set_name = format!(
                "eq~{}~{}",
                escaped_set,
                set.chars().take(3).collect::<String>().to_uppercase()
            );
        }
        if let Some(ref rarity) = params.rarity {
            let rarity_code = match rarity.to_lowercase().as_str() {
                "common" => "C",
                "uncommon" => "U",
                "rare" => "R",
                "mythic" | "mythic rare" => "M",
                _ => rarity,
            };
            request.rarity_name = format!("eq~{rarity}~{rarity_code}");
        }
        if let Some(ref artist) = params.artist {
            request.artist_name = artist.clone();
        }
        if let Some(ref power) = params.power {
            request.power = power.replace('-', "_");
        }
        if let Some(ref toughness) = params.toughness {
            request.toughness = toughness.replace('-', "_");
        }
        if let Some(ref loyalty) = params.loyalty {
            request.loyalty = loyalty.replace('-', "_");
        }
        if let Some(ref flavor) = params.flavor {
            request.flavor_text = flavor.clone();
        }
        if let Some(ref colors) = params.colors {
            if colors.starts_with('!') || colors.starts_with("not ") {
                let clean_colors = colors
                    .trim_start_matches('!')
                    .trim_start_matches("not ")
                    .trim();
                request.colors = format!("neq~{}", clean_colors.replace(',', "_"));
            } else {
                request.colors = colors.replace(',', "_");
            }
        }
        if let Some(ref format) = params.format {
            request.format_legalities = format.clone();
        }
        if let Some(ref language) = params.language {
            let lang_code = match language.to_lowercase().as_str() {
                "english" => "en-us",
                "japanese" => "ja-jp",
                "french" => "fr-fr",
                "german" => "de-de",
                "spanish" => "es-es",
                "italian" => "it-it",
                "portuguese" => "pt-br",
                "russian" => "ru-ru",
                "korean" => "ko-kr",
                "chinese simplified" | "simplified chinese" => "zh-cn",
                "chinese traditional" | "traditional chinese" => "zh-tw",
                _ => language,
            };
            request.language = format!("eq~{language}~{lang_code}");
        }

        request.page = params.page.to_string();
        request
    }

    fn format_query_with_operators(&self, query: &str) -> String {
        if query.contains(',') || query.contains('+') {
            let parts: Vec<&str> = if query.contains(',') {
                query.split(',').collect()
            } else {
                query.split('+').collect()
            };

            let operator = if query.contains(',') { "~OR~" } else { "~AND~" };

            parts
                .iter()
                .map(|part| format!("eq~{}", part.trim()))
                .collect::<Vec<String>>()
                .join(&format!(",{operator},"))
        } else {
            format!("eq~{query}")
        }
    }

    fn parse_server_action_response(&self, response: &str) -> Result<Value> {
        // Next.js server action responses have format:
        // 0:{"a":"$@1","f":"","b":"..."}
        // 1:{"apiVersion":"1","method":"CardData.search","data":{...}}

        for line in response.lines() {
            if let Some(colon_pos) = line.find(':') {
                let json_part = &line[colon_pos + 1..];

                // Try to parse this line as JSON
                if let Ok(parsed) = serde_json::from_str::<Value>(json_part) {
                    // Look for the card data in the response
                    if let Some(data) = parsed.get("data") {
                        return Ok(data.clone());
                    }
                    // If this line contains card data directly, return it
                    if parsed.get("apiVersion").is_some() || parsed.get("kind").is_some() {
                        return Ok(parsed);
                    }
                }
            }
        }

        // If no structured data found, try parsing the entire response as JSON
        match serde_json::from_str::<Value>(response) {
            Ok(json) => Ok(json),
            Err(_) => Err(GathererError::InvalidResponse),
        }
    }

    fn generate_search_cache_key(&self, params: &SearchParams) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Hash all search parameters
        params.name.hash(&mut hasher);
        params.rules.hash(&mut hasher);
        params.card_type.hash(&mut hasher);
        params.subtype.hash(&mut hasher);
        params.supertype.hash(&mut hasher);
        params.mana_cost.hash(&mut hasher);
        params.set.hash(&mut hasher);
        params.rarity.hash(&mut hasher);
        params.artist.hash(&mut hasher);
        params.power.hash(&mut hasher);
        params.toughness.hash(&mut hasher);
        params.loyalty.hash(&mut hasher);
        params.flavor.hash(&mut hasher);
        params.colors.hash(&mut hasher);
        params.format.hash(&mut hasher);
        params.language.hash(&mut hasher);
        params.page.hash(&mut hasher);

        format!("gatherer_search_{:x}", hasher.finish())
    }
}

impl Default for GathererClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default GathererClient")
    }
}

/// Builder for configuring a GathererClient
#[derive(Debug)]
pub struct GathererClientBuilder {
    timeout_secs: u64,
    verbose: bool,
    enable_cache: bool,
    cache_dir: Option<std::path::PathBuf>,
    cache_ttl_hours: u64,
}

impl GathererClientBuilder {
    pub fn new() -> Self {
        Self {
            timeout_secs: 30,
            verbose: false,
            enable_cache: true,
            cache_dir: None,
            cache_ttl_hours: 24,
        }
    }

    pub fn timeout_secs(mut self, timeout: u64) -> Self {
        self.timeout_secs = timeout;
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn enable_cache(mut self, enable: bool) -> Self {
        self.enable_cache = enable;
        self
    }

    pub fn cache_dir<P: Into<std::path::PathBuf>>(mut self, dir: P) -> Self {
        self.cache_dir = Some(dir.into());
        self
    }

    pub fn cache_ttl_hours(mut self, hours: u64) -> Self {
        self.cache_ttl_hours = hours;
        self
    }

    pub fn build(self) -> Result<GathererClient> {
        // Build HTTP client
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()?;

        // Build high-level cache for search responses
        let cache = if self.enable_cache {
            let mut cache_builder = DiskCache::builder();

            if let Some(cache_dir) = self.cache_dir {
                cache_builder = cache_builder.base_path(cache_dir);
            }

            Some(cache_builder.prefix("gatherer").build()?)
        } else {
            None
        };

        Ok(GathererClient {
            http_client,
            cache,
            verbose: self.verbose,
        })
    }
}

impl Default for GathererClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_gatherer_client_builder() {
        let client = GathererClient::builder()
            .timeout_secs(60)
            .verbose(true)
            .enable_cache(false)
            .build()
            .expect("Failed to build client");

        assert!(client.is_verbose());
        assert!(client.cache.is_none());
    }

    #[test]
    fn test_gatherer_client_with_cache() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let client = GathererClient::builder()
            .enable_cache(true)
            .cache_dir(temp_dir.path())
            .build()
            .expect("Failed to build client");

        assert!(client.cache.is_some());
    }

    #[test]
    fn test_search_params_default() {
        let params = SearchParams::default();
        assert_eq!(params.page, 0);
        assert!(params.name.is_none());
    }

    #[test]
    fn test_format_query_with_operators() {
        let client = GathererClient::new().expect("Failed to create client");

        // Test OR operation
        let result = client.format_query_with_operators("Creature,Instant");
        assert!(result.contains("~OR~"));

        // Test AND operation
        let result = client.format_query_with_operators("Creature+Legendary");
        assert!(result.contains("~AND~"));

        // Test simple query
        let result = client.format_query_with_operators("Creature");
        assert_eq!(result, "eq~Creature");
    }

    #[test]
    fn test_cache_key_generation() {
        let client = GathererClient::new().expect("Failed to create client");

        let params1 = SearchParams {
            name: Some("Lightning Bolt".to_string()),
            page: 1,
            ..Default::default()
        };

        let params2 = SearchParams {
            name: Some("Lightning Bolt".to_string()),
            page: 1,
            ..Default::default()
        };

        let params3 = SearchParams {
            name: Some("Counterspell".to_string()),
            page: 1,
            ..Default::default()
        };

        let key1 = client.generate_search_cache_key(&params1);
        let key2 = client.generate_search_cache_key(&params2);
        let key3 = client.generate_search_cache_key(&params3);

        assert_eq!(key1, key2); // Same parameters should generate same key
        assert_ne!(key1, key3); // Different parameters should generate different keys
        assert!(key1.starts_with("gatherer_search_"));
    }

    #[test]
    fn test_build_search_request() {
        let client = GathererClient::new().expect("Failed to create client");

        let params = SearchParams {
            name: Some("Lightning Bolt".to_string()),
            card_type: Some("Instant".to_string()),
            colors: Some("R".to_string()),
            rarity: Some("Common".to_string()),
            page: 1,
            ..Default::default()
        };

        let request = client.build_search_request(&params);

        assert_eq!(request.card_name, "Lightning Bolt");
        assert_eq!(request.instance_type, "eq~Instant");
        assert_eq!(request.colors, "R");
        assert_eq!(request.rarity_name, "eq~Common~C");
        assert_eq!(request.page, "1");
    }

    #[test]
    fn test_rarity_mapping() {
        let client = GathererClient::new().expect("Failed to create client");

        let test_cases = vec![
            ("common", "eq~common~C"),
            ("uncommon", "eq~uncommon~U"),
            ("rare", "eq~rare~R"),
            ("mythic", "eq~mythic~M"),
            ("mythic rare", "eq~mythic rare~M"),
        ];

        for (input, expected) in test_cases {
            let params = SearchParams {
                rarity: Some(input.to_string()),
                ..Default::default()
            };

            let request = client.build_search_request(&params);
            assert_eq!(request.rarity_name, expected);
        }
    }

    #[test]
    fn test_language_mapping() {
        let client = GathererClient::new().expect("Failed to create client");

        let test_cases = vec![
            ("english", "eq~english~en-us"),
            ("japanese", "eq~japanese~ja-jp"),
            ("french", "eq~french~fr-fr"),
            ("german", "eq~german~de-de"),
        ];

        for (input, expected) in test_cases {
            let params = SearchParams {
                language: Some(input.to_string()),
                ..Default::default()
            };

            let request = client.build_search_request(&params);
            assert_eq!(request.language, expected);
        }
    }

    #[test]
    fn test_color_negation() {
        let client = GathererClient::new().expect("Failed to create client");

        // Test negation with !
        let params1 = SearchParams {
            colors: Some("!RBW".to_string()),
            ..Default::default()
        };
        let request1 = client.build_search_request(&params1);
        assert_eq!(request1.colors, "neq~RBW");

        // Test negation with "not "
        let params2 = SearchParams {
            colors: Some("not RBW".to_string()),
            ..Default::default()
        };
        let request2 = client.build_search_request(&params2);
        assert_eq!(request2.colors, "neq~RBW");

        // Test normal colors
        let params3 = SearchParams {
            colors: Some("RBW".to_string()),
            ..Default::default()
        };
        let request3 = client.build_search_request(&params3);
        assert_eq!(request3.colors, "RBW");
    }
}
