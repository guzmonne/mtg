use mcp_core::{
    server::Server,
    tool_text_response,
    tools::ToolHandlerFn,
    transport::{ServerSseTransport, ServerStdioTransport},
    types::{CallToolRequest, CallToolResponse, ServerCapabilities, Tool, ToolCapabilities},
};
use prettytable::{format, Cell, Row, Table};
use serde_json::json;
use std::collections::HashMap;

use crate::prelude::*;

// Helper function to format Scryfall search results as pretty table
fn format_scryfall_search_results(
    response: &crate::scryfall::ScryfallSearchResponse,
) -> Result<String> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Cost"),
        Cell::new("Type"),
        Cell::new("Set"),
        Cell::new("Rarity"),
        Cell::new("P/T/L"),
    ]));

    for card in &response.data {
        let mana_cost = card.mana_cost.as_deref().unwrap_or("");
        let pt_loyalty = if let Some(loyalty) = &card.loyalty {
            loyalty.clone()
        } else if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
            format!("{}/{}", power, toughness)
        } else {
            "-".to_string()
        };

        table.add_row(Row::new(vec![
            Cell::new(&card.name),
            Cell::new(mana_cost),
            Cell::new(&card.type_line),
            Cell::new(&card.set_name),
            Cell::new(&card.rarity),
            Cell::new(&pt_loyalty),
        ]));
    }

    let mut buffer = Vec::new();
    table
        .print(&mut buffer)
        .map_err(|e| eyre!("Failed to format table: {}", e))?;
    let mut output =
        String::from_utf8(buffer).map_err(|e| eyre!("Failed to convert table to string: {}", e))?;

    // Add summary information
    let total_cards = response.total_cards.unwrap_or(response.data.len() as u32) as usize;
    output.push_str(&format!("\nFound {} cards", total_cards));
    if response.data.len() < total_cards {
        output.push_str(&format!(" (showing {} on this page)", response.data.len()));
    }

    // Display warnings if any
    if let Some(warnings) = &response.warnings {
        output.push_str("\n\n⚠️  Warnings:\n");
        for warning in warnings {
            output.push_str(&format!("   • {}\n", warning));
        }
    }

    Ok(output)
}

// Helper function to format single card details as pretty table
fn format_single_card_details(card: &crate::scryfall::ScryfallCard) -> Result<String> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    // Card name
    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(&card.name)]));

    // Mana cost
    if let Some(mana_cost) = &card.mana_cost {
        if !mana_cost.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Mana Cost"), Cell::new(mana_cost)]));
        }
    }

    // Mana value
    if card.cmc > 0.0 {
        table.add_row(Row::new(vec![
            Cell::new("Mana Value"),
            Cell::new(&card.cmc.to_string()),
        ]));
    }

    // Type line
    table.add_row(Row::new(vec![
        Cell::new("Type"),
        Cell::new(&card.type_line),
    ]));

    // Oracle text
    if let Some(oracle_text) = &card.oracle_text {
        if !oracle_text.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Oracle Text"),
                Cell::new(oracle_text),
            ]));
        }
    }

    // Power/Toughness
    if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
        table.add_row(Row::new(vec![
            Cell::new("Power/Toughness"),
            Cell::new(&format!("{}/{}", power, toughness)),
        ]));
    }

    // Loyalty
    if let Some(loyalty) = &card.loyalty {
        table.add_row(Row::new(vec![Cell::new("Loyalty"), Cell::new(loyalty)]));
    }

    // Set
    table.add_row(Row::new(vec![
        Cell::new("Set"),
        Cell::new(&format!("{} ({})", card.set_name, card.set.to_uppercase())),
    ]));

    // Rarity
    table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new(&card.rarity)]));

    // Artist
    if let Some(artist) = &card.artist {
        table.add_row(Row::new(vec![Cell::new("Artist"), Cell::new(artist)]));
    }

    // Flavor text
    if let Some(flavor_text) = &card.flavor_text {
        if !flavor_text.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Flavor Text"),
                Cell::new(flavor_text),
            ]));
        }
    }

    // Collector number
    table.add_row(Row::new(vec![
        Cell::new("Collector Number"),
        Cell::new(&card.collector_number),
    ]));

    // Legalities (show a few key formats)
    if let Some(legalities) = card.legalities.as_object() {
        let mut legal_formats = Vec::new();
        let key_formats = [
            "standard",
            "pioneer",
            "modern",
            "legacy",
            "vintage",
            "commander",
        ];

        for format in &key_formats {
            if let Some(status) = legalities.get(*format).and_then(|v| v.as_str()) {
                if status == "legal" {
                    legal_formats.push(format.to_string());
                }
            }
        }

        // Add legal formats to table
        if !legal_formats.is_empty() {
            table.add_row(Row::new(vec![
                Cell::new("Legal In"),
                Cell::new(&legal_formats.join(", ")),
            ]));
        }
    }

    // Convert table to string and return
    let mut buffer = Vec::new();
    table
        .print(&mut buffer)
        .map_err(|e| eyre!("Failed to format table: {}", e))?;
    let output =
        String::from_utf8(buffer).map_err(|e| eyre!("Failed to convert table to string: {}", e))?;
    Ok(output)
}

