# Deck Analysis

Analyze Magic: The Gathering deck lists to get comprehensive statistics including mana curve, type distribution, format legality, and more. Also access tournament deck lists from official events.

## Commands

### Deck Stats

Analyze deck statistics from a deck list.

```bash
mtg decks stats [OPTIONS] [DECK_LIST]
```

#### Options

- `[DECK_LIST]` - Deck list input (use '-' for stdin, provide deck list as string, or omit to read from stdin)
- `-f, --file <FILE>` - Read deck list from file
- `--format <FORMAT>` - Output format (pretty table or JSON) [default: pretty]

### Ranked Deck Lists

Access tournament deck lists from official Magic: The Gathering events.

#### List Command

```bash
mtg decks ranked list [OPTIONS]
```

##### Options

- `-f, --format <FORMAT>` - Filter by format (e.g., alchemy, standard, modern)
- `-l, --limit <LIMIT>` - Number of results to fetch [default: 20]
- `-s, --skip <SKIP>` - Number of results to skip [default: 0]
- `-p, --page <PAGE>` - Page number (1-based, automatically calculates skip based on limit)
- `--output <OUTPUT>` - Output format (pretty table or JSON) [default: pretty]

#### Show Command

```bash
mtg decks ranked show <ID_OR_URL> [OPTIONS]
```

Fetches and parses deck lists from a specific tournament article page.

##### Arguments

- `<ID_OR_URL>` - Either the ID from the list command or the full URL of the deck list article

##### Options

- `--output <OUTPUT>` - Output format (pretty table or JSON) [default: pretty]

### Compare Command

```bash
mtg decks compare <DECK1> <DECK2>
```

Compares two deck lists to find similarities and differences.

##### Arguments

- `<DECK1>` - First deck ID or article ID (if article ID, uses the first deck)
- `<DECK2>` - Second deck ID or article ID (if article ID, uses the first deck)

##### Features

- Shows shared cards between both decks with quantity differences
- Lists cards unique to each deck
- Displays main deck and sideboard counts separately
- Provides summary statistics

## Input Methods

The tool supports multiple ways to provide deck lists:

1. **From file**: `mtg decks stats --file deck.txt`
2. **From stdin (pipe)**: `cat deck.txt | mtg decks stats`
3. **From stdin (explicit)**: `mtg decks stats -`
4. **From stdin (default)**: `mtg decks stats` (reads from stdin if no other input)
5. **As argument**: `mtg decks stats "4 Lightning Bolt\n4 Mountain"`
6. **From cached deck ID**: `mtg decks stats 7d1d96bc86e2185c`
7. **From article ID**: `mtg decks stats 6b9a732534c4294a` (analyzes first deck, shows others)

## Deck List Format

The deck list should follow the standard format used by most MTG deck building tools:

```
Deck
4 Lightning Bolt (M21) 162
4 Mountain (ANB) 114
2 Shock (M21) 159

Sideboard
2 Counterspell (M21) 46
1 Negate (M21) 52
```

### Format Rules

- **Section Headers**: Use `Deck` and `Sideboard` to separate sections (case-insensitive)
- **Card Lines**: Must start with a number: `quantity cardname (set_code) collector_number`
- **Set Code**: Optional 3-letter set code in parentheses
- **Collector Number**: Optional collector number after set code
- **Ignored Lines**: Any line not starting with a number, "Deck", or "Sideboard" is ignored
- **Comments**: Lines starting with text (not numbers) are automatically ignored

### Supported Formats

The parser is flexible and supports various common formats:

```bash
# With set codes and collector numbers
4 Lightning Bolt (M21) 162

# With set codes only
4 Lightning Bolt (M21)

# Card name only
4 Lightning Bolt

# Mixed formats in same deck
4 Lightning Bolt (M21) 162
4 Mountain
2 Shock (ANB) 84
```

## Examples

### Basic Usage

