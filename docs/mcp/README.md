# MCP Server Documentation

The MTG CLI includes a powerful Model Context Protocol (MCP) server that enables AI assistants to access Magic: The Gathering data seamlessly.

## Table of Contents

- [Overview](overview.md) - What is MCP and how it works
- [Setup & Installation](setup.md) - Getting the MCP server running
- [Server Details](./enhanced.md) - Modern mcp-core implementation with SSE support
- [Tools](tools.md) - Interactive card search tools for AI assistants
- [Usage Examples](examples.md) - Comprehensive examples and integration patterns
- [Resources](resources.md) - Available data resources
- [Prompts](prompts.md) - Pre-built prompt templates

## What is MCP?

The Model Context Protocol (MCP) is a standardized way for AI assistants to access external data and tools. The MTG MCP server provides:

- **Resources** - Access to card, set, and type databases
- **Tools** - Interactive functions for searching and analysis
- **Prompts** - Pre-built templates for common MTG tasks

## Quick Start

### Start the Server

```bash
# Default: STDIO transport (compatible with existing clients)
mtg mcp

# Explicit STDIO transport
mtg mcp stdio

# SSE transport (for web applications)
mtg mcp sse --host 127.0.0.1 --port 3000
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

### 🚀 **Modern Architecture**

- Built on modern `mcp-core` library
- SSE transport for web applications
- Improved error handling with color-eyre integration
- Better memory efficiency and scalability

### Rich Data Access

- **20,000+** Magic cards with full details
- **500+** sets from Alpha to present
- **Complete** type system and format information

### Advanced Search

- **Gatherer Integration**: Official Wizards database with advanced filtering
- **Scryfall Integration**: Comprehensive third-party API with flexible query syntax
- Multi-parameter card filtering (name, type, colors, mana cost, etc.)
- Fuzzy and exact name matching
- Complex queries with AND/OR operators

### Interactive Tools

- **Card Search**: Find cards using Gatherer or Scryfall APIs
- **Advanced Filtering**: Power, toughness, loyalty, artist, flavor text
- **Format Legality**: Search by tournament format legality
- **Pagination Support**: Handle large result sets efficiently

### AI-Optimized

- Structured data for AI consumption
- Pre-built prompts for common tasks
- Context-aware responses

### Multiple Transport Options

- **STDIO**: Traditional stdin/stdout (default, compatible with existing clients)
- **SSE**: HTTP-based Server-Sent Events (for web integration)

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

Continue to [Overview](./overview.md) for detailed MCP concepts.
