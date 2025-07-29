use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Scryfall API error: {0}")]
    ScryfallApi(#[from] ScryfallApiError),

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
