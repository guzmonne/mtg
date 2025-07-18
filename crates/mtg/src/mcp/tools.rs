use rmcp::model::*;
use serde_json::Value;

pub async fn search_cards(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<CallToolResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.unwrap_or_default();
    let url = format!("{}/cards", base_url);
    
    let mut request = client.get(&url);
    
    // Add query parameters based on arguments
    if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
        request = request.query(&[("name", name)]);
    }
    if let Some(colors) = args.get("colors").and_then(|v| v.as_str()) {
        request = request.query(&[("colors", colors)]);
    }
    if let Some(card_type) = args.get("type").and_then(|v| v.as_str()) {
        request = request.query(&[("type", card_type)]);
    }
    if let Some(rarity) = args.get("rarity").and_then(|v| v.as_str()) {
        request = request.query(&[("rarity", rarity)]);
    }
    if let Some(set) = args.get("set").and_then(|v| v.as_str()) {
        request = request.query(&[("set", set)]);
    }
    if let Some(cmc) = args.get("cmc").and_then(|v| v.as_u64()) {
        request = request.query(&[("cmc", cmc.to_string())]);
    }
    
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20);
    request = request.query(&[("pageSize", limit.to_string())]);
    
    let response = request.send().await?;
    let json: Value = response.json().await?;
    
    let content = if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        let mut result = Vec::new();
        for card in cards.iter().take(limit as usize) {
            let name = card.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown");
            let mana_cost = card.get("manaCost").and_then(|m| m.as_str()).unwrap_or("N/A");
            let card_type = card.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
            let rarity = card.get("rarity").and_then(|r| r.as_str()).unwrap_or("N/A");
            let set = card.get("set").and_then(|s| s.as_str()).unwrap_or("N/A");
            
            result.push(format!("**{}** ({})\n- Type: {}\n- Rarity: {}\n- Set: {}\n", 
                name, mana_cost, card_type, rarity, set));
        }
        
        if result.is_empty() {
            "No cards found matching the search criteria.".to_string()
        } else {
            format!("Found {} cards:\n\n{}", result.len(), result.join("\n"))
        }
    } else {
        "No cards found.".to_string()
    };
    
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

pub async fn get_card(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<CallToolResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.ok_or("Missing arguments")?;
    let id = args.get("id").and_then(|v| v.as_str()).ok_or("Missing 'id' argument")?;
    
    let url = format!("{}/cards/{}", base_url, id);
    let response = client.get(&url).send().await?;
    
    if response.status() == 404 {
        return Ok(CallToolResult::error(vec![Content::text(format!("Card with ID '{}' not found.", id))]));
    }
    
    let json: Value = response.json().await?;
    
    let content = if let Some(card) = json.get("card") {
        let mut details = Vec::new();
        
        if let Some(name) = card.get("name").and_then(|n| n.as_str()) {
            details.push(format!("**Name:** {}", name));
        }
        if let Some(mana_cost) = card.get("manaCost").and_then(|m| m.as_str()) {
            details.push(format!("**Mana Cost:** {}", mana_cost));
        }
        if let Some(cmc) = card.get("cmc").and_then(|c| c.as_u64()) {
            details.push(format!("**CMC:** {}", cmc));
        }
        if let Some(card_type) = card.get("type").and_then(|t| t.as_str()) {
            details.push(format!("**Type:** {}", card_type));
        }
        if let Some(rarity) = card.get("rarity").and_then(|r| r.as_str()) {
            details.push(format!("**Rarity:** {}", rarity));
        }
        if let Some(set) = card.get("set").and_then(|s| s.as_str()) {
            details.push(format!("**Set:** {}", set));
        }
        if let Some(text) = card.get("text").and_then(|t| t.as_str()) {
            details.push(format!("**Text:** {}", text));
        }
        if let Some(power) = card.get("power").and_then(|p| p.as_str()) {
            if let Some(toughness) = card.get("toughness").and_then(|t| t.as_str()) {
                details.push(format!("**Power/Toughness:** {}/{}", power, toughness));
            }
        }
        if let Some(loyalty) = card.get("loyalty").and_then(|l| l.as_str()) {
            details.push(format!("**Loyalty:** {}", loyalty));
        }
        if let Some(artist) = card.get("artist").and_then(|a| a.as_str()) {
            details.push(format!("**Artist:** {}", artist));
        }
        
        details.join("\n")
    } else {
        "Card details not found.".to_string()
    };
    
    Ok(CallToolResult::success(vec![Content::text(content)]))
}
pub async fn list_sets(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<CallToolResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.unwrap_or_default();
    let url = format!("{}/sets", base_url);
    
    let mut request = client.get(&url);
    
    if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
        request = request.query(&[("name", name)]);
    }
    if let Some(block) = args.get("block").and_then(|v| v.as_str()) {
        request = request.query(&[("block", block)]);
    }
    
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20);
    request = request.query(&[("pageSize", limit.to_string())]);
    
    let response = request.send().await?;
    let json: Value = response.json().await?;
    
    let content = if let Some(sets) = json.get("sets").and_then(|s| s.as_array()) {
        let mut result = Vec::new();
        for set in sets.iter().take(limit as usize) {
            let name = set.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown");
            let code = set.get("code").and_then(|c| c.as_str()).unwrap_or("N/A");
            let set_type = set.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
            let release_date = set.get("releaseDate").and_then(|r| r.as_str()).unwrap_or("N/A");
            let block = set.get("block").and_then(|b| b.as_str()).unwrap_or("N/A");
            
            result.push(format!("**{}** ({})\n- Type: {}\n- Block: {}\n- Release Date: {}\n", 
                name, code, set_type, block, release_date));
        }
        
        if result.is_empty() {
            "No sets found matching the criteria.".to_string()
        } else {
            format!("Found {} sets:\n\n{}", result.len(), result.join("\n"))
        }
    } else {
        "No sets found.".to_string()
    };
    
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

