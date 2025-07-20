# MTG CLI Workflow Guide

This guide demonstrates common workflows for using the MTG CLI effectively, including how to go from search results to detailed card information.

## Basic Workflow: Search to Card Details

The most common workflow involves searching for cards and then getting detailed information about specific cards that interest you. The MTG CLI provides three different search engines, each with their own strengths:

- **MTG API** (`mtg api cards`): Official API with structured filters
- **Gatherer** (`mtg gatherer`): Wizards' official database with advanced search
- **Scryfall** (`mtg scryfall`): Powerful query syntax and comprehensive data

### 1. Search for Cards

#### Using Gatherer (Wizards' Official Database)

Use the `gatherer search` command to find cards matching your criteria:

```bash
# Search broadly by name
mtg gatherer search --name "Lightning" --pretty

# Search with specific filters
mtg gatherer search --card-type "Creature" --colors "R" --rarity "Rare" --pretty

# Search for cards in a specific set
mtg gatherer search --set "Magic: The Gathering—FINAL FANTASY" --pretty

# Search by game mechanics
mtg gatherer search --rules "flying" --card-type "Creature" --pretty

# Use boolean operators (OR with comma, AND with plus)
mtg gatherer search --supertype "Legendary,Snow" --card-type "Creature+Artifact" --pretty

# Exclude colors with ! prefix
mtg gatherer search --colors "!RBW" --card-type "Creature" --pretty

# Search with power/toughness ranges
mtg gatherer search --power "5-10" --toughness "2-5" --pretty

# Complex format legality queries
mtg gatherer search --format "Legal:Modern,Banned:Legacy" --pretty
```

#### Using Scryfall (Powerful Query Syntax)

Scryfall provides a more flexible query syntax that many players prefer:

```bash
# Basic name search
mtg scryfall search "Lightning Bolt" --pretty

# Color and type combinations
mtg scryfall search "c:red t:creature" --pretty

# Mana value and power/toughness
mtg scryfall search "c:blue t:instant mv<=2" --pretty

# Complex queries with multiple conditions
mtg scryfall search "c:wu t:creature mv<=3" --pretty

# Format legality
mtg scryfall search "f:modern c:red t:creature" --pretty

# Advanced syntax with operators
mtg scryfall search "c>=bant t:creature pow>=3" --pretty

# Oracle text search
mtg scryfall search "o:flying o:vigilance" --pretty

# Set and rarity filters
mtg scryfall search "s:war r:mythic" --pretty

# Using advanced search with individual filters
mtg scryfall advanced --colors "wu" --card-type "creature" --mv "<=3" --pretty
```

### 2. Copy Card Names from Results

From the search results, identify cards you want to learn more about. The search shows a table with card names in the first column:

```
 Name                          Type                                Cost  Set                                           Rarity       P/T/L
 Lightning Angel               Creature – Angel                    1URW  From the Vault: Angels                        Mythic Rare  3/4
 Lightning Bolt                Instant                             R     FINAL FANTASY Through the Ages                Uncommon     -
 Lightning Dragon              Creature – Dragon                   2RR   Vintage Masters                               Rare         4/4
```

### 3. Get Detailed Card Information

Use the exact card name from the search results to get detailed information:

#### Using Gatherer
```bash
# Get detailed information in table format
mtg gatherer card "Lightning Angel" --pretty

# Get complete JSON data for programmatic use
mtg gatherer card "Lightning Angel"

# Works with complex names (use quotes)
mtg gatherer card "Lightning, Army of One" --pretty
```

#### Using Scryfall
```bash
# Get detailed information in table format
mtg scryfall named "Lightning Bolt" --pretty

# Get complete JSON data for programmatic use
mtg scryfall named "Lightning Bolt"

# Get specific printing from a set
mtg scryfall named "Lightning Bolt" --set "lea" --pretty

# Get card by Scryfall ID
mtg scryfall id "56ebc372-aabd-4174-a943-c7bf59e5028d" --pretty

# Get card by set and collector number
mtg scryfall collector ktk 96 --pretty

# Get card by Arena ID
mtg scryfall arena 67330 --pretty
```

