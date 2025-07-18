# Setup & Installation

Complete guide to setting up the MTG MCP server for AI integration.

## Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Version 1.70 or later
- **Memory**: 256MB RAM minimum
- **Network**: Internet connection for MTG API access

### Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

## Installation

### 1. Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd mtg

# Build release version
cargo build --release

# Verify installation
./target/release/mtg --version
```

### 2. Development Build

```bash
# Build debug version (faster compilation)
cargo build

# Run with debug binary
./target/debug/mtg mcp
```

### 3. Install Globally (Optional)

```bash
# Install to ~/.cargo/bin
cargo install --path crates/mtg

# Now available as 'mtg' command
mtg --version
```

## Starting the Server

### Basic Startup

```bash
# Start MCP server (stdio mode)
./target/release/mtg mcp
```

The server:
- Initializes MCP protocol
- Connects to MTG API
- Waits for JSON-RPC messages on stdin
- Sends responses to stdout

### With Configuration

```bash
# Custom API URL
./target/release/mtg --api-base-url "https://api.magicthegathering.io/v1" mcp

# Custom timeout
./target/release/mtg --timeout 60 mcp

# Verbose logging
./target/release/mtg --verbose mcp

# Combined options
./target/release/mtg --api-base-url "https://api.magicthegathering.io/v1" --timeout 60 --verbose mcp
```

### Environment Variables

```bash
# Set defaults
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=30
export MTG_VERBOSE=1

# Start server
./target/release/mtg mcp
```

## AI Assistant Integration

### Claude Desktop

1. **Locate Configuration File**:
   - **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

2. **Add MTG Server**:
   ```json
   {
     "mcpServers": {
       "mtg": {
         "command": "/path/to/mtg",
         "args": ["mcp"],
         "env": {
           "MTG_TIMEOUT": "60"
         }
       }
     }
   }
   ```

3. **Restart Claude Desktop**

4. **Verify Connection**:
   - Look for MTG server in Claude's MCP panel
   - Try asking: "Search for Lightning Bolt cards"

### Other AI Assistants

For assistants supporting MCP:

```json
{
  "servers": {
    "mtg": {
      "command": "/path/to/mtg",
      "args": ["mcp"],
      "transport": "stdio"
    }
  }
}
```

## Testing the Installation

### 1. Manual Testing

Test server startup:
```bash
# Start server with verbose output
./target/release/mtg --verbose mcp

# Should show:
# Initializing MTG MCP Server
# Waiting for MCP initialization...
```

### 2. JSON-RPC Testing

Create a test script:
```bash
#!/bin/bash
# test-mcp.sh

echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}}' | ./target/release/mtg mcp
```

Expected response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2025-03-26",
    "capabilities": {
      "resources": {"subscribe": true, "listChanged": true},
      "tools": {"listChanged": true},
      "prompts": {"listChanged": true},
      "logging": {}
    },
    "serverInfo": {
      "name": "mtg-mcp-server",
      "version": "1.0.0"
    }
  }
}
```

### 3. Resource Testing

Test resource access:
```bash
#!/bin/bash
# test-resources.sh

# Initialize
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}}' | ./target/release/mtg mcp

# List resources
echo '{"jsonrpc": "2.0", "id": 2, "method": "resources/list"}' | ./target/release/mtg mcp
```

### 4. Tool Testing

Test tool execution:
```bash
#!/bin/bash
# test-tools.sh

# Initialize and call tool
{
  echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}}'
  echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "search_cards", "arguments": {"name": "Lightning Bolt", "limit": 1}}}'
} | ./target/release/mtg mcp
```

## Docker Setup

### Dockerfile

```dockerfile
FROM rust:1.70-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
WORKDIR /app
COPY . .

# Build release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/mtg /usr/local/bin/mtg

# Set entrypoint
ENTRYPOINT ["mtg", "mcp"]
```

### Build and Run

