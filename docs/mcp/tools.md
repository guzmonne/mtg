# MCP Tools

Tools provide AI assistants with interactive functions to search and analyze Magic: The Gathering cards using both official and third-party APIs.

## Available Tools

The MTG MCP server provides two powerful card search tools:

| Tool                        | Purpose                           | API Source | Parameters                                    |
| --------------------------- | --------------------------------- | ---------- | --------------------------------------------- |
| **gatherer_search_cards**   | Official Wizards database search | Gatherer   | name, rules, types, colors, mana, set, etc.  |
| **scryfall_search_cards**   | Advanced third-party search      | Scryfall   | query, name, oracle, colors, format, etc.    |

## gatherer_search_cards

Search for Magic cards using Wizards of the Coast's official Gatherer database with advanced filtering options.

### Parameters

All parameters are optional. You can combine multiple parameters for precise searches.

```json
{
  "name": "Lightning",           // Card name (partial matching)
  "rules": "deals 3 damage",     // Rules text search
  "card_type": "Instant",        // Card type (supports AND/OR: "Creature+Legendary" or "Instant,Sorcery")
  "subtype": "Human",            // Subtype (supports AND/OR: "Human+Soldier" or "Wizard,Cleric")
  "supertype": "Legendary",      // Supertype (e.g., "Legendary", "Snow")
  "mana_cost": "{R}",           // Mana cost (e.g., "{2}{U}", "1W(B/G)")
  "set": "Khans of Tarkir",     // Set name
  "rarity": "Rare",             // Rarity (Common, Uncommon, Rare, Mythic)
  "artist": "Christopher Rush", // Artist name
  "power": "3",                 // Power value or range ("3", "3-5")
  "toughness": "2",             // Toughness value or range ("2", "1-4")
  "loyalty": "4",               // Loyalty value or range ("3", "3-6")
  "flavor": "sparkmage",        // Flavor text search
  "colors": "R",                // Colors ("W", "U", "B", "R", "G", "!RBW" for exclusion)
  "format": "Legal:Standard",   // Format legality ("Legal:Standard", "Banned:Modern")
  "language": "English",        // Language (English, Japanese, French, German, etc.)
  "page": 1                     // Page number for pagination (default: 1)
}
```

### Examples

#### Basic Name Search

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "name": "Lightning Bolt"
    }
  }
}
```

**Response:**
```
Gatherer Card Search Tool

Gatherer search would be performed with parameters: name: Lightning Bolt

Note: This is a demonstration. The actual search functionality will query Wizards' Gatherer database and return detailed card information including names, types, mana costs, oracle text, and more.
```

#### Advanced Creature Search

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Creature",
      "subtype": "Dragon",
      "colors": "R",
      "power": "4-8",
      "rarity": "Rare"
    }
  }
}
```

#### Complex Type Filtering

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Creature+Legendary",  // AND operation
      "subtype": "Human,Wizard",          // OR operation
      "mana_cost": "{2}{U}{U}",
      "set": "Dominaria"
    }
  }
}
```

#### Format-Specific Search

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "format": "Legal:Standard",
      "card_type": "Instant",
      "colors": "!G",  // Not green
      "page": 1
    }
  }
}
```

### Color Syntax

- **Single Color**: `"R"` (Red cards only)
- **Multiple Colors**: `"WU"` (White and Blue cards)
- **Color Exclusion**: `"!G"` (Non-green cards)
- **Complex**: `"WUB"` (White, Blue, and Black cards)

### Type Operators

- **AND**: Use `+` between types: `"Creature+Legendary"`
- **OR**: Use `,` between types: `"Instant,Sorcery"`
- **Mixed**: `"Artifact+Creature,Enchantment+Creature"`

### Use Cases

- **Deck Building**: Find cards matching specific criteria for deck construction
- **Collection Management**: Search your collection by various attributes
- **Rules Research**: Find cards with specific rules text or interactions
- **Format Analysis**: Research legal cards in specific tournament formats
- **Educational**: Explore Magic's design space and card mechanics

