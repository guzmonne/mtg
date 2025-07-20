# MTG CLI Workflow Guide

This guide demonstrates common workflows for using the MTG CLI effectively, including how to go from search results to detailed card information.

## Basic Workflow: Search to Card Details

The most common workflow involves searching for cards and then getting detailed information about specific cards that interest you.

### 1. Search for Cards

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

Use the exact card name from the search results with the `gatherer card` command:

```bash
# Get detailed information in table format
mtg gatherer card "Lightning Angel" --pretty

# Get complete JSON data for programmatic use
mtg gatherer card "Lightning Angel"

# Works with complex names (use quotes)
mtg gatherer card "Lightning, Army of One" --pretty
```

## Advanced Workflows

### Filtering and Refinement

Start with broad searches and progressively narrow down:

```bash
# 1. Start broad
mtg gatherer search --card-type "Planeswalker" --pretty

# 2. Add color filter
mtg gatherer search --card-type "Planeswalker" --colors "U" --pretty

# 3. Add format restriction
mtg gatherer search --card-type "Planeswalker" --colors "U" --format "Modern" --pretty

# 4. Get details for interesting cards
mtg gatherer card "Jace, the Mind Sculptor" --pretty
```

### Set Exploration

Explore cards from specific Magic sets:

```bash
# 1. Find all mythic rares in a set
mtg gatherer search --set "War of the Spark" --rarity "Mythic" --pretty

# 2. Look at specific card types
mtg gatherer search --set "War of the Spark" --card-type "Planeswalker" --pretty

# 3. Get details for specific cards
mtg gatherer card "Nicol Bolas, Dragon-God" --pretty
```

### Deck Building Research

Research cards for specific deck archetypes:

```bash
# 1. Find aggressive red creatures
mtg gatherer search --card-type "Creature" --colors "R" --power "2" --mana-cost "{1}" --pretty

# 2. Look for control win conditions
mtg gatherer search --card-type "Planeswalker" --colors "UW" --format "Standard" --pretty

# 3. Research specific cards
mtg gatherer card "Teferi, Hero of Dominaria" --pretty
```

## Interactive Workflow with fzf

For users who have `fzf` (fuzzy finder) installed, here's a powerful bash function that combines search and card selection into an interactive workflow:

### Installation

First, install `fzf` if you haven't already:

```bash
# macOS with Homebrew
brew install fzf

# Ubuntu/Debian
sudo apt install fzf

# Arch Linux
sudo pacman -S fzf
```

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

All gatherer search options are supported. Add --pretty for table output."

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                echo "$help_text"
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
        echo "Error: 'mtg' command not found. Please install the MTG CLI first."
        return 1
    fi

    # Check if fzf exists
    if ! command -v fzf &> /dev/null; then
        echo "Error: 'fzf' not found. Please install fzf first:"
        echo "  brew install fzf    # macOS"
        echo "  sudo apt install fzf    # Ubuntu/Debian"
        return 1
    fi

    echo "Searching for cards..."

    # Get search results in JSON format
    local search_results
    if ! search_results=$(mtg gatherer search "${search_args[@]}" 2>/dev/null); then
        echo "Error: Search failed. Please check your search parameters."
        return 1
    fi

    # Extract card names using jq (fallback to grep if jq not available)
    local card_names
    if command -v jq &> /dev/null; then
        card_names=$(echo "$search_results" | jq -r '.items[]?.instanceName // empty' 2>/dev/null)
    else
        # Fallback: extract names from JSON without jq
        card_names=$(echo "$search_results" | grep -o '"instanceName": "[^"]*"' | sed 's/"instanceName": "\([^"]*\)"/\1/' | sort -u)
    fi

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your search criteria."
        return 1
    fi

    # Count results
    local card_count
    card_count=$(echo "$card_names" | wc -l | tr -d ' ')

    echo "Found $card_count cards. Use fzf to select one..."
    echo "Press Ctrl+C to cancel, Enter to select, or type to filter."
    echo ""

    # Use fzf to select a card
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=60% \
        --border \
        --prompt="Select a card: " \
        --preview="echo 'Card: {}'" \
        --preview-window=up:1 \
        --header="Found $card_count cards - Press Enter to select, Ctrl+C to cancel")

    # Check if user cancelled
    if [[ -z "$selected_card" ]]; then
        echo "Selection cancelled."
        return 0
    fi

    echo ""
    echo "Getting details for: $selected_card"
    echo "----------------------------------------"

    # Get detailed card information
    mtg gatherer card "$selected_card" $pretty_flag
}

# Shorter alias for convenience
alias mtgf='mtg_card_search'
```

### Usage Examples

After adding the function to your shell configuration and reloading it (`source ~/.bashrc` or restart your terminal), you can use:

```bash
# Basic search with interactive selection
mtg_card_search --name "Lightning"

# Search creatures with interactive selection and pretty output
mtg_card_search --card-type "Creature" --colors "R" --pretty

# Search specific set
mtg_card_search --set "War of the Spark" --rarity "Mythic"

# Use the shorter alias
mtgf --card-type "Planeswalker" --colors "U" --pretty

# Get help
mtg_card_search --help
```

### How the Interactive Function Works

1. **Search**: Runs your search query using `mtg gatherer search`
2. **Extract**: Extracts card names from the JSON results
3. **Select**: Uses `fzf` to present an interactive, searchable list
4. **Details**: Automatically runs `mtg gatherer card` on your selection

### Benefits of the Interactive Workflow

- **Fast filtering**: Type to instantly filter the card list
- **Visual selection**: See all matching cards at once
- **No copy-paste**: Automatically handles card name extraction
- **Flexible**: Supports all search parameters
- **Efficient**: Combines search and selection in one command

## Tips and Best Practices

### Card Name Handling

- **Always use quotes** for card names with spaces: `"Lightning Bolt"`
- **Include punctuation** exactly as shown: `"Jace, the Mind Sculptor"`
- **Case doesn't matter**: `"lightning bolt"` works the same as `"Lightning Bolt"`

### Search Optimization

- **Start broad, then narrow**: Begin with general searches and add filters
- **Use pagination**: Large result sets are paginated; use `--page` to navigate
- **Combine filters**: Use multiple parameters to find exactly what you need

### Output Formats

- **Pretty tables** (`--pretty`): Best for human reading and browsing
- **JSON format** (default): Best for scripting and programmatic use
- **Consistent data**: Both formats contain the same information

### Performance Tips

- **Specific searches** are faster than broad ones
- **Use set filters** when looking for cards from specific releases
- **Cache results** in scripts by saving JSON output to files

## Integration with Other Tools

### Shell Scripting

```bash
# Save search results for later processing
mtg gatherer search --card-type "Creature" --colors "R" > red_creatures.json

# Extract specific information
cat red_creatures.json | jq '.items[].instanceName' | sort
```

### Text Processing

```bash
# Find cards with specific rules text
mtg gatherer search --rules "flying" --pretty | grep "Creature"

# Count cards by rarity
mtg gatherer search --card-type "Creature" | jq '.items | group_by(.rarityName) | map({rarity: .[0].rarityName, count: length})'
```

This workflow documentation provides a comprehensive guide for efficiently using the MTG CLI to research and explore Magic: The Gathering cards.
