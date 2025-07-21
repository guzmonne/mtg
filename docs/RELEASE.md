# Release Process

This document describes the automated release process for the MTG CLI.

## Overview

The release process is fully automated using GitHub Actions and is triggered by creating git tags. When you create a tag, the system will:

1. **Update Version**: Automatically update the version in `crates/mtg/Cargo.toml`
2. **Build Binaries**: Cross-compile for all supported platforms
3. **Generate Changelog**: Create a changelog from commit messages since the last release
4. **Create Release**: Publish a GitHub release with all binaries attached

## Supported Platforms

- **Linux**: `x86_64-unknown-linux-gnu`
- **macOS Intel**: `x86_64-apple-darwin`
- **macOS Apple Silicon**: `aarch64-apple-darwin`
- **Windows**: `x86_64-pc-windows-msvc`

## Creating a Release

### Method 1: Using the Release Script (Recommended)

```bash
# Make sure you're on the master branch with a clean working directory
./scripts/release.sh 1.0.0
```

The script will:
- Validate the version format
- Check that you're on the master branch
- Ensure working directory is clean
- Run tests and linting
- Create and push the tag
- Provide links to monitor progress

### Method 2: Manual Process

```bash
# Ensure you're on master with latest changes
git checkout master
git pull origin master

# Run tests to ensure everything works
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings

# Create and push the tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## Version Format

Use [Semantic Versioning](https://semver.org/) format:
- `MAJOR.MINOR.PATCH` (e.g., `1.0.0`)
- `MAJOR.MINOR.PATCH-PRERELEASE` (e.g., `1.0.0-beta`, `1.0.0-rc.1`)

## What Happens After Tagging

1. **Version Update Job** (`update-version`):
   - Extracts version from the tag (removes `v` prefix)
   - Updates `crates/mtg/Cargo.toml` with the new version
   - Commits and pushes the change to master

2. **Build Job** (`build`):
   - Runs in parallel for all supported platforms
   - Uses Rust cross-compilation to build release binaries
   - Uploads artifacts for each platform

3. **Release Job** (`release`):
   - Downloads all build artifacts
   - Generates changelog from commit messages since last release
   - Creates GitHub release with:
     - Release notes including changelog
     - All platform binaries as downloadable assets
     - Installation instructions

## Changelog Generation

The changelog is automatically generated from commit messages between releases. It includes:
- First line of each commit since the last release
- Download links for all platform binaries
- Installation instructions

## Monitoring Releases

After creating a tag, you can monitor the release process:

1. **GitHub Actions**: Check the "Release" workflow in the Actions tab
2. **Releases Page**: View the created release once the workflow completes

## Troubleshooting

### Common Issues

1. **Tag already exists**: Delete the tag locally and remotely if needed:
   ```bash
   git tag -d v1.0.0
   git push origin :refs/tags/v1.0.0
   ```

2. **Build failures**: Check the GitHub Actions logs for specific error messages

3. **Permission issues**: Ensure the repository has proper GitHub Actions permissions

### Manual Cleanup

If a release fails and you need to clean up:

```bash
# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# Delete GitHub release (if created)
# Use GitHub web interface or gh CLI
gh release delete v1.0.0
```

## Security Notes

- The workflow uses `GITHUB_TOKEN` which has limited permissions
- No external secrets are required
- All builds happen in GitHub's secure runners
- Artifacts are signed by GitHub's infrastructure

## Examples

### Creating a Major Release
```bash
./scripts/release.sh 2.0.0
```

### Creating a Patch Release
```bash
./scripts/release.sh 1.0.1
```

### Creating a Pre-release
```bash
./scripts/release.sh 1.1.0-beta
```