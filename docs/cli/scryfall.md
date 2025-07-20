# Scryfall Commands

The `mtg scryfall` command provides comprehensive access to Scryfall's powerful search engine and Magic: The Gathering card database with multiple lookup methods and advanced search capabilities.

## Available Commands

### Card Lookup Methods
- `named <NAME>` - Get specific card by exact name
- `id <UUID>` - Get card by Scryfall UUID
- `collector <SET> <NUMBER>` - Get card by set code and collector number
- `arena <ID>` - Get card by Arena ID
- `mtgo <ID>` - Get card by MTGO ID
- `multiverse <ID>` - Get card by Multiverse ID
- `tcgplayer <ID>` - Get card by TCGPlayer ID
- `cardmarket <ID>` - Get card by Cardmarket ID
- `random` - Get a random card (optionally filtered)

### Search Methods
- `search <QUERY>` - Search cards using Scryfall syntax
- `advanced` - Advanced search with individual filter options
- `autocomplete <QUERY>` - Get card name suggestions

## Card Lookup Methods

### Get Card by Name

Retrieve detailed information about a specific card by its exact name:

```bash
# Get card by exact name
mtg scryfall named "Lightning Bolt" --pretty

# Get specific printing from a set
mtg scryfall named "Lightning Bolt" --set "lea" --pretty

# Get latest printing
mtg scryfall named "Jace, the Mind Sculptor" --pretty
```

### Get Card by ID

Lookup cards using various ID systems:

```bash
# Scryfall UUID
mtg scryfall id "56ebc372-aabd-4174-a943-c7bf59e5028d" --pretty

# Arena ID
mtg scryfall arena 67330 --pretty

# MTGO ID
mtg scryfall mtgo 12345 --pretty

# Multiverse ID
mtg scryfall multiverse 456789 --pretty

# TCGPlayer ID
mtg scryfall tcgplayer 98765 --pretty

# Cardmarket ID
mtg scryfall cardmarket 54321 --pretty
```

### Get Card by Set and Collector Number

Retrieve specific printings using set code and collector number:

```bash
# Basic lookup
mtg scryfall collector ktk 96 --pretty

# With language specification
mtg scryfall collector dom 1 --lang ja --pretty

# Special collector numbers (with symbols)
mtg scryfall collector war "★1" --pretty
```

### Get Random Card

Retrieve random cards with optional filtering:

```bash
# Completely random card
mtg scryfall random --pretty

# Random card matching criteria
mtg scryfall random --query "c:red t:creature" --pretty

# Random legendary creature
mtg scryfall random --query "t:legendary t:creature" --pretty
```

### Autocomplete

Get card name suggestions for partial queries:

```bash
# Get suggestions for "lightning"
mtg scryfall autocomplete "lightning"

# Include extra cards in suggestions
mtg scryfall autocomplete "token" --include-extras
```

## Search Methods

### Basic Search

Search for cards using Scryfall's powerful query syntax:

```bash
# Find all Lightning Bolt cards
mtg scryfall search "Lightning Bolt" --pretty

# Find red creatures
mtg scryfall search "c:red t:creature" --pretty

# Find cards with "draw" in their text
mtg scryfall search "o:draw" --pretty
```

### Enhanced Search Options

```bash
# Pretty table output
mtg scryfall search "c:blue t:instant" --pretty

# Pagination
mtg scryfall search "t:creature" --page 2 --pretty

# Enhanced sort options
mtg scryfall search "t:planeswalker" --order cmc --pretty
mtg scryfall search "c:red" --order power --dir desc --pretty
mtg scryfall search "t:creature" --order edhrec --pretty
mtg scryfall search "r:rare" --order usd --pretty

# Include extra cards (tokens, emblems, etc.)
mtg scryfall search "t:token" --include-extras --pretty

# Include multilingual cards
mtg scryfall search "Lightning Bolt" --include-multilingual --pretty

# Show different printings
mtg scryfall search "Lightning Bolt" --unique prints --pretty

# Export as CSV
mtg scryfall search "c:blue t:instant" --csv > blue_instants.csv
```

### Advanced Search

Use individual filter options for more precise searches with enhanced control:

#### Basic Filters

```bash
# Search by name (partial matching)
mtg scryfall advanced --name "Lightning" --pretty

# Search by oracle text
mtg scryfall advanced --oracle "draw a card" --pretty

# Search by type
mtg scryfall advanced --card-type "creature" --pretty
```

#### Color Filters