## Choosing the Right Search Engine

Each search engine has different strengths. Here's when to use each:

### MTG API (`mtg api cards`)
- **Best for**: Structured searches with specific filters
- **Strengths**: Official API, reliable, good for programmatic use
- **Use when**: You need specific filters like exact mana cost, power/toughness ranges

```bash
# Structured filtering
mtg api cards list --colors "Red,Blue" --type "Creature" --cmc 4 --pretty
```

### Gatherer (`mtg gatherer`)
- **Best for**: Official Wizards data, complex boolean searches
- **Strengths**: Most authoritative source, advanced search capabilities
- **Use when**: You need the most official/accurate data or complex search logic

```bash
# Complex boolean searches
mtg gatherer search --supertype "Legendary,Snow" --card-type "Creature+Artifact" --pretty
```

### Scryfall (`mtg scryfall`)
- **Best for**: Flexible queries, modern search syntax, comprehensive data, multiple lookup methods
- **Strengths**: Intuitive syntax, fast, excellent for exploration, ID-based lookups, autocomplete
- **Use when**: You want flexible search syntax, need specific card lookups, or are familiar with Scryfall's query language

```bash
# Intuitive query syntax
mtg scryfall search "c:wu t:creature mv<=3 f:modern" --pretty

# Multiple lookup methods
mtg scryfall named "Lightning Bolt" --pretty
mtg scryfall collector ktk 96 --pretty
mtg scryfall random --query "t:legendary" --pretty
mtg scryfall autocomplete "lightning"
```

## Advanced Workflows

### Filtering and Refinement

Start with broad searches and progressively narrow down:

#### Using Gatherer
```bash
# 1. Start broad
mtg gatherer search --card-type "Planeswalker" --pretty

# 2. Add color filter
mtg gatherer search --card-type "Planeswalker" --colors "U" --pretty

# 3. Add format restriction
mtg gatherer search --card-type "Planeswalker" --colors "U" --format "Legal:Modern" --pretty

# 4. Get details for interesting cards
mtg gatherer card "Jace, the Mind Sculptor" --pretty
```

#### Using Scryfall
```bash
# 1. Start broad
mtg scryfall search "t:planeswalker" --pretty

# 2. Add color filter
mtg scryfall search "t:planeswalker c:blue" --pretty

# 3. Add format restriction
mtg scryfall search "t:planeswalker c:blue f:modern" --pretty

# 4. Get details for interesting cards
mtg scryfall named "Jace, the Mind Sculptor" --pretty
```

### Set Exploration

Explore cards from specific Magic sets:

#### Using Gatherer
```bash
# 1. Find all mythic rares in a set
mtg gatherer search --set "War of the Spark" --rarity "Mythic" --pretty

# 2. Look at specific card types
mtg gatherer search --set "War of the Spark" --card-type "Planeswalker" --pretty

# 3. Get details for specific cards
mtg gatherer card "Nicol Bolas, Dragon-God" --pretty
```

#### Using Scryfall
```bash
# 1. Find all mythic rares in a set
mtg scryfall search "s:war r:mythic" --pretty

# 2. Look at specific card types
mtg scryfall search "s:war t:planeswalker" --pretty

# 3. Get details for specific cards
mtg scryfall named "Nicol Bolas, Dragon-God" --pretty
```

### Deck Building Research

Research cards for specific deck archetypes:

#### Using Gatherer
```bash
# 1. Find aggressive red creatures
mtg gatherer search --card-type "Creature" --colors "R" --power "2" --mana-cost "{1}" --pretty

# 2. Look for control win conditions
mtg gatherer search --card-type "Planeswalker" --colors "UW" --format "Legal:Standard" --pretty

# 3. Research specific cards
mtg gatherer card "Teferi, Hero of Dominaria" --pretty
```

