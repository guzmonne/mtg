# MCP Prompts

Prompts provide AI assistants with pre-built templates for common Magic: The Gathering analysis and discussion tasks.

## Available Prompts

The MTG MCP server provides three specialized prompts:

| Prompt            | Purpose                                   | Arguments             |
| ----------------- | ----------------------------------------- | --------------------- |
| **analyze_card**  | Card power level and competitive analysis | card_name, format     |
| **build_deck**    | Deck construction guidance                | theme, format, budget |
| **compare_cards** | Multi-card comparison and evaluation      | cards, criteria       |

## analyze_card

Comprehensive card analysis for competitive play evaluation.

### Arguments

```json
{
  "card_name": "Lightning Bolt", // Required: Card name to analyze
  "format": "Modern" // Optional: Format context (default: "Modern")
}
```

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "analyze_card",
    "arguments": {
      "card_name": "Lightning Bolt",
      "format": "Modern"
    }
  }
}
```

### Generated Prompt

The prompt generates a comprehensive analysis request including:

````
Please analyze the Magic: The Gathering card "Lightning Bolt" for competitive play in the Modern format.

Card Data:
```json
{
  "id": "409574",
  "name": "Lightning Bolt",
  "manaCost": "{R}",
  "cmc": 1,
  "colors": ["Red"],
  "type": "Instant",
  "rarity": "Common",
  "text": "Lightning Bolt deals 3 damage to any target.",
  "legalities": [
    {"format": "Modern", "legality": "Legal"},
    {"format": "Legacy", "legality": "Legal"},
    {"format": "Vintage", "legality": "Legal"}
  ]
}
````

Please provide a comprehensive analysis covering:

1. **Power Level Assessment**

   - Rate the card's overall power level (1-10)
   - Compare to similar cards in the format
   - Identify key strengths and weaknesses

2. **Competitive Viability**

   - Current meta relevance in Modern
   - Deck archetypes that would play this card
   - Matchup considerations

3. **Synergies and Combos**

   - Cards that work well with Lightning Bolt
   - Potential combo interactions
   - Build-around strategies

4. **Meta Considerations**

   - How the current meta affects this card's value
   - Sideboard considerations
   - Future meta predictions

5. **Historical Context**
   - Card's impact on Magic history
   - Previous meta positions
   - Design significance

````

### Use Cases

- **Deck Building**: Evaluate cards for inclusion
- **Meta Analysis**: Understand card positioning
- **Educational**: Learn about card evaluation
- **Content Creation**: Generate analysis content

### Supported Formats

- Standard
- Pioneer
- Modern
- Legacy
- Vintage
- Commander
- Pauper
- Historic

## build_deck

Comprehensive deck building guidance and strategy development.

### Arguments

```json
{
  "theme": "Burn",                   // Required: Deck theme or strategy
  "format": "Modern",                // Required: Target format
  "budget": "Budget"                 // Optional: Budget constraints
}
````

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "build_deck",
    "arguments": {
      "theme": "Tribal Goblins",
      "format": "Modern",
      "budget": "$200"
    }
  }
}
```

### Generated Prompt

```
Help me build a Magic: The Gathering deck with the following specifications:

**Deck Theme**: Tribal Goblins
**Format**: Modern
**Budget**: $200

Please provide comprehensive deck building guidance covering:

1. **Core Strategy**
   - Primary win conditions
   - Key synergies and interactions
   - Typical game plan

2. **Card Recommendations**
   - Essential cards for the strategy
   - Budget alternatives for expensive cards
   - Flexible slots and meta considerations

3. **Mana Base**
   - Land recommendations
   - Mana curve considerations
   - Color requirements

4. **Sideboard Strategy**
   - Key matchups to prepare for
   - Sideboard card recommendations
   - Sideboarding guide

5. **Upgrade Path**
   - Priority upgrades within budget
   - Long-term improvement suggestions
   - Meta adaptation strategies

6. **Sample Decklist**
   - Provide a complete 60-card main deck
   - Include 15-card sideboard
   - Estimate total cost

Please focus on cards legal in Modern format and consider the $200 budget constraint when making recommendations.
```

### Budget Categories

