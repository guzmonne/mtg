# CLI Documentation

The MTG CLI provides a powerful command-line interface for interacting with Magic: The Gathering card data.

## Table of Contents

- [Getting Started](./getting-started.md) - Installation and basic usage
- [Card Commands](./cards.md) - Search and retrieve card information
- [Set Commands](./sets.md) - Browse sets and generate booster packs
- [Type Commands](./types.md) - Access card types and format information
- [Deck Analysis](./deck.md) - Analyze deck lists with comprehensive statistics
- [Gatherer Commands](./gatherer.md) - Search using Wizards' official Gatherer database
- [Scryfall Commands](./scryfall.md) - Search using Scryfall's powerful search engine
- [Workflow Guide](./workflow.md) - Common workflows and interactive tools
- [Shell Completions](./completions.md) - Generate shell completions for enhanced CLI experience
- [Configuration](./configuration.md) - Environment variables and settings

## Overview

The MTG CLI is built around six main command categories:

### API

Access the official Magic: The Gathering API directly with subcommands for:
- **Cards** - Search for Magic cards with advanced filtering options including name, colors, type, rarity, set, and mana cost
- **Sets** - Browse Magic sets, view set information, and generate virtual booster packs
- **Types** - Access comprehensive information about card types, subtypes, supertypes, and game formats

### Gatherer

Search using Wizards of the Coast's official Gatherer advanced search functionality for the most authoritative card data.

### Scryfall

Search using Scryfall's powerful and flexible search engine with:
- **Multiple lookup methods** - Find cards by name, ID, set/collector number, Arena ID, MTGO ID, and more
- **Advanced search syntax** - Use Scryfall's comprehensive query language
- **Enhanced filtering** - Individual filter options for precise searches
- **Autocomplete** - Get card name suggestions
- **Random cards** - Discover cards with optional filtering

### Deck Analysis

Analyze Magic: The Gathering deck lists with comprehensive statistics:
- **Mana curve analysis** - Understand your deck's speed and consistency
- **Type distribution** - Breakdown by card types (creatures, spells, lands)
- **Color analysis** - Color identity and mana requirements
- **Format legality** - Check legality across all major formats
- **Complete statistics** - Card counts, averages, and detailed breakdowns

### Completions

Generate shell completion scripts for enhanced command-line experience across multiple shells.

### MCP

Start the Model Context Protocol server for AI integration.

## Quick Examples

```bash
# Search for a specific card
mtg api cards search "Lightning Bolt"

# Search using official Gatherer database
mtg gatherer search --name "Lightning Bolt" --rarity "Common"

# Search using Scryfall's powerful syntax
mtg scryfall search "c:red t:creature mv<=3" --pretty

# Get card by exact name
mtg scryfall named "Lightning Bolt" --pretty

# Get card by set and collector number
mtg scryfall collector ktk 96 --pretty

# Get random card
mtg scryfall random --query "t:legendary" --pretty

# Get autocomplete suggestions
mtg scryfall autocomplete "lightning"

# Analyze a deck list
mtg deck stats --file my_deck.txt

# Analyze deck from stdin
echo "Deck\n4 Lightning Bolt\n4 Mountain" | mtg deck stats -

# List recent sets
mtg api sets list --page-size 10

# Get all card types
mtg api types list

# Generate shell completions
mtg completions generate bash

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

Continue to [Getting Started](./getting-started.md) for installation instructions.