#### Using Scryfall
```bash
# 1. Find aggressive red creatures
mtg scryfall search "c:red t:creature mv<=2 pow>=2" --pretty

# 2. Look for control win conditions
mtg scryfall search "c:wu t:planeswalker f:standard" --pretty

# 3. Research specific cards
mtg scryfall named "Teferi, Hero of Dominaria" --pretty
```

### Format-Specific Research

#### Standard Legal Cards
```bash
# Scryfall: Current Standard creatures
mtg scryfall search "f:standard t:creature mv<=3" --pretty

# Gatherer: Standard legal planeswalkers
mtg gatherer search --format "Legal:Standard" --card-type "Planeswalker" --pretty
```

#### Commander Research
```bash
# Scryfall: Find potential commanders
mtg scryfall search "is:commander c:wu" --pretty

# Scryfall: Cards legal in commander with specific color identity
mtg scryfall search "id<=esper f:commander" --pretty
```

#### Modern Staples
```bash
# Scryfall: Modern legal instants and sorceries
mtg scryfall search "f:modern (t:instant or t:sorcery) mv<=2" --pretty
```

## Enhanced Scryfall Workflows

Scryfall provides multiple lookup methods and powerful query syntax for comprehensive card research:

### ID-Based Lookup Workflows

#### Collection Management
```bash
# Look up cards from different sources by their IDs
mtg scryfall arena 67330 --pretty        # From Arena deck export
mtg scryfall mtgo 12345 --pretty         # From MTGO collection
mtg scryfall tcgplayer 98765 --pretty    # From TCGPlayer order
mtg scryfall cardmarket 54321 --pretty   # From Cardmarket data

# Get specific printings by set and collector number
mtg scryfall collector ktk 96 --pretty   # Khans of Tarkir #96
mtg scryfall collector war "★1" --pretty # War of the Spark special
mtg scryfall collector dom 1 --lang ja --pretty # Japanese version
```

#### API Integration Workflow
```bash
# Get card by Scryfall UUID (from API responses)
mtg scryfall id "56ebc372-aabd-4174-a943-c7bf59e5028d" --pretty

# Export card data for applications
mtg scryfall named "Lightning Bolt" > lightning_bolt.json
mtg scryfall collector ktk 96 > ainok_tracker.json
```

#### Discovery and Research
```bash
# Get random cards for inspiration
mtg scryfall random --pretty
mtg scryfall random --query "t:legendary t:creature" --pretty
mtg scryfall random --query "c:bant f:commander" --pretty

# Get autocomplete suggestions for deck building
mtg scryfall autocomplete "lightning"
mtg scryfall autocomplete "jace" --include-extras
```

### Enhanced Search Workflows

#### Market Research with Enhanced Sorting
```bash
# Find cards by price
mtg scryfall search "f:standard" --order usd --pretty
mtg scryfall search "t:creature" --order tix --dir desc --pretty

# Find popular cards
mtg scryfall search "t:creature" --order edhrec --pretty
mtg scryfall search "f:modern" --order penny --pretty

# Sort by different criteria
mtg scryfall search "c:red t:creature" --order power --dir desc --pretty
mtg scryfall search "t:planeswalker" --order released --pretty
```

#### Data Export and Analysis
```bash
# Export search results as CSV for analysis
mtg scryfall search "f:standard" --csv > standard_cards.csv
mtg scryfall search "c:red t:creature mv<=3" --csv > red_aggro.csv

# Include different card types and variations
mtg scryfall search "Lightning Bolt" --unique prints --pretty
mtg scryfall search "t:token" --include-extras --pretty
mtg scryfall search "Lightning Bolt" --include-multilingual --pretty
```

### Advanced Query Patterns

Scryfall's query syntax enables unique search patterns that aren't easily achievable with other engines:

### Advanced Scryfall Queries

#### Power/Toughness Relationships
```bash
# Creatures where power > toughness
mtg scryfall search "t:creature pow>tou" --pretty

# Creatures with equal power and toughness
mtg scryfall search "t:creature pow=tou" --pretty

# High power, low toughness creatures
mtg scryfall search "t:creature pow>=4 tou<=2" --pretty
```