// Helper function to format Gatherer search results as pretty table
fn format_gatherer_search_results(response: &serde_json::Value) -> Result<String> {
    if let Some(items) = response.get("items").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Cost"),
            Cell::new("Type"),
            Cell::new("Set"),
            Cell::new("Rarity"),
            Cell::new("P/T/L"),
        ]));

        for item in items {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let mana_cost = item.get("manaCost").and_then(|v| v.as_str()).unwrap_or("");
            let type_line = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
            let set_name = item.get("setName").and_then(|v| v.as_str()).unwrap_or("");
            let rarity = item.get("rarity").and_then(|v| v.as_str()).unwrap_or("");

            // Handle power/toughness/loyalty
            let pt_loyalty = if let Some(loyalty) = item.get("loyalty").and_then(|v| v.as_str()) {
                loyalty.to_string()
            } else if let (Some(power), Some(toughness)) = (
                item.get("power").and_then(|v| v.as_str()),
                item.get("toughness").and_then(|v| v.as_str()),
            ) {
                format!("{}/{}", power, toughness)
            } else {
                "-".to_string()
            };

            table.add_row(Row::new(vec![
                Cell::new(name),
                Cell::new(mana_cost),
                Cell::new(type_line),
                Cell::new(set_name),
                Cell::new(rarity),
                Cell::new(&pt_loyalty),
            ]));
        }

        let mut buffer = Vec::new();
        table
            .print(&mut buffer)
            .map_err(|e| eyre!("Failed to format table: {}", e))?;
        let mut output = String::from_utf8(buffer)
            .map_err(|e| eyre!("Failed to convert table to string: {}", e))?;

        // Add summary information
        let total_items = response
            .get("totalItems")
            .and_then(|v| v.as_u64())
            .unwrap_or(items.len() as u64);
        output.push_str(&format!("\nFound {} cards", total_items));
        if items.len() < total_items as usize {
            output.push_str(&format!(" (showing {} on this page)", items.len()));
        }

        Ok(output)
    } else {
        Ok("No cards found.".to_string())
    }
}

// Scryfall named card lookup tool
pub struct ScryfallNamedTool;

