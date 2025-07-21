use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;
use std::collections::HashMap;

pub struct Mcp;

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "analyze_deck_list".to_string(),
            description: Some("Analyze a Magic: The Gathering deck list and provide comprehensive statistics including mana curve, type distribution, format legality, and more.".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "deck_list": {
                        "type": "string",
                        "description": "Deck list in standard format with 'Deck' and 'Sideboard' sections. Format: 'quantity cardname (set) collector_number'"
                    }
                },
                "required": ["deck_list"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let Some(deck_list) = args.get("deck_list").and_then(|v| v.as_str()) {
                    // Create a default Global config
                    let global = crate::Global {
                        api_base_url: "https://api.scryfall.com".to_string(),
                        verbose: false,
                        timeout: 30,
                    };

                    match crate::deck::analyze_deck_list_mcp(deck_list, global).await {
                        Ok(analysis) => tool_text_response!(analysis),
                        Err(e) => tool_text_response!(format!("Failed to analyze deck: {}", e)),
                    }
                } else {
                    tool_text_response!("Error: 'deck_list' parameter is required.")
                }
            })
        }
    }
}