#### Mana Cost Patterns
```bash
# Cards with X in their mana cost
mtg scryfall search "m:x" --pretty

# Cards with hybrid mana
mtg scryfall search "is:hybrid" --pretty

# Cards with Phyrexian mana
mtg scryfall search "is:phyrexian" --pretty

# Expensive spells (high mana value)
mtg scryfall search "mv>=8 (t:instant or t:sorcery)" --pretty
```

#### Special Card Properties
```bash
# Double-faced cards
mtg scryfall search "is:dfc" --pretty

# Split cards
mtg scryfall search "is:split" --pretty

# Cards that can be your commander
mtg scryfall search "is:commander c:bant" --pretty

# Reserved list cards
mtg scryfall search "is:reserved" --pretty
```

#### Text and Flavor Searches
```bash
# Cards mentioning specific keywords in oracle text
mtg scryfall search "o:flying o:vigilance" --pretty

# Cards with specific flavor text
mtg scryfall search "ft:jace" --pretty

# Cards by specific artist
mtg scryfall search "a:\"Rebecca Guay\"" --pretty
```

#### Complex Boolean Logic
```bash
# Either red creatures OR blue instants
mtg scryfall search "(c:red t:creature) or (c:blue t:instant)" --pretty

# Legendary creatures that are NOT red
mtg scryfall search "t:legendary t:creature -c:red" --pretty

# Artifacts that are either equipment OR vehicles
mtg scryfall search "t:artifact (t:equipment or t:vehicle)" --pretty
```

### Scryfall Sorting and Display Options

```bash
# Sort by different criteria
mtg scryfall search "c:red t:creature" --order cmc --pretty
mtg scryfall search "t:planeswalker" --order usd --pretty
mtg scryfall search "c:blue" --order power --dir desc --pretty

# Include extra cards (tokens, etc.)
mtg scryfall search "t:token" --include-extras --pretty

# Show different printings
mtg scryfall search "Lightning Bolt" --unique prints --pretty
```

## Interactive Workflow with fzf

For users who have `fzf` (fuzzy finder) installed, here are powerful bash functions that combine search and card selection into an interactive workflow. These work with both Gatherer and Scryfall:

### Installation

First, install the required dependencies:

```bash
# macOS with Homebrew
brew install fzf jq

# Ubuntu/Debian
sudo apt install fzf jq

# Arch Linux
sudo pacman -S fzf jq
```

**Note**: While `fzf` is required, `jq` is highly recommended. The basic `mtg_card_search` function has a fallback for systems without `jq`, but the advanced `mtg_card_browse` function requires it for parsing JSON responses.

### Bash Function

Add this function to your `~/.bashrc`, `~/.zshrc`, or `~/.bash_profile`:

