# Configuration

Learn how to configure the MTG CLI for optimal performance and convenience.

## Environment Variables

The MTG CLI supports several environment variables for default configuration:

### Core Settings

```bash
# API Configuration
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"

# Timeout Settings
export MTG_TIMEOUT=30

# Output Settings
export MTG_VERBOSE=1
```

### Setting Environment Variables

#### Bash/Zsh (Linux/macOS)

```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"' >> ~/.bashrc
echo 'export MTG_TIMEOUT=60' >> ~/.bashrc
echo 'export MTG_VERBOSE=1' >> ~/.bashrc

# Reload configuration
source ~/.bashrc
```

#### Fish Shell

```fish
# Add to ~/.config/fish/config.fish
set -gx MTG_API_BASE_URL "https://api.magicthegathering.io/v1"
set -gx MTG_TIMEOUT 60
set -gx MTG_VERBOSE 1
```

#### Windows (PowerShell)

```powershell
# Set for current session
$env:MTG_API_BASE_URL = "https://api.magicthegathering.io/v1"
$env:MTG_TIMEOUT = "60"
$env:MTG_VERBOSE = "1"

# Set permanently
[Environment]::SetEnvironmentVariable("MTG_API_BASE_URL", "https://api.magicthegathering.io/v1", "User")
[Environment]::SetEnvironmentVariable("MTG_TIMEOUT", "60", "User")
[Environment]::SetEnvironmentVariable("MTG_VERBOSE", "1", "User")
```

## Command-Line Options

Override environment variables with command-line flags:

### Global Options

```bash
# Custom API URL
mtg --api-base-url "https://custom-api.example.com/v1" cards search "Lightning Bolt"

# Custom timeout
mtg --timeout 120 sets list

# Verbose output
mtg --verbose cards search "Dragon"

# Combine options
mtg --api-base-url "https://custom-api.example.com/v1" --timeout 60 --verbose cards list
```

### Option Priority

Options are applied in this order (highest to lowest priority):

1. Command-line flags
2. Environment variables
3. Default values

## Configuration Profiles

### Development Profile

```bash
# ~/.mtg-dev
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=120
export MTG_VERBOSE=1

# Load with: source ~/.mtg-dev
```

### Production Profile

```bash
# ~/.mtg-prod
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=30
unset MTG_VERBOSE

# Load with: source ~/.mtg-prod
```

### Testing Profile

```bash
# ~/.mtg-test
export MTG_API_BASE_URL="https://test-api.example.com/v1"
export MTG_TIMEOUT=60
export MTG_VERBOSE=1

# Load with: source ~/.mtg-test
```

## Network Configuration

### Timeout Settings

Configure timeouts based on your network:

```bash
# Fast connection
export MTG_TIMEOUT=15

# Standard connection
export MTG_TIMEOUT=30

# Slow connection
export MTG_TIMEOUT=120

# Very slow/unreliable connection
export MTG_TIMEOUT=300
```

### Proxy Configuration

If you're behind a corporate proxy:

```bash
# HTTP proxy
export HTTP_PROXY="http://proxy.company.com:8080"
export HTTPS_PROXY="http://proxy.company.com:8080"

# With authentication
export HTTP_PROXY="http://username:password@proxy.company.com:8080"
export HTTPS_PROXY="http://username:password@proxy.company.com:8080"

# No proxy for certain domains
export NO_PROXY="localhost,127.0.0.1,.company.com"
```

### Custom API Endpoints

For testing or alternative APIs:

```bash
# Local development API
export MTG_API_BASE_URL="http://localhost:3000/api/v1"

# Alternative MTG API
export MTG_API_BASE_URL="https://alternative-mtg-api.com/v1"

# Custom enterprise API
export MTG_API_BASE_URL="https://internal-mtg-api.company.com/v1"
```

## Performance Tuning

### Optimal Settings for Different Use Cases

#### Interactive Use

```bash
export MTG_TIMEOUT=30
export MTG_VERBOSE=0
# Use default page sizes
```

#### Batch Processing

```bash
export MTG_TIMEOUT=120
export MTG_VERBOSE=1
# Use larger page sizes: --page-size 100
```

#### Development/Debugging

```bash
export MTG_TIMEOUT=60
export MTG_VERBOSE=1
# Use smaller page sizes for faster iteration
```

### Rate Limiting

The CLI includes built-in rate limiting, but you can optimize:

```bash
# For bulk operations, add delays between commands
mtg cards search "Lightning" --page-size 50
sleep 1
mtg cards search "Bolt" --page-size 50
```

## Output Customization

### Verbose Output

Enable detailed output for debugging:

```bash
export MTG_VERBOSE=1

# Or per command
mtg --verbose cards search "Lightning Bolt"
```

Verbose output includes:

- Request URLs
- Response times
- API rate limit information
- Detailed error messages

### Pagination Settings

Optimize pagination for your workflow:

```bash
# Small pages for quick browsing
mtg cards search "Dragon" --page-size 5

# Large pages for comprehensive results
mtg cards search "Creature" --page-size 100

# Multiple pages
mtg cards search "Artifact" --page 1 --page-size 50
mtg cards search "Artifact" --page 2 --page-size 50
```

## Advanced Usage

### API Keys

If using a custom API that requires authentication:

```bash
# Never put API keys in command line (visible in history)
# Instead, use environment variables
export MTG_API_KEY="your-secret-key"

# Or use a secure configuration file
echo "MTG_API_KEY=your-secret-key" > ~/.mtg-secrets
chmod 600 ~/.mtg-secrets
source ~/.mtg-secrets
```

### Useful Functions

```bash
# Search and get detailed info
mtg-detail() {
    local name="$1"
    echo "Searching for: $name"
    mtg cards search "$name" --exact --page-size 1
    echo
    echo "Getting detailed info..."
    local id=$(mtg cards search "$name" --exact --page-size 1 | grep -o '[0-9]\+' | head -1)
    if [ -n "$id" ]; then
        mtg cards get "$id"
    fi
}

# Generate multiple boosters
mtg-boosters() {
    local set_code="$1"
    local count="${2:-3}"
    for i in $(seq 1 $count); do
        echo "=== Booster Pack $i ==="
        mtg sets booster "$set_code"
        echo
    done
}
```

## Troubleshooting Configuration

### Check Current Configuration

```bash
# Check environment variables
env | grep MTG_

# Test with verbose output
mtg --verbose cards search "test" --page-size 1
```

### Common Issues

#### Timeout Too Short

```bash
# Symptoms: Frequent timeout errors
# Solution: Increase timeout
export MTG_TIMEOUT=120
```

#### Wrong API URL

```bash
# Symptoms: Connection errors, 404s
# Solution: Verify API URL
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
```

#### Proxy Issues

```bash
# Symptoms: Connection refused, proxy errors
# Solution: Configure proxy settings
export HTTP_PROXY="http://proxy.company.com:8080"
export HTTPS_PROXY="http://proxy.company.com:8080"
```

---

Next: [MCP Overview](../mcp/overview.md) | Back: [Type Commands](types.md)
