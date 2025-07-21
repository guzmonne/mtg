use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

// Scryfall ID card lookup tool
pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_get_card_by_id".to_string(),
            description: Some(
                "Get a specific Magic: The Gathering card by its Scryfall ID".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Scryfall card ID (UUID format, e.g., '5f70df6d-7e8d-4ba4-b425-b56c271f525c')"
                    }
                },
                "required": ["id"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                    let url = format!("https://api.scryfall.com/cards/{id}");

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
                                            "Card not found: {}",
                                            error_msg
                                        ))
                                    } else {
                                        match serde_json::from_value::<crate::scryfall::ScryfallCard>(
                                            json_value,
                                        ) {
                                            Ok(card) => {
                                                match super::utils::format_single_card_details(
                                                    &card,
                                                ) {
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
                            Err(e) => {
                                tool_text_response!(format!("Failed to read response: {}", e))
                            }
                        },
                        Err(e) => tool_text_response!(format!("Request failed: {}", e)),
                    }
                } else {
                    tool_text_response!("Error: 'id' parameter is required.")
                }
            })
        }
    }
}
