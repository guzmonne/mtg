# MTG CLI Agent Guidelines

## Current Development
- See [IN_PROGRESS.md](./IN_PROGRESS.md) for the comprehensive documentation of the `companion watch` command
- The companion watch feature is now **feature complete** with full game state tracking and Scryfall integration

## Build/Test Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo clippy -- -D warnings` - Lint with clippy (must pass)
- `cargo fmt -- --check` - Check formatting
- `bacon` - Watch mode (default: check), use `bacon test` for tests

## Git Hooks
- **Pre-commit hook**: Automatically runs `cargo fmt` before each commit
  - Located at `.git/hooks/pre-commit`
  - Ensures consistent code formatting across all commits
  - If formatting issues are found, it automatically fixes them and requires re-staging
  - Hook will pass if no Rust files are being committed

### Installing Git Hooks
Use the installation script to set up git hooks for new development environments:

```bash
# Install hooks with default settings
./scripts/install-git-hooks.sh

# Force reinstall hooks (overwrites existing)
./scripts/install-git-hooks.sh --force

# Install and test hooks
./scripts/install-git-hooks.sh --test

# Quiet installation
./scripts/install-git-hooks.sh --quiet
```

The script will:
- Check for required dependencies (cargo, git)
- Backup existing hooks before installation
- Install the pre-commit hook
- Optionally test the installed hooks

## Code Style
- Use `crate::prelude::*` for common imports (Result, eyre!, aeprintln!, etc.)
- Import order: std, external crates, crate modules
- Use `thiserror::Error` for error types with `#[from]` conversions
- Prefer `async fn` over `impl Future` for async functions
- Use `clap::Parser` and `clap::Subcommand` for CLI structure
- Snake_case for functions/variables, PascalCase for types/enums

## Error Handling
- Use `color_eyre::Result<T>` as return type
- Use `eyre!()` macro for error creation
- Implement `From` traits for error conversions
- Use structured error types with `thiserror`

## Testing
- Place tests in same file with `#[cfg(test)]`
- Use `tempfile` crate for temporary files in tests
- Test async functions with `#[tokio::test]`

## Project Structure
```
crates/mtg/src/
├── api/           # MTG API client (magicthegathering.io)
├── cache/         # Caching system for API responses
├── companion/     # MTG Arena companion features
│   ├── parse/     # Log parsing utilities
│   └── watch/     # Real-time log monitoring
├── decks/         # Deck management and analysis
├── gatherer/      # Gatherer website integration
├── mcp/           # Model Context Protocol server
├── scryfall/      # Scryfall API integration
├── completions/   # Shell completions
├── error.rs       # Error types
├── main.rs        # CLI entry point
└── prelude.rs     # Common imports
```

## Key Components

### Companion Watch System
- **Purpose**: Real-time monitoring of MTG Arena logs with comprehensive game state tracking
- **Status**: ✅ Feature Complete (see [IN_PROGRESS.md](./IN_PROGRESS.md) for full documentation)
- **Location**: `crates/mtg/src/companion/watch/`
- **Key files**:
  - `tailer.rs` - Log file monitoring with state management
  - `parser.rs` - Event parsing and game state tracking (1400+ lines)
  - `async_processor.rs` - Asynchronous card data fetching from Scryfall
  - `types.rs` - Data structures for events
  - `state.rs` - Persistent state tracking
  - `display.rs` - Output formatting
- **Features**:
  - Complete play-by-play match tracking
  - Real-time card details from Scryfall
  - Comprehensive annotation processing
  - Timer tracking and match analytics
  - Visual indicators and formatted output

### Scryfall Integration
- **Purpose**: Fetch card data, images, and prices
- **Location**: `crates/mtg/src/scryfall/`
- **Features**:
  - Card search with advanced queries
  - Autocomplete suggestions
  - Multiple lookup methods (name, ID, collector number)
  - Smart search that auto-detects query type
  - Caching to reduce API calls

### Caching System
- **Location**: `crates/mtg/src/cache.rs`
- **Features**:
  - SQLite-based persistent cache
  - Automatic expiration (24 hours default)
  - Request deduplication
  - Async-safe implementation

### API Integrations
1. **Scryfall API** (https://scryfall.com/docs/api)
   - Primary source for card data
   - Rate limit aware
   - Comprehensive card information

2. **magicthegathering.io API**
   - Legacy support
   - Basic card and set information

## Important Patterns

### Event Processing Pipeline
1. **LogTailer** reads raw log lines
2. **EventParser** identifies event types and extracts data
3. **Display modules** format output for users
4. **State persistence** tracks progress between runs

### Multi-line Event Handling
- Incoming events (`<==`) have data on the next line
- Outgoing events (`==>`) have data on the same line
- Parser must handle both formats correctly

### Async Operations
- All API calls use `tokio` for async execution
- File I/O uses async variants (`tokio::fs`)
- Proper error propagation with `?` operator

## Common Tasks

### Adding a New Event Type
1. Add event handler in `parser.rs`:
   ```rust
   "EventName" => self.handle_event_name(&event.raw_data),
   ```
2. Implement the handler method
3. Update `ParsedEvent` enum if needed
4. Add display logic

### Testing Log Parsing
1. Create test log file with sample events
2. Run with `--from-beginning` flag
3. Use `--verbose` for debugging
4. Check state persistence works correctly

## Environment Variables
- `MTGA_LOG_PATH` - Override default log directory
- `MTG_API_BASE_URL` - API endpoint (rarely needed)
- `MTG_TIMEOUT` - Request timeout in seconds
- `MTG_VERBOSE` - Enable verbose output

## Performance Considerations
- Log files can be large (100MB+)
- Use streaming/chunked reading
- Cache API responses aggressively
- Minimize memory allocations in hot paths
- State updates should be atomic

## Security Notes
- Never log sensitive user data
- Sanitize file paths
- Validate JSON before parsing
- Rate limit API requests
- Use secure HTTPS for all API calls