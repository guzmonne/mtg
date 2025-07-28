use crate::cache::error::{CacheError, Result};
use serde::{Deserialize, Serialize};

/// Enum-based serializer that supports different serialization formats
#[derive(Debug, Clone)]
pub enum Serializer {
    Json,
    Bincode,
}

impl Serializer {
    pub fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>> {
        match self {
            Serializer::Json => serde_json::to_vec(value)
                .map_err(|e| CacheError::Serialization(format!("JSON serialization failed: {e}"))),
            Serializer::Bincode => bincode::serialize(value).map_err(|e| {
                CacheError::Serialization(format!("Bincode serialization failed: {e}"))
            }),
        }
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self, data: &[u8]) -> Result<T> {
        match self {
            Serializer::Json => serde_json::from_slice(data).map_err(|e| {
                CacheError::Serialization(format!("JSON deserialization failed: {e}"))
            }),
            Serializer::Bincode => bincode::deserialize(data).map_err(|e| {
                CacheError::Serialization(format!("Bincode deserialization failed: {e}"))
            }),
        }
    }
}

/// Default serializer (JSON for readability)
pub fn default_serializer() -> Serializer {
    Serializer::Json
}
