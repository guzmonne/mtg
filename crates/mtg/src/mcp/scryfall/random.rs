use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

// Scryfall random card tool
pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_get_random_card".to_string(),
            description: Some(
                "Get a random Magic: The Gathering card, optionally filtered by search criteria"
                    .to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Optional search query to filter random results (e.g., 'c:red t:creature', 'mana>=4')"
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

                let mut url = "https://api.scryfall.com/cards/random?format=json".to_string();

                if let Some(query) = args.get("query").and_then(|v| v.as_str()) {
                    url.push_str(&format!("&q={}", urlencoding::encode(query)));
                }

                let client = reqwest::Client::builder()
                    .user_agent("mtg-cli/1.0")
                    .build()
                    .unwrap();

                match client.get(&url).send().await {
                    Ok(response) => match response.text().await {
                        Ok(response_text) => {
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
                                        "No random card found: {}",
                                        error_msg
                                    ))
                                } else {
                                    match serde_json::from_value::<crate::scryfall::ScryfallCard>(
                                        json_value,
                                    ) {
                                        Ok(card) => {
                                            match super::utils::format_single_card_details(&card) {
                                                Ok(formatted_output) => {
                                                    tool_text_response!(formatted_output)
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
                                tool_text_response!("Invalid response format from Scryfall API")
                            }
                        }
                        Err(e) => tool_text_response!(format!("Failed to read response: {}", e)),
                    },
                    Err(e) => tool_text_response!(format!("Request failed: {}", e)),
                }
            })
        }
    }
}
