# Getting Started

This guide will help you install and start using the MTG CLI.

## Installation

### Prerequisites

- **Rust** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Git** - For cloning the repository

### Building from Source

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd mtg
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

3. **Verify installation:**
   ```bash
   ./target/release/mtg --version
   ```

### Development Build

For development and testing:

```bash
cargo build
./target/debug/mtg --help
```

## First Steps

### 1. Check Help

```bash
mtg --help
```

### 2. Try Each Search Engine

```bash
# Scryfall (recommended) - intuitive syntax
mtg scryfall search "Lightning Bolt" --pretty

# Gatherer - official Wizards database
mtg gatherer search --name "Lightning Bolt" --pretty

# MTG API - structured parameters
mtg api cards search "Lightning Bolt" --pretty
```

### 3. Browse Sets

```bash
mtg api sets list --page-size 5
```

### 4. Get Card Types

```bash
mtg api types list
```

### 5. Set Up Shell Completions

```bash
# Generate completions for your shell
mtg completions generate bash > ~/.local/share/bash-completion/completions/mtg
mtg completions generate zsh > ~/.zsh/completions/_mtg
mtg completions generate fish > ~/.config/fish/completions/mtg.fish
```

## Configuration

### Environment Variables

Set up common configuration using environment variables:

```bash
# Set default API URL (optional)
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"

# Set default timeout
export MTG_TIMEOUT=60

# Enable verbose output
export MTG_VERBOSE=1
```

### Command-line Options

Override defaults for individual commands:

```bash
# Use custom API URL
mtg --api-base-url "https://custom-api.example.com/v1" api cards search "Mox"

# Set longer timeout for slow connections
mtg --timeout 120 api sets list

# Enable verbose output for debugging
mtg --verbose scryfall search "Lightning Bolt"
```

## Choosing a Search Engine

The MTG CLI provides three different search engines:

### 🔍 **Scryfall** (Recommended for most users)
- **Best for**: Flexible searches with intuitive syntax
- **Strengths**: Fast, comprehensive, modern query language
- **Use when**: You want the most user-friendly search experience

```bash
mtg scryfall search "c:red t:creature mv<=3" --pretty
```

### 🏛️ **Gatherer** (Official Wizards Database)
- **Best for**: Official data and complex boolean searches
- **Strengths**: Most authoritative source, advanced filtering
- **Use when**: You need official Wizards data or complex search logic

```bash
mtg gatherer search --card-type "Creature" --colors "R" --power "2-5" --pretty
```

### 🔧 **MTG API** (Structured Searches)
- **Best for**: Programmatic use and specific filters
- **Strengths**: Structured parameters, good for automation
- **Use when**: You need consistent, structured search parameters

```bash
mtg api cards list --colors "Red" --type "Creature" --cmc 3 --pretty
```

## Basic Usage Patterns

### Card Searches

#### MTG API
```bash
# Simple name search
mtg api cards search "Lightning"

# Exact name match
mtg api cards search "Lightning Bolt" --exact

# Search with pagination
mtg api cards search "Dragon" --page 2 --page-size 10

# Search by ID
mtg api cards get 409574
```

#### Scryfall (Recommended)
```bash
# Simple name search
mtg scryfall search "Lightning Bolt" --pretty

# Advanced query syntax
mtg scryfall search "c:red t:creature mv<=3" --pretty

# Complex searches
mtg scryfall search "f:modern c:wu t:creature" --pretty

# Get specific card
mtg scryfall card "Lightning Bolt" --pretty
```

### Gatherer Advanced Search

```bash
# Search by card name with pretty output
mtg gatherer search --name "Lightning Bolt" --pretty

# Find all Legendary creatures
mtg gatherer search --supertype "Legendary" --card-type "Creature" --pretty

# Search for cards that are NOT red, black, or white
mtg gatherer search --colors "!RBW" --pretty

# Complex search with multiple criteria
mtg gatherer search --card-type "Creature,Enchantment" --rarity "Rare" --power "5-10" --pretty

# Get specific card details
mtg gatherer card "Vivi Ornitier" --pretty
```

### Set Operations

```bash
# List all sets
mtg api sets list

# Search sets by name
mtg api sets search "Zendikar"

# Get specific set
mtg api sets get "ZEN"

# Generate booster pack
mtg api sets booster "KTK"
```

### Type Information

```bash
# List all types
mtg api types list

# Get subtypes
mtg api types subtypes

# Get supertypes
mtg api types supertypes

# Get formats
mtg api types formats
```

## Output Formatting

The CLI provides formatted output:

- **Tables** for list results
- **Detailed views** for individual items
- **Color coding** for different rarities and types
- **Pagination** for large result sets

### Example Output

```
 Name            Set  Type     Rarity  Mana Cost
 Lightning Bolt  2ED  Instant  Common  {R}
 Lightning Bolt  3ED  Instant  Common  {R}
 Lightning Bolt  4ED  Instant  Common  {R}

Found 25 cards matching 'Lightning Bolt'
```

## Common Issues

### Network Timeouts

If you experience timeouts, increase the timeout value:

```bash
mtg --timeout 120 scryfall search "c:red t:creature mv>=8"
```

### Rate Limiting

The CLI includes built-in rate limiting, but if you hit API limits:

- Wait a few seconds between requests
- Use smaller page sizes
- Consider caching results locally

### API Errors

If the MTG API is unavailable:

- Check your internet connection
- Verify the API URL is correct
- Try again later

---

Next: [Cards](./cards.md) | Back: [Sets](./sets.md)
