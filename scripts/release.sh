#!/bin/bash

# MTG CLI Release Script
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 1.0.0

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.0"
    exit 1
fi

VERSION=$1

# Validate version format (basic semver check)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo "Error: Version must be in semver format (e.g., 1.0.0 or 1.0.0-beta)"
    exit 1
fi

echo "🚀 Preparing release v$VERSION"

# Check if we're on master branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "master" ]; then
    echo "Error: Must be on master branch to create a release"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    echo "Error: Working directory is not clean. Please commit or stash your changes."
    git status --porcelain
    exit 1
fi

# Pull latest changes
echo "📥 Pulling latest changes..."
git pull origin master

# Check if tag already exists
if git tag -l | grep -q "^v$VERSION$"; then
    echo "Error: Tag v$VERSION already exists"
    exit 1
fi

# Run tests to make sure everything works
echo "🧪 Running tests..."
cargo test --verbose

# Check formatting and linting
echo "🔍 Checking code formatting and linting..."
cargo fmt -- --check
cargo clippy -- -D warnings

echo "✅ All checks passed!"

# Create and push the tag
echo "🏷️  Creating tag v$VERSION..."
git tag -a "v$VERSION" -m "Release v$VERSION"

echo "📤 Pushing tag to trigger release workflow..."
git push origin "v$VERSION"

echo ""
echo "🎉 Release v$VERSION has been initiated!"
echo ""
echo "The GitHub Actions workflow will now:"
echo "  1. Update the version in crates/mtg/Cargo.toml"
echo "  2. Build binaries for all platforms"
echo "  3. Create a GitHub release with changelog"
echo "  4. Upload all platform binaries as release assets"
echo ""
echo "You can monitor the progress at:"
echo "https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/actions"
echo ""
echo "The release will be available at:"
echo "https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/releases/tag/v$VERSION"