```bash
# Interactive MTG card search and selection
mtg_card_search() {
    local search_args=()
    local pretty_flag=""
    local help_text="Usage: mtg_card_search [search_options] [--pretty]

Search for MTG cards interactively using fzf.

Examples:
  mtg_card_search --name Lightning
  mtg_card_search --card-type Creature --colors R --pretty
  mtg_card_search --set \"War of the Spark\" --rarity Mythic

All gatherer search options are supported. Add --pretty for final card display."

    # Parse arguments to separate --pretty flag
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                echo "$help_text" 1>&2
                return 0
                ;;
            --pretty)
                pretty_flag="--pretty"
                shift
                ;;
            *)
                search_args+=("$1")
                shift
                ;;
        esac
    done

    # Check if mtg command exists
    if ! command -v mtg &> /dev/null; then
        echo "Error: 'mtg' command not found. Please install the MTG CLI first." 1>&2
        return 1
    fi

    # Check if fzf exists
    if ! command -v fzf &> /dev/null; then
        echo "Error: 'fzf' not found. Please install fzf first:" 1>&2
        echo "  brew install fzf    # macOS" 1>&2
        echo "  sudo apt install fzf    # Ubuntu/Debian" 1>&2
        return 1
    fi

    # Check if jq exists
    if ! command -v jq &> /dev/null; then
        echo "Error: 'jq' not found. Please install jq first:" 1>&2
        echo "  brew install jq    # macOS" 1>&2
        echo "  sudo apt install jq    # Ubuntu/Debian" 1>&2
        return 1
    fi

    echo "Searching for cards..." 1>&2

    # Get search results in JSON format (without --pretty)
    # Use a temporary file to avoid issues with large outputs
    local tmpfile=$(mktemp)
    trap "rm -f $tmpfile" EXIT

    if ! mtg gatherer search "${search_args[@]}" > "$tmpfile" 2>&1; then
        echo "Error: Search failed. Please check your search parameters." 1>&2
        cat "$tmpfile" 1>&2
        return 1
    fi

    # Extract card names using jq
    local card_names
    card_names=$(jq -r '.items[]?.instanceName // empty' < "$tmpfile" 2>/dev/null)

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your search criteria." 1>&2
        echo "Debug info: Check if the search returned valid JSON:" 1>&2
        jq '.' < "$tmpfile" 2>&1 | head -20 1>&2
        return 1
    fi

    # Count results
    local card_count
    card_count=$(echo "$card_names" | wc -l | tr -d ' ')

    echo "Found $card_count cards. Use fzf to select one..." 1>&2
    echo "Press Ctrl+C to cancel, Enter to select, or type to filter." 1>&2
    echo "" 1>&2

    # Create a preview command that shows card details
    local preview_cmd='mtg gatherer card --pretty {}'

    # Use fzf to select a card
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=80% \
        --border \
        --prompt="Select a card: " \
        --preview="$preview_cmd" \
        --preview-window=right:60% \
        --header="Found $card_count cards - Press Enter to select, Ctrl+C to cancel")

    # Check if user cancelled
    if [[ -z "$selected_card" ]]; then
        echo "Selection cancelled." 1>&2
        return 0
    fi

    echo "" 1>&2
    echo "Getting details for: $selected_card" 1>&2
    echo "----------------------------------------" 1>&2

    # Get detailed card information
    mtg gatherer card "$selected_card" $pretty_flag
}

# Shorter alias for convenience
alias mtgf='mtg_card_search'

# Interactive Scryfall search function
scryfall_search() {
    local search_args=()
    local pretty_flag=""
    local help_text="Usage: scryfall_search [search_query] [--pretty]

Search for MTG cards interactively using Scryfall and fzf.

Examples:
  scryfall_search \"c:red t:creature\"
  scryfall_search \"Lightning Bolt\" --pretty
  scryfall_search \"f:modern c:wu t:creature mv<=3\"

All Scryfall query syntax is supported. Add --pretty for final card display."

    # Parse arguments to separate --pretty flag
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                echo "$help_text" 1>&2
                return 0
                ;;
            --pretty)
                pretty_flag="--pretty"
                shift
                ;;
            *)
                search_args+=("$1")
                shift
                ;;
        esac
    done

    # Check if mtg command exists
    if ! command -v mtg &> /dev/null; then
        echo "Error: 'mtg' command not found. Please install the MTG CLI first." 1>&2
        return 1
    fi

    # Check if fzf exists
    if ! command -v fzf &> /dev/null; then
        echo "Error: 'fzf' not found. Please install fzf first:" 1>&2
        echo "  brew install fzf    # macOS" 1>&2
        echo "  sudo apt install fzf    # Ubuntu/Debian" 1>&2
        return 1
    fi

    # Check if jq exists
    if ! command -v jq &> /dev/null; then
        echo "Error: 'jq' not found. Please install jq first:" 1>&2
        echo "  brew install jq    # macOS" 1>&2
        echo "  sudo apt install jq    # Ubuntu/Debian" 1>&2
        return 1
    fi

    # Combine all search arguments into a single query
    local query="${search_args[*]}"
    if [[ -z "$query" ]]; then
        echo "Error: Please provide a search query." 1>&2
        echo "$help_text" 1>&2
        return 1
    fi

    echo "Searching Scryfall for: $query" 1>&2

    # Get search results in JSON format
    local tmpfile=$(mktemp)
    trap "rm -f $tmpfile" EXIT

    if ! mtg scryfall search "$query" > "$tmpfile" 2>&1; then
        echo "Error: Search failed. Please check your query syntax." 1>&2
        cat "$tmpfile" 1>&2
        return 1
    fi

    # Extract card names using jq
    local card_names
    card_names=$(jq -r '.data[]?.name // empty' < "$tmpfile" 2>/dev/null)

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your query: $query" 1>&2
        return 1
    fi

    # Count results
    local card_count
    card_count=$(echo "$card_names" | wc -l | tr -d ' ')

    echo "Found $card_count cards. Use fzf to select one..." 1>&2
    echo "Press Ctrl+C to cancel, Enter to select, or type to filter." 1>&2
    echo "" 1>&2

    # Create a preview command that shows card details
    local preview_cmd='mtg scryfall named --pretty {}'

    # Use fzf to select a card
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=80% \
        --border \
        --prompt="Select a card: " \
        --preview="$preview_cmd" \
        --preview-window=right:60% \
        --header="Found $card_count cards - Press Enter to select, Ctrl+C to cancel")

    # Check if user cancelled
    if [[ -z "$selected_card" ]]; then
        echo "Selection cancelled." 1>&2
        return 0
    fi

    echo "" 1>&2
    echo "Getting details for: $selected_card" 1>&2
    echo "----------------------------------------" 1>&2

    # Get detailed card information
    mtg scryfall named "$selected_card" $pretty_flag
}

# Shorter alias for convenience
alias scryfallf='scryfall_search'
```

