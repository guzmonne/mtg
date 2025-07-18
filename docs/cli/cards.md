# Card Commands

The `mtg cards` command provides comprehensive access to Magic: The Gathering card data.

## Available Commands

- `search <NAME>` - Search cards by name
- `list` - List cards with advanced filtering
- `get <ID>` - Get specific card by ID

## Card Search

### Basic Search

Search for cards by name (supports partial matching):

```bash
# Find all cards with "Lightning" in the name
mtg cards search "Lightning"

# Find cards with "Bolt" in the name
mtg cards search "Bolt"
```

### Exact Name Matching

Use the `--exact` flag for precise matches:

```bash
# Find only cards named exactly "Lightning Bolt"
mtg cards search "Lightning Bolt" --exact
```

### Pagination

Control result pagination:

```bash
# Get first 5 results
mtg cards search "Dragon" --page-size 5

# Get second page of results
mtg cards search "Dragon" --page 2 --page-size 10

# Get many results (max 100 per page)
mtg cards search "Creature" --page-size 100
```

### Language Support

Search for foreign language cards:

```bash
# Search for Japanese cards
mtg cards search "Lightning Bolt" --language "Japanese"

# Search for German cards
mtg cards search "Blitz" --language "German"
```

## Advanced Card Listing

The `list` command provides powerful filtering options:

### Basic Listing

```bash
# List recent cards
mtg cards list

# List with custom page size
mtg cards list --page-size 20
```

### Color Filtering

```bash
# Red cards only
mtg cards list --colors "Red"

# Multi-color cards (AND logic)
mtg cards list --colors "Red,Blue"

# Cards with any of these colors (OR logic)
mtg cards list --colors "Red|Blue|Green"

# Colorless cards
mtg cards list --colors "Colorless"
```

### Type Filtering

```bash
# Creatures only
mtg cards list --type "Creature"

# Instants and Sorceries
mtg cards list --type "Instant" --page 1
mtg cards list --type "Sorcery" --page 1

# Artifacts
mtg cards list --type "Artifact"
```

### Rarity Filtering

```bash
# Mythic Rare cards
mtg cards list --rarity "Mythic Rare"

# Common cards
mtg cards list --rarity "Common"

# Rare and Mythic
mtg cards list --rarity "Rare"
```

### Set Filtering

```bash
# Cards from specific set
mtg cards list --set "KTK"  # Khans of Tarkir

# Cards from Alpha
mtg cards list --set "LEA"

# Cards from recent sets
mtg cards list --set "MH3"  # Modern Horizons 3
```

### Mana Cost Filtering

```bash
# Cards with CMC 1
mtg cards list --cmc 1

# Cards with CMC 3
mtg cards list --cmc 3

# High CMC cards
mtg cards list --cmc 10
```

### Power and Toughness

```bash
# Creatures with power 1
mtg cards list --power 1

# Creatures with toughness 1
mtg cards list --toughness 1

# Powerful creatures
mtg cards list --power 5 --toughness 5
```

### Combining Filters

```bash
# Red creatures from Khans of Tarkir
mtg cards list --colors "Red" --type "Creature" --set "KTK"

# Cheap blue instants
mtg cards list --colors "Blue" --type "Instant" --cmc 1

# Expensive mythic artifacts
mtg cards list --type "Artifact" --rarity "Mythic Rare" --cmc 5
```

## Get Specific Card

Retrieve detailed information about a specific card:

```bash
# Get card by ID
mtg cards get 409574

# Get card by multiverse ID
mtg cards get 1234567
```

## Output Examples

### Search Results

```
 Name                    Set  Type                Rarity     Mana Cost
 Lightning Bolt          2ED  Instant             Common     {R}
 Lightning Bolt          3ED  Instant             Common     {R}
 Lightning Bolt          4ED  Instant             Common     {R}
 Lightning Bolt          5ED  Instant             Common     {R}
 Lightning Bolt          6ED  Instant             Common     {R}

Found 25 cards matching 'Lightning Bolt'
```

### Detailed Card View

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
```

## Command Options

### Global Options

- `--api-base-url <URL>` - Custom API endpoint
- `--timeout <SECONDS>` - Request timeout
- `--verbose` - Detailed output

### Search Options

- `--exact` - Exact name matching
- `--language <LANG>` - Foreign language search
- `--page <NUM>` - Page number (default: 1)
- `--page-size <SIZE>` - Results per page (default: 20, max: 100)

### List Options

- `--name <NAME>` - Filter by name
- `--colors <COLORS>` - Filter by colors (comma for AND, pipe for OR)
- `--type <TYPE>` - Filter by card type
- `--rarity <RARITY>` - Filter by rarity
- `--set <SET>` - Filter by set code
- `--cmc <NUMBER>` - Filter by converted mana cost
- `--power <NUMBER>` - Filter by power
- `--toughness <NUMBER>` - Filter by toughness
- `--loyalty <NUMBER>` - Filter by loyalty
- `--page <NUM>` - Page number
- `--page-size <SIZE>` - Results per page

## Color Codes

When specifying colors, use these values:

- `White` or `W`
- `Blue` or `U`
- `Black` or `B`
- `Red` or `R`
- `Green` or `G`
- `Colorless`

## Rarity Values

- `Common`
- `Uncommon`
- `Rare`
- `Mythic Rare`
- `Special`
- `Basic Land`

## Tips and Tricks

### Efficient Searching

```bash
# Use shorter names for broader results
mtg cards search "Bolt" --page-size 50

# Combine with exact for precision
mtg cards search "Lightning Bolt" --exact --page-size 100
```

### Finding Specific Cards

```bash
# Search by artist
mtg cards list --artist "John Avon"

# Search by flavor text (if supported)
mtg cards search "sparkmage"
```

### Performance Tips

- Use smaller page sizes for faster responses
- Be specific with filters to reduce result sets
- Use exact matching when you know the full name

---

Next: [Set Commands](./sets.md) | Back: [Getting Started](./getting-started.md)
