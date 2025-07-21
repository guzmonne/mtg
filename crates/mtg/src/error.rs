use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Scryfall API error: {0}")]
    ScryfallApi(#[from] ScryfallApiError),

    #[error("Cache error: {0}")]
    Cache(#[from] crate::cache::CacheError),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Comprehensive Scryfall API error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScryfallError {
    /// Always "error"
    pub object: String,
    /// HTTP status code
    pub status: u16,
    /// Computer-friendly error code
    pub code: String,
    /// Human-readable explanation
    pub details: String,
    /// Additional context for the error (e.g., "ambiguous")
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// Non-fatal warnings
    pub warnings: Option<Vec<String>>,
}

/// Enhanced API error types with structured information
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum ScryfallApiError {
    #[error("API Error ({code}): {details}")]
    ApiError {
        code: String,
        details: String,
        status: u16,
        error_type: Option<String>,
        warnings: Vec<String>,
    },

    #[error("Rate limit exceeded: retry after {retry_after}s")]
    RateLimit { retry_after: u64, details: String },

    #[error("Not found: {details}")]
    NotFound {
        details: String,
        suggestions: Vec<String>,
    },

    #[error("Bad request: {details}")]
    BadRequest {
        details: String,
        warnings: Vec<String>,
    },

    #[error("Server error: {details}")]
    ServerError { status: u16, details: String },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout error: request took longer than {timeout}s")]
    Timeout { timeout: u64 },

    #[error("Parse error: {details}")]
    Parse { details: String },
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::ScryfallApi(ScryfallApiError::Timeout {
                timeout: 30, // Default timeout, should be configurable
            })
        } else if err.is_connect() || err.is_request() {
            Error::ScryfallApi(ScryfallApiError::Network(err.to_string()))
        } else {
            Error::Network(err.to_string())
        }
    }
}

impl ScryfallApiError {
    /// Create an API error from a Scryfall error response
    pub fn from_scryfall_error(error: ScryfallError) -> Self {
        let warnings = error.warnings.unwrap_or_default();

        match error.status {
            404 => ScryfallApiError::NotFound {
                details: error.details,
                suggestions: Vec::new(), // Could be enhanced with suggestions
            },
            400 => ScryfallApiError::BadRequest {
                details: error.details,
                warnings,
            },
            429 => {
                // Extract retry-after from details if available
                let retry_after = extract_retry_after(&error.details).unwrap_or(60);
                ScryfallApiError::RateLimit {
                    retry_after,
                    details: error.details,
                }
            }
            500..=599 => ScryfallApiError::ServerError {
                status: error.status,
                details: error.details,
            },
            _ => ScryfallApiError::ApiError {
                code: error.code,
                details: error.details,
                status: error.status,
                error_type: error.error_type,
                warnings,
            },
        }
    }

    /// Get warnings associated with this error
    pub fn warnings(&self) -> Vec<String> {
        match self {
            ScryfallApiError::ApiError { warnings, .. } => warnings.clone(),
            ScryfallApiError::BadRequest { warnings, .. } => warnings.clone(),
            _ => Vec::new(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ScryfallApiError::RateLimit { .. }
                | ScryfallApiError::ServerError { .. }
                | ScryfallApiError::Network(_)
                | ScryfallApiError::Timeout { .. }
        )
    }

    /// Get suggested retry delay in seconds
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            ScryfallApiError::RateLimit { retry_after, .. } => Some(*retry_after),
            ScryfallApiError::ServerError { .. } => Some(5), // 5 second delay for server errors
            ScryfallApiError::Network(_) => Some(2),         // 2 second delay for network errors
            ScryfallApiError::Timeout { .. } => Some(1),     // 1 second delay for timeouts
            _ => None,
        }
    }
}

/// Extract retry-after value from error details string
fn extract_retry_after(details: &str) -> Option<u64> {
    // Look for patterns like "retry after 60 seconds" or "retry-after: 60"
    let re = regex::Regex::new(r"(?i)retry.?after[:\s]+(\d+)").ok()?;
    re.captures(details)?.get(1)?.as_str().parse().ok()
}

/// Image-related errors
#[derive(thiserror::Error, Debug)]
pub enum ImageError {
    #[error("Image download failed: {0}")]
    Download(String),

    #[error("Image format not supported: {0}")]
    UnsupportedFormat(String),

    #[error("Image corruption detected: {0}")]
    Corruption(String),

    #[error("Image cache error: {0}")]
    Cache(String),

    #[error("Image validation failed: {0}")]
    Validation(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;
