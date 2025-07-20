# Gatherer Command

The `gatherer` command provides access to the unofficial Gatherer advanced search functionality. Use this command to search for Magic: The Gathering cards using the same comprehensive search capabilities available on the Gatherer website.

## Overview

The Gatherer database contains the most authoritative and up-to-date Magic card information directly from Wizards of the Coast. This command interfaces with the official Gatherer advanced search API to provide detailed card data including rules text, format legalities, and official card images.

## Basic Usage

```bash
# Search for cards with advanced filters
mtg gatherer search [OPTIONS]

# Get a specific card by name
mtg gatherer card <CARD_NAME> [OPTIONS]
```

## Commands

### Search Command

The `search` subcommand provides comprehensive filtering and search capabilities across the entire Gatherer database.

### Card Command

The `card` subcommand fetches detailed information for a specific card by name.

```bash
# Get a specific card
mtg gatherer card "Lightning Bolt"

# Get card with formatted output
mtg gatherer card "Lightning Bolt" --pretty
```

This command:
- Searches for the exact card name in the Gatherer database
- Returns detailed card information including rules text, type line, and game statistics
- Supports both JSON (default) and pretty table output formats
- Handles card name variations and finds the most relevant match

## Search Parameters

### Card Identification

**Card Name** (`--name`, `-n`)
Search for cards by name or partial name match.

```bash
# Find cards with "Lightning" in the name
mtg gatherer search --name "Lightning"

# Search for exact card name
mtg gatherer search --name "Lightning Bolt"
```

**Set Name** (`--set`)
Filter results to cards from a specific set.

```bash
# Search within a specific set
mtg gatherer search --name "Vivi" --set "Magic: The Gathering—FINAL FANTASY"

# Find all rare cards from a set
mtg gatherer search --set "Guilds of Ravnica" --rarity "Rare"
```

### Card Properties

**Rarity** (`--rarity`, `-r`)
Filter by card rarity. Accepts standard rarity names or abbreviations.

```bash
# Search for mythic rare cards
mtg gatherer search --rarity "Mythic"

# Alternative rarity formats
mtg gatherer search --rarity "Rare"
mtg gatherer search --rarity "Common"
mtg gatherer search --rarity "Uncommon"
```

**Card Type** (`--card-type`, `-t`)
Search by card type such as Creature, Instant, or Sorcery.

```bash
# Find all planeswalkers
mtg gatherer search --card-type "Planeswalker"

# Search for artifacts
mtg gatherer search --card-type "Artifact"
```

**Subtype** (`--subtype`, `-s`)
Filter by creature subtypes or other card subtypes.

```bash
# Find all Dragons
mtg gatherer search --subtype "Dragon"

# Search for Human Wizards
mtg gatherer search --card-type "Creature" --subtype "Human Wizard"
```

### Game Mechanics

**Mana Cost** (`--mana-cost`, `-m`)
Search by specific mana cost patterns.

```bash
# Find cards with specific mana cost
mtg gatherer search --mana-cost "{2}{U}"

# Search for expensive spells
mtg gatherer search --mana-cost "{8}"
```

**Power and Toughness** (`--power`, `-p`, `--toughness`)
Search creatures by their power and toughness values.

```bash
# Find 1/1 creatures
mtg gatherer search --power "1" --toughness "1"

# Search for high-power creatures
mtg gatherer search --power "8"
```

**Loyalty** (`--loyalty`, `-l`)
Search planeswalkers by starting loyalty.

```bash
# Find planeswalkers with 4 starting loyalty
mtg gatherer search --loyalty "4" --card-type "Planeswalker"
```

### Text Content

**Rules Text** (`--rules`)
Search within card rules text for specific keywords or phrases.

```bash
# Find cards with flying
mtg gatherer search --rules "flying"

# Search for cards that draw cards
mtg gatherer search --rules "draw a card"
```

**Flavor Text** (`--flavor`)
Search within flavor text for specific words or phrases.

```bash
# Find cards with specific flavor text
mtg gatherer search --flavor "darkness"
```

### Additional Filters

**Colors** (`--colors`, `-c`)
Filter by card colors using standard color abbreviations.

```bash
# Find blue cards
mtg gatherer search --colors "U"

# Search for multicolored cards
mtg gatherer search --colors "WU"
```

