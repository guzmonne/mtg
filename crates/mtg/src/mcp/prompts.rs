use mcp_core::types::{Prompt, PromptArgument};

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
