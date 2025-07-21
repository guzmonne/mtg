use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

// Scryfall autocomplete tool
pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_autocomplete_card_names".to_string(),
            description: Some(
                "Get autocomplete suggestions for Magic: The Gathering card names".to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Partial card name to get suggestions for (e.g., 'light', 'jace')"
                    },
                    "include_extras": {
                        "type": "boolean",
                        "description": "Include extra cards like tokens and emblems (default: false)"
                    }
                },
                "required": ["query"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let Some(query) = args.get("query").and_then(|v| v.as_str()) {
                    let include_extras = args
                        .get("include_extras")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    let mut url = format!(
                        "https://api.scryfall.com/cards/autocomplete?q={}",
                        urlencoding::encode(query)
                    );
                    if include_extras {
                        url.push_str("&include_extras=true");
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
                                            "Autocomplete failed: {}",
                                            error_msg
                                        ))
                                    } else if object_type == "catalog" {
                                        if let Some(data) =
                                            json_value.get("data").and_then(|v| v.as_array())
                                        {
                                            if data.is_empty() {
                                                tool_text_response!(
                                                    "No card name suggestions found."
                                                )
                                            } else {
                                                let suggestions: Vec<String> = data
                                                    .iter()
                                                    .filter_map(|v| v.as_str())
                                                    .map(|s| s.to_string())
                                                    .collect();

                                                let mut output = format!(
                                                    "Card name suggestions for '{query}':\n\n",
                                                );
                                                for (i, suggestion) in
                                                    suggestions.iter().enumerate()
                                                {
                                                    output.push_str(&format!(
                                                        "{}. {}\n",
                                                        i + 1,
                                                        suggestion
                                                    ));
                                                }
                                                output.push_str(&format!(
                                                    "\nFound {} suggestions",
                                                    suggestions.len()
                                                ));

                                                tool_text_response!(output)
                                            }
                                        } else {
                                            tool_text_response!("No suggestions found in response.")
                                        }
                                    } else {
                                        tool_text_response!(
                                            "Unexpected response format from Scryfall API"
                                        )
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
                    tool_text_response!("Error: 'query' parameter is required.")
                }
            })
        }
    }
}
