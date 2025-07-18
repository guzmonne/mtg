# MCP Server Documentation

The MTG CLI includes a powerful Model Context Protocol (MCP) server that enables AI assistants to access Magic: The Gathering data seamlessly.

## Table of Contents

- [Overview](overview.md) - What is MCP and how it works
- [Setup & Installation](setup.md) - Getting the MCP server running
- [Resources](resources.md) - Available data resources
- [Tools](tools.md) - Interactive tools for AI assistants
- [Prompts](prompts.md) - Pre-built prompt templates
- [Integration Guide](integration.md) - Connecting with AI assistants

## What is MCP?

The Model Context Protocol (MCP) is a standardized way for AI assistants to access external data and tools. The MTG MCP server provides:

- **Resources** - Access to card, set, and type databases
- **Tools** - Interactive functions for searching and analysis
- **Prompts** - Pre-built templates for common MTG tasks

## Quick Start

### Start the Server
```bash
# Build the project
cargo build --release

# Start MCP server
./target/release/mtg mcp
```

### Test with Claude Desktop
Add to your Claude Desktop configuration:
```json
{
  "mcpServers": {
    "mtg": {
      "command": "/path/to/mtg",
      "args": ["mcp"]
    }
  }
}
```

## Key Features

### Rich Data Access
- **20,000+** Magic cards with full details
- **500+** sets from Alpha to present
- **Complete** type system and format information

### Advanced Search
- Multi-parameter card filtering
- Fuzzy and exact name matching
- Color, type, rarity, and set filtering

### Interactive Tools
- Booster pack generation
- Card analysis and comparison
- Deck building assistance

### AI-Optimized
- Structured data for AI consumption
- Pre-built prompts for common tasks
- Context-aware responses

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   AI Assistant  │◄──►│   MCP Server    │◄──►│  MTG API        │
│   (Claude, etc) │    │   (mtg mcp)     │    │  (External)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
        │                       │                       │
    JSON-RPC                HTTP/REST              Card Data
    over stdio              Requests               & Metadata
```

## Use Cases

### Game Analysis
- Card power level assessment
- Meta analysis and trends
- Deck optimization suggestions

### Educational Content
- Rules explanations
- Format introductions
- Historical context

### Deck Building
- Card recommendations
- Synergy identification
- Budget optimization

### Data Analysis
- Set statistics
- Rarity distributions
- Power creep analysis

## Integration Examples

### Claude Desktop
Perfect for casual Magic discussions and deck building.

### Custom Applications
Build MTG-focused tools with AI assistance.

### Educational Platforms
Integrate Magic content into learning systems.

### Tournament Tools
Enhance tournament software with AI insights.

---

Continue to [Overview](overview.md) for detailed MCP concepts.