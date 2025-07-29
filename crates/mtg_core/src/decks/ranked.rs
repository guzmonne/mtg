use crate::cache::{CacheStore, CachedHttpClient, DiskCache};
use crate::decks::{generate_short_hash, parse_deck_list, ParsedDeck};
use color_eyre::Result;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ParsedDecksResponse {
    pub url: String,
    pub decks: Vec<ParsedDeck>,
}

// Contentful API structures
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulFields {
    #[serde(rename = "metaTitle")]
    pub meta_title: Option<String>,
    #[serde(rename = "metaDescription")]
    pub meta_description: Option<String>,
    #[serde(rename = "articleTitle")]
    pub article_title: String,
    #[serde(rename = "outboundLink")]
    pub outbound_link: Option<String>,
    pub slug: String,
    pub author: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: String,
    #[serde(rename = "decklistBody")]
    pub decklist_body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulItemSys {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulItem {
    pub sys: ContentfulItemSys,
    pub fields: ContentfulFields,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulSys {
    #[serde(rename = "type")]
    pub sys_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulResponse {
    pub sys: ContentfulSys,
    pub total: u32,
    pub skip: u32,
    pub limit: u32,
    pub items: Vec<ContentfulItem>,
}

// Extended item with ID for display
#[derive(Debug, Clone)]
pub struct ContentfulItemWithId {
    pub id: String,
    pub item: ContentfulItem,
}

#[derive(Debug, Clone)]
pub struct RankedListParams {
    pub format_filter: Option<String>,
    pub limit: u32,
    pub skip: u32,
}

#[derive(Debug, Clone)]
pub struct RankedListResponse {
    pub contentful_response: ContentfulResponse,
    pub items_with_ids: Vec<ContentfulItemWithId>,
}

/// Client for fetching and parsing ranked deck lists from tournament articles
pub struct RankedDecksClient {
    http_client: CachedHttpClient,
    cache: DiskCache,
}

impl RankedDecksClient {
    /// Create a new RankedDecksClient with the provided HTTP client and cache
    pub fn new(http_client: CachedHttpClient, cache: DiskCache) -> Self {
        Self { http_client, cache }
    }

    /// Fetch ranked deck list from Contentful API
    pub async fn fetch_ranked_list(&self, params: RankedListParams) -> Result<RankedListResponse> {
        let url = self.build_contentful_url(&params);

        // Create headers for Contentful API
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "accept",
            "application/json, text/plain, */*".parse().unwrap(),
        );
        headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
        headers.insert(
            "authorization",
            "Bearer 55006dd7d868409c694628081e43f6ce5d1cee174943d8fcb03ca66507390427"
                .parse()
                .unwrap(),
        );
        headers.insert("cache-control", "no-cache".parse().unwrap());
        headers.insert("dnt", "1".parse().unwrap());
        headers.insert("origin", "https://magic.gg".parse().unwrap());
        headers.insert("pragma", "no-cache".parse().unwrap());
        headers.insert("priority", "u=1, i".parse().unwrap());
        headers.insert("referer", "https://magic.gg/".parse().unwrap());
        headers.insert(
            "sec-ch-ua",
            r#""Not)A;Brand";v="8", "Chromium";v="138""#.parse().unwrap(),
        );
        headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
        headers.insert("sec-ch-ua-platform", r#""macOS""#.parse().unwrap());
        headers.insert("sec-fetch-dest", "empty".parse().unwrap());
        headers.insert("sec-fetch-mode", "cors".parse().unwrap());
        headers.insert("sec-fetch-site", "cross-site".parse().unwrap());
        headers.insert(
            "x-contentful-user-agent",
            "sdk contentful.js/0.0.0-determined-by-semantic-release; platform browser; os macOS;"
                .parse()
                .unwrap(),
        );

        let response = self
            .http_client
            .get_with_headers(&url, Some(headers))
            .await?;
        let response_text = response.text()?;

        let contentful_response: ContentfulResponse = serde_json::from_str(&response_text)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to parse Contentful response: {}", e))?;

        // Cache each item and generate IDs
        let mut items_with_ids = Vec::new();

        for item in &contentful_response.items {
            let item_hash = generate_short_hash(&item);

            // Construct the link
            let link = self.construct_article_link(&item.fields);

            // Cache the item with the link included
            let cache_key = format!("ranked_article_{item_hash}");
            let mut item_json = serde_json::to_value(item)?;
            if let Some(obj) = item_json.as_object_mut() {
                obj.insert("link".to_string(), serde_json::Value::String(link));
            }
            self.cache.insert(&cache_key, item_json).await?;

            items_with_ids.push(ContentfulItemWithId {
                id: item_hash,
                item: item.clone(),
            });
        }

        Ok(RankedListResponse {
            contentful_response,
            items_with_ids,
        })
    }

    /// Build Contentful API URL with parameters
    fn build_contentful_url(&self, params: &RankedListParams) -> String {
        let mut url = format!(
            "https://cdn.contentful.com/spaces/ryplwhabvmmk/environments/master/entries?content_type=decklistArticle&include=10&order=-fields.publishedDate&limit={}&skip={}",
            params.limit, params.skip
        );

        // Add format filter if provided
        if let Some(format) = &params.format_filter {
            url.push_str(&format!("&fields.articleTitle%5Bmatch%5D={format}"));
        }

        url
    }

    /// Construct article link from fields
    fn construct_article_link(&self, fields: &ContentfulFields) -> String {
        if let Some(outbound) = &fields.outbound_link {
            outbound.clone()
        } else {
            format!("https://magic.gg/decklists/{}", fields.slug)
        }
    }

    /// Fetch and parse decks from an article identifier (ID or URL)
    pub async fn fetch_decks_from_article(&self, identifier: &str) -> Result<Vec<ParsedDeck>> {
        let url = self.resolve_url(identifier).await?;
        self.parse_decks_from_url(&url).await
    }

    /// Fetch and parse decks from an article, returning full response with URL
    pub async fn fetch_decks_response(&self, identifier: &str) -> Result<ParsedDecksResponse> {
        let url = self.resolve_url(identifier).await?;
        let decks = self.parse_decks_from_url(&url).await?;

        Ok(ParsedDecksResponse { url, decks })
    }

    /// Resolve an identifier to a URL (handles both URLs and cached IDs)
    async fn resolve_url(&self, identifier: &str) -> Result<String> {
        if identifier.starts_with("http://") || identifier.starts_with("https://") {
            Ok(identifier.to_string())
        } else {
            // It's an ID, fetch from cache
            let cache_key = format!("ranked_article_{identifier}");
            let cached_data: serde_json::Value =
                self.cache.get(&cache_key).await?.ok_or_else(|| {
                    color_eyre::eyre::eyre!("No cached item found with ID: {}", identifier)
                })?;

            // Extract the link from the cached item
            Ok(cached_data
                .get("link")
                .and_then(|v| v.as_str())
                .ok_or_else(|| color_eyre::eyre::eyre!("No link found in cached item"))?
                .to_string())
        }
    }

    /// Parse decks from a URL
    async fn parse_decks_from_url(&self, url: &str) -> Result<Vec<ParsedDeck>> {
        // Fetch the page
        let response = self.http_client.get(url).await?;
        let html_content = response.text()?;

        // Parse the HTML
        let document = Html::parse_document(&html_content);

        // Find all deck-list web components
        let deck_list_selector = Selector::parse("deck-list")
            .map_err(|e| color_eyre::eyre::eyre!("Invalid CSS selector: {:?}", e))?;
        let mut parsed_decks = Vec::new();

        for deck_element in document.select(&deck_list_selector) {
            if let Some(parsed_deck) = self.parse_deck_element(&deck_element).await? {
                parsed_decks.push(parsed_deck);
            }
        }

        Ok(parsed_decks)
    }

    /// Parse a single deck element from HTML
    async fn parse_deck_element(
        &self,
        deck_element: &scraper::ElementRef<'_>,
    ) -> Result<Option<ParsedDeck>> {
        // Extract attributes from deck-list element
        let deck_title = deck_element.value().attr("deck-title").map(String::from);
        let subtitle = deck_element.value().attr("subtitle").map(String::from);
        let event_date = deck_element.value().attr("event-date").map(String::from);
        let event_name = deck_element.value().attr("event-name").map(String::from);
        let format = deck_element.value().attr("format").map(String::from);

        // Parse main deck
        let main_deck_selector = Selector::parse("main-deck")
            .map_err(|e| color_eyre::eyre::eyre!("Invalid CSS selector: {:?}", e))?;
        let mut main_deck = Vec::new();

        if let Some(main_deck_element) = deck_element.select(&main_deck_selector).next() {
            let deck_content = main_deck_element.text().collect::<String>();
            if let Ok(parsed_list) = parse_deck_list(&deck_content) {
                main_deck = parsed_list.main_deck;
            }
        }

        // Parse sideboard
        let sideboard_selector = Selector::parse("side-board")
            .map_err(|e| color_eyre::eyre::eyre!("Invalid CSS selector: {:?}", e))?;
        let mut sideboard = Vec::new();

        if let Some(sideboard_element) = deck_element.select(&sideboard_selector).next() {
            let deck_content = sideboard_element.text().collect::<String>();
            if let Ok(parsed_list) = parse_deck_list(&deck_content) {
                sideboard = parsed_list.main_deck; // Use main_deck since we're parsing just the sideboard content
            }
        }

        // Generate hash for this deck
        let deck_data = serde_json::json!({
            "title": deck_title,
            "subtitle": subtitle,
            "event_date": event_date,
            "event_name": event_name,
            "format": format,
            "main_deck": &main_deck,
            "sideboard": &sideboard,
        });
        let deck_hash = generate_short_hash(&deck_data);

        // Cache the parsed deck
        let cache_key = format!("parsed_deck_{deck_hash}");
        let mut deck_data_with_id = deck_data.clone();
        deck_data_with_id["id"] = serde_json::json!(deck_hash.clone());
        self.cache.insert(&cache_key, deck_data_with_id).await?;

        Ok(Some(ParsedDeck {
            id: deck_hash,
            title: deck_title,
            subtitle,
            event_date,
            event_name,
            format,
            main_deck,
            sideboard,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::DiskCacheBuilder;

    #[tokio::test]
    async fn test_ranked_decks_client_creation() {
        let cache = DiskCacheBuilder::new()
            .prefix("test_ranked")
            .build()
            .unwrap();
        let http_client = CachedHttpClient::builder()
            .cache_prefix("test_http")
            .build()
            .unwrap();
        let _client = RankedDecksClient::new(http_client, cache);
    }

    #[test]
    fn test_url_detection() {
        assert!("https://example.com".starts_with("http"));
        assert!("http://example.com".starts_with("http"));
        assert!(!"abc123".starts_with("http"));
    }
}
