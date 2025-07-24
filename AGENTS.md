# MTG CLI Agent Guidelines

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