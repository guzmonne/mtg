use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

// Scryfall named card lookup tool
pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_get_card_by_name".to_string(),
            description: Some(
                "Get a specific Magic: The Gathering card by its exact name using Scryfall API"
                    .to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Exact card name to look up (e.g., 'Lightning Bolt', 'Jace, the Mind Sculptor')"
                    },
                    "set": {
                        "type": "string",
                        "description": "Optional set code to get specific printing (e.g., 'lea', 'ktk')"
                    }
                },
                "required": ["name"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
                    scryfall_base_url: "https://api.scryfall.com".to_string(),
                    scryfall_user_agent: None,
                    scryfall_rate_limit_ms: 100,
                };

                if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
                    if name.trim().is_empty() {
                        tool_text_response!("Error: Card name cannot be empty.")
                    } else {
                        let set_code = args.get("set").and_then(|v| v.as_str());

                        // Build URL for named card lookup
                        let url = if let Some(set) = set_code {
                            format!(
                                "https://api.scryfall.com/cards/named?exact={}&set={}",
                                urlencoding::encode(name),
                                urlencoding::encode(set)
                            )
                        } else {
                            format!(
                                "https://api.scryfall.com/cards/named?exact={}",
                                urlencoding::encode(name)
                            )
                        };

                        let client = reqwest::Client::builder()
                            .timeout(std::time::Duration::from_secs(global.timeout))
                            .user_agent("mtg-cli/1.0")
                            .build()
                            .unwrap();

                        match client.get(&url).send().await {
                            Ok(response) => {
                                match response.text().await {
                                    Ok(response_text) => {
                                        // Parse the response
                                        let json_value: serde_json::Value =
                                            match serde_json::from_str(&response_text) {
                                                Ok(val) => val,
                                                Err(e) => {
                                                    return tool_text_response!(format!(
                                                        "Failed to parse response: {}",
                                                        e
                                                    ));
                                                }
                                            };
                                        if let Some(object_type) =
                                            json_value.get("object").and_then(|v| v.as_str())
                                        {
                                            if object_type == "error" {
                                                let error_msg = json_value
                                                    .get("details")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("Unknown error");
                                                tool_text_response!(format!(
                                                    "Card not found: {}",
                                                    error_msg
                                                ))
                                            } else {
                                                // Parse as card response
                                                match serde_json::from_value::<
                                                    crate::scryfall::Card,
                                                >(
                                                    json_value
                                                ) {
                                                    Ok(card) => {
                                                        match super::utils::format_single_card_details(&card) {
                                                            Ok(formatted_output) => {
                                                                tool_text_response!(
                                                                    formatted_output
                                                                )
                                                            }
                                                            Err(e) => tool_text_response!(format!(
                                                                "Failed to format card details: {}",
                                                                e
                                                            )),
                                                        }
                                                    }
                                                    Err(e) => tool_text_response!(format!(
                                                        "Failed to parse card data: {}",
                                                        e
                                                    )),
                                                }
                                            }
                                        } else {
                                            tool_text_response!(
                                                "Invalid response format from Scryfall API"
                                            )
                                        }
                                    }
                                    Err(e) => tool_text_response!(format!(
                                        "Failed to read response: {}",
                                        e
                                    )),
                                }
                            }
                            Err(e) => tool_text_response!(format!("Request failed: {}", e)),
                        }
                    }
                } else {
                    tool_text_response!("Error: 'name' parameter is required.")
                }
            })
        }
    }
}