- **Budget**: Under $100
- **Mid-range**: $100-$300
- **Competitive**: $300-$800
- **Premium**: $800+

### Popular Themes

- **Aggro**: Burn, Affinity, Humans
- **Midrange**: Jund, Abzan, Sultai
- **Control**: UW Control, Jeskai Control
- **Combo**: Storm, Ad Nauseam, Amulet Titan
- **Tribal**: Goblins, Elves, Merfolk, Spirits

### Use Cases

- **New Player Guidance**: Learn deck building fundamentals
- **Format Entry**: Build first deck in new format
- **Budget Optimization**: Maximize power within constraints
- **Meta Adaptation**: Adjust existing strategies

## compare_cards

Multi-card comparison and evaluation for deck building decisions.

### Arguments

```json
{
  "cards": "Lightning Bolt,Shock,Lava Spike", // Required: Comma-separated card names
  "criteria": "mana efficiency" // Optional: Specific comparison criteria
}
```

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "compare_cards",
    "arguments": {
      "cards": "Lightning Bolt,Shock,Lava Spike",
      "criteria": "burn deck inclusion"
    }
  }
}
```

### Generated Prompt

````
Please compare the following Magic: The Gathering cards for burn deck inclusion:

**Cards to Compare**: Lightning Bolt, Shock, Lava Spike

**Card Data**:

Lightning Bolt:
```json
{
  "name": "Lightning Bolt",
  "manaCost": "{R}",
  "cmc": 1,
  "type": "Instant",
  "text": "Lightning Bolt deals 3 damage to any target.",
  "rarity": "Common"
}
````

Shock:

```json
{
  "name": "Shock",
  "manaCost": "{R}",
  "cmc": 1,
  "type": "Instant",
  "text": "Shock deals 2 damage to any target.",
  "rarity": "Common"
}
```

Lava Spike:

```json
{
  "name": "Lava Spike",
  "manaCost": "{R}",
  "cmc": 1,
  "type": "Sorcery",
  "text": "Lava Spike deals 3 damage to target player or planeswalker.",
  "rarity": "Common"
}
```

Please provide a detailed comparison focusing on burn deck inclusion:

1. **Direct Comparison**

   - Damage per mana efficiency
   - Targeting flexibility
   - Speed and timing considerations

2. **Deck Building Implications**

   - Which cards fit best in aggressive strategies
   - Situational advantages of each option
   - Meta considerations

3. **Ranking and Recommendations**

   - Rank the cards for the specified criteria
   - Explain the reasoning behind rankings
   - Suggest optimal combinations

4. **Alternative Considerations**
   - Similar cards worth considering
   - Format-specific implications
   - Budget considerations

````

### Comparison Criteria

- **Mana Efficiency**: Cost-to-effect ratio
- **Versatility**: Multiple use cases
- **Power Level**: Raw strength comparison
- **Meta Relevance**: Current format positioning
- **Synergy**: Combo potential
- **Budget**: Cost considerations

### Use Cases

- **Deck Optimization**: Choose between similar effects
- **Meta Decisions**: Adapt to format changes
- **Educational**: Learn card evaluation
- **Brewing**: Explore alternative options

## Prompt Integration Patterns

### Chaining Prompts

AI assistants can use prompts in sequence:

```python
# 1. Analyze key cards
analysis = get_prompt("analyze_card", {
    "card_name": "Lightning Bolt",
    "format": "Modern"
})

# 2. Build deck around analyzed card
deck_guide = get_prompt("build_deck", {
    "theme": "Burn",
    "format": "Modern",
    "budget": "$150"
})

# 3. Compare card options
comparison = get_prompt("compare_cards", {
    "cards": "Lightning Bolt,Lava Spike,Rift Bolt",
    "criteria": "burn deck efficiency"
})
````

### Dynamic Data Integration

Prompts automatically fetch current card data:

```python
def analyze_with_current_data(card_name, format):
    # Prompt automatically fetches latest card data
    prompt = get_prompt("analyze_card", {
        "card_name": card_name,
        "format": format
    })

    # Data includes current legality, errata, etc.
    return prompt