```bash
# Analyze deck from file
mtg decks stats --file my_deck.txt

# Analyze deck from stdin (pipe)
cat my_deck.txt | mtg decks stats

# Analyze deck from stdin (explicit)
mtg decks stats -

# Analyze deck from command line
mtg decks stats "Deck
4 Lightning Bolt
4 Mountain"

# With comments and metadata (ignored lines)
echo "// My Burn Deck
Deck
4 Lightning Bolt (M21) 162
// This is a comment
4 Mountain
Sideboard
2 Counterspell" | mtg decks stats

# Analyze a cached deck using its ID
mtg decks stats 7d1d96bc86e2185c

# Analyze using an article ID (will fetch and analyze first deck)
mtg decks stats 6b9a732534c4294a

# Workflow: Fetch tournament decks and analyze them
mtg decks ranked show 6b9a732534c4294a --output json | jq -r '.decks[0].id' | xargs mtg decks stats
```

### Output Formats

#### Pretty Table (Default)

```bash
mtg decks stats --file deck.txt
```

Output:
```
=== DECK ANALYSIS ===

Basic Statistics:
 Metric              Value 
 Total Cards         60 
 Main Deck           60 
 Sideboard           0 
 Unique Cards        8 
 Average Mana Value  1.85 

Mana Curve:
 Mana Value  Cards  Percentage 
 0           20     33.3% 
 1           24     40.0% 
 2           8      13.3% 
 3           8      13.3% 

Card Types:
 Type      Cards  Percentage 
 Land      20     33.3% 
 Creature  16     26.7% 
 Instant   24     40.0% 

Format Legality:
 Format     Legal 
 STANDARD   ✓ 
 PIONEER    ✓ 
 MODERN     ✓ 
 LEGACY     ✓ 
 VINTAGE    ✓ 
 COMMANDER  ✓ 

Main Deck (60 cards):
 Qty  Name                 Mana Cost  Type                Set 
 4    Lightning Bolt       {R}        Instant             m21 
 4    Ghitu Lavarunner     {R}        Creature — Human    fdn 
 20   Mountain                        Basic Land — Mountain anb 
```

#### JSON Output

```bash
mtg decks stats --file deck.txt --format json
```

Output:
```json
{
  "deck_list": {
    "main_deck": [
      {
        "quantity": 4,
        "name": "Lightning Bolt",
        "set_code": "M21",
        "collector_number": "162",
        "card_details": { ... }
      }
    ],
    "sideboard": []
  },
  "statistics": {
    "total_cards": 60,
    "main_deck_cards": 60,
    "sideboard_cards": 0,
    "unique_cards": 8,
    "average_mana_value": 1.85,
    "mana_curve": {
      "0": 20,
      "1": 24,
      "2": 8,
      "3": 8
    },
    "color_distribution": {
      "R": 40
    },
    "type_distribution": {
      "Land": 20,
      "Creature": 16,
      "Instant": 24
    },
    "format_legality": {
      "standard": true,
      "modern": true,
      "legacy": true,
      "vintage": true,
      "commander": true
    }
  }
}
```

## Statistics Provided

### Basic Statistics
- **Total Cards**: Combined main deck and sideboard count
- **Main Deck Cards**: Number of cards in main deck
- **Sideboard Cards**: Number of cards in sideboard
- **Unique Cards**: Number of different card names
- **Average Mana Value**: Average converted mana cost of non-land cards

### Mana Curve Analysis
- Distribution of cards by mana value
- Percentage breakdown for curve analysis
- Helps evaluate deck's speed and consistency

### Type Distribution
- Breakdown by card types (Creature, Instant, Sorcery, etc.)
- Percentage of each type in the deck
- Useful for understanding deck composition

### Color Analysis
- Color identity distribution
- Multi-color vs single-color breakdown
- Helps with mana base planning

### Format Legality
- Legal/illegal status in major formats
- Covers Standard, Pioneer, Modern, Legacy, Vintage, Commander
- Based on current card legality data from Scryfall

## Use Cases

### Deck Building

Analyze your deck's mana curve and type distribution:

```bash
mtg decks stats --file aggro_deck.txt
```

Check if your deck is legal in specific formats before tournaments.

### Collection Management

Analyze high-value or vintage collections:

```bash
mtg decks stats --file vintage_collection.txt --format json
```

