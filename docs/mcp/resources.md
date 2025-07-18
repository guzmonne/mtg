# MCP Resources

Resources provide AI assistants with read-only access to Magic: The Gathering data through standardized URIs.

## Available Resources

The MTG MCP server provides three main resources:

| Resource | URI | Description |
|----------|-----|-------------|
| **Cards** | `mtg://cards` | Complete card database with 20,000+ cards |
| **Sets** | `mtg://sets` | All Magic sets from Alpha to present |
| **Types** | `mtg://types` | Card types, subtypes, supertypes, and formats |

## Cards Resource

### URI: `mtg://cards`

Provides access to the complete Magic: The Gathering card database.

### Data Structure

```json
{
  "cards": [
    {
      "id": "409574",
      "name": "Lightning Bolt",
      "manaCost": "{R}",
      "cmc": 1,
      "colors": ["Red"],
      "colorIdentity": ["R"],
      "type": "Instant",
      "types": ["Instant"],
      "rarity": "Common",
      "set": "2ED",
      "setName": "Unlimited Edition",
      "text": "Lightning Bolt deals 3 damage to any target.",
      "artist": "Christopher Rush",
      "number": "161",
      "power": null,
      "toughness": null,
      "loyalty": null,
      "multiverseid": 1234,
      "variations": ["409575", "409576"],
      "imageUrl": "http://gatherer.wizards.com/Handlers/Image.ashx?multiverseid=1234&type=card",
      "watermark": null,
      "border": "black",
      "timeshifted": false,
      "hand": null,
      "life": null,
      "reserved": false,
      "releaseDate": "1993-12-01",
      "starter": true,
      "rulings": [
        {
          "date": "2004-10-04",
          "text": "The damage is dealt to the target when Lightning Bolt resolves."
        }
      ],
      "foreignNames": [
        {
          "name": "Blitz",
          "text": "Der Blitz fügt einem Ziel deiner Wahl 3 Schadenspunkte zu.",
          "type": "Spontanzauber",
          "flavor": null,
          "imageUrl": "http://gatherer.wizards.com/Handlers/Image.ashx?multiverseid=5678&type=card",
          "language": "German",
          "multiverseid": 5678
        }
      ],
      "printings": ["LEA", "LEB", "2ED", "3ED", "4ED"],
      "originalText": "Lightning Bolt deals 3 damage to target creature or player.",
      "originalType": "Instant",
      "legalities": [
        {
          "format": "Standard",
          "legality": "Not Legal"
        },
        {
          "format": "Modern",
          "legality": "Legal"
        },
        {
          "format": "Legacy",
          "legality": "Legal"
        },
        {
          "format": "Vintage",
          "legality": "Legal"
        },
        {
          "format": "Commander",
          "legality": "Legal"
        }
      ]
    }
  ]
}
```

### Key Fields

- **id**: Unique card identifier
- **name**: Card name
- **manaCost**: Mana cost in {X} notation
- **cmc**: Converted mana cost
- **colors**: Array of card colors
- **type**: Full type line
- **rarity**: Card rarity
- **set**: Set code
- **text**: Rules text
- **legalities**: Format legality information

### Usage Examples

AI assistants can access this resource to:
- Get comprehensive card information
- Analyze card databases
- Research Magic history
- Build card databases

## Sets Resource

### URI: `mtg://sets`

Provides information about all Magic: The Gathering sets.

### Data Structure

```json
{
  "sets": [
    {
      "code": "KTK",
      "name": "Khans of Tarkir",
      "type": "expansion",
      "border": "black",
      "mkm_id": 1234,
      "mkm_name": "Khans of Tarkir",
      "releaseDate": "2014-09-26",
      "block": "Khans of Tarkir",
      "onlineOnly": false,
      "booster": [
        "rare",
        "uncommon",
        "uncommon",
        "uncommon",
        "common",
        "common",
        "common",
        "common",
        "common",
        "common",
        "common",
        "common",
        "common",
        "common",
        "land"
      ],
      "cards": [
        {
          "id": "386616",
          "name": "Abzan Ascendancy",
          "manaCost": "{W}{B}{G}",
          "cmc": 3,
          "colors": ["White", "Black", "Green"],
          "type": "Enchantment",
          "rarity": "Rare",
          "set": "KTK",
          "text": "When Abzan Ascendancy enters the battlefield, put a +1/+1 counter on each creature you control.\nWhenever a nontoken creature you control dies, create a 1/1 white Spirit creature token with flying."
        }
      ]
    }
  ]
}
```

### Key Fields

- **code**: Three-letter set code
- **name**: Full set name
- **type**: Set type (expansion, core, masters, etc.)
- **releaseDate**: Release date
- **block**: Block name (if applicable)
- **booster**: Booster pack composition
- **cards**: Array of cards in the set

### Usage Examples

AI assistants can use this resource to:
- Browse Magic sets chronologically
- Understand set mechanics and themes
- Generate booster pack simulations
- Research Magic history and design

## Types Resource

### URI: `mtg://types`

Provides comprehensive information about the Magic type system.

### Data Structure

