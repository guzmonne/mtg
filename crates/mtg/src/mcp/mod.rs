use crate::prelude::*;
use rmcp::{model::*, service::RequestContext, ServerHandler, ServiceExt};
use serde_json::Value;
use std::sync::Arc;

pub mod prompts;
pub mod resources;
pub mod tools;

pub struct MtgMcpServer {
    api_base_url: String,
    client: reqwest::Client,
}

impl MtgMcpServer {
    pub fn new(api_base_url: String, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_base_url,
            client,
        }
    }
}

impl ServerHandler for MtgMcpServer {
    async fn initialize(
        &self,
        params: InitializeRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<InitializeResult, rmcp::ErrorData> {
        log::info!("Initializing MTG MCP Server");
        log::info!("Client info: {:?}", params.client_info);

        Ok(InitializeResult {
               protocol_version: ProtocolVersion::LATEST,
               capabilities: ServerCapabilities {
                   resources: Some(ResourcesCapability {
                       subscribe: Some(true),
                       list_changed: Some(true),
                   }),
                   tools: Some(ToolsCapability {
                       list_changed: Some(true),
                   }),
                   prompts: Some(PromptsCapability {
                       list_changed: Some(true),
                   }),
                   logging: Some(Default::default()),
                   completions: Some(Default::default()),
                   experimental: Some(Default::default()),
               },
               server_info: Implementation {
                   name: "mtg-mcp-server".to_string(),
                   version: "1.0.0".to_string(),
               },
               instructions: Some("MTG MCP Server provides access to Magic: The Gathering card data, sets, and types through the magicthegathering.io API.".to_string()),
           })
    }

    async fn list_resources(
        &self,
        _params: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        Ok(ListResourcesResult {
            resources: vec![
                Annotated {
                    raw: RawResource {
                        uri: "mtg://cards".to_string(),
                        name: "Magic Cards Database".to_string(),
                        description: Some("Access to all Magic: The Gathering cards".to_string()),
                        mime_type: Some("application/json".to_string()),
                        size: None,
                    },
                    annotations: Default::default(),
                },
                Annotated {
                    raw: RawResource {
                        uri: "mtg://sets".to_string(),
                        name: "Magic Sets Database".to_string(),
                        description: Some("Access to all Magic: The Gathering sets".to_string()),
                        mime_type: Some("application/json".to_string()),
                        size: None,
                    },
                    annotations: Default::default(),
                },
                Annotated {
                    raw: RawResource {
                        uri: "mtg://types".to_string(),
                        name: "Magic Card Types".to_string(),
                        description: Some(
                            "Card types, subtypes, supertypes, and formats".to_string(),
                        ),
                        mime_type: Some("application/json".to_string()),
                        size: None,
                    },
                    annotations: Default::default(),
                },
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        params: ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        match params.uri.as_str() {
            "mtg://cards" => resources::read_cards_resource(&self.client, &self.api_base_url)
                .await
                .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "mtg://sets" => resources::read_sets_resource(&self.client, &self.api_base_url)
                .await
                .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "mtg://types" => resources::read_types_resource(&self.client, &self.api_base_url)
                .await
                .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            _ => Err(rmcp::ErrorData::new(
                ErrorCode::RESOURCE_NOT_FOUND,
                format!("Unknown resource URI: {}", params.uri),
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _params: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        Ok(ListToolsResult {
           tools: vec![
               Tool {
                   name: "search_cards".into(),
                   description: Some("Search for Magic: The Gathering cards by name, type, colors, etc.".into()),
                   input_schema: Arc::new(serde_json::json!({
                       "type": "object",
                       "properties": {
                           "name": {
                               "type": "string",
                               "description": "Card name to search for"
                           },
                           "colors": {
                               "type": "string",
                               "description": "Card colors (comma-separated for AND, pipe-separated for OR)"
                           },
                           "type": {
                               "type": "string",
                               "description": "Card type"
                           },
                           "rarity": {
                               "type": "string",
                               "description": "Card rarity (Common, Uncommon, Rare, Mythic Rare)"
                           },
                           "set": {
                               "type": "string",
                               "description": "Set code"
                           },
                           "cmc": {
                               "type": "integer",
                               "description": "Converted mana cost"
                           },
                           "limit": {
                               "type": "integer",
                               "description": "Maximum number of results to return",
                               "default": 20
                           }
                       }
                   }).as_object().unwrap().clone()),
                   annotations: None,
               },
               Tool {
                   name: "get_card".into(),
                   description: Some("Get detailed information about a specific Magic card by ID".into()),
                   input_schema: Arc::new(serde_json::json!({
                       "type": "object",
                       "properties": {
                           "id": {
                               "type": "string",
                               "description": "Card ID or multiverseid"
                           }
                       },
                       "required": ["id"]
                   }).as_object().unwrap().clone()),
                   annotations: None,
               },
               Tool {
                   name: "list_sets".into(),
                   description: Some("List Magic: The Gathering sets with optional filtering".into()),
                   input_schema: Arc::new(serde_json::json!({
                       "type": "object",
                       "properties": {
                           "name": {
                               "type": "string",
                               "description": "Set name to filter by"
                           },
                           "block": {
                               "type": "string",
                               "description": "Block name to filter by"
                           },
                           "limit": {
                               "type": "integer",
                               "description": "Maximum number of results to return",
                               "default": 20
                           }
                       }
                   }).as_object().unwrap().clone()),
                   annotations: None,
               },
               Tool {
                   name: "generate_booster".into(),
                   description: Some("Generate a booster pack for a specific Magic set".into()),
                   input_schema: Arc::new(serde_json::json!({
                       "type": "object",
                       "properties": {
                           "set_code": {
                               "type": "string",
                               "description": "Set code to generate booster for (e.g., 'ktk' for Khans of Tarkir)"
                           }
                       },
                       "required": ["set_code"]
                   }).as_object().unwrap().clone()),
                   annotations: None,
               },
               Tool {
                   name: "get_card_types".into(),
                   description: Some("Get all Magic card types, subtypes, supertypes, or formats".into()),
                   input_schema: Arc::new(serde_json::json!({
                       "type": "object",
                       "properties": {
                           "category": {
                               "type": "string",
                               "enum": ["types", "subtypes", "supertypes", "formats"],
                               "description": "Category of types to retrieve"
                           }
                       },
                       "required": ["category"]
                   }).as_object().unwrap().clone()),
                   annotations: None,
               },
           ],
           next_cursor: None,
       })
    }

    async fn call_tool(
        &self,
        params: CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        match params.name.as_ref() {
            "search_cards" => tools::search_cards(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "get_card" => tools::get_card(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "list_sets" => tools::list_sets(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "generate_booster" => tools::generate_booster(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "get_card_types" => tools::get_card_types(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            _ => Err(rmcp::ErrorData::new(
                ErrorCode::METHOD_NOT_FOUND,
                format!("Unknown tool: {}", params.name),
                None,
            )),
        }
    }

    async fn list_prompts(
        &self,
        _params: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        Ok(ListPromptsResult {
           prompts: vec![
               Prompt {
                   name: "analyze_card".to_string(),
                   description: Some("Analyze a Magic card's power level, synergies, and competitive viability".to_string()),
                   arguments: Some(vec![
                       PromptArgument {
                           name: "card_name".to_string(),
                           description: Some("Name of the card to analyze".to_string()),
                           required: Some(true),
                       },
                       PromptArgument {
                           name: "format".to_string(),
                           description: Some("Format to analyze the card for (e.g., Standard, Modern, Commander)".to_string()),
                           required: Some(false),
                       },
                   ]),
               },
               Prompt {
                   name: "build_deck".to_string(),
                   description: Some("Help build a Magic deck around specific cards or themes".to_string()),
                   arguments: Some(vec![
                       PromptArgument {
                           name: "theme".to_string(),
                           description: Some("Deck theme or key cards to build around".to_string()),
                           required: Some(true),
                       },
                       PromptArgument {
                           name: "format".to_string(),
                           description: Some("Format for the deck (Standard, Modern, Commander, etc.)".to_string()),
                           required: Some(true),
                       },
                       PromptArgument {
                           name: "budget".to_string(),
                           description: Some("Budget constraints for the deck".to_string()),
                           required: Some(false),
                       },
                   ]),
               },
               Prompt {
                   name: "compare_cards".to_string(),
                   description: Some("Compare multiple Magic cards for deck building decisions".to_string()),
                   arguments: Some(vec![
                       PromptArgument {
                           name: "cards".to_string(),
                           description: Some("Comma-separated list of card names to compare".to_string()),
                           required: Some(true),
                       },
                       PromptArgument {
                           name: "criteria".to_string(),
                           description: Some("Specific criteria to compare (e.g., mana efficiency, power level, synergy)".to_string()),
                           required: Some(false),
                       },
                   ]),
               },
           ],
           next_cursor: None,
       })
    }

    async fn get_prompt(
        &self,
        params: GetPromptRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        match params.name.as_ref() {
            "analyze_card" => prompts::analyze_card_prompt(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "build_deck" => prompts::build_deck_prompt(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            "compare_cards" => prompts::compare_cards_prompt(
                &self.client,
                &self.api_base_url,
                params.arguments.map(serde_json::Value::Object),
            )
            .await
            .map_err(|e| rmcp::ErrorData::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)),
            _ => Err(rmcp::ErrorData::new(
                ErrorCode::METHOD_NOT_FOUND,
                format!("Unknown prompt: {}", params.name),
                None,
            )),
        }
    }

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities {
                resources: Some(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                prompts: Some(PromptsCapability {
                    list_changed: Some(true),
                }),
                logging: Some(Default::default()),
                completions: Some(Default::default()),
                experimental: Some(Default::default()),
            },
            server_info: Implementation {
                name: "mtg-mcp-server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("MTG MCP Server provides access to Magic: The Gathering card data, sets, and types through the magicthegathering.io API.".to_string()),
        }
    }
}

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    log::info!("Starting MTG MCP Server");

    let server = MtgMcpServer::new(global.api_base_url.clone(), global.timeout);

    // Use stdio transport for MCP
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    // Start the MCP server
    let mcp_server = server.serve(transport).await?;

    // Wait for the server to finish
    let quit_reason = mcp_server.waiting().await?;
    log::info!("MCP Server shut down: {:?}", quit_reason);

    Ok(())
}
