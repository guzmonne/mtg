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

### 2. Search for a Card
```bash
mtg cards search "Black Lotus"
```

### 3. Browse Sets
```bash
mtg sets list --page-size 5
```

### 4. Get Card Types
```bash
mtg types list
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
mtg --api-base-url "https://custom-api.example.com/v1" cards search "Mox"

# Set longer timeout for slow connections
mtg --timeout 120 sets list

# Enable verbose output for debugging
mtg --verbose cards search "Lightning Bolt"
```

## Basic Usage Patterns

### Card Searches

```bash
# Simple name search
mtg cards search "Lightning"

# Exact name match
mtg cards search "Lightning Bolt" --exact

# Search with pagination
mtg cards search "Dragon" --page 2 --page-size 10

# Search by ID
mtg cards get 409574
```

### Set Operations

```bash
# List all sets
mtg sets list

# Search sets by name
mtg sets search "Zendikar"

# Get specific set
mtg sets get "ZEN"

# Generate booster pack
mtg sets booster "KTK"
```

### Type Information

```bash
# List all types
mtg types list

# Get subtypes
mtg types subtypes

# Get supertypes
mtg types supertypes

# Get formats
mtg types formats
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
mtg --timeout 120 cards search "Expensive Query"
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

## Next Steps

- [Card Commands](cards.md) - Learn advanced card search techniques
- [Set Commands](sets.md) - Explore set browsing and booster generation
- [Type Commands](types.md) - Master type and format queries
- [MCP Server](../mcp/overview.md) - Set up AI integration

---

Need help? Check the [troubleshooting guide](../troubleshooting/) or open an issue.