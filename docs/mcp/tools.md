# MCP Tools

Tools provide AI assistants with interactive functions to search, analyze, and manipulate Magic: The Gathering data.

## Available Tools

The MTG MCP server provides five powerful tools:

| Tool                 | Purpose                       | Parameters                                  |
| -------------------- | ----------------------------- | ------------------------------------------- |
| **search_cards**     | Advanced card search          | name, colors, type, rarity, set, cmc, limit |
| **get_card**         | Detailed card information     | id                                          |
| **list_sets**        | Set browsing and filtering    | name, block, limit                          |
| **generate_booster** | Virtual booster pack creation | set_code                                    |
| **get_card_types**   | Type system queries           | category                                    |

## search_cards

Advanced card search with multiple filtering options.

### Parameters

```json
{
  "name": "Lightning", // Card name (partial matching)
  "colors": "Red,Blue", // Colors (comma=AND, pipe=OR)
  "type": "Creature", // Card type
  "rarity": "Rare", // Card rarity
  "set": "KTK", // Set code
  "cmc": 3, // Converted mana cost
  "limit": 20 // Maximum results (default: 20)
}
```

### Examples

#### Basic Name Search

```json
{
  "method": "tools/call",
  "params": {
    "name": "search_cards",
    "arguments": {
      "name": "Lightning Bolt"
    }
  }
}
```

Response:

```
Found 25 cards matching 'Lightning Bolt':

**Lightning Bolt** (2ED)
- Type: Instant
- Rarity: Common
- Mana Cost: {R}
- Set: Unlimited Edition

**Lightning Bolt** (3ED)
- Type: Instant
- Rarity: Common
- Mana Cost: {R}
- Set: Revised Edition

...
```

#### Advanced Filtering

```json
{
  "method": "tools/call",
  "params": {
    "name": "search_cards",
    "arguments": {
      "colors": "Red,Blue",
      "type": "Creature",
      "cmc": 4,
      "limit": 10
    }
  }
}
```

#### Color Logic

```json
// Cards that are both Red AND Blue
{
  "colors": "Red,Blue"
}

// Cards that are Red OR Blue OR Green
{
  "colors": "Red|Blue|Green"
}

// Colorless cards
{
  "colors": "Colorless"
}
```

### Use Cases

- **Deck Building**: Find cards matching specific criteria
- **Collection Management**: Search for cards by various attributes
- **Meta Analysis**: Research cards in specific formats
- **Educational**: Explore Magic's card design space

## get_card

Retrieve detailed information about a specific card.

### Parameters

```json
{
  "id": "409574" // Card ID or multiverse ID
}
```

### Example

```json
{
  "method": "tools/call",
  "params": {
    "name": "get_card",
    "arguments": {
      "id": "409574"
    }
  }
}
```

Response:

```
=== Lightning Bolt ===

Name: Lightning Bolt
Mana Cost: {R}
CMC: 1
Type: Instant
Rarity: Common
Set: Masters 25 (A25)
Artist: Christopher Rush

Text: Lightning Bolt deals 3 damage to any target.

Flavor: The sparkmage shrieked, calling on the rage of the storms of his youth.
To his surprise, the sky responded with a fierce energy he'd never tasted before.

Legality:
  Standard: Not Legal
  Modern: Legal
  Legacy: Legal
  Vintage: Legal
  Commander: Legal

Printings: LEA, LEB, 2ED, 3ED, 4ED, 5ED, 6ED, 7ED, 8ED, 9ED, 10E, M10, M11, PRM, A25

Rulings:
  2004-10-04: The damage is dealt to the target when Lightning Bolt resolves.
```

### Use Cases

- **Card Analysis**: Deep dive into specific cards
- **Rules Questions**: Get official rulings and text
- **Collection Tracking**: Detailed card information
- **Educational**: Learn about individual cards

## list_sets

Browse and filter Magic: The Gathering sets.

### Parameters

```json
{
  "name": "Zendikar", // Set name filter (partial matching)
  "block": "Innistrad", // Block name filter
  "limit": 20 // Maximum results (default: 20)
}
```

### Examples

#### List Recent Sets

```json
{
  "method": "tools/call",
  "params": {
    "name": "list_sets",
    "arguments": {
      "limit": 10
    }
  }
}
```