```json
{
  "types": [
    "Artifact",
    "Battle",
    "Creature",
    "Enchantment",
    "Instant",
    "Land",
    "Planeswalker",
    "Sorcery",
    "Tribal"
  ],
  "subtypes": [
    "Advisor",
    "Angel",
    "Artifact",
    "Aura",
    "Beast",
    "Bird",
    "Cleric",
    "Dragon",
    "Elf",
    "Equipment",
    "Forest",
    "Goblin",
    "Human",
    "Island",
    "Knight",
    "Mountain",
    "Plains",
    "Swamp",
    "Warrior",
    "Wizard"
  ],
  "supertypes": [
    "Basic",
    "Legendary",
    "Snow",
    "World"
  ],
  "formats": [
    "Standard",
    "Pioneer",
    "Modern",
    "Legacy",
    "Vintage",
    "Commander",
    "Brawl",
    "Pauper",
    "Penny Dreadful"
  ]
}
```

### Categories

#### Types
Primary card types that determine when and how cards can be played:
- **Artifact**: Non-creature permanents
- **Creature**: Permanents that can attack and block
- **Enchantment**: Permanent magical effects
- **Instant**: Immediate-effect spells
- **Land**: Mana-producing permanents
- **Planeswalker**: Powerful ally permanents
- **Sorcery**: Main-phase-only spells

#### Subtypes
More specific classifications within each type:
- **Creature Subtypes**: Races (Human, Elf) and Classes (Warrior, Wizard)
- **Land Subtypes**: Basic types (Plains, Island) and special types (Gate, Desert)
- **Artifact Subtypes**: Equipment, Vehicle, Treasure
- **Planeswalker Subtypes**: Character names (Jace, Chandra)

#### Supertypes
Special designations that modify card behavior:
- **Basic**: Basic lands
- **Legendary**: Unique permanents (legend rule applies)
- **Snow**: Snow permanents
- **World**: Old supertype for unique enchantments

#### Formats
Official Magic formats where cards can be legal:
- **Standard**: Most recent sets
- **Modern**: Sets from Eighth Edition forward
- **Legacy**: All sets with restricted list
- **Commander**: 100-card singleton multiplayer

### Usage Examples

AI assistants can use this resource to:
- Explain Magic's type system
- Help with deck building constraints
- Understand format legality
- Research tribal strategies

## Resource Access Patterns

### Reading Resources

AI assistants access resources using the MCP protocol:

```json
{
  "method": "resources/read",
  "params": {
    "uri": "mtg://cards"
  }
}
```

### Caching Behavior

- **Cards Resource**: Cached for 15 minutes
- **Sets Resource**: Cached for 1 hour
- **Types Resource**: Cached for 24 hours

### Data Freshness

Resources are updated based on:
- **Cards**: New card releases and errata
- **Sets**: New set announcements
- **Types**: Rules updates and new mechanics

## Performance Characteristics

### Response Sizes

| Resource | Typical Size | Max Size |
|----------|--------------|----------|
| Cards | 50-100 MB | 200 MB |
| Sets | 10-20 MB | 50 MB |
| Types | 1-5 KB | 10 KB |

### Response Times

- **First Request**: 500-2000ms (API fetch)
- **Cached Request**: 10-50ms (memory)
- **Timeout**: 30 seconds default

## Best Practices

### Efficient Usage

1. **Cache Awareness**: Resources are cached, so repeated access is fast
2. **Selective Processing**: Process only needed data from large resources
3. **Error Handling**: Handle network timeouts gracefully
4. **Rate Limiting**: Respect API rate limits for fresh data

### Data Processing

```python
# Example: Processing cards resource
import json

def process_cards_resource(resource_data):
    cards = json.loads(resource_data)["cards"]
    
    # Filter for specific criteria
    red_creatures = [
        card for card in cards 
        if "Red" in card.get("colors", []) 
        and "Creature" in card.get("types", [])
    ]
    
    return red_creatures
```

### Error Scenarios

Resources may fail due to:
- **Network Issues**: MTG API unavailable
- **Rate Limiting**: Too many requests
- **Data Corruption**: Invalid JSON response
- **Timeout**: Request takes too long

Handle gracefully:
```json
{
  "error": {
    "code": -32000,
    "message": "Failed to fetch cards data: Connection timeout"
  }
}
```

## Integration Examples

### Card Analysis

```python
# AI Assistant using cards resource
def analyze_card_power_level(card_name):
    # Read cards resource
    cards_data = read_resource("mtg://cards")
    
    # Find specific card
    card = find_card_by_name(cards_data, card_name)
    
    # Analyze power level based on:
    # - Mana cost efficiency
    # - Format legality
    # - Historical context
    
    return analysis
```

### Set Research

```python
# AI Assistant using sets resource
def research_set_mechanics(set_code):
    # Read sets resource
    sets_data = read_resource("mtg://sets")
    
    # Find specific set
    set_info = find_set_by_code(sets_data, set_code)
    
    # Analyze mechanics and themes
    return set_analysis
```

### Format Guidance

```python
# AI Assistant using types resource
def explain_format_legality():
    # Read types resource
    types_data = read_resource("mtg://types")
    
    # Extract format information
    formats = types_data["formats"]
    
    # Provide format explanations
    return format_guide
```

---

Next: [Tools](tools.md) | Back: [Setup & Installation](setup.md)