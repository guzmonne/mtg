#!/bin/bash

# Test what the search command actually outputs
echo "=== Testing raw output ==="
mtg gatherer search --name Vivi 2>&1 | head -20

echo -e "\n=== Testing if it's valid JSON ==="
if mtg gatherer search --name Vivi 2>&1 | jq empty 2>/dev/null; then
    echo "Valid JSON"
else
    echo "Invalid JSON"
fi

echo -e "\n=== Trying to extract just the JSON ==="
# Try to find where JSON starts (usually with { or [)
output=$(mtg gatherer search --name Vivi 2>&1)
json_part=$(echo "$output" | sed -n '/^{/,/^}/p')
echo "Extracted JSON (first 500 chars):"
echo "${json_part:0:500}"

echo -e "\n=== Testing if extracted part is valid JSON ==="
if echo "$json_part" | jq empty 2>/dev/null; then
    echo "Valid JSON after extraction"
    echo "Number of items: $(echo "$json_part" | jq '.items | length')"
else
    echo "Still invalid JSON"
fi