```bash
# Build image
docker build -t mtg-mcp .

# Run server
docker run -i mtg-mcp

# With environment variables
docker run -e MTG_TIMEOUT=60 -e MTG_VERBOSE=1 -i mtg-mcp
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  mtg-mcp:
    build: .
    environment:
      - MTG_TIMEOUT=60
      - MTG_VERBOSE=1
    stdin_open: true
    tty: true
```

## Configuration Options

### Command Line Options

```bash
mtg [OPTIONS] mcp

OPTIONS:
    --api-base-url <URL>    MTG API base URL [default: https://api.magicthegathering.io/v1]
    --timeout <SECONDS>     Request timeout [default: 30]
    --verbose               Enable verbose logging
    -h, --help             Print help
    -V, --version          Print version
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MTG_API_BASE_URL` | MTG API endpoint | `https://api.magicthegathering.io/v1` |
| `MTG_TIMEOUT` | Request timeout (seconds) | `30` |
| `MTG_VERBOSE` | Enable verbose logging | `false` |

### Configuration File

Create `~/.mtg-config`:
```bash
# MTG CLI Configuration
export MTG_API_BASE_URL="https://api.magicthegathering.io/v1"
export MTG_TIMEOUT=60
export MTG_VERBOSE=1
```

Load with:
```bash
source ~/.mtg-config
./target/release/mtg mcp
```

## Troubleshooting

### Common Issues

#### Server Won't Start
```bash
# Check binary exists
ls -la ./target/release/mtg

# Check permissions
chmod +x ./target/release/mtg

# Test basic functionality
./target/release/mtg --help
```

#### Connection Timeouts
```bash
# Increase timeout
./target/release/mtg --timeout 120 mcp

# Test API connectivity
curl -s "https://api.magicthegathering.io/v1/cards?pageSize=1"
```

#### JSON-RPC Errors
```bash
# Enable verbose logging
./target/release/mtg --verbose mcp

# Check JSON format
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | jq .
```

#### Claude Desktop Integration
```bash
# Check configuration file location
ls -la ~/Library/Application\ Support/Claude/claude_desktop_config.json

# Validate JSON
cat ~/Library/Application\ Support/Claude/claude_desktop_config.json | jq .

# Check binary path
which mtg
```

### Debug Mode

Enable maximum debugging:
```bash
# Set environment
export RUST_LOG=debug
export MTG_VERBOSE=1

# Run with debug output
./target/debug/mtg --verbose mcp 2>&1 | tee debug.log
```

### Network Issues

Test network connectivity:
```bash
# Test MTG API
curl -v "https://api.magicthegathering.io/v1/cards?pageSize=1"

# Test with proxy
export HTTP_PROXY="http://proxy.company.com:8080"
export HTTPS_PROXY="http://proxy.company.com:8080"
./target/release/mtg mcp
```

## Performance Tuning

### Memory Usage

Monitor memory usage:
```bash
# Linux/macOS
ps aux | grep mtg

# Detailed memory info
cat /proc/$(pgrep mtg)/status | grep -E "(VmRSS|VmSize)"
```

### Response Times

Optimize for performance:
```bash
# Shorter timeout for faster failures
export MTG_TIMEOUT=15

# Use release build
cargo build --release

# Monitor response times
./target/release/mtg --verbose mcp 2>&1 | grep "Response time"
```

## Updates and Maintenance

### Updating the Server

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Restart server
./target/release/mtg mcp
```

### Health Monitoring

Create a health check script:
```bash
#!/bin/bash
# health-check.sh

timeout 10s echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "health", "version": "1.0"}}}' | ./target/release/mtg mcp > /dev/null

if [ $? -eq 0 ]; then
    echo "MTG MCP Server: HEALTHY"
    exit 0
else
    echo "MTG MCP Server: UNHEALTHY"
    exit 1
fi
```

---

Next: [Resources](resources.md) | Back: [Overview](overview.md)