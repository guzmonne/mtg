# Usage Examples

Comprehensive examples demonstrating how to use the MTG MCP server's card search tools in various scenarios.

## Table of Contents

- [Direct Card Lookup](#direct-card-lookup)
- [Card Name Suggestions](#card-name-suggestions)
- [Random Card Discovery](#random-card-discovery)
- [Basic Card Searches](#basic-card-searches)
- [Advanced Filtering](#advanced-filtering)
- [Deck Building Scenarios](#deck-building-scenarios)
- [Format-Specific Searches](#format-specific-searches)
- [Educational Use Cases](#educational-use-cases)
- [Collection Management](#collection-management)
- [Competitive Analysis](#competitive-analysis)
- [Integration Examples](#integration-examples)

## Direct Card Lookup

### Get Card by Exact Name

**Scenario**: Get detailed information for a specific card

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_get_card_by_name",
    "arguments": {
      "name": "Lightning Bolt"
    }
  }
}
```

**Response**:
```
 Name              Lightning Bolt 
 Mana Cost         {R} 
 Mana Value        1 
 Type              Instant 
 Oracle Text       Lightning Bolt deals 3 damage to any target. 
 Set               Ravnica: Clue Edition (CLU) 
 Rarity            uncommon 
 Artist            Christopher Moeller 
 Collector Number  141 
 Legal In          modern, legacy, vintage, commander 
```

### Get Card by Scryfall ID

**Scenario**: Retrieve a card using its unique Scryfall identifier

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_get_card_by_id",
    "arguments": {
      "id": "5f70df6d-7e8d-4ba4-b425-b56c271f525c"
    }
  }
}
```

### Get Card by Set and Collector Number

**Scenario**: Get a specific printing from a particular set

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_get_card_by_collector",
    "arguments": {
      "set_code": "ktk",
      "collector_number": "150"
    }
  }
}
```

## Card Name Suggestions

### Autocomplete Card Names

**Scenario**: Help users find cards when they only remember part of the name

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_autocomplete_card_names",
    "arguments": {
      "query": "light"
    }
  }
}
```

**Response**:
```
Card name suggestions for 'light':

1. Lightform
2. Light 'Em Up
3. Light Up the Night
4. Light of Day
5. Light the Way
6. Lightning Bolt
7. Lightning Helix
...
Found 20 suggestions
```

### Include Extra Cards

**Scenario**: Get suggestions including tokens and emblems

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_autocomplete_card_names",
    "arguments": {
      "query": "angel",
      "include_extras": true
    }
  }
}
```

## Random Card Discovery

### Get Any Random Card

**Scenario**: Discover new cards for inspiration

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_get_random_card",
    "arguments": {}
  }
}
```

### Get Filtered Random Card

**Scenario**: Get a random card matching specific criteria

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_get_random_card",
    "arguments": {
      "query": "c:red t:creature mana>=4"
    }
  }
}
```

**Use Cases**:
- **Deck inspiration**: Find random cards in your colors
- **Challenge brewing**: Build around a random card
- **Learning**: Discover cards you've never seen
- **Cube design**: Find interesting inclusions

## Basic Card Searches

### Simple Name Search

**Scenario**: Find all printings of Lightning Bolt

**Gatherer Approach**:

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

**Scary fall Approach**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "name": "Lightning Bolt"
    }
  }
}
```

### Partial Name Matching

**Scenario**: Find all cards with "Lightning" in the name

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "name": "Lightning"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "Lightning"
    }
  }
}
```

### Search by Card Type

**Scenario**: Find all Planeswalkers

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Planeswalker"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "card_type": "planeswalker"
    }
  }
}
```

## Advanced Filtering

### Multi-Parameter Search

**Scenario**: Find red creatures with power 4 or greater

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Creature",
      "colors": "R",
      "power": "4-20"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:creature c:red pow>=4"
    }
  }
}
```

### Complex Type Filtering

**Scenario**: Find Legendary Artifact Creatures

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Artifact+Creature",
      "supertype": "Legendary"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:\"legendary artifact creature\""
    }
  }
}
```

### Mana Cost Filtering

**Scenario**: Find expensive spells (CMC 7+)

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Instant,Sorcery",
      "mana_cost": "{7}"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:instant OR t:sorcery mana>=7"
    }
  }
}
```

## Deck Building Scenarios

### Aggro Deck Cards

**Scenario**: Find cheap, aggressive creatures for a red aggro deck

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:creature c:red mana<=3 (o:haste OR pow>=2)",
      "format": "standard",
      "order": "cmc"
    }
  }
}
```

### Control Deck Tools

**Scenario**: Find card draw and removal for a blue-white control deck

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "c:wu (o:\"draw cards\" OR o:destroy OR o:exile) f:standard",
      "order": "cmc"
    }
  }
}
```

### Commander Deck Building

**Scenario**: Find cards for a 5-color Commander deck

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "identity": "wubrg",
      "mv": "<=4",
      "format": "commander",
      "order": "edhrec"
    }
  }
}
```

