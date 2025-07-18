# MTG CLI Documentation

Welcome to the comprehensive documentation for the Magic: The Gathering CLI tool with Model Context Protocol (MCP) integration.

## Documentation Structure

### CLI Usage
Complete guide to using the MTG CLI for card searches, set browsing, and more.
- [Getting Started](cli/getting-started.md)
- [Card Commands](cli/cards.md)
- [Set Commands](cli/sets.md)
- [Type Commands](cli/types.md)
- [Configuration](cli/configuration.md)

### MCP Server
Detailed documentation for the Model Context Protocol server integration.
- [Overview](mcp/overview.md)
- [Setup & Installation](mcp/setup.md)
- [Resources](mcp/resources.md)
- [Tools](mcp/tools.md)
- [Prompts](mcp/prompts.md)

### Examples & Testing
Practical examples and testing scenarios.
- [CLI Examples](examples/cli-examples.md)
- [MCP Testing](examples/mcp-testing.md)
- [Integration Examples](examples/integration-examples.md)

### API Reference
Technical reference for developers.
- [CLI Commands](api/cli-reference.md)
- [MCP Protocol](api/mcp-reference.md)
- [Data Structures](api/data-structures.md)

### Troubleshooting
Common issues and solutions.
- [Installation Issues](troubleshooting/installation.md)
- [Runtime Errors](troubleshooting/runtime.md)
- [MCP Connection Issues](troubleshooting/mcp.md)

## Quick Start

### CLI Usage
```bash
# Install and run
cargo build --release
./target/release/mtg cards search "Lightning Bolt"
```

### MCP Server
```bash
# Start MCP server
./target/release/mtg mcp

# Or with custom configuration
./target/release/mtg --api-base-url https://api.magicthegathering.io/v1 mcp
```

## Key Features

- **Card Search**: Advanced search with multiple filters
- **Set Browsing**: Explore Magic sets and generate booster packs
- **Type Information**: Access card types, subtypes, and formats
- **AI Integration**: MCP server for seamless AI assistant integration
- **High Performance**: Async operations with rate limiting
- **Formatted Output**: Formatted tables and rich CLI experience

## External Resources

- [Magic: The Gathering API](https://magicthegathering.io/)
- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [Rust Documentation](https://doc.rust-lang.org/)

---

For questions or issues, check the [troubleshooting guide](troubleshooting/) or open an issue on GitHub.