**Artist** (`--artist`, `-a`)
Search for cards by artist name.

```bash
# Find cards by specific artist
mtg gatherer search --artist "Rebecca Guay"
```

**Format Legality** (`--format`, `-f`)
Filter by format legality such as Standard, Modern, or Legacy.

```bash
# Find Standard-legal cards
mtg gatherer search --format "Standard"

# Search for Modern-legal artifacts
mtg gatherer search --format "Modern" --card-type "Artifact"
```

### Display and Navigation Options

**Pretty Output** (`--pretty`)
Display results in a formatted table instead of JSON.

```bash
# Show results in table format
mtg gatherer search --name "Lightning" --pretty

# Combine with other search parameters
mtg gatherer search --rarity "Rare" --card-type "Creature" --pretty
```

**Page Navigation** (`--page`)
Navigate through paginated results (default: page 1).

```bash
# View page 2 of results
mtg gatherer search --rarity "Common" --page 2

# Jump to page 10
mtg gatherer search --name "Dragon" --page 10 --pretty

# Use with any search parameters
mtg gatherer search --card-type "Planeswalker" --page 3 --pretty
```

## Advanced Search Examples

### Combining Multiple Criteria

Search for specific card combinations by combining multiple parameters:

```bash
# Find rare blue instants in Standard (table format)
mtg gatherer search --card-type "Instant" --colors "U" --rarity "Rare" --format "Standard" --pretty

# Search for expensive artifacts (JSON format)
mtg gatherer search --card-type "Artifact" --mana-cost "{6}" --rarity "Mythic"

# Find powerful creatures (table format)
mtg gatherer search --card-type "Creature" --power "5" --rarity "Rare" --pretty
```

### Set-Specific Searches

Explore cards from specific Magic sets:

```bash
# Find all mythic rares from a set (table format)
mtg gatherer search --set "War of the Spark" --rarity "Mythic" --pretty

# Search for planeswalkers in a set (JSON format)
mtg gatherer search --set "War of the Spark" --card-type "Planeswalker"
```

### Mechanic-Based Searches

Search for cards with specific game mechanics:

```bash
# Find cards with trample (table format)
mtg gatherer search --rules "trample" --pretty

# Search for cards that create tokens (JSON format)
mtg gatherer search --rules "create.*token"

# Find cards with enters-the-battlefield effects (table format)
mtg gatherer search --rules "enters the battlefield" --pretty
```

## Output Format

The command supports two output formats: JSON (default) and formatted table (with `--pretty`).

### JSON Output (Default)

By default, the command returns detailed card information in JSON format, including:

- **Card identification**: Name, multiverse ID, set information
- **Game properties**: Mana cost, type line, power/toughness, loyalty
- **Rules text**: Complete Oracle text and flavor text
- **Format legalities**: Legal status in all major formats
- **Visual elements**: Image URLs and set symbols
- **Localization**: Available languages and translations

### Pretty Table Output

Use the `--pretty` flag to display results in a formatted table instead of JSON:

```bash
mtg gatherer search --name "Lightning Bolt" --pretty
```

This displays results in a clean table format:

```
+----------------+---------+------+--------------------------------+----------+-------+
| Name           | Type    | Cost | Set                            | Rarity   | P/T/L |
+----------------+---------+------+--------------------------------+----------+-------+
| Lightning Bolt | Instant | R    | FINAL FANTASY Through the Ages | Uncommon | -     |
+----------------+---------+------+--------------------------------+----------+-------+

Found 1 cards (showing 1 on page 1 of 1)
```

The table includes:
- **Name**: Card name
- **Type**: Complete type line
- **Cost**: Mana cost
- **Set**: Set name
- **Rarity**: Card rarity
- **P/T/L**: Power/Toughness for creatures, Loyalty for planeswalkers, or "-" for other types

### Pagination

When search results span multiple pages, pagination information appears at the bottom:

```bash
mtg gatherer search --rarity "Common" --pretty
```

Output includes pagination summary and navigation commands:

```
[table with results...]

Found 10959 cards (showing 72 on page 1 of 153)

Pagination commands:
Next page: mtg gatherer search --rarity "Common" --pretty --page 2
Jump to page: mtg gatherer search --rarity "Common" --pretty --page <PAGE_NUMBER>
```