### Educational Analysis

Study classic deck archetypes and understand mana curve principles:

```bash
mtg decks stats "Deck
4 Lightning Bolt
4 Monastery Swiftspear
4 Lava Spike
20 Mountain"
```

### Tournament Preparation

Verify deck legality and analyze meta positioning:

```bash
mtg decks stats --file tournament_deck.txt | grep "Format Legality" -A 10
```

## Integration Examples

### Shell Scripting

```bash
#!/bin/bash
# Analyze multiple deck variants

for deck_file in decks/*.txt; do
    echo "Analyzing $deck_file:"
    mtg decks stats --file "$deck_file" --format json | jq '.statistics.average_mana_value'
done
```

### Data Processing

```bash
# Extract mana curve data
mtg decks stats --file deck.txt --format json | jq '.statistics.mana_curve'

# Check format legality
mtg decks stats --file deck.txt --format json | jq '.statistics.format_legality.standard'

# Get card count by type
mtg decks stats --file deck.txt --format json | jq '.statistics.type_distribution'
```

## Error Handling

The tool provides helpful error messages for common issues:

```bash
# Empty deck list
echo "" | mtg decks stats
# Error: Deck list is empty. Please provide a valid deck list.

# No valid card lines
echo "Just comments here" | mtg decks stats
# Error: No valid card lines found. Make sure lines with cards start with a number (e.g., '4 Lightning Bolt').

# Invalid card line format
echo "4" | mtg decks stats
# Error: Invalid card line format: '4'. Expected format: 'QUANTITY CARD_NAME [SET_INFO]'

# File not found
mtg decks stats --file nonexistent.txt
# Error: Failed to read file 'nonexistent.txt': No such file or directory

# Invalid quantity
echo "zero Lightning Bolt" | mtg decks stats
# Error: Invalid quantity 'zero' in line: 'zero Lightning Bolt'
```

## Performance Notes

- **API Calls**: The tool fetches card details from Scryfall API for each unique card
- **Rate Limiting**: Built-in rate limiting prevents API abuse
- **Caching**: Consider using local caching for repeated analysis of the same cards
- **Timeout**: Default 30-second timeout for API requests (configurable with `--timeout`)

## Ranked Deck Lists

The `mtg decks ranked` commands fetch and parse tournament deck lists from official Magic: The Gathering events hosted on magic.gg.

### List Examples

```bash
# List recent tournament deck lists
mtg decks ranked list

# Filter by format (e.g., alchemy)
mtg decks ranked list --format alchemy

# Get more results
mtg decks ranked list --limit 50

# Paginate through results using skip
mtg decks ranked list --skip 20 --limit 20

# Paginate through results using page
mtg decks ranked list --page 2 --limit 20

# Output as JSON
mtg decks ranked list --format standard --output json
```

### Show Examples

```bash
# Show deck lists from a specific article using ID
mtg decks ranked show a1b2c3d4e5f6g7h8

# Show deck lists from a specific article using URL
mtg decks ranked show "https://magic.gg/decklists/traditional-standard-ranked-decklists-july-21-2025"

# Output as JSON
mtg decks ranked show a1b2c3d4e5f6g7h8 --output json
```

### Output Format

#### List Command - Pretty Table (Default)

```bash
mtg decks ranked list --format alchemy --limit 5
```

Output:
```
 Id               Title                                             Author                Published    Link 
 a1b2c3d4e5f6g7h8 Neon Dynasty Championship Alchemy Decklists R-Z  Wizards of the Coast  2022-03-11   https://magic.wizards.com/en/content/neon-dynasty-championship-alchemy-decklists-4 
 b2c3d4e5f6g7h8i9 Neon Dynasty Championship Alchemy Decklists L-P  Wizards of the Coast  2022-03-11   https://magic.wizards.com/en/content/neon-dynasty-championship-alchemy-decklists-3 
 c3d4e5f6g7h8i9j0 Neon Dynasty Championship Alchemy Decklists G-K  Wizards of the Coast  2022-03-11   https://magic.wizards.com/en/content/neon-dynasty-championship-alchemy-decklists-2 
 d4e5f6g7h8i9j0k1 Neon Dynasty Championship Alchemy Decklists A-F  Wizards of the Coast  2022-03-11   https://magic.wizards.com/en/content/neon-dynasty-championship-alchemy-decklists-1 

Format: ALCHEMY
Total Results: 4
Showing: 1 - 4

Page 1 of 1
```