## scryfall_search_cards

Search for Magic cards using Scryfall's comprehensive API with flexible query syntax and extensive filtering options.

### Parameters

You can use either a direct `query` parameter with Scryfall syntax, or individual parameters that will be combined into a query.

```json
{
  "query": "c:red t:creature",   // Direct Scryfall query syntax
  "name": "Lightning",           // Card name (alternative to query)
  "oracle": "deals damage",      // Oracle text search
  "card_type": "creature",       // Card type (lowercase)
  "colors": "wu",               // Colors (w=white, u=blue, b=black, r=red, g=green)
  "identity": "wu",             // Color identity for Commander
  "mana": "{2}{U}",             // Mana cost
  "mv": ">=4",                  // Mana value/CMC with operators (>=, <=, <, >, =)
  "power": ">=3",               // Power with operators
  "toughness": "<=2",           // Toughness with operators
  "loyalty": "4",               // Loyalty value
  "set": "ktk",                 // Set code (3-letter)
  "rarity": "rare",             // Rarity (common, uncommon, rare, mythic)
  "artist": "Rebecca Guay",     // Artist name
  "flavor": "ancient magic",    // Flavor text search
  "format": "standard",         // Format legality (standard, modern, legacy, etc.)
  "language": "en",             // Language code (en, ja, de, fr, etc.)
  "page": 1,                    // Page number (default: 1)
  "order": "name"               // Sort order (name, cmc, power, released, etc.)
}
```

### Examples

#### Direct Query Syntax

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "c:red t:creature mana>=4"
    }
  }
}
```

#### Advanced Parameter Search

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "name": "Jace",
      "card_type": "planeswalker",
      "colors": "u",
      "format": "standard",
      "order": "released"
    }
  }
}
```

#### Commander Deck Building

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "identity": "wubrg",  // 5-color identity
      "card_type": "creature",
      "mv": "<=3",
      "format": "commander"
    }
  }
}
```

#### Complex Query Examples

```json
// Find expensive artifacts in Modern
{
  "query": "t:artifact mana>=6 f:modern"
}

// Find cheap removal spells
{
  "query": "o:destroy o:target o:creature mana<=2"
}

// Find cards with specific power/toughness
{
  "query": "pow=tou t:creature"  // Power equals toughness
}

// Find cards by artist
{
  "query": "a:\"John Avon\" t:land"
}
```

### Scryfall Query Syntax

Scryfall supports powerful query syntax:

#### Basic Operators
- `c:red` - Color
- `t:creature` - Type
- `o:flying` - Oracle text contains "flying"
- `mana:3` - Mana value equals 3
- `mana>=4` - Mana value 4 or greater
- `pow>=3` - Power 3 or greater
- `tou<=2` - Toughness 2 or less

#### Advanced Operators
- `is:commander` - Legal commanders
- `is:reserved` - Reserved list cards
- `is:reprint` - Reprinted cards
- `f:standard` - Legal in Standard
- `banned:modern` - Banned in Modern
- `s:ktk` - From Khans of Tarkir set
- `r:mythic` - Mythic rare cards

#### Logical Operators
- `AND` - Both conditions (default)
- `OR` - Either condition
- `NOT` - Exclude condition
- `()` - Grouping

### Use Cases

- **Competitive Deck Building**: Find tournament-legal cards with precise criteria
- **Commander Brewing**: Search by color identity and format legality
- **Collection Analysis**: Research card values, printings, and availability
- **Meta Research**: Analyze format staples and banned cards
- **Educational**: Learn advanced search techniques and card interactions

## Tool Integration Patterns

### Chaining Searches

AI assistants can use both tools for comprehensive research:

```python
# 1. Use Scryfall for broad search
scryfall_results = call_tool("scryfall_search_cards", {
    "query": "c:red t:creature mana<=3 f:standard"
})