#### Filter by Name

```json
{
  "method": "tools/call",
  "params": {
    "name": "list_sets",
    "arguments": {
      "name": "Ravnica",
      "limit": 5
    }
  }
}
```

Response:

```
Found 5 sets matching 'Ravnica':

**Ravnica: City of Guilds** (RAV)
- Type: expansion
- Block: Ravnica
- Release Date: 2005-10-07

**Guildpact** (GPT)
- Type: expansion
- Block: Ravnica
- Release Date: 2006-02-03

**Dissension** (DIS)
- Type: expansion
- Block: Ravnica
- Release Date: 2006-05-05

**Return to Ravnica** (RTR)
- Type: expansion
- Block: Return to Ravnica
- Release Date: 2012-10-05

**Gatecrash** (GTC)
- Type: expansion
- Block: Return to Ravnica
- Release Date: 2013-02-01
```

#### Filter by Block

```json
{
  "method": "tools/call",
  "params": {
    "name": "list_sets",
    "arguments": {
      "block": "Innistrad"
    }
  }
}
```

### Use Cases

- **Set Research**: Explore Magic's history
- **Draft Preparation**: Research sets for limited play
- **Collection Planning**: Understand set releases
- **Meta Analysis**: Study format evolution

## generate_booster

Generate virtual booster packs from specific sets.

### Parameters

```json
{
  "set_code": "KTK" // Three-letter set code
}
```

### Example

```json
{
  "method": "tools/call",
  "params": {
    "name": "generate_booster",
    "arguments": {
      "set_code": "KTK"
    }
  }
}
```

Response:

```
**Booster Pack for KTK**

=== Mythic Rare ===
• Sorin, Solemn Visitor

=== Rare ===
• Siege Rhino

=== Uncommons ===
• Abzan Charm
• Jeskai Charm
• Mardu Charm

=== Commons ===
• Monastery Swiftspear
• Ponyback Brigade
• Dismal Backwater
• Jungle Hollow
• Scoured Barrens
• Swiftwater Cliffs
• Thornwood Falls
• Wind-Scarred Crag
• Blossoming Sands
• Bloodfell Caves

Total: 15 cards (1 Mythic, 1 Rare, 3 Uncommons, 10 Commons)
```

### Popular Set Codes

- **KTK** - Khans of Tarkir
- **RTR** - Return to Ravnica
- **ISD** - Innistrad
- **ZEN** - Zendikar
- **A25** - Masters 25
- **MH3** - Modern Horizons 3

### Use Cases

- **Draft Simulation**: Practice limited formats
- **Pack Opening**: Virtual pack opening experience
- **Set Analysis**: Understand rarity distributions
- **Educational**: Learn about booster compositions

## get_card_types

Query the Magic type system for types, subtypes, supertypes, and formats.

### Parameters

```json
{
  "category": "types" // "types", "subtypes", "supertypes", or "formats"
}
```

### Examples

#### Get All Card Types

```json
{
  "method": "tools/call",
  "params": {
    "name": "get_card_types",
    "arguments": {
      "category": "types"
    }
  }
}
```

Response:

```
=== Card Types ===

Primary Types:
• Artifact - Non-creature permanents with various effects
• Battle - Siege-like permanents from March of the Machine
• Creature - Permanents that can attack and block
• Enchantment - Permanent magical effects
• Instant - Spells with immediate effects
• Land - Mana-producing permanents
• Planeswalker - Powerful ally permanents
• Sorcery - Main-phase-only spells
• Tribal - Type-matters spells

Total: 9 types
```

#### Get Subtypes

```json
{
  "method": "tools/call",
  "params": {
    "name": "get_card_types",
    "arguments": {
      "category": "subtypes"
    }
  }
}
```

#### Get Formats

```json
{
  "method": "tools/call",
  "params": {
    "name": "get_card_types",
    "arguments": {
      "category": "formats"
    }
  }
}
```

Response:

```
=== Game Formats ===

Constructed Formats:
• Standard - Most recent sets (rotates every ~2 years)
• Pioneer - Sets from Return to Ravnica forward
• Modern - Sets from Eighth Edition forward
• Legacy - All sets with restricted list
• Vintage - All sets with power restrictions

Limited Formats:
• Draft - Booster draft format
• Sealed - Sealed deck format

Casual Formats:
• Commander - 100-card singleton multiplayer
• Brawl - Standard-legal Commander variant
• Pauper - Commons only

Total: 9 formats
```

