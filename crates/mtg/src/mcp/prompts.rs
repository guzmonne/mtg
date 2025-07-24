use mcp_core::types::{Prompt, PromptArgument};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_building_prompt() {
        let prompt = deck_building_prompt();
        assert_eq!(prompt.name, "deck_builder");
        assert!(prompt.description.is_some());
        assert!(prompt.arguments.is_some());

        let args = prompt.arguments.unwrap();
        assert_eq!(args.len(), 3);

        // Check required format argument
        let format_arg = &args[0];
        assert_eq!(format_arg.name, "format");
        assert_eq!(format_arg.required, Some(true));

        // Check optional archetype argument
        let archetype_arg = &args[1];
        assert_eq!(archetype_arg.name, "archetype");
        assert_eq!(archetype_arg.required, Some(false));

        // Check optional budget argument
        let budget_arg = &args[2];
        assert_eq!(budget_arg.name, "budget");
        assert_eq!(budget_arg.required, Some(false));
    }

    #[test]
    fn test_card_search_prompt() {
        let prompt = card_search_prompt();
        assert_eq!(prompt.name, "card_searcher");
        assert!(prompt.description.is_some());
        assert!(prompt.arguments.is_some());

        let args = prompt.arguments.unwrap();
        assert_eq!(args.len(), 2);

        // Check required description argument
        let desc_arg = &args[0];
        assert_eq!(desc_arg.name, "description");
        assert_eq!(desc_arg.required, Some(true));

        // Check optional format argument
        let format_arg = &args[1];
        assert_eq!(format_arg.name, "format");
        assert_eq!(format_arg.required, Some(false));
    }

    #[test]
    fn test_synergy_finder_prompt() {
        let prompt = synergy_finder_prompt();
        assert_eq!(prompt.name, "synergy_finder");
        assert!(prompt.description.is_some());
        assert!(prompt.arguments.is_some());

        let args = prompt.arguments.unwrap();
        assert_eq!(args.len(), 2);

        // Check required cards argument
        let cards_arg = &args[0];
        assert_eq!(cards_arg.name, "cards");
        assert_eq!(cards_arg.required, Some(true));

        // Check optional format argument
        let format_arg = &args[1];
        assert_eq!(format_arg.name, "format");
        assert_eq!(format_arg.required, Some(false));
    }
}

#[allow(dead_code)]
pub fn deck_building_prompt() -> Prompt {
    Prompt {
        name: "deck_builder".to_string(),
        description: Some(
            "Interactive deck building assistant for Magic: The Gathering".to_string(),
        ),
        arguments: Some(vec![
            PromptArgument {
                name: "format".to_string(),
                description: Some(
                    "Target format (standard, modern, legacy, commander, etc.)".to_string(),
                ),
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

#[allow(dead_code)]
pub fn card_search_prompt() -> Prompt {
    Prompt {
        name: "card_searcher".to_string(),
        description: Some("Advanced card search with natural language queries".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "description".to_string(),
                description: Some("Natural language description of desired card".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "format".to_string(),
                description: Some("Format legality requirement".to_string()),
                required: Some(false),
            },
        ]),
    }
}

#[allow(dead_code)]
pub fn synergy_finder_prompt() -> Prompt {
    Prompt {
        name: "synergy_finder".to_string(),
        description: Some("Find cards that synergize with your existing cards".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "cards".to_string(),
                description: Some("Comma-separated list of card names".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "format".to_string(),
                description: Some("Format constraint".to_string()),
                required: Some(false),
            },
        ]),
    }
}