```bash
# Single color
mtg scryfall advanced --colors "r" --pretty

# Multiple colors (exact)
mtg scryfall advanced --colors "wu" --pretty

# Color identity for commander
mtg scryfall advanced --identity "bant" --pretty
```

#### Mana and Stats

```bash
# Mana cost
mtg scryfall advanced --mana "{2}{U}" --pretty

# Mana value/CMC
mtg scryfall advanced --mv "3" --pretty
mtg scryfall advanced --mv ">=4" --pretty

# Power and toughness
mtg scryfall advanced --power ">=3" --pretty
mtg scryfall advanced --toughness "2" --pretty

# Loyalty
mtg scryfall advanced --loyalty "4" --pretty
```

#### Set and Rarity

```bash
# Specific set
mtg scryfall advanced --set "ktk" --pretty

# Rarity
mtg scryfall advanced --rarity "mythic" --pretty

# Combine filters
mtg scryfall advanced --set "war" --rarity "rare" --colors "u" --pretty
```

#### Other Filters

```bash
# Artist
mtg scryfall advanced --artist "Rebecca Guay" --pretty

# Flavor text
mtg scryfall advanced --flavor "Jace" --pretty

# Format legality
mtg scryfall advanced --format "standard" --pretty

# Language
mtg scryfall advanced --language "ja" --pretty
```

#### Enhanced Advanced Search Options

```bash
# All search enhancement options available
mtg scryfall advanced --name "Dragon" --colors "r" \
  --order cmc --dir asc --include-extras \
  --unique prints --pretty

# Pagination with advanced search
mtg scryfall advanced --card-type "planeswalker" \
  --page 2 --order released --pretty
```

## ID-Based Lookup Examples

### Real-World Examples

```bash
# Look up a specific card from Khans of Tarkir
mtg scryfall collector ktk 96 --pretty
# Returns: Ainok Tracker

# Get the latest Lightning Bolt printing
mtg scryfall named "Lightning Bolt" --pretty

# Look up a card by its Scryfall ID (from API responses)
mtg scryfall id "56ebc372-aabd-4174-a943-c7bf59e5028d" --pretty
# Returns: Phantom Nishoba

# Find a card by Arena ID (useful for Arena players)
mtg scryfall arena 67330 --pretty

# Get Japanese version of a card
mtg scryfall collector dom 1 --lang ja --pretty
```

### When to Use Each Method

- **`named`** - When you know the exact card name
- **`id`** - When you have a Scryfall UUID from API responses
- **`collector`** - When you want a specific printing from a set
- **`arena`** - When working with Arena deck lists or IDs
- **`mtgo`** - When working with MTGO collection data
- **`multiverse`** - When working with Gatherer or older systems
- **`tcgplayer`** - When integrating with TCGPlayer data
- **`cardmarket`** - When integrating with Cardmarket data
- **`random`** - For discovery or testing purposes

## Scryfall Search Syntax

Scryfall uses a powerful query syntax. Here are the most common keywords:

### Colors and Color Identity

```bash
# Colors
mtg scryfall search "c:red"           # Red cards
mtg scryfall search "c:wu"            # White and blue cards
mtg scryfall search "c:colorless"     # Colorless cards
mtg scryfall search "c>=bant"         # At least Bant colors

# Color identity (for Commander)
mtg scryfall search "id:esper"        # Esper identity
mtg scryfall search "id<=wu"          # At most white and blue
```

### Card Types

```bash
mtg scryfall search "t:creature"      # Creatures
mtg scryfall search "t:instant"       # Instants
mtg scryfall search "t:legendary"     # Legendary cards
mtg scryfall search "t:artifact t:creature"  # Artifact creatures
```

### Card Text

```bash
mtg scryfall search "o:flying"        # Cards with flying
mtg scryfall search "o:\"draw a card\""  # Exact phrase
mtg scryfall search "o:~ enters"      # Use ~ for card name
```

### Mana Costs and Values

```bash
mtg scryfall search "m:{2}{U}"        # Specific mana cost
mtg scryfall search "mv=3"            # Mana value 3
mtg scryfall search "mv>=5"           # Mana value 5 or more
mtg scryfall search "mv:even"         # Even mana values
```

### Power, Toughness, and Loyalty

```bash
mtg scryfall search "pow>=8"          # Power 8 or more
mtg scryfall search "tou=1"           # Toughness 1
mtg scryfall search "pow>tou"         # Power greater than toughness
mtg scryfall search "loy=3"           # Loyalty 3
```

### Sets and Rarity

