use crate::cache::{
    disk::{CacheStore, DiskCache},
    error::{CacheError, Result},
    serializer::Serializer,
};
use reqwest::{header::HeaderMap, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

/// Cached HTTP response with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub url: String,
    pub cached_at: std::time::SystemTime,
}

impl CachedResponse {
    /// Convert to reqwest::Response-like structure for compatibility
    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK)
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Get response body as text
    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| CacheError::Serialization(format!("Invalid UTF-8 in response body: {e}")))
    }

    /// Get response body as JSON
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        serde_json::from_slice(&self.body)
            .map_err(|e| CacheError::Serialization(format!("JSON deserialization failed: {e}")))
    }

    /// Get header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }
}

/// HTTP client with caching capabilities
#[derive(Debug, Clone)]
pub struct CachedHttpClient {
    client: reqwest::Client,
    cache: DiskCache,
    default_ttl: Option<Duration>,
}

impl CachedHttpClient {
    /// Create a new cached HTTP client with default settings
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Create a builder for configuring the cached HTTP client
    pub fn builder() -> CachedHttpClientBuilder {
        CachedHttpClientBuilder::new()
    }

    /// Generate cache key for a request
    fn cache_key(&self, url: &Url, headers: Option<&HeaderMap>) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();

        // Hash the URL (without fragment)
        let mut url_for_hash = url.clone();
        url_for_hash.set_fragment(None);
        hasher.update(url_for_hash.as_str().as_bytes());

        // Hash relevant headers (excluding cache-control, authorization, etc.)
        if let Some(headers) = headers {
            let mut header_pairs: Vec<_> = headers
                .iter()
                .filter(|(name, _)| {
                    let name_str = name.as_str().to_lowercase();
                    !matches!(
                        name_str.as_str(),
                        "authorization"
                            | "cookie"
                            | "date"
                            | "cache-control"
                            | "expires"
                            | "last-modified"
                            | "etag"
                            | "if-none-match"
                            | "if-modified-since"
                            | "user-agent"
                            | "x-request-id"
                    )
                })
                .map(|(name, value)| (name.as_str(), value.to_str().unwrap_or("")))
                .collect();

            header_pairs.sort_by_key(|(name, _)| *name);

            for (name, value) in header_pairs {
                hasher.update(name.as_bytes());
                hasher.update(b":");
                hasher.update(value.as_bytes());
                hasher.update(b"\n");
            }
        }

        format!("{:x}", hasher.finalize())
    }

    /// Check if a cached response is still valid
    fn is_cache_valid(&self, cached: &CachedResponse) -> bool {
        if let Some(ttl) = self.default_ttl {
            if let Ok(elapsed) = cached.cached_at.elapsed() {
                return elapsed < ttl;
            }
        }

        // If no TTL is set, cache is always valid (manual cleanup only)
        true
    }

    /// Perform a GET request with caching
    pub async fn get(&self, url: &str) -> Result<CachedResponse> {
        self.get_with_headers(url, None).await
    }

    /// Perform a GET request with custom headers and caching
    pub async fn get_with_headers(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> Result<CachedResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| CacheError::InvalidConfiguration(format!("Invalid URL: {e}")))?;

        let cache_key = self.cache_key(&parsed_url, headers.as_ref());

        // Try to get from cache first
        if let Some(cached) = self.cache.get(&cache_key).await? {
            if self.is_cache_valid(&cached) {
                return Ok(cached);
            }
        }

        // Make the actual HTTP request
        let mut request = self.client.get(parsed_url.clone());

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request.send().await.map_err(|e| {
            CacheError::Io(std::io::Error::other(format!("HTTP request failed: {e}")))
        })?;

        // Convert response to cacheable format
        let status = response.status().as_u16();
        let headers_map: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_lowercase(),
                    value.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(|e| {
                CacheError::Io(std::io::Error::other(format!(
                    "Failed to read response body: {e}"
                )))
            })?
            .to_vec();

        let cached_response = CachedResponse {
            status,
            headers: headers_map,
            body,
            url: parsed_url.to_string(),
            cached_at: std::time::SystemTime::now(),
        };

        // Cache the response (only cache successful responses)
        if (200..300).contains(&status) {
            self.cache
                .insert(&cache_key, cached_response.clone())
                .await?;
        }

        Ok(cached_response)
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> Result<crate::cache::types::CacheStats> {
        self.cache.stats(None).await
    }

    /// Clean cache by age
    pub async fn clean_older_than(
        &self,
        age: Duration,
    ) -> Result<crate::cache::types::CleanReport> {
        self.cache.clean_older_than(age, None).await
    }

    /// Clean cache to stay under size limit
    pub async fn clean_to_size_limit(
        &self,
        max_bytes: u64,
    ) -> Result<crate::cache::types::CleanReport> {
        self.cache.clean_to_size_limit(max_bytes, None).await
    }

    /// Clear all cached responses
    pub async fn clear_cache(&self) -> Result<()> {
        <DiskCache as CacheStore<&str, CachedResponse>>::clear(&self.cache).await
    }
}

