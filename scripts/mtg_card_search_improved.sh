#!/bin/bash

# Interactive MTG card search and selection with proper escaping
mtg_card_search() {
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

    # Check if jq exists
    if ! command -v jq &> /dev/null; then
        echo "Error: 'jq' not found. Please install jq first:" 1>&2
        echo "  brew install jq    # macOS" 1>&2
        echo "  sudo apt install jq    # Ubuntu/Debian" 1>&2
        return 1
    fi

    echo "Searching for cards..." 1>&2

    # Get search results in JSON format (without --pretty)
    # Use a temporary file to avoid issues with large outputs
    local tmpfile=$(mktemp)
    trap "rm -f $tmpfile" EXIT

    if ! mtg gatherer search "${search_args[@]}" > "$tmpfile" 2>&1; then
        echo "Error: Search failed. Please check your search parameters." 1>&2
        cat "$tmpfile" 1>&2
        return 1
    fi

    # Extract card names using jq
    local card_names
    card_names=$(jq -r '.items[]?.instanceName // empty' < "$tmpfile" 2>/dev/null)

    if [[ -z "$card_names" ]]; then
        echo "No cards found matching your search criteria." 1>&2
        echo "Debug info: Check if the search returned valid JSON:" 1>&2
        jq '.' < "$tmpfile" 2>&1 | head -20 1>&2
        return 1
    fi

    # Count results
    local card_count
    card_count=$(echo "$card_names" | wc -l | tr -d ' ')

    echo "Found $card_count cards. Use fzf to select one..." 1>&2
    echo "Press Ctrl+C to cancel, Enter to select, or type to filter." 1>&2
    echo "" 1>&2

    # Create a preview script that properly escapes the card name
    local preview_script=$(mktemp)
    cat > "$preview_script" << 'EOF'
#!/bin/bash
# Escape single quotes by replacing them with '\''
card_name="${1//\'/\'\\\'\'}"
eval "mtg gatherer card '$card_name' --pretty 2>&1 || echo 'Error: Could not fetch card details'"
EOF
    chmod +x "$preview_script"
    trap "rm -f $tmpfile $preview_script" EXIT

    # Use fzf to select a card with the preview script
    local selected_card
    selected_card=$(echo "$card_names" | fzf \
        --height=80% \
        --border \
        --prompt="Select a card: " \
        --preview="$preview_script {}" \
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

# Shorter alias for convenience
alias mtgf='mtg_card_search'