```bash
mtg scryfall search "s:ktk"           # Khans of Tarkir
mtg scryfall search "r:mythic"        # Mythic rare
mtg scryfall search "r>=rare"         # Rare or mythic
```

### Format Legality

```bash
mtg scryfall search "f:standard"      # Legal in Standard
mtg scryfall search "f:modern"        # Legal in Modern
mtg scryfall search "banned:legacy"   # Banned in Legacy
```

### Artist and Flavor

```bash
mtg scryfall search "a:\"Rebecca Guay\""  # Artist
mtg scryfall search "ft:Jace"         # Flavor text mentions Jace
```

### Special Properties

```bash
mtg scryfall search "is:split"        # Split cards
mtg scryfall search "is:transform"    # Transform cards
mtg scryfall search "is:commander"    # Can be commander
mtg scryfall search "is:reserved"     # Reserved list
```

## Advanced Syntax Examples

### Complex Queries

```bash
# Cheap red creatures in Standard
mtg scryfall search "c:red t:creature mv<=3 f:standard"

# Expensive artifacts that tap for mana
mtg scryfall search "t:artifact o:\"add\" o:\"mana\" mv>=4"

# Legendary creatures that can be commanders
mtg scryfall search "t:legendary t:creature is:commander"

# Cards with X in their mana cost
mtg scryfall search "m:x"

# Multicolor instants and sorceries
mtg scryfall search "c:m (t:instant or t:sorcery)"
```

### Using OR and Parentheses

```bash
# Multiple options
mtg scryfall search "t:goblin or t:elf"

# Grouped conditions
mtg scryfall search "c:red (t:instant or t:sorcery)"

# Complex combinations
mtg scryfall search "(c:red or c:blue) t:creature mv<=2"
```

### Negation

```bash
# Not red
mtg scryfall search "-c:red t:creature"

# Not reprints
mtg scryfall search "not:reprint s:war"

# Exclude specific types
mtg scryfall search "c:blue -t:creature -t:land"
```

## Output Examples

### Search Results (Pretty Format)

```
 Name                          Cost          Type                                Set                                          Rarity    P/T/L 
 Arc Lightning                 {2}{R}        Sorcery                             Khans of Tarkir                              uncommon  - 
 Ball Lightning                {R}{R}{R}     Creature — Elemental                Foundations                                  rare      6/1 
 Burst Lightning               {R}           Instant                             Foundations                                  common    - 
 Chain Lightning               {R}           Sorcery                             Dominaria Remastered                         common    - 
 Lightning Bolt                {R}           Instant                             Ravnica: Clue Edition                        uncommon  - 

Found 58 cards (showing 5 on page 1)

Next page: mtg scryfall search "Lightning" --page 2
Jump to page: mtg scryfall search "Lightning" --page <PAGE_NUMBER>
```

### Detailed Card View (Pretty Format)

```
 Name              Lightning Bolt 
 Mana Cost         {R} 
 Mana Value        1 
 Type              Instant 
 Oracle Text       Lightning Bolt deals 3 damage to any target. 
 Set               Ravnica: Clue Edition (CLU) 
 Rarity            uncommon 
 Artist            Christopher Moeller 
 Flavor Text       The sparkmage shrieked, calling on the rage of the storms of his youth. To his surprise, the sky responded with a fierce energy he'd never thought to see again. 
 Collector Number  141 
 Legal In          modern, legacy, vintage, commander 
```

### Autocomplete Results

```bash
$ mtg scryfall autocomplete "lightning"
Lightning Axe
Lightning Bolt
Lightning Mare
Lightning Blow
Lightning Dart
Lightning Wolf
Lightning Rift
Lightning Spear
Lightning Cloud
Lightning Storm
Lightning Coils
Lightning Surge
Lightning Angel
Lightning Helix
Lightning Blast
Lightning Runner
Lightning Dragon
Lightning Diadem
Lightning Reaver
Lightning Hounds
```

### CSV Export Example

```bash
$ mtg scryfall search "c:blue t:instant" --csv
"Name","Mana Cost","Type Line","Set","Rarity","Power","Toughness"
"Counterspell","{U}{U}","Instant","Foundations","Common","",""
"Negate","{1}{U}","Instant","Foundations","Common","",""
"Opt","{U}","Instant","Foundations","Common","",""
```

## Command Options Reference

### Search Command Options

