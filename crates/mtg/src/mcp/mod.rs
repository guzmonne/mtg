use mcp_core::{
    server::Server,
    tool_text_response,
    tools::ToolHandlerFn,
    transport::{ServerSseTransport, ServerStdioTransport},
    types::{CallToolRequest, CallToolResponse, ServerCapabilities, Tool, ToolCapabilities},
};
use serde_json::json;

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
    .register_tool(EchoTool::tool(), EchoTool::call())
    .register_tool(PingTool::tool(), PingTool::call())
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
    .register_tool(EchoTool::tool(), EchoTool::call())
    .register_tool(PingTool::tool(), PingTool::call())
    .build();

    let transport = ServerSseTransport::new(host, port, server_protocol);
    Server::start(transport).await.map_err(|e| eyre!(e))
}

// Simple echo tool for testing
pub struct EchoTool;

impl EchoTool {
    fn tool() -> Tool {
        Tool {
            name: "echo".to_string(),
            description: Some("Echo back the message you send".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo back"
                    }
                },
                "required": ["message"]
            }),
            annotations: None,
        }
    }

    fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let message = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("message"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("No message provided")
                    .to_string();

                tool_text_response!(format!("Echo: {}", message))
            })
        }
    }
}

// Simple ping tool for testing
pub struct PingTool;

impl PingTool {
    fn tool() -> Tool {
        Tool {
            name: "ping".to_string(),
            description: Some("Simple ping tool that returns 'pong'".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
            annotations: None,
        }
    }

    fn call() -> ToolHandlerFn {
        |_request: CallToolRequest| {
            Box::pin(async move {
                tool_text_response!("pong")
            })
        }
    }
}