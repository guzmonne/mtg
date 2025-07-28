use crate::prelude::*;
use mtg_core::scryfall::{QueryIntent, SmartSearchResult};

use super::{convert_core_card_to_cli, convert_core_response_to_cli, display_single_card_details};

pub async fn run(query: &str, pretty: bool, page: u32, global: crate::Global) -> Result<()> {
    let query = query.trim();

    // Create Scryfall client using the global client
    let scryfall_client = global.create_scryfall_client()?;

    // Validate query first
    if let Err(validation_error) = scryfall_client.validate_query(query) {
        aeprintln!("{validation_error}");
        return Ok(());
    }

    // Try to detect what kind of query this is
    if let Some(intent) = scryfall_client.detect_query_intent(query) {
        if global.verbose {
            match &intent {
                QueryIntent::ExactCardName(name) => {
                    aeprintln!("Auto-detected: Exact card name lookup for '{name}'");
                }
                QueryIntent::SetCollector(set, collector) => {
                    aeprintln!("Auto-detected: Set/collector lookup for {set} {collector}");
                }
                QueryIntent::ArenaId(id) => {
                    aeprintln!("Auto-detected: Arena ID lookup for {id}");
                }
                QueryIntent::MtgoId(id) => {
                    aeprintln!("Auto-detected: MTGO ID lookup for {id}");
                }
                QueryIntent::ScryfallId(_) => {
                    aeprintln!("Auto-detected: Scryfall UUID lookup");
                }
                QueryIntent::SearchQuery(search_query) => {
                    aeprintln!("Auto-detected: Search query '{search_query}'");
                }
            }
        }

        // Use smart search from mtg_core
        let result = scryfall_client.smart_search(query).await?;

        match result {
            SmartSearchResult::SingleCard(card) => {
                let cli_card = convert_core_card_to_cli(&card);
                if pretty {
                    display_single_card_details(&cli_card)?;
                } else {
                    println!("{}", serde_json::to_string_pretty(&cli_card)?);
                }
            }
            SmartSearchResult::SearchResults(response) => {
                let cli_response = convert_core_response_to_cli(&response);
                if pretty {
                    let params = super::search::Params {
                        query: query.to_string(),
                        pretty: true,
                        page,
                        order: "name".to_string(),
                        dir: "auto".to_string(),
                        include_extras: false,
                        include_multilingual: false,
                        include_variations: false,
                        unique: "cards".to_string(),
                        csv: false,
                    };
                    super::search::display_pretty_results(&cli_response, &params)?;
                } else {
                    println!("{}", serde_json::to_string_pretty(&cli_response)?);
                }
            }
        }
    } else {
        // Fallback to search if we can't detect intent
        if global.verbose {
            aeprintln!("Could not auto-detect intent, falling back to search");
        }

        let search_params = mtg_core::scryfall::SearchParams {
            q: query.to_string(),
            page: Some(page),
            order: Some("name".to_string()),
            dir: Some("auto".to_string()),
            include_extras: Some(false),
            include_multilingual: Some(false),
            include_variations: Some(false),
            unique: Some("cards".to_string()),
        };

        let response = scryfall_client.search_cards(search_params).await?;
        let cli_response = convert_core_response_to_cli(&response);

        if pretty {
            let params = super::search::Params {
                query: query.to_string(),
                pretty: true,
                page,
                order: "name".to_string(),
                dir: "auto".to_string(),
                include_extras: false,
                include_multilingual: false,
                include_variations: false,
                unique: "cards".to_string(),
                csv: false,
            };
            super::search::display_pretty_results(&cli_response, &params)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&cli_response)?);
        }
    }

    Ok(())
}
