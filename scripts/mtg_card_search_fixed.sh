#!/bin/bash

# Fixed version of mtg_card_search function
mtg_card_search_fixed() {
    local search_args=()
    local pretty_flag=""
    local help_text="Usage: mtg_card_search [search_options] [--pretty]

Search for MTG cards interactively using fzf.

Examples:
  mtg_card_search --name Lightning
  mtg_card_search --card-type Creature --colors R --pretty
  mtg_card_search --set \"War of the Spark\" --rarity Mythic

All gatherer search options are supported. Add --pretty for final card display."

    # Parse arguments to separate --pretty flag
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                echo "$help_text" 1>&2
                return 0
                ;;
            --pretty)
                pretty_flag="--pretty"
                shift
                ;;
            *)
                search_args+=("$1")
                shift
                ;;
        esac
    done

    # Check if mtg command exists
    if ! command -v mtg &> /dev/null; then
        echo "Error: 'mtg' command not found. Please install the MTG CLI first." 1>&2
        return 1
    fi

    # Check if fzf exists
    if ! command -v fzf &> /dev/null; then
        echo "Error: 'fzf' not found. Please install fzf first:" 1>&2
        echo "  brew install fzf    # macOS" 1>&2
        echo "  sudo apt install fzf    # Ubuntu/Debian" 1>&2
        return 1
    fi

    echo "Searching for cards..." 1>&2

    # Get search results in JSON format (without --pretty)
    local search_results
    local search_error
    search_results=$(mtg gatherer search "${search_args[@]}" 2>&1)
    local exit_code=$?
    
    if [[ $exit_code -ne 0 ]]; then
        echo "Error: Search failed with exit code $exit_code" 1>&2
        echo "Error output: $search_results" 1>&2
        return 1
    fi

    # Debug: Show first 200 chars of response
    echo "Debug: First 200 chars of response: ${search_results:0:200}" 1>&2

    # Check if we got valid JSON
    if ! echo "$search_results" | jq empty 2>/dev/null; then
        echo "Error: Invalid JSON response from search." 1>&2
        echo "Response: $search_results" 1>&2
        return 1
    fi

    # Extract card names using jq
    local card_names
    if command -v jq &> /dev/null; then
        # Debug: Show what jq sees
        echo "Debug: Number of items: $(echo "$search_results" | jq '.items | length')" 1>&2
        
        card_names=$(echo "$search_results" | jq -r '.items[]?.instanceName // empty' 2>&1)
        local jq_exit=$?
        
        if [[ $jq_exit -ne 0 ]]; then
            echo "Error: jq failed to parse JSON" 1>&2
            echo "jq output: $card_names" 1>&2
            return 1
        fi
    else
        echo "Error: jq is required for this function. Please install jq." 1>&2
        return 1
    fi

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your search criteria." 1>&2
        echo "Debug: Full search results:" 1>&2
        echo "$search_results" | jq '.' 1>&2
        return 1
    fi

    # Count results
    local card_count
    card_count=$(echo "$card_names" | wc -l | tr -d ' ')

    echo "Found $card_count cards. Use fzf to select one..." 1>&2
    echo "Press Ctrl+C to cancel, Enter to select, or type to filter." 1>&2
    echo "" 1>&2

    # Create a preview command that shows card details
    local preview_cmd="mtg gatherer card '{}' --pretty 2>/dev/null | head -20"

    # Use fzf to select a card
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=80% \
        --border \
        --prompt="Select a card: " \
        --preview="$preview_cmd" \
        --preview-window=right:60% \
        --header="Found $card_count cards - Press Enter to select, Ctrl+C to cancel")

    # Check if user cancelled
    if [[ -z "$selected_card" ]]; then
        echo "Selection cancelled." 1>&2
        return 0
    fi

    echo "" 1>&2
    echo "Getting details for: $selected_card" 1>&2
    echo "----------------------------------------" 1>&2

    # Get detailed card information
    mtg gatherer card "$selected_card" $pretty_flag
}

# Test the function
mtg_card_search_fixed "$@"