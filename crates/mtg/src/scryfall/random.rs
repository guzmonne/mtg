use crate::prelude::*;

use super::{convert_core_card_to_cli, display_single_card_details};

pub async fn run(query: Option<&str>, pretty: bool, global: crate::Global) -> Result<()> {
    if global.verbose {
        println!("Getting random card");
        if let Some(q) = query {
            println!("With query: {q}");
        }
    }

    // Create Scryfall client using the global client
    let scryfall_client = global.create_scryfall_client()?;

    // Get random card
    let card = scryfall_client.get_random_card(query).await?;
    let cli_card = convert_core_card_to_cli(&card);

    if pretty {
        display_single_card_details(&cli_card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&cli_card)?);
    }

    Ok(())
}