### Usage Examples

After adding the functions to your shell configuration and reloading it (`source ~/.bashrc` or restart your terminal), you can use:

#### Gatherer Interactive Search (mtg_card_search / mtgf)

```bash
# Basic search with interactive selection
mtg_card_search --name "Lightning"

# Search creatures with interactive selection and pretty output
mtg_card_search --card-type "Creature" --colors "R" --pretty

# Search specific set
mtg_card_search --set "War of the Spark" --rarity "Mythic"

# Use boolean operators
mtg_card_search --supertype "Legendary,Snow" --card-type "Creature+Artifact"

# Use the shorter alias
mtgf --card-type "Planeswalker" --colors "U" --pretty

# Get help
mtg_card_search --help
```

#### Scryfall Interactive Search (scryfall_search / scryfallf)

```bash
# Basic search with interactive selection
scryfall_search "Lightning Bolt"

# Search with Scryfall syntax
scryfall_search "c:red t:creature mv<=3" --pretty

# Complex queries
scryfall_search "f:modern c:wu t:creature"

# Advanced syntax
scryfall_search "pow>tou c:green" --pretty

# Use the shorter alias
scryfallf "is:commander c:bant" --pretty

# Get help
scryfall_search --help
```

#### Advanced Browse with Pagination (mtg_card_browse / mtgb)

```bash
# Browse all creatures with pagination
mtg_card_browse --card-type "Creature"

# Start from a specific page
mtg_card_browse --rarity "Mythic" --page 3

# Browse with complex queries
mtg_card_browse --colors "!RBW" --power "5-10" --pretty

# Use the shorter alias
mtgb --format "Legal:Modern,Banned:Legacy" --pretty

# Get help
mtg_card_browse --help
```

### How the Interactive Functions Work

#### mtg_card_search (Simple Version)

1. **Search**: Runs your search query using `mtg gatherer search` (JSON format)
2. **Extract**: Extracts card names from the JSON results
3. **Preview**: Shows card details in the preview pane while browsing
4. **Select**: Uses `fzf` to present an interactive, searchable list
5. **Details**: Automatically runs `mtg gatherer card` on your selection