Navigate between pages using the `--page` parameter:

```bash
# Go to page 2
mtg gatherer search --rarity "Common" --pretty --page 2

# Jump to page 10
mtg gatherer search --rarity "Common" --pretty --page 10
```

The pagination commands automatically include all your current search parameters, making it easy to navigate through large result sets while maintaining your filters.

### Verbose Output

Use the `--verbose` flag to see additional debugging information:

```bash
mtg gatherer search --name "Lightning Bolt" --verbose
```

This displays:

- Request payload sent to the API
- Response status and headers
- Raw response data before processing

## Response Structure

Each card result contains comprehensive information:

```json
{
  "instanceName": "Lightning Bolt",
  "oracleName": "Lightning Bolt",
  "instanceManaText": "R",
  "instanceType": "Instant",
  "instanceText": "Lightning Bolt deals 3 damage to any target.",
  "rarityName": "Common",
  "setName": "Magic 2011",
  "artistName": "Christopher Moeller",
  "formatLegalities": [
    { "formatName": "Modern", "legality": "Legal" },
    { "formatName": "Legacy", "legality": "Legal" }
  ],
  "imageUrls": {
    "default": "https://gatherer-static.wizards.com/Cards/medium/...",
    "medium": "https://gatherer-static.wizards.com/Cards/medium/..."
  }
}
```

## Error Handling

The command handles various error conditions gracefully:

- **Network timeouts**: Configurable via `--timeout` global option
- **Invalid parameters**: Clear error messages for malformed input
- **API errors**: Detailed error reporting from Gatherer service
- **Empty results**: Informative messages when no cards match criteria

## Performance Considerations

- **Response time**: Searches typically complete within 2-5 seconds
- **Result limits**: API returns up to 72 cards per page
- **Rate limiting**: Respectful request timing to avoid service disruption
- **Caching**: No local caching; always returns current data

## Integration with Other Commands

The `gatherer` command complements other MTG CLI commands:

- Use with `cards` command to cross-reference different data sources
- Compare results with `sets` command for set-specific information
- Verify format legalities with `types` command data

## Troubleshooting

### Common Issues

**No results returned**

- Verify search parameters are correctly formatted
- Check set names for exact spelling and punctuation
- Try broader search criteria

**Timeout errors**

- Increase timeout with `--timeout` global option
- Check network connectivity
- Retry with simpler search parameters

**Unexpected results**

- Use `--verbose` flag to inspect request payload
- Verify parameter formatting matches expected API format
- Check for typos in card names or set names

### Getting Help

For additional assistance:

```bash
# View command help
mtg gatherer search --help

# View global options
mtg --help
```

## Examples by Use Case

### Deck Building

Find cards for specific deck archetypes:

```bash
# Aggressive red creatures (table format for easy browsing)
mtg gatherer search --card-type "Creature" --colors "R" --power "2" --mana-cost "{1}" --pretty

# Control win conditions (JSON format for detailed data)
mtg gatherer search --card-type "Planeswalker" --colors "UW" --format "Standard"
```

### Collection Management

Identify valuable or interesting cards:

```bash
# Find mythic rares from recent sets (table format)
mtg gatherer search --rarity "Mythic" --set "Streets of New Capenna" --pretty

# Search for cards by favorite artist (table format)
mtg gatherer search --artist "Seb McKinnon" --pretty
```

### Format Research

Explore format-specific card pools:

```bash
# Modern-legal artifacts (table format)
mtg gatherer search --card-type "Artifact" --format "Modern" --pretty

# Pioneer planeswalkers (table format)
mtg gatherer search --card-type "Planeswalker" --format "Pioneer" --pretty
```

### Rules Research

Find cards with specific mechanics or interactions:

```bash
# Cards that exile from graveyard (JSON format for detailed rules text)
mtg gatherer search --rules "exile.*graveyard"

# Instant-speed card draw (table format for quick overview)
mtg gatherer search --card-type "Instant" --rules "draw.*card" --pretty
```

### Single Card Lookup

Get detailed information for specific cards:

```bash
# Get complete card details (JSON format)
mtg gatherer card "Lightning Bolt"

# Get card details in table format
mtg gatherer card "Jace, the Mind Sculptor" --pretty

# Look up cards with complex names
mtg gatherer card "Vivi, Spark of Life"
```
