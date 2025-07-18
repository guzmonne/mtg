use rmcp::model::*;
use serde_json::Value;

pub async fn analyze_card_prompt(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<GetPromptResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.unwrap_or_default();
    let card_name = args.get("card_name").and_then(|v| v.as_str()).unwrap_or("Lightning Bolt");
    let format = args.get("format").and_then(|v| v.as_str()).unwrap_or("Modern");
    
    // Fetch card data
    let url = format!("{}/cards", base_url);
    let response = client
        .get(&url)
        .query(&[("name", card_name)])
        .query(&[("pageSize", "1")])
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    let card_data = if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        if let Some(card) = cards.first() {
            serde_json::to_string_pretty(card)?
        } else {
            format!("Card '{}' not found", card_name)
        }
    } else {
        format!("No data found for card '{}'", card_name)
    };
    
    let prompt_text = format!(
        r#"Please analyze the Magic: The Gathering card "{}" for competitive play in the {} format.

Card Data:
```json
{}
```

Please provide a comprehensive analysis covering:

1. **Power Level Assessment**
   - Rate the card's overall power level (1-10)
   - Compare to similar cards in the format
   - Identify key strengths and weaknesses

2. **Competitive Viability**
   - Current meta relevance in {}
   - Deck archetypes that would play this card
   - Synergies with popular cards in the format

3. **Strategic Analysis**
   - Best use cases and timing
   - Common play patterns
   - Counterplay and answers

4. **Format-Specific Considerations**
   - How the card performs in the current {} meta
   - Historical performance if applicable
   - Future potential with upcoming sets

Please be specific and provide concrete examples where possible."#,
        card_name, format, card_data, format, format
    );
    
    Ok(GetPromptResult {
        description: Some(format!("Analysis of {} for {} format", card_name, format)),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text {
                text: prompt_text,
            },
        }],
    })
}

pub async fn build_deck_prompt(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<GetPromptResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.unwrap_or_default();
    let theme = args.get("theme").and_then(|v| v.as_str()).unwrap_or("Aggro");
    let format = args.get("format").and_then(|v| v.as_str()).unwrap_or("Standard");
    let budget = args.get("budget").and_then(|v| v.as_str()).unwrap_or("No specific budget");
    
    // Fetch some relevant cards based on theme
    let search_term = match theme.to_lowercase().as_str() {
        s if s.contains("aggro") => "haste",
        s if s.contains("control") => "counter",
        s if s.contains("combo") => "draw",
        s if s.contains("midrange") => "creature",
        _ => theme,
    };
    
    let url = format!("{}/cards", base_url);
    let response = client
        .get(&url)
        .query(&[("text", search_term)])
        .query(&[("pageSize", "10")])
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    let sample_cards = if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        cards.iter()
            .take(5)
            .filter_map(|card| {
                let name = card.get("name").and_then(|n| n.as_str())?;
                let mana_cost = card.get("manaCost").and_then(|m| m.as_str()).unwrap_or("N/A");
                let card_type = card.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
                Some(format!("- {} ({}) - {}", name, mana_cost, card_type))
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        "No sample cards found".to_string()
    };
    
    let prompt_text = format!(
        r#"Help me build a Magic: The Gathering deck with the following specifications:

**Deck Theme/Strategy:** {}
**Format:** {}
**Budget:** {}

**Sample Cards Related to Theme:**
{}

Please provide a comprehensive deck building guide including:

1. **Core Strategy**
   - Main win conditions
   - Key synergies to build around
   - Typical game plan

2. **Card Categories**
   - Threats (creatures/planeswalkers)
   - Removal/interaction
   - Card advantage engines
   - Mana base considerations

3. **Specific Card Recommendations**
   - Must-have cards for this strategy
   - Budget alternatives if applicable
   - Flexible slots for meta adjustments

4. **Sideboard Strategy** (if applicable to format)
   - Common matchups to prepare for
   - Sideboard card suggestions
   - Sideboarding plans

5. **Mulligan Guide**
   - Ideal opening hands
   - Cards to mulligan away
   - Key cards to keep

6. **Play Tips**
   - Common sequencing decisions
   - Matchup-specific advice
   - Common mistakes to avoid

Please tailor the recommendations specifically for the {} format and consider the current meta."#,
        theme, format, budget, sample_cards, format
    );
    
    Ok(GetPromptResult {
        description: Some(format!("Deck building guide for {} {} deck", format, theme)),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text {
                text: prompt_text,
            },
        }],
    })
}

pub async fn compare_cards_prompt(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<GetPromptResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.unwrap_or_default();
    let cards_str = args.get("cards").and_then(|v| v.as_str()).unwrap_or("Lightning Bolt,Shock");
    let criteria = args.get("criteria").and_then(|v| v.as_str()).unwrap_or("overall power level");
    
    let card_names: Vec<&str> = cards_str.split(',').map(|s| s.trim()).collect();
    
    // Fetch data for each card
    let mut card_data = Vec::new();
    for card_name in &card_names {
        let url = format!("{}/cards", base_url);
        let response = client
            .get(&url)
            .query(&[("name", card_name)])
            .query(&[("pageSize", "1")])
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
            if let Some(card) = cards.first() {
                card_data.push((card_name, serde_json::to_string_pretty(card)?));
            } else {
                card_data.push((card_name, format!("Card '{}' not found", card_name)));
            }
        }
    }
    
    let cards_data_text = card_data
        .iter()
        .map(|(name, data)| format!("**{}:**\n```json\n{}\n```", name, data))
        .collect::<Vec<_>>()
        .join("\n\n");
    
    let prompt_text = format!(
        r#"Please compare the following Magic: The Gathering cards based on {}:

{}

Please provide a detailed comparison covering:

1. **Direct Comparison**
   - Side-by-side analysis of key attributes
   - Mana efficiency comparison
   - Power level assessment

2. **Situational Analysis**
   - When to choose each card
   - Deck archetypes that prefer each option
   - Meta considerations

3. **Synergy Potential**
   - Cards that work well with each option
   - Combo potential
   - Build-around considerations

4. **Format Considerations**
   - Performance in different formats
   - Legality restrictions
   - Historical impact

5. **Final Recommendation**
   - Which card is better overall
   - Specific use cases for each
   - Factors that might change the evaluation

Focus particularly on: {}

Please provide specific examples and concrete reasoning for your analysis."#,
        criteria, cards_data_text, criteria
    );
    
    Ok(GetPromptResult {
        description: Some(format!("Comparison of {} cards based on {}", card_names.join(", "), criteria)),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text {
                text: prompt_text,
            },
        }],
    })
}