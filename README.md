# MTG CLI

A powerful command-line interface for Magic: The Gathering card data with AI integration capabilities.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

## Overview

MTG CLI is a comprehensive command-line tool that provides access to Magic: The Gathering card data through multiple APIs. Built with Rust for performance and reliability, it offers both traditional CLI functionality and modern AI integration through the Model Context Protocol (MCP).

## Features

### Core Functionality

- **Multiple Search Engines**: Support for MTG API, Wizards' Gatherer, and Scryfall APIs
- **Advanced Card Search**: Search with filters for name, colors, type, rarity, set, mana cost, and more
- **Set Browsing**: Explore Magic sets and generate virtual booster packs
- **Type Information**: Access comprehensive card types, subtypes, supertypes, and game formats
- **Shell Completions**: Auto-completion support for Bash, Zsh, Fish, PowerShell, and Elvish
- **High Performance**: Async operations with built-in caching and timeout controls

### AI Integration (MCP)

- **MCP Server**: Model Context Protocol server for seamless AI assistant integration
- **Rich Data Access**: 20,000+ Magic cards with full details and metadata
- **Interactive Tools**: 8 comprehensive tools for card lookup, search, and deck analysis
- **Pre-built Prompts**: Templates for common Magic-related AI tasks
- **Multiple Transports**: STDIO and SSE support for different integration needs

### Output & Usability

- **Formatted Tables**: Clean, readable output with proper alignment
- **Pagination**: Handle large result sets efficiently
- **Environment Variables**: Configurable defaults for common options
- **Verbose Mode**: Detailed logging and debugging information

## Installation

### Pre-built Binaries

Download pre-built binaries for your operating system from [GitHub Releases](https://github.com/cloudbridgeuy/mtg/releases). Binaries are available for:

- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

### Build from Source

**Prerequisites:**
- Rust (1.70 or later) - [Install Rust](https://rustup.rs/)
- Git

**Steps:**

1. **Clone and build:**
   ```bash
   git clone https://github.com/cloudbridgeuy/mtg.git
   cd mtg
   cargo build --bin mtg --release
   ```

2. **Install globally (optional):**
   ```bash
   cargo install --bin mtg --path crates/mtg
   ```

3. **Verify installation:**
   ```bash
   ./target/release/mtg --version
   # or if installed globally:
   mtg --version
   ```

## Quick Start

### Command Overview

The CLI provides access to three main APIs:

- **`api`** - MTG API for basic card, set, and type queries
- **`scryfall`** - Scryfall API for advanced search with flexible syntax
- **`gatherer`** - Wizards' Gatherer database for official card data

### Basic Usage

```bash
# MTG API - Basic card search
mtg api cards search "Lightning Bolt"
mtg api sets list --page-size 10
mtg api types list

# Scryfall API - Advanced search
mtg scryfall search "c:blue t:instant mv<=3" --pretty
mtg scryfall named "Lightning Bolt" --pretty
mtg scryfall collector ktk 96 --pretty

# Gatherer API - Official Wizards data
mtg gatherer search --name "Lightning Bolt" --rarity "Common"
mtg gatherer search --colors "Red" --type "Instant"

# Generate booster pack
mtg api sets booster "KTK"
```

### Set Up Shell Completions

```bash
# Bash
mtg completions generate bash > ~/.local/share/bash-completion/completions/mtg

# Zsh
mtg completions generate zsh > ~/.zsh/completions/_mtg

# Fish
mtg completions generate fish > ~/.config/fish/completions/mtg.fish

# PowerShell
mtg completions generate powershell >> $PROFILE
```

### MCP Server for AI Integration

```bash
# Start MCP server (STDIO transport)
mtg mcp

# Start with SSE transport for web applications
mtg mcp sse --host 127.0.0.1 --port 3000

# With custom configuration
mtg --api-base-url https://api.magicthegathering.io/v1 --timeout 60 mcp
```

## Usage Guide

### Global Options

All commands support these global options:

- `--api-base-url <URL>` - Override the default MTG API URL
- `--timeout <SECONDS>` - Set request timeout (default: 30)
- `--verbose` - Enable verbose output
- `--help` - Show help information
- `--version` - Show version information

### Environment Variables

Configure defaults using environment variables:

```bash
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=60
export MTG_VERBOSE=1
```

### Card Commands

```bash
# Basic search
mtg api cards search "Lightning"

# Exact name matching
mtg api cards search "Lightning Bolt" --exact

# Advanced filtering
mtg api cards list --colors "Red,Blue" --type "Creature" --cmc 4

# Get specific card by ID
mtg api cards get 409574

# Search with pagination
mtg api cards search "Dragon" --page 2 --page-size 20
```

### Set Commands

```bash
# List all sets
mtg api sets list

# Search sets by name
mtg api sets search "Zendikar"

# Get specific set information
mtg api sets get "KTK"

# Generate booster pack
mtg api sets booster "ISD"
```

### Type Commands

```bash
# List all card types
mtg api types list

# Get subtypes
mtg api types subtypes

# Get supertypes
mtg api types supertypes

# Get game formats
mtg api types formats
```

### Completion Commands

```bash
# Generate completions for your shell
mtg completions generate bash
mtg completions generate zsh
mtg completions generate fish
mtg completions generate powershell
mtg completions generate elvish
```

## MCP Server for AI Integration

The MTG CLI includes a powerful Model Context Protocol (MCP) server that enables AI assistants to access Magic: The Gathering data seamlessly.

### Key Features

- **Rich Data Access**: 20,000+ Magic cards with full details and metadata
- **Interactive Tools**: 8 comprehensive tools for card lookup, search, and deck analysis
- **Multiple APIs**: Access to MTG API, Scryfall, and Gatherer through unified interface
- **Advanced Search**: Complex queries with 15+ sort options and filtering
- **Deck Analysis**: Comprehensive statistics including mana curve and format legality
- **Multiple Transports**: STDIO (default) and SSE support for different integration needs

### Claude Desktop Integration

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "mtg": {
      "command": "/path/to/mtg",
      "args": ["mcp"],
      "env": {
        "MTG_TIMEOUT": "60"
      }
    }
  }
}
```

### Available MCP Components

- **Resources**: Access to complete card, set, and type databases
- **Tools**: Interactive functions for searching, analysis, and booster generation
- **Prompts**: Pre-built templates for card analysis, deck building, and comparisons

## Examples

### Advanced Card Searches

```bash
# Find expensive red creatures
mtg api cards list --colors "Red" --type "Creature" --cmc 6 --rarity "Mythic Rare"

