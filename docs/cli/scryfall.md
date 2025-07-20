# Scryfall Commands

The `mtg scryfall` command provides access to Scryfall's powerful search engine and comprehensive Magic: The Gathering card database.

## Available Commands

- `search <QUERY>` - Search cards using Scryfall syntax
- `advanced` - Advanced search with individual filter options
- `card <NAME>` - Get specific card by exact name

## Basic Search

### Simple Search

Search for cards using Scryfall's powerful query syntax:

```bash
# Find all Lightning Bolt cards
mtg scryfall search "Lightning Bolt"

# Find red creatures
mtg scryfall search "c:red t:creature"

# Find cards with "draw" in their text
mtg scryfall search "o:draw"
```

### Search Options

```bash
# Pretty table output
mtg scryfall search "c:blue t:instant" --pretty

# Pagination
mtg scryfall search "t:creature" --page 2

# Sort by different criteria
mtg scryfall search "t:planeswalker" --order cmc
mtg scryfall search "c:red" --order power --dir desc

# Include extra cards (tokens, emblems, etc.)
mtg scryfall search "t:token" --include-extras

# Show different printings
mtg scryfall search "Lightning Bolt" --unique prints
```

## Advanced Search

Use individual filter options for more precise searches:

### Basic Filters

```bash
# Search by name (partial matching)
mtg scryfall advanced --name "Lightning"

# Search by oracle text
mtg scryfall advanced --oracle "draw a card"

# Search by type
mtg scryfall advanced --card-type "creature"
```

### Color Filters

```bash
# Single color
mtg scryfall advanced --colors "r"

# Multiple colors (exact)
mtg scryfall advanced --colors "wu"

# Color identity for commander
mtg scryfall advanced --identity "bant"
```

### Mana and Stats

```bash
# Mana cost
mtg scryfall advanced --mana "{2}{U}"

# Mana value/CMC
mtg scryfall advanced --mv "3"
mtg scryfall advanced --mv ">=4"

# Power and toughness
mtg scryfall advanced --power ">=3"
mtg scryfall advanced --toughness "2"

# Loyalty
mtg scryfall advanced --loyalty "4"
```

### Set and Rarity

```bash
# Specific set
mtg scryfall advanced --set "ktk"

# Rarity
mtg scryfall advanced --rarity "mythic"

# Combine filters
mtg scryfall advanced --set "war" --rarity "rare" --colors "u"
```

### Other Filters

```bash
# Artist
mtg scryfall advanced --artist "Rebecca Guay"

# Flavor text
mtg scryfall advanced --flavor "Jace"

# Format legality
mtg scryfall advanced --format "standard"

# Language
mtg scryfall advanced --language "ja"
```

## Get Specific Card

Retrieve detailed information about a specific card:

```bash
# Get card by exact name
mtg scryfall card "Lightning Bolt"

# Get specific printing from a set
mtg scryfall card "Lightning Bolt" --set "lea"

# Pretty table output
mtg scryfall card "Jace, the Mind Sculptor" --pretty
```

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

### Search Results

```
 Name                    Cost      Type                Set           Rarity     P/T/L
 Lightning Bolt          {R}       Instant             Masters 25    Uncommon   -
 Lightning Bolt          {R}       Instant             Player Rew    Special    -
 Lightning Bolt          {R}       Instant             Beatdown      Common     -
 Lightning Bolt          {R}       Instant             Fourth Ed     Common     -
 Lightning Bolt          {R}       Instant             Revised       Common     -

Found 87 cards (showing 5 on page 1)

Next page: mtg scryfall search "Lightning Bolt" --page 2
```

### Detailed Card View

```
Name             Lightning Bolt
Mana Cost        {R}
Mana Value       1
Type             Instant
Oracle Text      Lightning Bolt deals 3 damage to any target.
Set              Masters 25 (A25)
Rarity           uncommon
Artist           Christopher Rush
Collector Number 141
Legal In         legacy, vintage, commander, modern, pauper
```

## Command Options

### Search Options

- `--pretty` - Display results in formatted table
- `--page <NUM>` - Page number (default: 1)
- `--order <ORDER>` - Sort order (name, cmc, power, toughness, artist, set, released, rarity, usd, tix, eur)
- `--dir <DIR>` - Sort direction (auto, asc, desc)
- `--include-extras` - Include tokens, emblems, etc.
- `--include-multilingual` - Include non-English cards
- `--include-variations` - Include different printings
- `--unique <MODE>` - Unique mode (cards, prints, art)

### Advanced Search Options

All the individual filter options plus:
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

### Card Options

- `--pretty` - Display in formatted table
- `--set <CODE>` - Get specific printing from set

### Global Options

- `--api-base-url <URL>` - Custom API endpoint (not applicable to Scryfall)
- `--timeout <SECONDS>` - Request timeout
- `--verbose` - Detailed output

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

- Use specific filters to narrow results
- Cache is automatically used for repeated searches
- Use `--pretty` for readable output
- Combine filters efficiently to reduce API calls

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

---

Next: [README.md](./README.md) | Back: [Gatherer Commands](./gatherer.md)