#### mtg_card_browse (Advanced Version)

1. **Paginated Search**: Handles multi-page results with navigation
2. **Rich Display**: Shows card type, set, and rarity in the list
3. **Navigation**: Allows moving between pages within fzf
4. **Live Preview**: Shows card details while browsing
5. **Smart Selection**: Handles both navigation and card selection

### Benefits of the Interactive Workflow

- **Fast filtering**: Type to instantly filter the card list
- **Visual selection**: See all matching cards at once with live preview
- **No copy-paste**: Automatically handles card name extraction
- **Pagination support**: Browse through large result sets easily
- **Rich information**: See card details before selecting
- **Flexible**: Supports all search parameters and boolean operators
- **Efficient**: Combines search, browse, and selection in one command

## Tips and Best Practices

### Card Name Handling

- **Always use quotes** for card names with spaces: `"Lightning Bolt"`
- **Include punctuation** exactly as shown: `"Jace, the Mind Sculptor"`
- **Case doesn't matter**: `"lightning bolt"` works the same as `"Lightning Bolt"`

### Search Engine Selection

- **Use Scryfall** for flexible, intuitive queries and modern search patterns
- **Use Gatherer** for official Wizards data and complex boolean logic
- **Use MTG API** for structured, programmatic searches with specific filters

### Search Optimization

#### General Tips
- **Start broad, then narrow**: Begin with general searches and add filters
- **Use pagination**: Large result sets are paginated; use `--page` to navigate
- **Combine filters**: Use multiple parameters to find exactly what you need

#### Scryfall-Specific Tips
- **Learn the syntax**: Scryfall's query language is very powerful once you know it
- **Use operators**: `>=`, `<=`, `>`, `<`, `=`, `!=` work with numbers
- **Combine with OR**: Use `(condition1) or (condition2)` for complex logic
- **Negate with `-`**: Use `-c:red` to exclude red cards

#### Gatherer-Specific Tips
- **Use boolean operators**: Comma for OR, plus for AND in multi-value fields
- **Exact matching**: Gatherer is more strict about exact matches
- **Format syntax**: Use "Legal:Format" or "Banned:Format" for legality

### Output Formats

- **Pretty tables** (`--pretty`): Best for human reading and browsing
- **JSON format** (default): Best for scripting and programmatic use
- **Consistent data**: Both formats contain the same information

### Performance Tips

- **Specific searches** are faster than broad ones
- **Use set filters** when looking for cards from specific releases
- **Cache results** in scripts by saving JSON output to files
- **Scryfall is generally faster** for most queries due to its optimized API

## Integration with Other Tools

### Shell Scripting

#### Gatherer
```bash
# Save search results for later processing
mtg gatherer search --card-type "Creature" --colors "R" > red_creatures.json

# Extract specific information
cat red_creatures.json | jq '.items[].instanceName' | sort
```

#### Scryfall
```bash
# Save Scryfall search results
mtg scryfall search "c:red t:creature" > red_creatures_scryfall.json

# Extract card names from Scryfall results
cat red_creatures_scryfall.json | jq '.data[].name' | sort

# Get specific card data
cat red_creatures_scryfall.json | jq '.data[] | {name: .name, mana_cost: .mana_cost, cmc: .cmc}'
```

### Text Processing

#### Gatherer
```bash
# Find cards with specific rules text
mtg gatherer search --rules "flying" --pretty | grep "Creature"

# Count cards by rarity
mtg gatherer search --card-type "Creature" | jq '.items | group_by(.rarityName) | map({rarity: .[0].rarityName, count: length})'
```

#### Scryfall
```bash
# Find cards with specific oracle text
mtg scryfall search "o:flying" --pretty | grep "Creature"

# Count cards by rarity from Scryfall
mtg scryfall search "t:creature" | jq '.data | group_by(.rarity) | map({rarity: .[0].rarity, count: length})'

# Extract mana costs and analyze
mtg scryfall search "c:red t:creature" | jq '.data[].mana_cost' | sort | uniq -c
```

