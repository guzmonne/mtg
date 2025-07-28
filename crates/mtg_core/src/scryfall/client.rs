use color_eyre::Result;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Configuration for the Scryfall API client
#[derive(Debug, Clone)]
pub struct ScryfallClientConfig {
    /// Base URL for the Scryfall API
    pub base_url: String,
    /// Request timeout duration
    pub timeout: Duration,
    /// User agent string
    pub user_agent: String,
    /// Additional headers to include with requests
    pub headers: HeaderMap,
    /// Whether to enable verbose logging
    pub verbose: bool,
    /// Rate limiting delay between requests (in milliseconds)
    pub rate_limit_delay: Option<Duration>,
}

impl Default for ScryfallClientConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.scryfall.com".to_string(),
            timeout: Duration::from_secs(30),
            user_agent: "mtg-cli/1.0".to_string(),
            headers: HeaderMap::new(),
            verbose: false,
            rate_limit_delay: Some(Duration::from_millis(100)), // Scryfall recommends 50-100ms between requests
        }
    }
}

/// Builder for configuring a Scryfall API client
#[derive(Debug, Clone)]
pub struct ScryfallClientBuilder {
    config: ScryfallClientConfig,
}

impl Default for ScryfallClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ScryfallClientBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: ScryfallClientConfig::default(),
        }
    }

    /// Set the base URL for the API
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.config.base_url = url.into();
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the timeout in seconds (convenience method)
    pub fn timeout_secs(mut self, seconds: u64) -> Self {
        self.config.timeout = Duration::from_secs(seconds);
        self
    }

    /// Set the user agent string
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.config.user_agent = user_agent.into();
        self
    }

    /// Add a custom header
    pub fn header<K, V>(mut self, key: K, value: V) -> Result<Self>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let header_name: HeaderName = key.as_ref().parse().map_err(|e| {
            color_eyre::eyre::eyre!("Invalid header name '{}': {}", key.as_ref(), e)
        })?;
        let header_value = HeaderValue::from_str(value.as_ref()).map_err(|e| {
            color_eyre::eyre::eyre!("Invalid header value '{}': {}", value.as_ref(), e)
        })?;
        self.config.headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Enable or disable verbose logging
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Set rate limiting delay between requests
    pub fn rate_limit_delay(mut self, delay: Option<Duration>) -> Self {
        self.config.rate_limit_delay = delay;
        self
    }

    /// Set rate limiting delay in milliseconds (convenience method)
    pub fn rate_limit_delay_ms(mut self, milliseconds: Option<u64>) -> Self {
        self.config.rate_limit_delay = milliseconds.map(Duration::from_millis);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<ScryfallClient> {
        ScryfallClient::with_config(self.config)
    }
}

/// Generic Scryfall API client
#[derive(Debug, Clone)]
pub struct ScryfallClient {
    client: reqwest::Client,
    config: ScryfallClientConfig,
    last_request_time: std::sync::Arc<std::sync::Mutex<Option<std::time::Instant>>>,
}

impl Default for ScryfallClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Scryfall client")
    }
}