### Budget Deck Options

**Scenario**: Find budget-friendly cards under $1

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:standard usd<=1 (t:creature OR t:instant OR t:sorcery)",
      "order": "usd"
    }
  }
}
```

## Format-Specific Searches

### Standard Legal Cards

**Scenario**: Find current Standard-legal removal spells

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "format": "Legal:Standard",
      "rules": "destroy target creature"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:standard o:\"destroy target creature\"",
      "order": "cmc"
    }
  }
}
```

### Modern Staples

**Scenario**: Find popular Modern format cards

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:modern -f:standard",
      "order": "edhrec"
    }
  }
}
```

### Legacy Power Cards

**Scenario**: Find powerful Legacy-only cards

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "format": "Legal:Legacy,Banned:Modern"
    }
  }
}
```

### Pioneer Format Research

**Scenario**: Research Pioneer-legal cards from specific sets

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:pioneer s:rtr",
      "order": "name"
    }
  }
}
```

## Educational Use Cases

### Learning Card Types

**Scenario**: Explore different creature subtypes

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "card_type": "Creature",
      "subtype": "Dragon",
      "page": 1
    }
  }
}
```

### Understanding Mechanics

**Scenario**: Find cards with specific mechanics

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "o:flying o:vigilance",
      "order": "cmc"
    }
  }
}
```

### Set Analysis

**Scenario**: Study cards from a specific set

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "set": "Khans of Tarkir",
      "rarity": "Rare"
    }
  }
}
```

### Power Level Comparison

**Scenario**: Compare creatures with same mana cost

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:creature mana:3 c:red",
      "order": "power"
    }
  }
}
```

## Collection Management

### Inventory Tracking

**Scenario**: Find expensive cards from your collection sets

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "s:mh3 OR s:ltr OR s:woe",
      "rarity": "mythic",
      "order": "usd"
    }
  }
}
```

### Reprint Analysis

**Scenario**: Find cards that have been reprinted recently

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "is:reprint year>=2023",
      "order": "released"
    }
  }
}
```

### Artist Collections

**Scenario**: Find cards by favorite artists

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "artist": "Rebecca Guay"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "artist": "Rebecca Guay",
      "order": "released"
    }
  }
}
```

## Competitive Analysis

### Meta Game Research

**Scenario**: Find cards commonly played together

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:standard (o:\"whenever\" OR o:\"enters the battlefield\")",
      "order": "edhrec"
    }
  }
}
```

### Sideboard Options

**Scenario**: Find sideboard cards against specific strategies

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "f:modern (o:\"artifact\" AND o:\"destroy\") mana<=3",
      "order": "cmc"
    }
  }
}
```

### Banned Card Analysis

**Scenario**: Research why cards were banned

**Gatherer**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "gatherer_search_cards",
    "arguments": {
      "format": "Banned:Standard"
    }
  }
}
```

**Scryfall**:

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "banned:standard",
      "order": "released"
    }
  }
}
```

## Integration Examples

### Claude Desktop Conversation

**User**: "I'm building a red aggro deck for Standard. Can you help me find some good creatures?"

**Claude**: I'll help you find some excellent red creatures for Standard aggro! Let me search for efficient, aggressive creatures.

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:creature c:red f:standard mana<=3 (o:haste OR pow>=2)",
      "order": "cmc"
    }
  }
}
```

**User**: "What about some burn spells to go with them?"

**Claude**: Great choice! Let me find some efficient burn spells for your aggro deck.

```json
{
  "method": "tools/call",
  "params": {
    "name": "scryfall_search_cards",
    "arguments": {
      "query": "t:instant c:red f:standard mana<=3 o:damage",
      "order": "cmc"
    }
  }
}
```

### Python Integration

```python
import json
import subprocess

class MTGCardSearch:
    def __init__(self, mtg_binary_path="mtg"):
        self.mtg_path = mtg_binary_path
        self.process = None

    def start_server(self):
        """Start the MCP server process"""
        self.process = subprocess.Popen(
            [self.mtg_path, "mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # Initialize the server
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "python-client", "version": "1.0"}
            }
        }

        self.process.stdin.write(json.dumps(init_request) + "\n")
        self.process.stdin.flush()

        # Read initialization response
        response = self.process.stdout.readline()
        return json.loads(response)

    def search_gatherer(self, **params):
        """Search using Gatherer API"""
        request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "gatherer_search_cards",
                "arguments": params
            }
        }

        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()

        response = self.process.stdout.readline()
        return json.loads(response)

    def search_scryfall(self, **params):
        """Search using Scryfall API"""
        request = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "scryfall_search_cards",
                "arguments": params
            }
        }

        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()

        response = self.process.stdout.readline()
        return json.loads(response)

    def close(self):
        """Close the server process"""
        if self.process:
            self.process.terminate()