impl Default for CachedHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default CachedHttpClient")
    }
}

/// Builder for configuring CachedHttpClient
#[derive(Debug)]
pub struct CachedHttpClientBuilder {
    client_builder: reqwest::ClientBuilder,
    cache_prefix: Option<String>,
    cache_base_path: Option<std::path::PathBuf>,
    cache_serializer: Option<Serializer>,
    default_ttl: Option<Duration>,
}

impl CachedHttpClientBuilder {
    pub fn new() -> Self {
        Self {
            client_builder: reqwest::Client::builder(),
            cache_prefix: None,
            cache_base_path: None,
            cache_serializer: None,
            default_ttl: None,
        }
    }

    /// Set HTTP client timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.client_builder = self.client_builder.timeout(timeout);
        self
    }

    /// Set user agent
    pub fn user_agent<V>(mut self, value: V) -> Self
    where
        V: TryInto<reqwest::header::HeaderValue>,
        V::Error: Into<http::Error>,
    {
        self.client_builder = self.client_builder.user_agent(value);
        self
    }

    /// Set default headers
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.client_builder = self.client_builder.default_headers(headers);
        self
    }

    /// Disable caching (for compatibility)
    pub fn disable_cache(self) -> Self {
        // For now, just return self - we could add a flag later
        self
    }

    /// Set cache prefix for organizing cached responses
    pub fn cache_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.cache_prefix = Some(prefix.into());
        self
    }

    /// Set cache base path
    pub fn cache_base_path<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.cache_base_path = Some(path.into());
        self
    }

    /// Set cache serializer
    pub fn cache_serializer(mut self, serializer: Serializer) -> Self {
        self.cache_serializer = Some(serializer);
        self
    }

    /// Set default TTL for cached responses
    pub fn default_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    /// Build the CachedHttpClient
    pub fn build(self) -> Result<CachedHttpClient> {
        let client = self.client_builder.build().map_err(|e| {
            CacheError::InvalidConfiguration(format!("Failed to build HTTP client: {e}"))
        })?;

        let mut cache_builder = DiskCache::builder();

        if let Some(prefix) = self.cache_prefix {
            cache_builder = cache_builder.prefix(prefix);
        }

        if let Some(base_path) = self.cache_base_path {
            cache_builder = cache_builder.base_path(base_path);
        }

        if let Some(serializer) = self.cache_serializer {
            cache_builder = cache_builder.with_serializer(serializer);
        }

        let cache = cache_builder.build()?;

        Ok(CachedHttpClient {
            client,
            cache,
            default_ttl: self.default_ttl,
        })
    }
}

impl Default for CachedHttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_cache_key_generation() {
        let temp_dir = TempDir::new().unwrap();
        let client = CachedHttpClient::builder()
            .cache_base_path(temp_dir.path())
            .cache_prefix("test")
            .build()
            .unwrap();

        let url1 = Url::parse("https://api.example.com/cards?name=bolt").unwrap();
        let url2 = Url::parse("https://api.example.com/cards?name=counter").unwrap();
        let url3 = Url::parse("https://api.example.com/cards?name=bolt").unwrap(); // Same as url1

        let key1 = client.cache_key(&url1, None);
        let key2 = client.cache_key(&url2, None);
        let key3 = client.cache_key(&url3, None);

        assert_ne!(key1, key2); // Different URLs should have different keys
        assert_eq!(key1, key3); // Same URLs should have same keys
    }

    #[tokio::test]
    async fn test_cached_response_methods() {
        let response = CachedResponse {
            status: 200,
            headers: {
                let mut headers = HashMap::new();
                headers.insert("content-type".to_string(), "application/json".to_string());
                headers
            },
            body: b"{\"name\":\"Lightning Bolt\"}".to_vec(),
            url: "https://api.example.com/cards/bolt".to_string(),
            cached_at: std::time::SystemTime::now(),
        };

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.bytes(), b"{\"name\":\"Lightning Bolt\"}");
        assert_eq!(response.text().unwrap(), "{\"name\":\"Lightning Bolt\"}");
        assert_eq!(
            response.header("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(response.header("nonexistent"), None);

        // Test JSON deserialization
        #[derive(Deserialize, PartialEq, Debug)]
        struct Card {
            name: String,
        }

        let card: Card = response.json().unwrap();
        assert_eq!(
            card,
            Card {
                name: "Lightning Bolt".to_string()
            }
        );
    }

    #[test]
    fn test_builder_pattern() {
        let temp_dir = TempDir::new().unwrap();

        let client = CachedHttpClient::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("test-agent")
            .cache_prefix("test/http")
            .cache_base_path(temp_dir.path())
            .cache_serializer(Serializer::Json)
            .default_ttl(Duration::from_secs(3600))
            .build();

        assert!(client.is_ok());
    }
}