impl ScryfallNamedTool {
    fn tool() -> Tool {
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

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
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
                                                return tool_text_response!(format!(
                                                    "Card not found: {}",
                                                    error_msg
                                                ));
                                            } else {
                                                // Parse as card response
                                                match serde_json::from_value::<
                                                    crate::scryfall::ScryfallCard,
                                                >(
                                                    json_value
                                                ) {
                                                    Ok(card) => {
                                                        match format_single_card_details(&card) {
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

// Scryfall ID card lookup tool
pub struct ScryfallIdTool;

impl ScryfallIdTool {
    fn tool() -> Tool {
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

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let Some(id) = args.get("id").and_then(|v| v.as_str()) {
                    let url = format!("https://api.scryfall.com/cards/{}", id);

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
                                        return tool_text_response!(format!(
                                            "Card not found: {}",
                                            error_msg
                                        ));
                                    } else {
                                        match serde_json::from_value::<crate::scryfall::ScryfallCard>(
                                            json_value,
                                        ) {
                                            Ok(card) => match format_single_card_details(&card) {
                                                Ok(formatted_output) => {
                                                    tool_text_response!(formatted_output)
                                                }
                                                Err(e) => tool_text_response!(format!(
                                                    "Failed to format card details: {}",
                                                    e
                                                )),
                                            },
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

// Scryfall collector number lookup tool
pub struct ScryfallCollectorTool;

impl ScryfallCollectorTool {
    fn tool() -> Tool {
        Tool {
            name: "scryfall_get_card_by_collector".to_string(),
            description: Some(
                "Get a specific Magic: The Gathering card by set code and collector number"
                    .to_string(),
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "set_code": {
                        "type": "string",
                        "description": "Three-letter set code (e.g., 'ktk', 'lea', 'war')"
                    },
                    "collector_number": {
                        "type": "string",
                        "description": "Collector number within the set (e.g., '1', '42', '150a')"
                    },
                    "lang": {
                        "type": "string",
                        "description": "Optional language code (default: 'en')"
                    }
                },
                "required": ["set_code", "collector_number"]
            }),
            annotations: None,
        }
    }

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let (Some(set_code), Some(collector_number)) = (
                    args.get("set_code").and_then(|v| v.as_str()),
                    args.get("collector_number").and_then(|v| v.as_str()),
                ) {
                    let lang = args.get("lang").and_then(|v| v.as_str()).unwrap_or("en");
                    let url = format!(
                        "https://api.scryfall.com/cards/{}/{}/{}?format=json",
                        set_code, collector_number, lang
                    );

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
                                        return tool_text_response!(format!(
                                            "Card not found: {}",
                                            error_msg
                                        ));
                                    } else {
                                        match serde_json::from_value::<crate::scryfall::ScryfallCard>(
                                            json_value,
                                        ) {
                                            Ok(card) => match format_single_card_details(&card) {
                                                Ok(formatted_output) => {
                                                    tool_text_response!(formatted_output)
                                                }
                                                Err(e) => tool_text_response!(format!(
                                                    "Failed to format card details: {}",
                                                    e
                                                )),
                                            },
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
                    tool_text_response!(
                        "Error: 'set_code' and 'collector_number' parameters are required."
                    )
                }
            })
        }
    }
}

// Scryfall random card tool
pub struct ScryfallRandomTool;

impl ScryfallRandomTool {
    fn tool() -> Tool {
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

    fn call() -> ToolHandlerFn {
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
                                    return tool_text_response!(format!(
                                        "No random card found: {}",
                                        error_msg
                                    ));
                                } else {
                                    match serde_json::from_value::<crate::scryfall::ScryfallCard>(
                                        json_value,
                                    ) {
                                        Ok(card) => match format_single_card_details(&card) {
                                            Ok(formatted_output) => {
                                                tool_text_response!(formatted_output)
                                            }
                                            Err(e) => tool_text_response!(format!(
                                                "Failed to format card details: {}",
                                                e
                                            )),
                                        },
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

// Scryfall autocomplete tool
pub struct ScryfallAutocompleteTool;

impl ScryfallAutocompleteTool {
    fn tool() -> Tool {
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

    fn call() -> ToolHandlerFn {
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
                                        return tool_text_response!(format!(
                                            "Autocomplete failed: {}",
                                            error_msg
                                        ));
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
                                                    "Card name suggestions for '{}':\n\n",
                                                    query
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

// Deck analysis tool
pub struct DeckAnalysisTool;

impl DeckAnalysisTool {
    fn tool() -> Tool {
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

    fn call() -> ToolHandlerFn {
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
                        Err(e) => tool_text_response!(format!("Failed to analyze deck: {}", e))
                    }
                } else {
                    tool_text_response!("Error: 'deck_list' parameter is required.")
                }
            })
        }
    }
}

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    log::info!("Starting MTG MCP Server (STDIO)");

    let server_protocol = Server::builder(
        "mtg-mcp-server".to_string(),
        "1.0.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(ScryfallNamedTool::tool(), ScryfallNamedTool::call())
    .register_tool(ScryfallIdTool::tool(), ScryfallIdTool::call())
    .register_tool(ScryfallCollectorTool::tool(), ScryfallCollectorTool::call())
    .register_tool(ScryfallRandomTool::tool(), ScryfallRandomTool::call())
    .register_tool(
        ScryfallAutocompleteTool::tool(),
        ScryfallAutocompleteTool::call(),
    )
    .register_tool(DeckAnalysisTool::tool(), DeckAnalysisTool::call())
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
    .register_tool(ScryfallNamedTool::tool(), ScryfallNamedTool::call())
    .register_tool(ScryfallIdTool::tool(), ScryfallIdTool::call())
    .register_tool(ScryfallCollectorTool::tool(), ScryfallCollectorTool::call())
    .register_tool(ScryfallRandomTool::tool(), ScryfallRandomTool::call())
    .register_tool(
        ScryfallAutocompleteTool::tool(),
        ScryfallAutocompleteTool::call(),
    )
    .register_tool(DeckAnalysisTool::tool(), DeckAnalysisTool::call())
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
            description: Some("Search for Magic: The Gathering cards using Wizards' official Gatherer database. Provides comprehensive search with detailed filtering options including power/toughness, mana cost, card types, and format legality.".to_string()),
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
                    name: args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    rules: args
                        .get("rules")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    card_type: args
                        .get("card_type")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    subtype: args
                        .get("subtype")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    supertype: args
                        .get("supertype")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    mana_cost: args
                        .get("mana_cost")
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
                    flavor: args
                        .get("flavor")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    colors: args
                        .get("colors")
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
                    pretty: true, // Return pretty formatted output for MCP
                    page: args.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32,
                };

                // Create a default Global config for the search
                let global = crate::Global {
                    api_base_url: "https://api.magicthegathering.io/v1".to_string(),
                    verbose: false,
                    timeout: 30,
                };

                // Check if any search parameters were provided
                let has_params = search_params.name.is_some()
                    || search_params.rules.is_some()
                    || search_params.card_type.is_some()
                    || search_params.subtype.is_some()
                    || search_params.supertype.is_some()
                    || search_params.mana_cost.is_some()
                    || search_params.set.is_some()
                    || search_params.rarity.is_some()
                    || search_params.artist.is_some()
                    || search_params.power.is_some()
                    || search_params.toughness.is_some()
                    || search_params.loyalty.is_some()
                    || search_params.flavor.is_some()
                    || search_params.colors.is_some()
                    || search_params.format.is_some()
                    || search_params.language.is_some();

                if !has_params {
                    tool_text_response!("Error: No search parameters provided. Please specify at least one search parameter such as 'name', 'card_type', 'colors', etc.")
                } else {
                    // Perform the actual search
                    match crate::gatherer::search_cards_json(search_params, global).await {
                        Ok(card_data) => {
                            // Check if we got any results
                            if let Some(items) = card_data.get("items").and_then(|v| v.as_array()) {
                                if items.is_empty() {
                                    tool_text_response!(
                                        "No cards found matching the search criteria."
                                    )
                                } else {
                                    // Return the data as a formatted table
                                    match format_gatherer_search_results(&card_data) {
                                        Ok(formatted_output) => {
                                            tool_text_response!(formatted_output)
                                        }
                                        Err(e) => tool_text_response!(format!(
                                            "Failed to format card data: {}",
                                            e
                                        )),
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

                        match crate::scryfall::search_cards_json(search_params, global).await {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!("No cards found matching the search query.")
                                } else {
                                    match format_scryfall_search_results(&search_response) {
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
                    let advanced_params = crate::scryfall::AdvancedSearchParams {
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
                        match crate::scryfall::advanced_search_json(advanced_params, global).await {
                            Ok(search_response) => {
                                if search_response.data.is_empty() {
                                    tool_text_response!(
                                        "No cards found matching the search criteria."
                                    )
                                } else {
                                    match format_scryfall_search_results(&search_response) {
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
