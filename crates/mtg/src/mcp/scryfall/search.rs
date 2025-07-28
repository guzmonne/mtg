use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

// Scryfall search tool for MTG cards
pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_search_cards".to_string(),
            description: Some("Search for Magic: The Gathering cards using Scryfall API with flexible query syntax. Supports both simple queries and advanced search parameters with multiple sort options.".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query using Scryfall syntax (e.g., 'c:red t:creature', 'Lightning Bolt', 'mana>=4 t:artifact')"
                    },
                    "name": {
                        "type": "string",
                        "description": "Card name (partial matching allowed, alternative to query)"
                    },
                    "oracle": {
                        "type": "string",
                        "description": "Oracle text to search for"
                    },
                    "card_type": {
                        "type": "string",
                        "description": "Card type (e.g., 'creature', 'instant')"
                    },
                    "colors": {
                        "type": "string",
                        "description": "Colors (e.g., 'w', 'wu', 'wubrg')"
                    },
                    "identity": {
                        "type": "string",
                        "description": "Color identity for commander (e.g., 'w', 'wu')"
                    },
                    "mana": {
                        "type": "string",
                        "description": "Mana cost (e.g., '{2}{U}', '3')"
                    },
                    "mv": {
                        "type": "string",
                        "description": "Mana value/CMC (e.g., '3', '>=4', '<2')"
                    },
                    "power": {
                        "type": "string",
                        "description": "Power (e.g., '2', '>=3', '*')"
                    },
                    "toughness": {
                        "type": "string",
                        "description": "Toughness (e.g., '2', '>=3', '*')"
                    },
                    "loyalty": {
                        "type": "string",
                        "description": "Loyalty (e.g., '3', '>=4')"
                    },
                    "set": {
                        "type": "string",
                        "description": "Set code (e.g., 'ktk', 'war')"
                    },
                    "rarity": {
                        "type": "string",
                        "description": "Rarity (common, uncommon, rare, mythic)"
                    },
                    "artist": {
                        "type": "string",
                        "description": "Artist name"
                    },
                    "flavor": {
                        "type": "string",
                        "description": "Flavor text to search for"
                    },
                    "format": {
                        "type": "string",
                        "description": "Format legality (e.g., 'standard', 'modern', 'legacy')"
                    },
                    "language": {
                        "type": "string",
                        "description": "Language (e.g., 'en', 'ja', 'de')"
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number for pagination (default: 1)",
                        "default": 1
                    },
                    "order": {
                        "type": "string",
                        "description": "Sort order (name, set, released, rarity, color, usd, tix, eur, cmc, power, toughness, edhrec, penny, artist, review, spoiled, updated)",
                        "default": "name",
                        "enum": ["name", "set", "released", "rarity", "color", "usd", "tix", "eur", "cmc", "power", "toughness", "edhrec", "penny", "artist", "review", "spoiled", "updated"]
                    },
                    "dir": {
                        "type": "string",
                        "description": "Sort direction (auto, asc, desc)",
                        "default": "auto",
                        "enum": ["auto", "asc", "desc"]
                    },
                    "include_extras": {
                        "type": "boolean",
                        "description": "Include extra cards like tokens and emblems (default: false)"
                    },
                    "include_multilingual": {
                        "type": "boolean",
                        "description": "Include cards in other languages (default: false)"
                    },
                    "include_variations": {
                        "type": "boolean",
                        "description": "Include card variations (default: false)"
                    },
                    "unique": {
                        "type": "string",
                        "description": "Unique strategy (cards, art, prints)",
                        "default": "cards",
                        "enum": ["cards", "art", "prints"]
                    }
                },
                "required": []
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                // Create a default Global config for the search
                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
                    scryfall_base_url: "https://api.scryfall.com".to_string(),
                    scryfall_user_agent: None,
                    scryfall_rate_limit_ms: 100,
                };

                // Check if we have a direct query or need to build from advanced parameters
                if let Some(query) = args.get("query").and_then(|v| v.as_str()) {
                    if query.trim().is_empty() {
                        tool_text_response!("Error: Query parameter cannot be empty. Please provide a valid Scryfall search query.")
                    } else {
                        // Direct query search
                        let search_params = crate::scryfall::search::Params {
                            query: query.to_string(),
                            pretty: true,
                            page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                            order: args
                                .get("order")
                                .and_then(|v| v.as_str())
                                .unwrap_or("name")
                                .to_string(),
                            dir: "auto".to_string(),
                            include_extras: false,
                            include_multilingual: false,
                            include_variations: false,
                            unique: "cards".to_string(),
                            csv: false,
                        };

                        match crate::scryfall::search::json(search_params, global).await {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!("No cards found matching the search query.")
                                } else {
                                    match super::utils::format_scryfall_search_results(
                                        &search_response,
                                    ) {
                                        Ok(formatted_output) => {
                                            tool_text_response!(formatted_output)
                                        }
                                        Err(e) => tool_text_response!(format!(
                                            "Failed to format search results: {}",
                                            e
                                        )),
                                    }
                                }
                            }
                            Err(e) => {
                                tool_text_response!(format!("Scryfall search failed: {}. Please check your query syntax and try again.", e))
                            }
                        }
                    }
                } else {
                    // Advanced search using individual parameters
                    let advanced_params = crate::scryfall::search::AdvancedParams {
                        name: args
                            .get("name")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        oracle: args
                            .get("oracle")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        card_type: args
                            .get("card_type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        colors: args
                            .get("colors")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        identity: args
                            .get("identity")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        mana: args
                            .get("mana")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        mv: args
                            .get("mv")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        power: args
                            .get("power")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        toughness: args
                            .get("toughness")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        loyalty: args
                            .get("loyalty")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        set: args
                            .get("set")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        rarity: args
                            .get("rarity")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        artist: args
                            .get("artist")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        flavor: args
                            .get("flavor")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        format: args
                            .get("format")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        language: args
                            .get("language")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        pretty: true,
                        page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                        order: args
                            .get("order")
                            .and_then(|v| v.as_str())
                            .unwrap_or("name")
                            .to_string(),
                        dir: "auto".to_string(),
                        include_extras: false,
                        include_multilingual: false,
                        include_variations: false,
                        unique: "cards".to_string(),
                    };

                    // Check if any advanced parameters were provided
                    let has_advanced_params = advanced_params.name.is_some()
                        || advanced_params.oracle.is_some()
                        || advanced_params.card_type.is_some()
                        || advanced_params.colors.is_some()
                        || advanced_params.identity.is_some()
                        || advanced_params.mana.is_some()
                        || advanced_params.mv.is_some()
                        || advanced_params.power.is_some()
                        || advanced_params.toughness.is_some()
                        || advanced_params.loyalty.is_some()
                        || advanced_params.set.is_some()
                        || advanced_params.rarity.is_some()
                        || advanced_params.artist.is_some()
                        || advanced_params.flavor.is_some()
                        || advanced_params.format.is_some()
                        || advanced_params.language.is_some();

                    if !has_advanced_params {
                        tool_text_response!("Error: No search parameters provided. Please specify either a 'query' parameter or at least one advanced search parameter such as 'name', 'card_type', 'colors', etc.")
                    } else {
                        match crate::scryfall::search::advanced_json(advanced_params, global).await
                        {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!(
                                        "No cards found matching the search criteria."
                                    )
                                } else {
                                    match super::utils::format_scryfall_search_results(
                                        &search_response,
                                    ) {
                                        Ok(formatted_output) => {
                                            tool_text_response!(formatted_output)
                                        }
                                        Err(e) => tool_text_response!(format!(
                                            "Failed to format search results: {}",
                                            e
                                        )),
                                    }
                                }
                            }
                            Err(e) => {
                                tool_text_response!(format!("Scryfall advanced search failed: {}. Please check your search parameters and try again.", e))
                            }
                        }
                    }
                }
            })
        }
    }
}