- `--pretty` - Display results in formatted table
- `--page <NUM>` - Page number (default: 1)
- `--order <ORDER>` - Sort order: `name`, `set`, `released`, `rarity`, `color`, `usd`, `tix`, `eur`, `cmc`, `power`, `toughness`, `edhrec`, `penny`, `artist`, `review`
- `--dir <DIR>` - Sort direction: `auto`, `asc`, `desc`
- `--include-extras` - Include tokens, emblems, etc.
- `--include-multilingual` - Include non-English cards
- `--include-variations` - Include different printings/variants
- `--unique <MODE>` - Unique mode: `cards`, `prints`, `art`
- `--csv` - Export results in CSV format

### Advanced Search Options

All search options plus individual filter options:
- `--name <NAME>` - Card name (partial matching)
- `--oracle <TEXT>` - Oracle text search
- `--card-type <TYPE>` - Card type
- `--colors <COLORS>` - Color filter
- `--identity <COLORS>` - Color identity
- `--mana <COST>` - Mana cost
- `--mv <VALUE>` - Mana value with comparisons
- `--power <VALUE>` - Power with comparisons
- `--toughness <VALUE>` - Toughness with comparisons
- `--loyalty <VALUE>` - Loyalty with comparisons
- `--set <CODE>` - Set code
- `--rarity <RARITY>` - Rarity
- `--artist <NAME>` - Artist name
- `--flavor <TEXT>` - Flavor text search
- `--format <FORMAT>` - Format legality
- `--language <LANG>` - Language code

### Card Lookup Options

#### Named Command
- `--pretty` - Display in formatted table
- `--set <CODE>` - Get specific printing from set

#### ID Commands (id, arena, mtgo, etc.)
- `--pretty` - Display in formatted table

#### Collector Command
- `--lang <LANG>` - Language code (e.g., "en", "ja", "de")
- `--pretty` - Display in formatted table

#### Random Command
- `--query <QUERY>` - Filter random results with Scryfall query
- `--pretty` - Display in formatted table

#### Autocomplete Command
- `--include-extras` - Include extra cards in suggestions

### Global Options

- `--timeout <SECONDS>` - Request timeout (default: 30)
- `--verbose` - Detailed output including cache information

## Color Codes

When specifying colors, use these single-letter codes:

- `w` - White
- `u` - Blue  
- `b` - Black
- `r` - Red
- `g` - Green
- `c` - Colorless
- `m` - Multicolor

You can also use color names and guild/shard names:
- Guild names: `azorius`, `dimir`, `rakdos`, `gruul`, `selesnya`, etc.
- Shard names: `bant`, `esper`, `grixis`, `jund`, `naya`
- Wedge names: `abzan`, `jeskai`, `sultai`, `mardu`, `temur`

## Rarity Values

- `common` or `c`
- `uncommon` or `u`
- `rare` or `r`
- `mythic` or `m`
- `special` or `s`
- `bonus` or `b`

## Format Names

- `standard`
- `pioneer`
- `modern`
- `legacy`
- `vintage`
- `commander`
- `brawl`
- `pauper`
- `penny` (Penny Dreadful)
- `historic`
- `alchemy`

## Tips and Tricks

### Efficient Searching

```bash
# Use abbreviations for common searches
mtg scryfall search "c:r t:c mv<=3"  # Cheap red creatures

# Combine multiple conditions
mtg scryfall search "c:blue o:draw t:instant f:standard"

# Use comparison operators
mtg scryfall search "pow>=5 tou>=5"  # Big creatures
```

### Finding Specific Cards

```bash
# Exact name matching
mtg scryfall search "!\"Lightning Bolt\""

# Cards with specific abilities
mtg scryfall search "o:flying o:vigilance"

# Cards by collector number
mtg scryfall search "cn:100 s:war"
```

### Format-Specific Searches

```bash
# Standard-legal red aggro cards
mtg scryfall search "c:red mv<=3 (t:creature or (t:instant o:damage)) f:standard"

# Commander-legal artifacts
mtg scryfall search "t:artifact f:commander mv>=4"

# Pauper commons
mtg scryfall search "r:common f:pauper"
```

### Performance Tips

- **Automatic Caching**: All commands (except `random`) use intelligent caching for faster repeated queries
- **Efficient Filtering**: Use specific filters to narrow results and reduce API calls
- **Batch Operations**: Use `--csv` export for large datasets
- **Smart Pagination**: Use pagination for large result sets instead of fetching everything
- **ID Lookups**: Use specific ID lookups when you know the exact card you want

### Cache Behavior

The MTG CLI automatically caches all Scryfall API responses to improve performance:

