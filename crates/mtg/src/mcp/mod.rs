use mcp_core::{
    server::Server,
    transport::{ServerSseTransport, ServerStdioTransport},
    types::{ServerCapabilities, ToolCapabilities},
};

use crate::prelude::*;

mod deck;
mod scryfall;

pub async fn run_mcp_server(_global: crate::Global) -> Result<()> {
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
    .register_tool(scryfall::named::Mcp::tool(), scryfall::named::Mcp::call())
    .register_tool(scryfall::id::Mcp::tool(), scryfall::id::Mcp::call())
    .register_tool(
        scryfall::collector::Mcp::tool(),
        scryfall::collector::Mcp::call(),
    )
    .register_tool(scryfall::random::Mcp::tool(), scryfall::random::Mcp::call())
    .register_tool(
        scryfall::autocomplete::Mcp::tool(),
        scryfall::autocomplete::Mcp::call(),
    )
    .register_tool(deck::analysis::Mcp::tool(), deck::analysis::Mcp::call())
    .register_tool(scryfall::search::Mcp::tool(), scryfall::search::Mcp::call())
    .build();

    let transport = ServerStdioTransport::new(server_protocol);
    Server::start(transport).await.map_err(|e| eyre!(e))
}

pub async fn run_sse_server(_global: crate::Global, host: String, port: u16) -> Result<()> {
    log::info!("Starting MTG MCP Server (SSE) on {host}:{port}");

    let server_protocol = Server::builder(
        "mtg-mcp-server".to_string(),
        "2.0.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(scryfall::named::Mcp::tool(), scryfall::named::Mcp::call())
    .register_tool(scryfall::id::Mcp::tool(), scryfall::id::Mcp::call())
    .register_tool(
        scryfall::collector::Mcp::tool(),
        scryfall::collector::Mcp::call(),
    )
    .register_tool(scryfall::random::Mcp::tool(), scryfall::random::Mcp::call())
    .register_tool(
        scryfall::autocomplete::Mcp::tool(),
        scryfall::autocomplete::Mcp::call(),
    )
    .register_tool(deck::analysis::Mcp::tool(), deck::analysis::Mcp::call())
    .register_tool(scryfall::search::Mcp::tool(), scryfall::search::Mcp::call())
    .build();

    let transport = ServerSseTransport::new(host, port, server_protocol);
    Server::start(transport).await.map_err(|e| eyre!(e))
}
