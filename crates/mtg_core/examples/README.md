# Scryfall Client Examples

This directory contains examples demonstrating how to use the generic Scryfall API client with the builder pattern.

## Running Examples

To run any example, use:

```bash
cargo run --example <example_name>
```

## Available Examples

### Basic Usage
```bash
cargo run --example basic_usage
```
Demonstrates basic client creation and fetching sets from the Scryfall API.

### Builder Pattern
```bash
cargo run --example builder_pattern
```
Shows how to use the builder pattern to customize client configuration with timeouts, user agents, rate limiting, and custom headers.

### Search with Parameters
```bash
cargo run --example search_with_params
```
Demonstrates searching for cards with query parameters.

### Raw Response Handling
```bash
cargo run --example raw_response
```
Shows how to get raw response text instead of parsed JSON, useful for CSV or custom parsing.

### Error Handling
```bash
cargo run --example error_handling
```
Demonstrates proper error handling for API errors vs network/parsing errors.

### Rate Limiting
```bash
cargo run --example rate_limiting
```
Shows how to configure custom rate limiting between requests.

### No Rate Limiting
```bash
cargo run --example no_rate_limiting
```
Demonstrates disabling rate limiting (use with caution!).

### Custom Base URL
```bash
cargo run --example custom_base_url
```
Shows how to configure a custom base URL for testing or alternative endpoints.

## Builder Pattern Features

The `ScryfallClient::builder()` supports:

- `timeout_secs(u64)` - Request timeout in seconds
- `user_agent(String)` - Custom user agent string
- `verbose(bool)` - Enable verbose logging
- `rate_limit_delay_ms(Option<u64>)` - Delay between requests in milliseconds
- `rate_limit_delay(Option<Duration>)` - Delay between requests as Duration
- `header(String, String)` - Add custom headers
- `base_url(String)` - Custom base URL for testing
- `build()` - Create the configured client

## Notes

- All examples use `color_eyre` for error handling
- Examples that make network requests may take a moment to complete
- The Scryfall API has rate limits, so be respectful when testing
- Some examples (like `custom_base_url`) are expected to fail unless you have a test server running