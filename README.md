# MTG CLI

A powerful command-line interface for Magic: The Gathering card data with AI integration capabilities.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

## Overview

MTG CLI is a comprehensive command-line tool that provides access to Magic: The Gathering card data through the [MTG API](https://magicthegathering.io/). Built with Rust for performance and reliability, it offers both traditional CLI functionality and modern AI integration through the Model Context Protocol (MCP).

## Features

### Core Functionality

- **Card Search**: Advanced search with multiple filters (name, colors, type, rarity, set, mana cost)
- **Set Browsing**: Explore Magic sets and generate virtual booster packs
- **Type Information**: Access comprehensive card types, subtypes, supertypes, and game formats
- **Multiple Search Engines**: Support for both Wizards' Gatherer and Scryfall APIs
- **Shell Completions**: Auto-completion support for Bash, Zsh, Fish, PowerShell, and Elvish
- **High Performance**: Async operations with built-in caching and timeout controls

### AI Integration

- **MCP Server**: Model Context Protocol server for seamless AI assistant integration
- **Rich Data Access**: 20,000+ Magic cards with full details and metadata
- **Interactive Tools**: Card analysis, deck building assistance, and booster pack simulation
- **Pre-built Prompts**: Templates for common Magic-related AI tasks

### Output & Usability

- **Formatted Tables**: Clean, readable output with proper alignment
- **Pagination**: Handle large result sets efficiently
- **Environment Variables**: Configurable defaults for common options
- **Verbose Mode**: Detailed logging and debugging information

## Installation

### Prerequisites

- **Rust** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Git** - For cloning the repository

### Build from Source

1. **Clone the repository:**

   ```bash
   git clone https://github.com/cloudbridgeuy/mtg.git
   cd mtg
   ```

2. **Build the project:**

   ```bash
   cargo build --bin mtg --release
   ```

3. **Install globally (optional):**

   ```bash
   cargo install --bin mtg --path crates/mtg
   ```

4. **Verify installation:**
   ```bash
   ./target/release/mtg --version
   # or if installed globally:
   mtg --version
   ```

## Quick Start

### Basic Usage

```bash
# Search for a specific card
mtg api cards search "Lightning Bolt"

# List recent Magic sets
mtg api sets list --page-size 10

# Get all card types
mtg api types list

# Generate booster pack
mtg api sets booster "KTK"

# Search using Scryfall
mtg scryfall search "c:blue t:instant mv<=3"
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

### Start MCP Server for AI Integration

```bash
# Start MCP server
mtg mcp

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

## AI Integration with MCP

The MTG CLI includes a Model Context Protocol (MCP) server that enables AI assistants to access Magic: The Gathering data seamlessly.

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

### Available MCP Features

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

## Configuration

### Configuration Files

Create persistent configuration:

```bash
# ~/.mtg-config
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=60
export MTG_VERBOSE=1

# Load with: source ~/.mtg-config
```

### Network Configuration

For corporate environments:

```bash
# Proxy settings
export HTTP_PROXY="http://proxy.company.com:8080"
export HTTPS_PROXY="http://proxy.company.com:8080"

# Custom timeout for slow connections
export MTG_TIMEOUT=120
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

- [Getting Started](docs/cli/getting-started.md) - Installation and basic usage
- [Card Commands](docs/cli/cards.md) - Advanced card search techniques
- [Set Commands](docs/cli/sets.md) - Set browsing and booster generation
- [Type Commands](docs/cli/types.md) - Type system and format queries
- [Gatherer Commands](docs/cli/gatherer.md) - Official Wizards search functionality
- [Scryfall Commands](docs/cli/scryfall.md) - Powerful Scryfall search engine
- [Shell Completions](docs/cli/completions.md) - Completion setup for all shells
- [MCP Integration](docs/mcp/overview.md) - AI assistant integration guide

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
