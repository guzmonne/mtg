# MTG CLI Agent Guidelines

## Build/Test Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo clippy -- -D warnings` - Lint with clippy (must pass)
- `cargo fmt -- --check` - Check formatting
- `bacon` - Watch mode (default: check), use `bacon test` for tests

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