# MTG CLI Documentation

Welcome to the comprehensive documentation for the Magic: The Gathering CLI tool with Model Context Protocol (./mcp) integration.

## Documentation Structure

### CLI Usage

Complete guide to using the MTG CLI for card searches, set browsing, and more.

- [Getting Started](./cli/getting-started.md)
- [Card Commands](./cli/cards.md)
- [Set Commands](./cli/sets.md)
- [Type Commands](./cli/types.md)
- [Shell Completions](./cli/completions.md)

### MCP Server

Detailed documentation for the Model Context Protocol server integration.

- [Overview](./mcp/overview.md)
- [Setup & Installation](./mcp/setup.md)
- [Resources](./mcp/resources.md)
- [Tools](./mcp/tools.md)
- [Prompts](./mcp/prompts.md)

## Quick Start

### CLI Usage

```bash
# Install and run with cargo
cargo build --bin mtg --release

# Run it from the `./target/release` directory
./target/release/mtg cards search "Lightning Bolt"

# Use `xtask` to install it to `~/.local/bin`
cargo xtask install -p ~/.local/bin --name mtg

# Run it from your $PATH
mtg cards search "Lightning Bolt"
```

### MCP Server

```bash
# Start MCP server
mtg mcp

# Or with custom configuration
mtg --api-base-url https://api.magicthegathering.io/v1 mcp
```

## Key Features

- **Card Search**: Advanced search with multiple filters
- **Set Browsing**: Explore Magic sets and generate booster packs
- **Type Information**: Access card types, subtypes, and formats
- **Shell Completions**: Auto-completion support for Bash, Zsh, Fish, PowerShell, and Elvish
- **AI Integration**: MCP server for seamless AI assistant integration
- **High Performance**: Async operations with rate limiting
- **Formatted Output**: Formatted tables and rich CLI experience

## External Resources

- [Magic: The Gathering API](https://magicthegathering.io/)
- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [Rust Documentation](https://doc.rust-lang.org/)