```

## Prompt Customization

### Format-Specific Analysis

Each format gets tailored analysis:

- **Standard**: Rotation considerations, current meta
- **Modern**: Historical context, power level comparison
- **Legacy**: Combo potential, efficiency requirements
- **Commander**: Multiplayer dynamics, political considerations
- **Pauper**: Rarity restrictions, budget optimization

### Budget-Aware Recommendations

Deck building prompts consider budget constraints:

- **Ultra Budget** (<$50): Basic lands, commons/uncommons
- **Budget** ($50-$150): Some rares, budget mana base
- **Mid-range** ($150-$400): Competitive core, good mana
- **High-end** ($400+): Optimal builds, premium cards

## Advanced Usage

### Custom Analysis Criteria

```json
{
  "method": "prompts/get",
  "params": {
    "name": "compare_cards",
    "arguments": {
      "cards": "Tarmogoyf,Scavenging Ooze,Grim Flayer",
      "criteria": "midrange creature evaluation for current meta"
    }
  }
}
```

### Multi-Format Analysis

```json
{
  "method": "prompts/get",
  "params": {
    "name": "analyze_card",
    "arguments": {
      "card_name": "Brainstorm",
      "format": "Legacy vs Vintage comparison"
    }
  }
}
```

### Comprehensive Deck Analysis

```json
{
  "method": "prompts/get",
  "params": {
    "name": "build_deck",
    "arguments": {
      "theme": "Control deck with win condition flexibility",
      "format": "Pioneer",
      "budget": "Competitive but not premium"
    }
  }
}
```

## Integration Examples

### Card Evaluation Workflow

```python
def comprehensive_card_evaluation(card_name):
    # Step 1: Detailed analysis
    analysis = get_prompt("analyze_card", {
        "card_name": card_name,
        "format": "Modern"
    })

    # Step 2: Find similar cards for comparison
    similar_cards = find_similar_cards(card_name)
    comparison = get_prompt("compare_cards", {
        "cards": f"{card_name},{','.join(similar_cards)}",
        "criteria": "competitive viability"
    })

    return {
        "analysis": analysis,
        "comparison": comparison
    }
```

### Deck Building Assistant

```python
def build_deck_with_analysis(theme, format, budget):
    # Step 1: Get deck building guidance
    deck_guide = get_prompt("build_deck", {
        "theme": theme,
        "format": format,
        "budget": budget
    })

    # Step 2: Analyze key cards mentioned
    key_cards = extract_key_cards(deck_guide)
    card_analyses = []

    for card in key_cards:
        analysis = get_prompt("analyze_card", {
            "card_name": card,
            "format": format
        })
        card_analyses.append(analysis)

    return {
        "deck_guide": deck_guide,
        "card_analyses": card_analyses
    }
```

### Meta Analysis Tool

```python
def analyze_format_meta(format_name):
    # Get top cards in format
    top_cards = get_format_staples(format_name)

    # Analyze each card
    analyses = []
    for card in top_cards:
        analysis = get_prompt("analyze_card", {
            "card_name": card,
            "format": format_name
        })
        analyses.append(analysis)

    # Compare similar cards
    comparisons = []
    card_groups = group_similar_cards(top_cards)

    for group in card_groups:
        comparison = get_prompt("compare_cards", {
            "cards": ",".join(group),
            "criteria": f"{format_name} meta positioning"
        })
        comparisons.append(comparison)

    return {
        "format": format_name,
        "card_analyses": analyses,
        "comparisons": comparisons
    }
```

## Performance and Caching

### Response Times

- **analyze_card**: 200-500ms (includes data fetch)
- **build_deck**: 100-300ms (template generation)
- **compare_cards**: 300-800ms (multiple card fetch)

### Caching Strategy

- **Templates**: Cached indefinitely (static)
- **Card Data**: Fetched fresh for each prompt
- **Generated Content**: Not cached (dynamic)

### Optimization Tips

1. **Batch Requests**: Use multiple prompts in sequence
2. **Specific Criteria**: Provide detailed comparison criteria
3. **Format Focus**: Specify exact format for better analysis
4. **Budget Clarity**: Be specific about budget constraints

---

Next: [README](./README.md) | Back: [Tools](./tools.md)