### Advanced Data Analysis

```bash
# Compare card counts between different sources
echo "Gatherer red creatures:"
mtg gatherer search --card-type "Creature" --colors "R" | jq '.totalItems'

echo "Scryfall red creatures:"
mtg scryfall search "c:red t:creature" | jq '.total_cards'

# Find cards that exist in one source but not another
mtg gatherer search --card-type "Creature" --colors "R" | jq -r '.items[].instanceName' | sort > gatherer_red.txt
mtg scryfall search "c:red t:creature" | jq -r '.data[].name' | sort > scryfall_red.txt
comm -23 gatherer_red.txt scryfall_red.txt  # Cards in Gatherer but not Scryfall
```

This workflow documentation provides a comprehensive guide for efficiently using the MTG CLI to research and explore Magic: The Gathering cards.

## Troubleshooting the Interactive Functions

If the `mtg_card_search` or `mtg_card_browse` functions aren't working:

### Common Issues and Solutions

1. **"No cards found" when cards should exist**

   - Make sure the `mtg` command is in your PATH
   - Check that the search works directly: `mtg gatherer search --name "Vivi"`
   - Ensure you're not redirecting stderr: the function needs both stdout and stderr

2. **JSON parsing errors**

   - Verify jq is installed: `which jq`
   - Test the search returns valid JSON: `mtg gatherer search --name "Vivi" | jq .`
   - Check for error messages in the search output

3. **Function not found**
   - Ensure you've sourced your shell config: `source ~/.bashrc` or `source ~/.zshrc`
   - Verify the function is loaded: `type mtg_card_search`

### Debug Mode

Add this debug version to help troubleshoot issues:

```bash
mtg_card_search_debug() {
    echo "Debug: Running search with args: $@" 1>&2
    local results=$(mtg gatherer search "$@" 2>&1)
    echo "Debug: Exit code: $?" 1>&2
    echo "Debug: First 500 chars of output:" 1>&2
    echo "${results:0:500}" 1>&2
    echo "Debug: Attempting to parse JSON..." 1>&2
    echo "$results" | jq '.items | length' 1>&2
    echo "Debug: Card names:" 1>&2
    echo "$results" | jq -r '.items[]?.instanceName' | head -5 1>&2
}

# Minimal working version - no error checking, just the basics
mtg_card_search_simple() {
    local tmpfile=$(mktemp)
    mtg gatherer search "$@" > "$tmpfile" 2>/dev/null
    local names=$(jq -r '.items[]?.instanceName' < "$tmpfile" 2>/dev/null)
    rm -f "$tmpfile"

    if [[ -z "$names" ]]; then
        echo "No cards found" 1>&2
        return 1
    fi

    local selected=$(echo "$names" | fzf --height=80% --border)
    if [[ -n "$selected" ]]; then
        mtg gatherer card "$selected" --pretty
    fi
}
```

### Alternative: Using Process Substitution

If the temporary file approach doesn't work, try this version using process substitution:

```bash
mtg_card_search_alt() {
    local search_args=()
    local pretty_flag=""

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --pretty)
                pretty_flag="--pretty"
                shift
                ;;
            *)
                search_args+=("$1")
                shift
                ;;
        esac
    done

    echo "Searching for cards..." 1>&2

    # Use process substitution to avoid variable issues
    local card_names
    card_names=$(mtg gatherer search "${search_args[@]}" 2>/dev/null | jq -r '.items[]?.instanceName // empty')

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your search criteria." 1>&2
        return 1
    fi

    # Use fzf to select
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=80% \
        --border \
        --prompt="Select a card: " \
        --preview="mtg scryfall named '{}' --pretty 2>/dev/null | head -20" \
        --preview-window=right:60%)

    if [[ -n "$selected_card" ]]; then
        echo "Getting details for: $selected_card" 1>&2
        mtg scryfall named "$selected_card" $pretty_flag
    fi
}
```
