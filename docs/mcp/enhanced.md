# MCP Server

The MTG MCP Server uses the modern `mcp-core` library to provide excellent performance, enhanced error handling, and multiple transport options including SSE (Server-Sent Events) for HTTP-based connections.

## Features

### 🚀 **Performance Improvements**
- Built on `mcp-core` for better efficiency and scalability
- Async/await throughout with proper error handling
- Reduced memory overhead and faster response times

### 🔧 **Enhanced Architecture**
- Cleaner tool definitions using macros
- Better type safety and validation
- Improved error messages and debugging

### 🌐 **Multiple Transport Options**
- **STDIO**: Traditional stdin/stdout transport (compatible with existing clients)
- **SSE**: HTTP-based Server-Sent Events transport for web integration

### 🔍 **Advanced Card Search Tools**
- **Gatherer Integration**: Official Wizards database with comprehensive filtering
- **Scryfall Integration**: Third-party API with flexible query syntax
- **Multi-parameter Search**: Complex filtering by type, color, mana cost, format, etc.
- **Pagination Support**: Handle large result sets efficiently

## Usage

### STDIO Transport (Default)

```bash
# Start MCP server with STDIO transport (default)
mtg mcp

# Explicit STDIO transport
mtg mcp stdio
```

This is compatible with existing MCP clients that expect stdin/stdout communication.

### SSE Transport (For web applications)

```bash
# Start MCP server with SSE transport on default host/port
mtg mcp sse

# Start on custom host and port
mtg mcp sse --host 0.0.0.0 --port 8080
```

The SSE transport provides HTTP endpoints for web-based MCP communication.

## Client Integration

### SSE Client Example

```rust
use mcp_core::{
    client::ClientBuilder,
    transport::ClientSseTransportBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientBuilder::new(
        ClientSseTransportBuilder::new("http://localhost:3000/sse".to_string()).build(),
    )
    .set_protocol_version(mcp_core::types::ProtocolVersion::V2024_11_05)
    .set_client_info("mtg-client".to_string(), "1.0.0".to_string())
    .build();

    client.open().await?;
    client.initialize().await?;

    // Search for cards
    let response = client
        .call_tool(
            "search_cards",
            Some(serde_json::json!({
                "name": "Lightning Bolt",
                "limit": 5
            })),
        )
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}
```

### STDIO Client Example

```rust
use mcp_core::{
    client::ClientBuilder,
    transport::ClientStdioTransport,
};

#[tokio::main]
async fn main() -> Result<()> {
    let transport = ClientStdioTransport::new("./target/debug/mtg", &["mcp", "stdio"])?;
    let client = ClientBuilder::new(transport.clone())
        .set_protocol_version(mcp_core::types::ProtocolVersion::V2024_11_05)
        .set_client_info("mtg-client".to_string(), "1.0.0".to_string())
        .build();

    client.open().await?;
    client.initialize().await?;

    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);

    Ok(())
}
```

## Available Tools

The enhanced MCP server currently provides demonstration tools to showcase the mcp-core integration:

### 🔄 **echo**
Echo back the message you send (demonstration tool).

**Parameters:**
- `message` (string, required): The message to echo back

### 🏓 **ping**
Simple ping tool that returns 'pong' (demonstration tool).

**Parameters:**
- None

## Future Enhancements

The enhanced server is designed to support the full MTG API functionality. Future versions will include:

- **search_cards**: Search for Magic: The Gathering cards by various criteria
- **get_card**: Get detailed information about a specific Magic card by ID  
- **list_sets**: List Magic: The Gathering sets with optional filtering
- **generate_booster**: Generate a booster pack for a specific Magic set
- **get_card_types**: Get all Magic card types, subtypes, supertypes, or formats

These tools will be implemented using the mcp-core architecture for improved performance and reliability.

## Configuration

### Environment Variables

All the same environment variables from the main CLI are supported:

- `MTG_API_BASE_URL`: MTG API Base URL (default: https://api.magicthegathering.io/v1)
- `MTG_TIMEOUT`: Request timeout in seconds (default: 30)
- `MTG_VERBOSE`: Enable verbose logging (default: false)

### Logging

The enhanced server uses the standard `log` crate (same as the rest of the MTG CLI). Set the `RUST_LOG` environment variable to control log levels:

```bash
# Enable debug logging
RUST_LOG=debug mtg mcp stdio

# Enable info logging
RUST_LOG=info mtg mcp sse
```

## Architecture Benefits

| Feature | Implementation |
|---------|----------------|
| Transport | STDIO (default) + SSE |
| Performance | Excellent with mcp-core |
| Error Handling | Enhanced with color-eyre integration |
| Tool Definition | Function-based with mcp-core |
| Type Safety | Strong typing |
| Debugging | Consistent log crate integration |
| Web Integration | Full SSE support |
| Memory Usage | Optimized |
| Concurrent Connections | Highly scalable |

## Usage Examples

### Default Usage

```bash
# Start with default STDIO transport
mtg mcp
```

### Web Integration

For web applications or HTTP-based integrations:
```bash
mtg mcp sse --host 0.0.0.0 --port 3000
```

Then connect using HTTP clients to the SSE endpoint.

## Troubleshooting

### Common Issues

1. **Port already in use (SSE)**
   ```bash
   # Use a different port
   mtg mcp sse --port 3001
   ```

2. **Connection refused (SSE)**
   ```bash
   # Check if server is running and accessible
   curl http://localhost:3000/sse
   ```

3. **Tool not found errors**
   ```bash
   # Enable debug logging to see available tools
   RUST_LOG=debug mtg mcp stdio
   ```

### Performance Tuning

For high-throughput scenarios:

```bash
# Increase timeout for slow API responses
MTG_TIMEOUT=60 mtg mcp sse --host 0.0.0.0 --port 3000

# Enable verbose logging for monitoring
MTG_VERBOSE=true RUST_LOG=info mtg mcp sse
```

## Security Considerations

### SSE Transport

- The SSE transport binds to `127.0.0.1` by default for security
- Use `--host 0.0.0.0` only in trusted environments
- Consider adding authentication for production deployments
- Monitor connection counts and implement rate limiting if needed

### STDIO Transport

- STDIO transport is inherently secure as it uses process communication
- Recommended for local development and CLI integrations
- No network exposure concerns