impl ScryfallClient {
    /// Create a new client with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ScryfallClientConfig::default())
    }

    /// Create a new client with custom configuration
    pub fn with_config(config: ScryfallClientConfig) -> Result<Self> {
        let mut headers = config.headers.clone();
        headers.insert(USER_AGENT, HeaderValue::from_str(&config.user_agent)?);

        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            config,
            last_request_time: std::sync::Arc::new(std::sync::Mutex::new(None)),
        })
    }

    /// Get a builder for configuring a new client
    pub fn builder() -> ScryfallClientBuilder {
        ScryfallClientBuilder::new()
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Check if verbose logging is enabled
    pub fn is_verbose(&self) -> bool {
        self.config.verbose
    }

    /// Apply rate limiting if configured
    async fn apply_rate_limit(&self) {
        if let Some(delay) = self.config.rate_limit_delay {
            let sleep_duration = {
                let last_request = self.last_request_time.lock().unwrap();
                if let Some(last_time) = *last_request {
                    let elapsed = last_time.elapsed();
                    if elapsed < delay {
                        Some(delay - elapsed)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(duration) = sleep_duration {
                tokio::time::sleep(duration).await;
            }

            *self.last_request_time.lock().unwrap() = Some(std::time::Instant::now());
        }
    }

    /// Make a GET request to a Scryfall API endpoint
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.apply_rate_limit().await;

        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!(
                "{}/{}",
                self.config.base_url.trim_end_matches('/'),
                endpoint.trim_start_matches('/')
            )
        };

        if self.config.verbose {
            println!("GET {url}");
        }

        let response = self.client.get(&url).send().await?;

        if self.config.verbose {
            println!("Response status: {}", response.status());
        }

        let response_text = response.text().await?;

        if self.config.verbose {
            println!("Response length: {} characters", response_text.len());
        }

        // Check for API errors first
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
                if object_type == "error" {
                    let error_details = json_value
                        .get("details")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    return Err(color_eyre::eyre::eyre!(
                        "Scryfall API error: {}",
                        error_details
                    ));
                }
            }
        }

        // Parse the successful response
        let result: T = serde_json::from_str(&response_text)?;
        Ok(result)
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params<T, P>(&self, endpoint: &str, params: P) -> Result<T>
    where
        T: DeserializeOwned,
        P: IntoIterator<Item = (String, String)>,
    {
        self.apply_rate_limit().await;

        let base_url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!(
                "{}/{}",
                self.config.base_url.trim_end_matches('/'),
                endpoint.trim_start_matches('/')
            )
        };

        // Build query string
        let query_params: Vec<(String, String)> = params.into_iter().collect();
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = if query_string.is_empty() {
            base_url
        } else {
            format!("{base_url}?{query_string}")
        };

        if self.config.verbose {
            println!("GET {url}");
        }

        let response = self.client.get(&url).send().await?;

        if self.config.verbose {
            println!("Response status: {}", response.status());
        }

        let response_text = response.text().await?;

        if self.config.verbose {
            println!("Response length: {} characters", response_text.len());
        }

        // Check for API errors first
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
                if object_type == "error" {
                    let error_details = json_value
                        .get("details")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    return Err(color_eyre::eyre::eyre!(
                        "Scryfall API error: {}",
                        error_details
                    ));
                }
            }
        }

        // Parse the successful response
        let result: T = serde_json::from_str(&response_text)?;
        Ok(result)
    }

    /// Make a raw GET request returning the response text
    pub async fn get_raw(&self, endpoint: &str) -> Result<String> {
        self.apply_rate_limit().await;

        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!(
                "{}/{}",
                self.config.base_url.trim_end_matches('/'),
                endpoint.trim_start_matches('/')
            )
        };

        if self.config.verbose {
            println!("GET {url} (raw)");
        }

        let response = self.client.get(&url).send().await?;

        if self.config.verbose {
            println!("Response status: {}", response.status());
        }

        let response_text = response.text().await?;

        if self.config.verbose {
            println!("Response length: {} characters", response_text.len());
        }

        Ok(response_text)
    }

    /// Make a raw GET request with query parameters returning the response text
    pub async fn get_raw_with_params<P>(&self, endpoint: &str, params: P) -> Result<String>
    where
        P: IntoIterator<Item = (String, String)>,
    {
        self.apply_rate_limit().await;

        let base_url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!(
                "{}/{}",
                self.config.base_url.trim_end_matches('/'),
                endpoint.trim_start_matches('/')
            )
        };

        // Build query string
        let query_params: Vec<(String, String)> = params.into_iter().collect();
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = if query_string.is_empty() {
            base_url
        } else {
            format!("{base_url}?{query_string}")
        };

        if self.config.verbose {
            println!("GET {url} (raw)");
        }

        let response = self.client.get(&url).send().await?;

        if self.config.verbose {
            println!("Response status: {}", response.status());
        }

        let response_text = response.text().await?;

        if self.config.verbose {
            println!("Response length: {} characters", response_text.len());
        }

        Ok(response_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ScryfallClientConfig::default();
        assert_eq!(config.base_url, "https://api.scryfall.com");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.user_agent, "mtg-cli/1.0");
        assert!(!config.verbose);
        assert_eq!(config.rate_limit_delay, Some(Duration::from_millis(100)));
    }

    #[test]
    fn test_builder_pattern() -> Result<()> {
        let client = ScryfallClient::builder()
            .base_url("https://custom.api.com")
            .timeout_secs(60)
            .user_agent("custom-agent/2.0")
            .verbose(true)
            .rate_limit_delay_ms(Some(200))
            .header("X-Custom-Header", "custom-value")?
            .build()?;

        assert_eq!(client.base_url(), "https://custom.api.com");
        assert!(client.is_verbose());
        Ok(())
    }

    #[test]
    fn test_default_client() -> Result<()> {
        let client = ScryfallClient::new()?;
        assert_eq!(client.base_url(), "https://api.scryfall.com");
        assert!(!client.is_verbose());
        Ok(())
    }

    #[test]
    fn test_builder_default() -> Result<()> {
        let client = ScryfallClient::builder().build()?;
        assert_eq!(client.base_url(), "https://api.scryfall.com");
        assert!(!client.is_verbose());
        Ok(())
    }

    // Integration tests (require network access)
    #[cfg(feature = "integration-tests")]
    mod integration {
        use super::*;
        use crate::scryfall::sets::ScryfallSetList;

        #[tokio::test]
        async fn test_get_sets() -> Result<()> {
            let client = ScryfallClient::new()?;
            let sets: ScryfallSetList = client.get("sets").await?;

            assert_eq!(sets.object, "list");
            assert!(!sets.data.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn test_get_specific_set() -> Result<()> {
            let client = ScryfallClient::new()?;
            let set: crate::scryfall::sets::ScryfallSet = client.get("sets/dom").await?;

            assert_eq!(set.object, "set");
            assert_eq!(set.code, "dom");
            assert_eq!(set.name, "Dominaria");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_with_params() -> Result<()> {
            let client = ScryfallClient::new()?;
            let params = vec![
                ("q".to_string(), "Lightning Bolt".to_string()),
                ("unique".to_string(), "cards".to_string()),
            ];

            let response: serde_json::Value =
                client.get_with_params("cards/search", params).await?;

            assert_eq!(response["object"], "list");
            Ok(())
        }

        #[tokio::test]
        async fn test_rate_limiting() -> Result<()> {
            let client = ScryfallClient::builder()
                .rate_limit_delay_ms(Some(50))
                .build()?;

            let start = std::time::Instant::now();

            // Make two requests
            let _: ScryfallSetList = client.get("sets").await?;
            let _: ScryfallSetList = client.get("sets").await?;

            let elapsed = start.elapsed();

            // Should take at least 50ms due to rate limiting
            assert!(elapsed >= Duration::from_millis(50));
            Ok(())
        }

        #[tokio::test]
        async fn test_custom_user_agent() -> Result<()> {
            let client = ScryfallClient::builder()
                .user_agent("test-client/1.0")
                .build()?;

            // This test just ensures the client can be created with custom user agent
            // The actual user agent is sent in headers, which we can't easily test here
            let _: ScryfallSetList = client.get("sets").await?;
            Ok(())
        }

        #[tokio::test]
        async fn test_error_handling() -> Result<()> {
            let client = ScryfallClient::new()?;

            // Try to get a non-existent set
            let result: color_eyre::Result<crate::scryfall::sets::ScryfallSet> =
                client.get("sets/nonexistent").await;

            assert!(result.is_err());
            let error_msg = result.unwrap_err().to_string();
            assert!(error_msg.contains("Scryfall API error"));
            Ok(())
        }
    }
}