# Search for artifacts from specific set
mtg api cards list --type "Artifact" --set "KTK" --page-size 50

# Find cards legal in Modern format
mtg api cards list --format "Modern" --rarity "Rare"
```

### Set Analysis

```bash
# Research Khans of Tarkir
mtg api sets get "KTK"

# Generate multiple booster packs
for i in {1..3}; do
  echo "=== Booster Pack $i ==="
  mtg api sets booster "KTK"
  echo
done
```

### Deck Building Research

```bash
# Find all Dragons
mtg api cards list --subtype "Dragon" --page-size 100

# Research tribal strategies
mtg api types subtypes | grep -i "warrior\|wizard\|goblin"

# Check format legality
mtg api types formats
```



## Development

### Building

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- api cards search "Lightning Bolt"
```

### Project Structure

```
mtg/
├── crates/
│   └── mtg/           # Main CLI application
├── docs/              # Comprehensive documentation
├── xtask/             # Build automation
├── Cargo.toml         # Workspace configuration
└── README.md          # This file
```

### Dependencies

Key dependencies include:

- `clap` - Command-line argument parsing
- `clap_complete` - Shell completion generation
- `tokio` - Async runtime
- `reqwest` - HTTP client for API calls
- `serde` - JSON serialization/deserialization
- `prettytable` - Formatted table output
- `rmcp` - Model Context Protocol server
- `color-eyre` - Enhanced error reporting

## Documentation

Comprehensive documentation is available in the [`docs/`](docs/) directory:

### CLI Usage
- [Getting Started](docs/cli/getting-started.md) - Installation and basic usage
- [Card Commands](docs/cli/cards.md) - MTG API card search techniques
- [Set Commands](docs/cli/sets.md) - Set browsing and booster generation
- [Type Commands](docs/cli/types.md) - Type system and format queries
- [Gatherer Commands](docs/cli/gatherer.md) - Official Wizards search functionality
- [Scryfall Commands](docs/cli/scryfall.md) - Powerful Scryfall search engine
- [Shell Completions](docs/cli/completions.md) - Completion setup for all shells

### MCP Integration
- [MCP Overview](docs/mcp/overview.md) - AI assistant integration guide
- [Setup & Installation](docs/mcp/setup.md) - Getting the MCP server running
- [Tools](docs/mcp/tools.md) - Interactive card search tools for AI assistants
- [Resources](docs/mcp/resources.md) - Available data resources
- [Prompts](docs/mcp/prompts.md) - Pre-built prompt templates

## Troubleshooting

### Common Issues

**Network Timeouts:**

```bash
# Increase timeout
mtg --timeout 120 api cards search "Complex Query"
```

**API Rate Limiting:**

- Use smaller page sizes
- Add delays between requests
- The CLI includes built-in rate limiting

**Connection Issues:**

```bash
# Test API connectivity
curl -s "https://api.magicthegathering.io/v1/cards?pageSize=1"

# Check with verbose output
mtg --verbose api cards search "test"
```

**Shell Completions Not Working:**

```bash
# Verify completion file exists
ls -la ~/.local/share/bash-completion/completions/mtg

# Reload shell configuration
source ~/.bashrc  # or ~/.zshrc
```

### Debug Mode

Enable detailed debugging:

```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
mtg --verbose api cards search "Lightning Bolt"
```

## Contributing

Contributions are welcome! Whether you're fixing bugs, adding features, improving documentation, or enhancing the AI integration capabilities.

### Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add tests if applicable
5. Run the test suite: `cargo test`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Add tests for new functionality
- Update documentation for user-facing changes
- Ensure shell completions work with new commands
- Test MCP integration if modifying server functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Magic: The Gathering API](https://magicthegathering.io/) for providing comprehensive card data
- [Model Context Protocol](https://spec.modelcontextprotocol.io/) for enabling AI integration
- The Rust community for excellent crates and tooling
- Wizards of the Coast for creating Magic: The Gathering

## Links

- **Documentation**: [docs/](docs/)
- **MTG API**: https://magicthegathering.io/
- **MCP Specification**: https://spec.modelcontextprotocol.io/
- **Issues**: https://github.com/cloudbridgeuy/mtg/issues

---

_Built with ❤️ and Rust for the Magic: The Gathering community_