- **Search results** are cached based on query parameters
- **Card lookups** are cached by ID/name/collector number
- **Autocomplete** suggestions are cached
- **Random cards** are NOT cached (intentionally random each time)
- Cache respects TTL and automatically expires old entries
- Use `--verbose` to see cache hit/miss information

## Error Handling

If a search returns no results:
```
No cards found matching your search criteria.
```

If a card name is not found:
```
Card 'Invalid Name' not found: No card found
```

For invalid syntax:
```
Invalid search syntax: <error details>
```

## Complete Examples Workflow

### Building a Commander Deck

```bash
# Find a commander
mtg scryfall search "t:legendary t:creature id:bant" --pretty

# Get specific commander details
mtg scryfall named "Rafiq of the Many" --pretty

# Find cards for the deck
mtg scryfall advanced --identity "bant" --format "commander" \
  --card-type "creature" --mv "<=4" --pretty

# Look up specific cards by collector number
mtg scryfall collector ala 185 --pretty  # Rafiq of the Many

# Get random cards for inspiration
mtg scryfall random --query "id:bant f:commander" --pretty
```

### Standard Deck Research

```bash
# Find current Standard legal cards
mtg scryfall search "f:standard" --order released --pretty

# Look for specific archetypes
mtg scryfall advanced --format "standard" --colors "r" \
  --card-type "creature" --mv "<=3" --pretty

# Check card prices
mtg scryfall search "f:standard c:red t:creature" --order usd --pretty

# Export deck list to CSV
mtg scryfall search "f:standard c:red" --csv > standard_red.csv
```

### Collection Management

```bash
# Look up cards by various IDs from different sources
mtg scryfall arena 67330 --pretty        # From Arena export
mtg scryfall mtgo 12345 --pretty         # From MTGO collection
mtg scryfall tcgplayer 98765 --pretty    # From TCGPlayer order

# Find all printings of a card
mtg scryfall search "!\"Lightning Bolt\"" --unique prints --pretty

# Get specific language versions
mtg scryfall collector ktk 96 --lang ja --pretty
```

### Market Research

```bash
# Find expensive cards
mtg scryfall search "usd>=100" --order usd --dir desc --pretty

# Check EDHREC popularity
mtg scryfall search "t:creature" --order edhrec --pretty

# Find budget alternatives
mtg scryfall search "o:\"draw a card\" usd<=5" --order usd --pretty
```

### API Integration Examples

```bash
# Get card data in JSON for scripts
mtg scryfall named "Lightning Bolt" > lightning_bolt.json

# Export search results for analysis
mtg scryfall search "f:standard" --csv > standard_cards.csv

# Get autocomplete data for applications
mtg scryfall autocomplete "light" > suggestions.txt
```

## Integration with Other Tools

### Using with jq for JSON Processing

```bash
# Extract just the card names
mtg scryfall search "c:red t:creature" | jq '.data[].name'

# Get mana costs
mtg scryfall named "Lightning Bolt" | jq '.mana_cost'

# Filter by specific criteria
mtg scryfall search "t:creature" | jq '.data[] | select(.cmc <= 3)'
```

### Scripting Examples

```bash
#!/bin/bash
# Get all cards from a set
SET_CODE="ktk"
mtg scryfall search "s:${SET_CODE}" --csv > "${SET_CODE}_cards.csv"

# Look up multiple cards by collector number
for i in {1..10}; do
    mtg scryfall collector "${SET_CODE}" "$i" --pretty
done
```

## Troubleshooting

### Common Issues

**Card not found by name:**
```bash
# Try exact name matching
mtg scryfall search "!\"Exact Card Name\""

# Or use partial matching
mtg scryfall advanced --name "partial name"
```

**No results for search:**
```bash
# Check if filters are too restrictive
mtg scryfall search "c:red t:creature mv<=2 f:standard" --verbose

# Try broader search first
mtg scryfall search "c:red t:creature"
```

**Slow performance:**
```bash
# Use --verbose to see cache information
mtg scryfall search "t:creature" --verbose

# Use specific ID lookups when possible
mtg scryfall id "uuid-here" --pretty
```

### Error Messages

- **"No card found"** - Card name doesn't exist or is misspelled
- **"Invalid search syntax"** - Check Scryfall query syntax
- **"Rate limited"** - Wait a moment and try again (rare with caching)
- **"Network error"** - Check internet connection

---

Next: [README.md](./README.md) | Back: [Gatherer Commands](./gatherer.md)