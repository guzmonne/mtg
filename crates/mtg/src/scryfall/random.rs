use crate::prelude::*;

use super::{display_single_card_details, parse_scryfall_card_response};

pub async fn run(query: Option<&str>, pretty: bool, global: crate::Global) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = if let Some(q) = query {
        format!(
            "https://api.scryfall.com/cards/random?q={}",
            urlencoding::encode(q)
        )
    } else {
        "https://api.scryfall.com/cards/random".to_string()
    };

    if global.verbose {
        println!("Getting random card");
        if let Some(q) = query {
            println!("With query: {q}");
        }
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}