#### Show Command - Pretty Output

```bash
mtg decks ranked show a1b2c3d4e5f6g7h8
```

Output:
```
=== DECK LISTS FROM https://magic.gg/decklists/traditional-standard-ranked-decklists-july-21-2025 ===

Found 3 deck(s)

Deck 1 - ID: x1y2z3a4b5c6d7e8
================================================================================
Title: Mono Red Aggro
Subtitle: 7-0 Traditional Standard Ranked
Event: Traditional Standard Ranked
Date: 2025-07-21
Format: standard

Main Deck (60 cards):
  4x Lightning Bolt
  4x Monastery Swiftspear
  4x Ghitu Lavarunner
  4x Lava Spike
  4x Rift Bolt
  20x Mountain

Sideboard (15 cards):
  3x Smash to Smithereens
  2x Roiling Vortex
  4x Skullcrack
  3x Searing Blood
  3x Exquisite Firecraft

Deck 2 - ID: y2z3a4b5c6d7e8f9
================================================================================
Title: Azorius Control
Subtitle: 7-0 Traditional Standard Ranked
Event: Traditional Standard Ranked
Date: 2025-07-21
Format: standard

Main Deck (60 cards):
  4x Counterspell
  4x Memory Deluge
  3x Teferi, Hero of Dominaria
  24x Island
  25x Plains

Sideboard (15 cards):
  2x Dovin's Veto
  3x Rest in Peace
  4x Mystical Dispute
  3x Cleansing Nova
  3x Narset, Parter of Veils
```

#### JSON Output

```bash
mtg decks ranked list --format alchemy --output json --limit 2
```

The JSON output includes the full response from the Contentful API with all metadata.

### Use Cases

1. **Tournament Preparation**: Research recent tournament-winning deck lists
2. **Meta Analysis**: Track popular decks in specific formats
3. **Deck Building Inspiration**: Find competitive deck ideas
4. **Historical Research**: Access archived tournament results
5. **Deck Collection**: Parse and cache actual deck lists from tournament articles

### Integration

```bash
# Get all Standard deck lists and extract URLs
mtg decks ranked list --format standard --output json | jq -r '.items[].fields.outbound_link'

# Count deck lists by format
for format in standard modern legacy; do
    count=$(mtg decks ranked list --format $format --output json | jq '.total')
    echo "$format: $count deck lists"
done

# Fetch and parse deck lists from a specific article
mtg decks ranked show a1b2c3d4e5f6g7h8 --output json | jq '.decks[].title'

# Extract all main deck cards from an article
mtg decks ranked show a1b2c3d4e5f6g7h8 --output json | jq -r '.decks[].main_deck[] | "\(.quantity)x \(.name)"'

# Get deck IDs for further analysis
mtg decks ranked show "https://magic.gg/decklists/some-tournament" --output json | jq -r '.decks[].id'

# Complete workflow: List tournaments, fetch decks, and analyze them
# 1. Get tournament article ID
ARTICLE_ID=$(mtg decks ranked list --limit 1 --output json | jq -r '.items[0].id')

# 2. Option A: Analyze first deck directly with article ID
mtg decks stats $ARTICLE_ID

# 2. Option B: Get specific deck ID and analyze
DECK_ID=$(mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[0].id')
mtg decks stats $DECK_ID

# Batch analyze all decks from a tournament
mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[].id' | while read deck_id; do
    echo "Analyzing deck: $deck_id"
    mtg decks stats $deck_id --format json | jq '.statistics.average_mana_value'
done

# Compare two decks from the same tournament
DECK1=$(mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[0].id')
DECK2=$(mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[1].id')
mtg decks compare $DECK1 $DECK2

# Compare decks from different tournaments
ARTICLE1=$(mtg decks ranked list --limit 1 --skip 0 --output json | jq -r '.items[0].id')
ARTICLE2=$(mtg decks ranked list --limit 1 --skip 1 --output json | jq -r '.items[0].id')
mtg decks compare $ARTICLE1 $ARTICLE2
```