# Usage example
def find_deck_cards():
    searcher = MTGCardSearch()

    try:
        # Start the server
        init_response = searcher.start_server()
        print("Server initialized:", init_response["result"]["serverInfo"])

        # Find aggressive creatures
        creatures = searcher.search_scryfall(
            query="t:creature c:red f:standard mana<=3 pow>=2",
            order="cmc"
        )
        print("Found creatures:", creatures)

        # Find burn spells
        spells = searcher.search_gatherer(
            card_type="Instant",
            colors="R",
            rules="damage",
            format="Legal:Standard"
        )
        print("Found spells:", spells)

    finally:
        searcher.close()

if __name__ == "__main__":
    find_deck_cards()
```

### JavaScript/Node.js Integration

```javascript
const { spawn } = require("child_process");

class MTGCardSearch {
  constructor(mtgBinaryPath = "mtg") {
    this.mtgPath = mtgBinaryPath;
    this.process = null;
    this.requestId = 1;
  }

  async startServer() {
    return new Promise((resolve, reject) => {
      this.process = spawn(this.mtgPath, ["mcp"]);

      this.process.stdout.on("data", (data) => {
        const response = JSON.parse(data.toString());
        resolve(response);
      });

      this.process.stderr.on("data", (data) => {
        reject(new Error(data.toString()));
      });

      // Initialize server
      const initRequest = {
        jsonrpc: "2.0",
        id: this.requestId++,
        method: "initialize",
        params: {
          protocolVersion: "2024-11-05",
          capabilities: {},
          clientInfo: { name: "node-client", version: "1.0" },
        },
      };

      this.process.stdin.write(JSON.stringify(initRequest) + "\n");
    });
  }

  async searchScryfall(params) {
    return new Promise((resolve, reject) => {
      const request = {
        jsonrpc: "2.0",
        id: this.requestId++,
        method: "tools/call",
        params: {
          name: "scryfall_search_cards",
          arguments: params,
        },
      };

      this.process.stdout.once("data", (data) => {
        try {
          const response = JSON.parse(data.toString());
          resolve(response);
        } catch (error) {
          reject(error);
        }
      });

      this.process.stdin.write(JSON.stringify(request) + "\n");
    });
  }

  close() {
    if (this.process) {
      this.process.kill();
    }
  }
}

// Usage example
async function buildCommander() {
  const searcher = new MTGCardSearch();

  try {
    await searcher.startServer();
    console.log("Server started");

    // Find 5-color commanders
    const commanders = await searcher.searchScryfall({
      query: "t:legendary t:creature id:wubrg is:commander",
      order: "edhrec",
    });

    console.log("Found commanders:", commanders);
  } catch (error) {
    console.error("Error:", error);
  } finally {
    searcher.close();
  }
}

buildCommander();
```

### Web Application Integration

```html
<!DOCTYPE html>
<html>
  <head>
    <title>MTG Card Search</title>
  </head>
  <body>
    <div id="app">
      <h1>MTG Card Search</h1>

      <form id="search-form">
        <input type="text" id="card-name" placeholder="Card name" />
        <select id="search-api">
          <option value="scryfall">Scryfall</option>
          <option value="gatherer">Gatherer</option>
        </select>
        <button type="submit">Search</button>
      </form>

      <div id="results"></div>
    </div>

    <script>
      // Using SSE transport for web integration
      const eventSource = new EventSource("http://localhost:3000/sse");

      eventSource.onmessage = function (event) {
        const response = JSON.parse(event.data);
        if (response.method === "tools/call") {
          displayResults(response.result);
        }
      };

      document
        .getElementById("search-form")
        .addEventListener("submit", function (e) {
          e.preventDefault();

          const cardName = document.getElementById("card-name").value;
          const api = document.getElementById("search-api").value;

          const toolName =
            api === "scryfall"
              ? "scryfall_search_cards"
              : "gatherer_search_cards";

          fetch("http://localhost:3000/tools/call", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              name: toolName,
              arguments: { name: cardName },
            }),
          });
        });

      function displayResults(results) {
        const resultsDiv = document.getElementById("results");
        resultsDiv.innerHTML =
          "<h2>Search Results</h2><pre>" +
          JSON.stringify(results, null, 2) +
          "</pre>";
      }
    </script>
  </body>
</html>
```

## Best Practices

### Efficient Searching

1. **Use Specific Parameters**: More specific searches return faster results
2. **Leverage Pagination**: Use `page` parameter for large result sets
3. **Choose the Right API**: Gatherer for official data, Scryfall for flexibility
4. **Cache Results**: Store frequently accessed data locally

### Error Handling

```python
def safe_search(searcher, search_params):
    try:
        result = searcher.search_scryfall(**search_params)
        if 'error' in result:
            print(f"Search error: {result['error']}")
            return None
        return result
    except Exception as e:
        print(f"Connection error: {e}")
        return None
```

### Performance Optimization

1. **Batch Requests**: Group related searches together
2. **Use Appropriate Timeouts**: Set reasonable timeout values
3. **Monitor Response Times**: Track search performance
4. **Implement Retry Logic**: Handle temporary failures gracefully

---

Next: [Prompts](./prompts.md) | Back: [Tools](./tools.md)