### Use Cases

- **Rules Education**: Learn Magic's type system
- **Deck Building**: Understand format constraints
- **Tribal Strategies**: Research creature types
- **Format Analysis**: Compare format characteristics

## Tool Integration Patterns

### Chaining Tools

AI assistants can chain tools for complex workflows:

```python
# 1. Search for cards
search_result = call_tool("search_cards", {
    "name": "Lightning",
    "limit": 5
})

# 2. Get detailed info for first result
card_id = extract_card_id(search_result)
card_details = call_tool("get_card", {"id": card_id})

# 3. Generate booster from same set
set_code = extract_set_code(card_details)
booster = call_tool("generate_booster", {"set_code": set_code})
```

### Error Handling

Tools return structured error information:

```json
{
  "content": [
    {
      "type": "text",
      "text": "Card with ID '999999' not found."
    }
  ],
  "isError": true
}
```

### Performance Optimization

- **Caching**: Tool results are not cached (dynamic data)
- **Rate Limiting**: Built-in rate limiting prevents API abuse
- **Timeouts**: 30-second default timeout per tool call
- **Pagination**: Use `limit` parameter to control response size

## Tool Comparison

| Tool             | Response Time | Cache | Use Case           |
| ---------------- | ------------- | ----- | ------------------ |
| search_cards     | 200-800ms     | No    | Interactive search |
| get_card         | 100-400ms     | No    | Detailed analysis  |
| list_sets        | 200-600ms     | No    | Set browsing       |
| generate_booster | 300-1000ms    | No    | Pack simulation    |
| get_card_types   | 50-200ms      | Yes   | Reference data     |

## Advanced Usage

### Complex Searches

```json
// Find expensive red creatures from recent sets
{
  "name": "search_cards",
  "arguments": {
    "colors": "Red",
    "type": "Creature",
    "cmc": 6,
    "rarity": "Mythic Rare",
    "limit": 10
  }
}
```

### Batch Operations

```python
# Generate multiple boosters
boosters = []
for i in range(3):
    booster = call_tool("generate_booster", {"set_code": "KTK"})
    boosters.append(booster)
```

### Data Analysis

```python
# Analyze set composition
sets = call_tool("list_sets", {"limit": 100})
set_analysis = analyze_set_trends(sets)

# Research card power level
cards = call_tool("search_cards", {
    "type": "Creature",
    "cmc": 4,
    "limit": 50
})
power_analysis = analyze_power_level(cards)
```

## Integration Examples

### Deck Building Assistant

```python
def suggest_cards_for_deck(deck_theme, format):
    # Search for cards matching theme
    cards = call_tool("search_cards", {
        "name": deck_theme,
        "limit": 20
    })

    # Filter by format legality
    legal_cards = filter_by_format(cards, format)

    # Get detailed info for top suggestions
    suggestions = []
    for card in legal_cards[:5]:
        details = call_tool("get_card", {"id": card["id"]})
        suggestions.append(details)

    return suggestions
```

### Set Analysis Tool

```python
def analyze_set_mechanics(set_code):
    # Generate multiple boosters
    boosters = []
    for i in range(10):
        booster = call_tool("generate_booster", {"set_code": set_code})
        boosters.append(booster)

    # Analyze rarity distribution
    rarity_stats = calculate_rarity_distribution(boosters)

    # Get set information
    sets = call_tool("list_sets", {"name": set_code})
    set_info = find_set_by_code(sets, set_code)

    return {
        "set_info": set_info,
        "rarity_stats": rarity_stats,
        "sample_boosters": boosters[:3]
    }
```

### Card Comparison Tool

```python
def compare_cards(card_names):
    comparisons = []

    for name in card_names:
        # Search for card
        search_result = call_tool("search_cards", {
            "name": name,
            "limit": 1
        })

        if search_result:
            # Get detailed info
            card_id = extract_card_id(search_result)
            details = call_tool("get_card", {"id": card_id})
            comparisons.append(details)

    return analyze_card_comparison(comparisons)
```

---

Next: [Prompts](./prompts.md) | Back: [Resources](./resources.md)