### Caching

All deck commands utilize caching to improve performance:

1. **List Command**: Caches each tournament article metadata with a unique ID
2. **Show Command**: 
   - Can retrieve article URLs from cached IDs
   - Caches each parsed deck with its own unique ID
   - Cached decks can be retrieved later for analysis
3. **Stats Command**:
   - Can analyze decks directly from their cached ID
   - Avoids re-parsing deck lists for repeated analysis
   - Still fetches fresh card details from Scryfall API

The cache is stored in `~/.local/share/mtg/cache/` with each item having a 16-character hash ID.

#### ID Formats

The stats command accepts two types of IDs:

1. **Deck ID**: 16-character hexadecimal string (e.g., `7d1d96bc86e2185c`)
   - Generated from the SHA-256 hash of the deck's content
   - Directly retrieves a specific deck from cache
   - Obtained from `decks ranked show` output

2. **Article ID**: 16-character hexadecimal string (e.g., `6b9a732534c4294a`)
   - Generated from tournament article metadata
   - Fetches all decks from the article if not cached
   - Analyzes the first deck and lists others with their IDs
   - Obtained from `decks ranked list` output

The command automatically determines the ID type and handles it appropriately:
- If it's a cached deck ID, it uses it directly
- If it's an article ID, it fetches the article and analyzes the first deck
- If the ID is not found in either category, it returns an error

### Compare Examples

```bash
# Compare two specific deck IDs
mtg decks compare 7d1d96bc86e2185c a2b3c4d5e6f7g8h9

# Compare using article IDs (uses first deck from each article)
mtg decks compare 6b9a732534c4294a 8c9d0e1f2a3b4c5d

# Compare decks from the same tournament
ARTICLE_ID="6b9a732534c4294a"
DECK1=$(mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[0].id')
DECK2=$(mtg decks ranked show $ARTICLE_ID --output json | jq -r '.decks[1].id')
mtg decks compare $DECK1 $DECK2
```

#### Sample Output

```
Deck Comparison
Deck 1: Mono Red Aggro
Deck 2: Boros Burn

Summary
Shared cards: 5
Unique to Deck 1: 8
Unique to Deck 2: 10

Shared Cards
 Card Name             Deck 1 (Main/Side)  Deck 2 (Main/Side)  Difference 
 Lightning Bolt        4/0                 4/0                 = 
 Monastery Swiftspear  4/0                 4/0                 = 
 Lava Spike           4/0                 3/0                 ±1 
 Mountain             20/0                10/0                ±10 
 Skullcrack           0/4                 0/3                 ±1 

Unique to Mono Red Aggro
 Card Name          Main  Side  Total 
 Ghitu Lavarunner   4     0     4 
 Rift Bolt          4     0     4 
 Searing Blood      0     3     3 

Unique to Boros Burn
 Card Name          Main  Side  Total 
 Sacred Foundry     4     0     4 
 Boros Charm        4     0     4 
 Path to Exile      0     2     2 
```

## Tips

1. **Large Decks**: For decks with many unique cards, the analysis may take longer due to API calls
2. **Offline Mode**: Currently requires internet connection for card details
3. **Set Codes**: Including set codes helps ensure correct card versions are analyzed
4. **Format Validation**: Use the format legality check before tournament play
5. **JSON Output**: Use JSON format for programmatic processing and integration
6. **Tournament Data**: Use `ranked list` to find articles, then `ranked show` to parse actual deck lists
7. **Cached IDs**: The ID shown in `ranked list` can be used with `ranked show` to avoid re-fetching URLs
8. **Pagination**: Use `--page` for easier navigation or `--skip` for precise control
9. **Deck Comparison**: Use `compare` to quickly identify differences between similar decks or track meta evolution

---

Next: [Scryfall Commands](./scryfall.md) | Back: [Getting Started](./getting-started.md)