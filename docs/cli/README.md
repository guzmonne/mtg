# CLI Documentation

The MTG CLI provides a powerful command-line interface for interacting with Magic: The Gathering card data.

## Table of Contents

- [Getting Started](getting-started.md) - Installation and basic usage
- [Card Commands](cards.md) - Search and retrieve card information
- [Set Commands](sets.md) - Browse sets and generate booster packs
- [Type Commands](types.md) - Access card types and format information
- [Configuration](configuration.md) - Environment variables and settings

## Overview

The MTG CLI is built around four main command categories:

### Cards
Search for Magic cards with advanced filtering options including name, colors, type, rarity, set, and mana cost.

### Sets
Browse Magic sets, view set information, and generate virtual booster packs.

### Types
Access comprehensive information about card types, subtypes, supertypes, and game formats.

### MCP
Start the Model Context Protocol server for AI integration.

## Quick Examples

```bash
# Search for a specific card
mtg cards search "Lightning Bolt"

# List recent sets
mtg sets list --page-size 10

# Get all card types
mtg types list

# Start MCP server
mtg mcp
```

## Global Options

All commands support these global options:

- `--api-base-url <URL>` - Override the default MTG API URL
- `--timeout <SECONDS>` - Set request timeout (default: 30)
- `--verbose` - Enable verbose output
- `--help` - Show help information
- `--version` - Show version information

## Environment Variables

Configure the CLI using environment variables:

- `MTG_API_BASE_URL` - Default API base URL
- `MTG_TIMEOUT` - Default timeout in seconds
- `MTG_VERBOSE` - Enable verbose mode (any value)

---

Continue to [Getting Started](getting-started.md) for installation instructions.