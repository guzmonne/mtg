# MCP Prompts

Prompts provide AI assistants with pre-built templates for common Magic: The Gathering analysis and discussion tasks.

## Available Prompts

The MTG MCP server provides three specialized prompts:

| Prompt              | Purpose                                   | Arguments                    |
| ------------------- | ----------------------------------------- | ---------------------------- |
| **deck_builder**    | Interactive deck building assistant       | format, archetype, budget    |
| **card_searcher**   | Advanced card search with natural language| description, format          |
| **synergy_finder**  | Find cards that synergize with existing cards | cards, format           |

## Current Implementation Status

**Note**: The prompts are currently implemented as data structures with proper MCP protocol support. The server advertises prompt capabilities (`"prompts":{}`) but the current version of mcp-core (0.1.50) may not fully support prompt registration. The prompt definitions are ready for when full prompt support is available.

## deck_builder

Interactive deck building assistant for Magic: The Gathering.

### Arguments

```json
{
  "format": "standard",        // Required: Target format (standard, modern, legacy, commander, etc.)
  "archetype": "aggro",        // Optional: Deck archetype (aggro, control, midrange, combo)
  "budget": "200"              // Optional: Budget constraint in USD
}
```

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "deck_builder",
    "arguments": {
      "format": "modern",
      "archetype": "burn",
      "budget": "150"
    }
  }
}
```

### Description

This prompt helps users build Magic: The Gathering decks by providing comprehensive guidance based on format, archetype, and budget constraints. It's designed to assist both new and experienced players in creating competitive and fun decks.

### Use Cases

- **New Player Guidance**: Learn deck building fundamentals
- **Format Entry**: Build first deck in new format  
- **Budget Optimization**: Maximize power within constraints
- **Meta Adaptation**: Adjust existing strategies

## card_searcher

Advanced card search with natural language queries.

### Arguments

```json
{
  "description": "red creatures with haste",  // Required: Natural language description of desired card
  "format": "standard"                        // Optional: Format legality requirement
}
```

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "card_searcher",
    "arguments": {
      "description": "cheap removal spells that can hit creatures and planeswalkers",
      "format": "modern"
    }
  }
}
```

### Description

This prompt enables natural language card searches, making it easier for users to find cards based on descriptions rather than exact names or complex query syntax.

### Use Cases

- **Deck Building**: Find cards that fit specific roles
- **Learning**: Discover new cards through descriptions
- **Brewing**: Explore cards for creative strategies
- **Research**: Find cards with specific characteristics

## synergy_finder

Find cards that synergize with your existing cards.

### Arguments

```json
{
  "cards": "Lightning Bolt,Monastery Swiftspear",  // Required: Comma-separated list of card names
  "format": "modern"                               // Optional: Format constraint
}
```

### Example Usage

```json
{
  "method": "prompts/get",
  "params": {
    "name": "synergy_finder",
    "arguments": {
      "cards": "Tarmogoyf,Lightning Bolt,Thoughtseize",
      "format": "modern"
    }
  }
}
```

### Description

This prompt analyzes existing cards and suggests other cards that work well together, helping users discover synergies and build more cohesive decks.

### Use Cases

- **Deck Optimization**: Find cards that work well with existing choices
- **Synergy Discovery**: Learn about card interactions
- **Brewing**: Build around specific card combinations
- **Educational**: Understand how cards work together

## Implementation Details

### Data Structures

The prompts are implemented as Rust functions that return `Prompt` structs with the following structure:

```rust
pub fn deck_building_prompt() -> Prompt {
    Prompt {
        name: "deck_builder".to_string(),
        description: Some("Interactive deck building assistant for Magic: The Gathering".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "format".to_string(),
                description: Some("Target format (standard, modern, legacy, commander, etc.)".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "archetype".to_string(),
                description: Some("Deck archetype (aggro, control, midrange, combo)".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "budget".to_string(),
                description: Some("Budget constraint in USD".to_string()),
                required: Some(false),
            },
        ]),
    }
}
```

### Server Capabilities

The MCP server advertises prompt capabilities in its initialization response:

```json
{
  "capabilities": {
    "prompts": {},
    "tools": {}
  }
}
```

### Future Enhancements

When full prompt support becomes available in mcp-core, the following features will be implemented:

1. **Dynamic Prompt Generation**: Prompts that adapt based on current meta and card data
2. **Prompt Chaining**: Ability to use multiple prompts in sequence
3. **Custom Templates**: User-defined prompt templates
4. **Context Awareness**: Prompts that remember previous interactions

## Testing

To test the prompt data structures:

```bash
# Build the project
cargo build

# Test MCP server initialization (should show prompts capability)
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0.0"}}}' | ./target/debug/mtg mcp stdio
```

Expected response should include:
```json
{
  "capabilities": {
    "prompts": {},
    "tools": {}
  }
}
```

## Integration with AI Assistants

Once full prompt support is available, AI assistants will be able to:

1. **List Available Prompts**: Discover what prompts are available
2. **Get Prompt Templates**: Retrieve prompt templates with arguments
3. **Execute Prompts**: Run prompts with specific arguments to get tailored responses

### Example Integration Flow

```python
# 1. List available prompts
prompts = client.list_prompts()

# 2. Get specific prompt template
deck_prompt = client.get_prompt("deck_builder", {
    "format": "modern",
    "archetype": "control", 
    "budget": "300"
})

# 3. Use prompt to guide deck building conversation
response = ai_assistant.chat(deck_prompt.content)
```

---

Next: [README](./README.md) | Back: [Tools](./tools.md)