# 2. Use Gatherer for detailed official information
gatherer_results = call_tool("gatherer_search_cards", {
    "name": "specific_card_name",
    "format": "Legal:Standard"
})
```

### Comparative Analysis

```python
# Compare search results between APIs
def compare_search_apis(search_term):
    gatherer_results = call_tool("gatherer_search_cards", {
        "name": search_term
    })
    
    scryfall_results = call_tool("scryfall_search_cards", {
        "name": search_term
    })
    
    return analyze_differences(gatherer_results, scryfall_results)
```

### Format-Specific Searches

```python
# Find cards legal in multiple formats
def find_format_staples():
    modern_cards = call_tool("scryfall_search_cards", {
        "query": "f:modern -f:standard",  # Modern but not Standard
        "order": "edhrec"
    })
    
    legacy_cards = call_tool("gatherer_search_cards", {
        "format": "Legal:Legacy,Banned:Modern"
    })
    
    return cross_reference_results(modern_cards, legacy_cards)
```

## Error Handling

Both tools provide structured error information:

```json
{
  "content": [
    {
      "type": "text",
      "text": "Error: No search parameters provided. Please provide either a 'query' or specific search parameters like 'name', 'card_type', etc."
    }
  ],
  "isError": true
}
```

## Performance Considerations

### Response Times
- **Gatherer**: 500-2000ms (depends on complexity)
- **Scryfall**: 200-800ms (generally faster)

### Rate Limiting
- Built-in rate limiting prevents API abuse
- Automatic retry with exponential backoff
- Caching for improved performance

### Best Practices
- Use specific parameters to reduce response size
- Implement pagination for large result sets
- Cache frequently accessed data
- Combine searches strategically

## API Comparison

| Feature              | Gatherer                    | Scryfall                     |
| -------------------- | --------------------------- | ---------------------------- |
| **Data Source**      | Official Wizards database  | Comprehensive third-party    |
| **Search Syntax**    | Parameter-based             | Flexible query language      |
| **Update Frequency** | Official release schedule   | Real-time updates            |
| **Image Quality**    | Official high-resolution    | Multiple sizes available     |
| **Price Data**       | Not available               | Multiple price sources       |
| **Rulings**          | Official rulings only       | Comprehensive rulings        |
| **Language Support** | Multiple languages          | Extensive language support   |
| **Format Legality**  | Official tournament data    | Real-time format tracking    |

## Advanced Usage Examples

### Deck Building Assistant

```json
// Find budget creatures for aggro deck
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:creature mana<=3 c:red f:standard usd<=1",
      "order": "edhrec"
    }
  }
}
```

### Meta Analysis

```json
// Research banned cards in Modern
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "format": "Banned:Modern",
      "card_type": "Instant,Sorcery"
    }
  }
}
```

### Collection Management

```json
// Find expensive cards from specific set
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "set": "mh3",
      "rarity": "mythic",
      "order": "usd"
    }
  }
}
```

### Educational Research

```json
// Study design evolution
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Creature",
      "power": "1",
      "toughness": "1",
      "mana_cost": "{W}",
      "page": 1
    }
  }
}
```

## Integration Examples

### Claude Desktop Configuration

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

### Custom Application Integration

```python
import json
import subprocess

def search_mtg_cards(search_params, api="scryfall"):
    tool_name = f"{api}_search_cards"
    
    # Prepare MCP request
    request = {
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": search_params
        }
    }
    
    # Call MCP server
    process = subprocess.Popen(
        ["mtg", "mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    response, error = process.communicate(json.dumps(request))
    return json.loads(response)

# Example usage
results = search_mtg_cards({
    "name": "Lightning Bolt",
    "format": "modern"
}, api="scryfall")
```

### Web Application Integration

```javascript
// Using SSE transport for web applications
const eventSource = new EventSource('http://localhost:3000/sse');

eventSource.onmessage = function(event) {
    const response = JSON.parse(event.data);
    if (response.method === 'tools/call') {
        handleCardSearchResults(response.result);
    }
};

function searchCards(params) {
    fetch('http://localhost:3000/tools/call', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            name: 'scryfall_search_cards',
            arguments: params
        })
    });
}
```

---

Next: [Prompts](./prompts.md) | Back: [Resources](./resources.md)