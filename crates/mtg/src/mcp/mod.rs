use mcp_core::{
    server::Server,
    tool_text_response,
    tools::ToolHandlerFn,
    transport::{ServerSseTransport, ServerStdioTransport},
    types::{CallToolRequest, CallToolResponse, ServerCapabilities, Tool, ToolCapabilities},
};
use serde_json::json;
use std::collections::HashMap;

use crate::prelude::*;

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    log::info!("Starting MTG MCP Server (STDIO)");

    let server_protocol = Server::builder(
        "mtg-mcp-server".to_string(),
        "2.0.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(GathererSearchTool::tool(), GathererSearchTool::call())
    .register_tool(ScryfallSearchTool::tool(), ScryfallSearchTool::call())
    .build();

    let transport = ServerStdioTransport::new(server_protocol);
    Server::start(transport).await.map_err(|e| eyre!(e))
}

pub async fn run_sse_server(global: crate::Global, host: String, port: u16) -> Result<()> {
    log::info!("Starting MTG MCP Server (SSE) on {}:{}", host, port);

    let server_protocol = Server::builder(
        "mtg-mcp-server".to_string(),
        "2.0.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(GathererSearchTool::tool(), GathererSearchTool::call())
    .register_tool(ScryfallSearchTool::tool(), ScryfallSearchTool::call())
    .build();

    let transport = ServerSseTransport::new(host, port, server_protocol);
    Server::start(transport).await.map_err(|e| eyre!(e))
}

// Gatherer search tool for MTG cards
pub struct GathererSearchTool;

impl GathererSearchTool {
    fn tool() -> Tool {
        Tool {
            name: "gatherer_search_cards".to_string(),
            description: Some("Search for Magic: The Gathering cards using Wizards' Gatherer database with advanced search parameters".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Card name to search for (partial matching allowed)"
                    },
                    "rules": {
                        "type": "string", 
                        "description": "Rules text to search for"
                    },
                    "card_type": {
                        "type": "string",
                        "description": "Card type (e.g., 'Creature', 'Instant', 'Creature,Enchantment' for OR, 'Creature+Legendary' for AND)"
                    },
                    "subtype": {
                        "type": "string",
                        "description": "Card subtype (e.g., 'Human', 'Wizard', 'Human,Wizard' for OR, 'Human+Soldier' for AND)"
                    },
                    "supertype": {
                        "type": "string",
                        "description": "Card supertype (e.g., 'Legendary', 'Snow')"
                    },
                    "mana_cost": {
                        "type": "string",
                        "description": "Mana cost (e.g., '{2}{U}', '1W(B/G)(W/P)')"
                    },
                    "set": {
                        "type": "string",
                        "description": "Set name (e.g., 'Magic: The Gathering—FINAL FANTASY')"
                    },
                    "rarity": {
                        "type": "string",
                        "description": "Rarity (Common, Uncommon, Rare, Mythic)"
                    },
                    "artist": {
                        "type": "string",
                        "description": "Artist name"
                    },
                    "power": {
                        "type": "string",
                        "description": "Power value or range (e.g., '5', '5-10')"
                    },
                    "toughness": {
                        "type": "string",
                        "description": "Toughness value or range (e.g., '2', '2-5')"
                    },
                    "loyalty": {
                        "type": "string",
                        "description": "Loyalty value or range (e.g., '3', '3-6')"
                    },
                    "flavor": {
                        "type": "string",
                        "description": "Flavor text to search for"
                    },
                    "colors": {
                        "type": "string",
                        "description": "Colors (e.g., 'W', 'U', 'B', 'R', 'G', '!RBW' for not these colors)"
                    },
                    "format": {
                        "type": "string",
                        "description": "Format legality (e.g., 'Legal:Standard', 'Banned:Modern')"
                    },
                    "language": {
                        "type": "string",
                        "description": "Language (e.g., 'English', 'Japanese', 'French', 'German', 'Spanish', 'Italian')"
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number for pagination (default: 1)",
                        "default": 1
                    }
                },
                "required": []
            }),
            annotations: None,
        }
    }

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);
                
                // Convert MCP arguments to SearchParams
                let search_params = crate::gatherer::SearchParams {
                    name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    rules: args.get("rules").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    card_type: args.get("card_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    subtype: args.get("subtype").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    supertype: args.get("supertype").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    mana_cost: args.get("mana_cost").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    set: args.get("set").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    rarity: args.get("rarity").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    artist: args.get("artist").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    power: args.get("power").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    toughness: args.get("toughness").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    loyalty: args.get("loyalty").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    flavor: args.get("flavor").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    colors: args.get("colors").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    format: args.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    language: args.get("language").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    pretty: false, // Always return JSON for MCP
                    page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                };

                // Create a default Global config for the search
                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
                };

                // Check if any search parameters were provided
                let has_params = search_params.name.is_some() || 
                                search_params.rules.is_some() || 
                                search_params.card_type.is_some() ||
                                search_params.subtype.is_some() ||
                                search_params.supertype.is_some() ||
                                search_params.mana_cost.is_some() ||
                                search_params.set.is_some() ||
                                search_params.rarity.is_some() ||
                                search_params.artist.is_some() ||
                                search_params.power.is_some() ||
                                search_params.toughness.is_some() ||
                                search_params.loyalty.is_some() ||
                                search_params.flavor.is_some() ||
                                search_params.colors.is_some() ||
                                search_params.format.is_some() ||
                                search_params.language.is_some();

                if !has_params {
                    tool_text_response!("Error: No search parameters provided. Please specify at least one search parameter such as 'name', 'card_type', 'colors', etc.")
                } else {
                    // Perform the actual search
                    match crate::gatherer::search_cards_json(search_params, global).await {
                        Ok(card_data) => {
                            // Check if we got any results
                            if let Some(items) = card_data.get("items").and_then(|v| v.as_array()) {
                                if items.is_empty() {
                                    tool_text_response!("No cards found matching the search criteria.")
                                } else {
                                    // Return the JSON data as a formatted string
                                    match serde_json::to_string_pretty(&card_data) {
                                        Ok(json_str) => tool_text_response!(json_str),
                                        Err(e) => tool_text_response!(format!("Failed to serialize card data: {}", e))
                                    }
                                }
                            } else {
                                tool_text_response!("No cards found matching the search criteria.")
                            }
                        }
                        Err(e) => {
                            tool_text_response!(format!("Gatherer search failed: {}. Please check your search parameters and try again.", e))
                        }
                    }
                }
            })
        }
    }
}

