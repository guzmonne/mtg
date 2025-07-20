#!/bin/bash

# Test script for MTG gatherer caching functionality

echo "Testing MTG gatherer caching..."
echo "==============================="

# Build the project first
echo "Building project..."
cd /Users/guzmanmonne/Projects/Rust/mtg
cargo build --package mtg --release

# Set verbose mode to see cache hits/misses
export MTG_VERBOSE=true

echo -e "\n1. First request (should be cache miss):"
time ./target/release/mtg gatherer card "Lightning Bolt"

echo -e "\n2. Second request (should be cache hit):"
time ./target/release/mtg gatherer card "Lightning Bolt"

echo -e "\n3. Search request (cache miss):"
time ./target/release/mtg gatherer search --name "Vivien" --page 1

echo -e "\n4. Same search request (cache hit):"
time ./target/release/mtg gatherer search --name "Vivien" --page 1

echo -e "\n5. Different page (cache miss):"
time ./target/release/mtg gatherer search --name "Vivien" --page 2

echo -e "\n6. Checking cache directory:"
ls -la ~/.local/share/mtg/cache/

echo -e "\n7. Cache file count:"
ls ~/.local/share/mtg/cache/*.json 2>/dev/null | wc -l

echo -e "\nCache test completed!"