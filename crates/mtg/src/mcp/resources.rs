use rmcp::model::*;
use serde_json::Value;

pub async fn read_cards_resource(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<ReadResourceResult, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/cards", base_url);
    let response = client
        .get(&url)
        .query(&[("pageSize", "50")])
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    let content = serde_json::to_string_pretty(&json)?;
    
    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: "mtg://cards".to_string(),
            mime_type: Some("application/json".to_string()),
            text: content,
        }],
    })
}

pub async fn read_sets_resource(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<ReadResourceResult, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/sets", base_url);
    let response = client
        .get(&url)
        .query(&[("pageSize", "50")])
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    let content = serde_json::to_string_pretty(&json)?;
    
    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: "mtg://sets".to_string(),
            mime_type: Some("application/json".to_string()),
            text: content,
        }],
    })
}

pub async fn read_types_resource(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<ReadResourceResult, Box<dyn std::error::Error + Send + Sync>> {
    // Fetch all type categories
    let types_url = format!("{}/types", base_url);
    let subtypes_url = format!("{}/subtypes", base_url);
    let supertypes_url = format!("{}/supertypes", base_url);
    let formats_url = format!("{}/formats", base_url);
    
    let (types_resp, subtypes_resp, supertypes_resp, formats_resp) = tokio::try_join!(
        client.get(&types_url).send(),
        client.get(&subtypes_url).send(),
        client.get(&supertypes_url).send(),
        client.get(&formats_url).send(),
    )?;
    
    let types: Value = types_resp.json().await?;
    let subtypes: Value = subtypes_resp.json().await?;
    let supertypes: Value = supertypes_resp.json().await?;
    let formats: Value = formats_resp.json().await?;
    
    let combined = serde_json::json!({
        "types": types,
        "subtypes": subtypes,
        "supertypes": supertypes,
        "formats": formats
    });
    
    let content = serde_json::to_string_pretty(&combined)?;
    
    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: "mtg://types".to_string(),
            mime_type: Some("application/json".to_string()),
            text: content,
        }],
    })
}