// Scryfall search tool for MTG cards
pub struct ScryfallSearchTool;

impl ScryfallSearchTool {
    fn tool() -> Tool {
        Tool {
            name: "scryfall_search_cards".to_string(),
            description: Some("Search for Magic: The Gathering cards using Scryfall API with flexible query syntax".to_string()),
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
                        "description": "Sort order (name, cmc, power, toughness, artist, set, released, rarity, usd, tix, eur)",
                        "default": "name"
                    }
                },
                "required": []
            }),
            annotations: None,
        }
    }

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);
                
                // Create a default Global config for the search
                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
                };

                // Check if we have a direct query or need to build from advanced parameters
                if let Some(query) = args.get("query").and_then(|v| v.as_str()) {
                    if query.trim().is_empty() {
                        tool_text_response!("Error: Query parameter cannot be empty. Please provide a valid Scryfall search query.")
                    } else {
                        // Direct query search
                        let search_params = crate::scryfall::SearchParams {
                            query: query.to_string(),
                            pretty: false,
                            page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                            order: args.get("order").and_then(|v| v.as_str()).unwrap_or("name").to_string(),
                            dir: "auto".to_string(),
                            include_extras: false,
                            include_multilingual: false,
                            include_variations: false,
                            unique: "cards".to_string(),
                        };

                        match crate::scryfall::search_cards_json(search_params, global).await {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!("No cards found matching the search query.")
                                } else {
                                    match serde_json::to_string_pretty(&search_response) {
                                        Ok(json_str) => tool_text_response!(json_str),
                                        Err(e) => tool_text_response!(format!("Failed to serialize search results: {}", e))
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
                    let advanced_params = crate::scryfall::AdvancedSearchParams {
                        name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        oracle: args.get("oracle").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        card_type: args.get("card_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        colors: args.get("colors").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        identity: args.get("identity").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        mana: args.get("mana").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        mv: args.get("mv").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        power: args.get("power").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        toughness: args.get("toughness").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        loyalty: args.get("loyalty").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        set: args.get("set").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        rarity: args.get("rarity").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        artist: args.get("artist").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        flavor: args.get("flavor").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        format: args.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        language: args.get("language").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        pretty: false,
                        page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                        order: args.get("order").and_then(|v| v.as_str()).unwrap_or("name").to_string(),
                    };

                    // Check if any advanced parameters were provided
                    let has_advanced_params = advanced_params.name.is_some() || 
                                            advanced_params.oracle.is_some() || 
                                            advanced_params.card_type.is_some() ||
                                            advanced_params.colors.is_some() ||
                                            advanced_params.identity.is_some() ||
                                            advanced_params.mana.is_some() ||
                                            advanced_params.mv.is_some() ||
                                            advanced_params.power.is_some() ||
                                            advanced_params.toughness.is_some() ||
                                            advanced_params.loyalty.is_some() ||
                                            advanced_params.set.is_some() ||
                                            advanced_params.rarity.is_some() ||
                                            advanced_params.artist.is_some() ||
                                            advanced_params.flavor.is_some() ||
                                            advanced_params.format.is_some() ||
                                            advanced_params.language.is_some();

                    if !has_advanced_params {
                        tool_text_response!("Error: No search parameters provided. Please specify either a 'query' parameter or at least one advanced search parameter such as 'name', 'card_type', 'colors', etc.")
                    } else {
                        match crate::scryfall::advanced_search_json(advanced_params, global).await {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!("No cards found matching the search criteria.")
                                } else {
                                    match serde_json::to_string_pretty(&search_response) {
                                        Ok(json_str) => tool_text_response!(json_str),
                                        Err(e) => tool_text_response!(format!("Failed to serialize search results: {}", e))
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