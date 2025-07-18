# MCP Overview

Understanding the Model Context Protocol and how the MTG server implements it.

## What is MCP?

The Model Context Protocol (MCP) is an open standard that enables AI assistants to securely access external data and tools. It provides a standardized way for AI models to:

- **Read data** from external sources
- **Execute tools** to perform actions
- **Use prompts** for structured interactions

## MCP Architecture

### Protocol Flow

```
AI Assistant                MCP Server               External API
     │                           │                        │
     │ 1. Initialize             │                        │
     ├──────────────────────────►│                        │
     │                           │                        │
     │ 2. List Resources/Tools    │                        │
     ├──────────────────────────►│                        │
     │                           │                        │
     │ 3. Call Tool/Read Resource │                        │
     ├──────────────────────────►│ 4. Fetch Data         │
     │                           ├───────────────────────►│
     │                           │ 5. Return Data         │
     │ 6. Return Result          │◄───────────────────────┤
     │◄──────────────────────────┤                        │
```

### Communication

- **Transport**: JSON-RPC 2.0 over stdio
- **Initialization**: Capability negotiation
- **Security**: Sandboxed execution environment

## MTG MCP Server Components

### 1. Resources

Static data sources that AI assistants can read:

```
mtg://cards   - Complete card database
mtg://sets    - All Magic sets information  
mtg://types   - Type system and formats
```

### 2. Tools

Interactive functions that AI assistants can execute:

```
search_cards      - Advanced card search
get_card         - Detailed card information
list_sets        - Set browsing and filtering
generate_booster - Virtual booster pack creation
get_card_types   - Type system queries
```

### 3. Prompts

Pre-built templates for common Magic tasks:

```
analyze_card    - Card power level analysis
build_deck      - Deck construction assistance
compare_cards   - Card comparison and evaluation
```

## Request/Response Flow

### Resource Access

```json
// AI Request
{
  "method": "resources/read",
  "params": {
    "uri": "mtg://cards"
  }
}

// Server Response
{
  "result": {
    "contents": [{
      "type": "text",
      "text": "{ \"cards\": [...] }"
    }]
  }
}
```

### Tool Execution

```json
// AI Request
{
  "method": "tools/call",
  "params": {
    "name": "search_cards",
    "arguments": {
      "name": "Lightning Bolt",
      "limit": 5
    }
  }
}

// Server Response
{
  "result": {
    "content": [{
      "type": "text", 
      "text": "Found 25 cards matching 'Lightning Bolt':\n\n**Lightning Bolt** (2ED)\n- Type: Instant\n- Rarity: Common\n- Mana Cost: {R}\n..."
    }],
    "isError": false
  }
}
```

### Prompt Usage

```json
// AI Request
{
  "method": "prompts/get",
  "params": {
    "name": "analyze_card",
    "arguments": {
      "card_name": "Lightning Bolt",
      "format": "Modern"
    }
  }
}

// Server Response
{
  "result": {
    "description": "Analysis of Lightning Bolt for Modern format",
    "messages": [{
      "role": "user",
      "content": {
        "type": "text",
        "text": "Please analyze the Magic: The Gathering card \"Lightning Bolt\" for competitive play in the Modern format.\n\nCard Data:\n```json\n{...}\n```\n\nPlease provide a comprehensive analysis covering:\n1. Power Level Assessment\n2. Competitive Viability\n3. Synergies and Combos\n4. Meta Considerations"
      }
    }]
  }
}
```

## Security Model

### Sandboxing
- Server runs in isolated environment
- No file system access beyond configuration
- Network access limited to MTG API

### Authentication
- No authentication required for public MTG data
- Rate limiting prevents abuse
- Timeout controls prevent hanging

### Data Privacy
- No user data stored
- All requests are stateless
- No logging of sensitive information

## Configuration

### Server Capabilities

The MTG MCP server advertises these capabilities:

```json
{
  "capabilities": {
    "resources": {
      "subscribe": true,
      "listChanged": true
    },
    "tools": {
      "listChanged": true
    },
    "prompts": {
      "listChanged": true
    },
    "logging": {}
  }
}
```

### Protocol Version

- **Supported**: MCP Protocol 2025-03-26
- **Backward Compatible**: Yes, with older versions
- **Future Proof**: Designed for protocol evolution

## Performance Characteristics

### Response Times
- **Resource reads**: 100-500ms (cached)
- **Tool calls**: 200-1000ms (API dependent)
- **Prompt generation**: 50-200ms

### Throughput
- **Concurrent requests**: Up to 10
- **Rate limiting**: 100 requests/minute
- **Timeout**: 30 seconds default

### Caching
- **Resource caching**: 15 minutes
- **Tool result caching**: None (dynamic data)
- **Prompt caching**: Indefinite (static templates)

## Debugging and Monitoring

### Verbose Mode

Enable detailed logging:
```bash
mtg --verbose mcp
```

Provides:
- Request/response logging
- Timing information
- Error details
- API interaction logs

### Health Checks

The server provides health information:
- Connection status
- API availability
- Response times
- Error rates

## Advanced Features

### Dynamic Resources

Resources update automatically:
- New cards added to database
- Set information updates
- Format legality changes

### Extensible Tools

Tool system supports:
- Parameter validation
- Error handling
- Result formatting
- Performance optimization

### Smart Prompts

Prompts include:
- Context-aware templates
- Dynamic data injection
- Format-specific guidance
- Best practice recommendations

## Integration Patterns

### Direct Integration
```bash
# Start server
mtg mcp

# Connect AI assistant via stdio
```

### Proxy Integration
```bash
# Use with MCP proxy
mcp-proxy --server "mtg mcp" --port 8080
```

### Container Integration
```dockerfile
FROM rust:1.70
COPY . /app
WORKDIR /app
RUN cargo build --release
CMD ["./target/release/mtg", "mcp"]
```

---

Next: [Setup & Installation](setup.md) | Back: [MCP Documentation](README.md)