pub async fn generate_booster(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<CallToolResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.ok_or("Missing arguments")?;
    let set_code = args.get("set_code").and_then(|v| v.as_str()).ok_or("Missing 'set_code' argument")?;
    
    let url = format!("{}/sets/{}/booster", base_url, set_code);
    let response = client.get(&url).send().await?;
    
    if response.status() == 404 {
        return Ok(CallToolResult::error(vec![Content::text(format!("Set '{}' not found or booster generation not available.", set_code))]));
    }
    
    let json: Value = response.json().await?;
    
    let content = if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        let mut result = Vec::new();
        result.push(format!("**Booster Pack for {}**\n", set_code.to_uppercase()));
        
        // Group cards by rarity
        let mut rares = Vec::new();
        let mut uncommons = Vec::new();
        let mut commons = Vec::new();
        let mut others = Vec::new();
        
        for card in cards {
            let name = card.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown");
            let rarity = card.get("rarity").and_then(|r| r.as_str()).unwrap_or("Unknown");
            let mana_cost = card.get("manaCost").and_then(|m| m.as_str()).unwrap_or("");
            
            let card_line = if mana_cost.is_empty() {
                format!("- {}", name)
            } else {
                format!("- {} ({})", name, mana_cost)
            };
            
            match rarity {
                "Mythic Rare" | "Rare" => rares.push(card_line),
                "Uncommon" => uncommons.push(card_line),
                "Common" => commons.push(card_line),
                _ => others.push(card_line),
            }
        }
        
        if !rares.is_empty() {
            result.push("**Rare/Mythic:**".to_string());
            result.extend(rares);
            result.push("".to_string());
        }
        if !uncommons.is_empty() {
            result.push("**Uncommon:**".to_string());
            result.extend(uncommons);
            result.push("".to_string());
        }
        if !commons.is_empty() {
            result.push("**Common:**".to_string());
            result.extend(commons);
            result.push("".to_string());
        }
        if !others.is_empty() {
            result.push("**Other:**".to_string());
            result.extend(others);
        }
        
        result.join("\n")
    } else {
        "Failed to generate booster pack.".to_string()
    };
    
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

pub async fn get_card_types(
    client: &reqwest::Client,
    base_url: &str,
    arguments: Option<Value>,
) -> Result<CallToolResult, Box<dyn std::error::Error + Send + Sync>> {
    let args = arguments.ok_or("Missing arguments")?;
    let category = args.get("category").and_then(|v| v.as_str()).ok_or("Missing 'category' argument")?;
    
    let url = format!("{}/{}", base_url, category);
    let response = client.get(&url).send().await?;
    let json: Value = response.json().await?;
    
    let content = match category {
        "types" => {
            if let Some(types) = json.get("types").and_then(|t| t.as_array()) {
                let type_list: Vec<String> = types
                    .iter()
                    .filter_map(|t| t.as_str())
                    .map(|s| format!("- {}", s))
                    .collect();
                format!("**Magic Card Types:**\n\n{}", type_list.join("\n"))
            } else {
                "No types found.".to_string()
            }
        },
        "subtypes" => {
            if let Some(subtypes) = json.get("subtypes").and_then(|s| s.as_array()) {
                let subtype_list: Vec<String> = subtypes
                    .iter()
                    .filter_map(|s| s.as_str())
                    .map(|s| format!("- {}", s))
                    .collect();
                format!("**Magic Card Subtypes:**\n\n{}", subtype_list.join("\n"))
            } else {
                "No subtypes found.".to_string()
            }
        },
        "supertypes" => {
            if let Some(supertypes) = json.get("supertypes").and_then(|s| s.as_array()) {
                let supertype_list: Vec<String> = supertypes
                    .iter()
                    .filter_map(|s| s.as_str())
                    .map(|s| format!("- {}", s))
                    .collect();
                format!("**Magic Card Supertypes:**\n\n{}", supertype_list.join("\n"))
            } else {
                "No supertypes found.".to_string()
            }
        },
        "formats" => {
            if let Some(formats) = json.get("formats").and_then(|f| f.as_array()) {
                let format_list: Vec<String> = formats
                    .iter()
                    .filter_map(|f| f.as_str())
                    .map(|s| format!("- {}", s))
                    .collect();
                format!("**Magic Game Formats:**\n\n{}", format_list.join("\n"))
            } else {
                "No formats found.".to_string()
            }
        },
        _ => format!("Unknown category: {}", category),
    };
    
    Ok(CallToolResult::success(vec![Content::text(content)]))
}