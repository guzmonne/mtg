#!/usr/bin/env bash
#
# Pre-commit checks script that runs cargo fmt, cargo check, and cargo test to ensure code quality
# This script can be run standalone or called from the git pre-commit hook
# The script stops execution immediately if any check fails
#

set -e  # Exit immediately if any command fails

# Check if cargo is available
if ! command -v cargo >/dev/null 2>&1; then
	echo "Error: cargo is not installed or not in PATH"
	exit 1
fi

# Check if we're in a Rust project
if [ ! -f "Cargo.toml" ]; then
	echo "Error: No Cargo.toml found in project root"
	exit 1
fi

# Check if there are any staged Rust files (only when called from git hook)
if [ -n "$GIT_INDEX_FILE" ]; then
	rust_files=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(rs)$')
	if [ -z "$rust_files" ]; then
		echo "No Rust files staged for commit, skipping checks..."
		exit 0
	fi
fi

echo "🔍 Running pre-commit checks on Rust files..."
echo ""

# 1. Run cargo fmt
echo "📝 Running cargo fmt..."
if ! cargo fmt --all -- --check >/dev/null 2>&1; then
	echo "Code formatting issues detected. Running cargo fmt to fix them..."
	cargo fmt --all

	# Re-stage formatted Rust files (only when called from git hook)
	if [ -n "$GIT_INDEX_FILE" ] && [ -n "$rust_files" ]; then
		echo "Re-staging formatted Rust files..."
		echo "$rust_files" | xargs git add
		echo ""
		echo "❌ Files have been formatted and re-staged. Please review the changes and commit again."
		exit 1
	fi
fi

echo "✅ Code formatting check passed"
echo ""

# 2. Run cargo check
echo "🔧 Running cargo check..."
if ! cargo check --all 2>&1; then
	echo ""
	echo "❌ Cargo check failed. Please fix the compilation errors before committing."
	exit 1
fi

echo "✅ Cargo check passed"
echo ""

# 3. Run cargo test
echo "🧪 Running cargo test..."
if ! cargo test --all 2>&1; then
	echo ""
	echo "❌ Tests failed. Please fix the failing tests before committing."
	exit 1
fi

echo "✅ All tests passed"
echo ""

echo "✨ All pre-commit checks passed successfully!"
exit 0
