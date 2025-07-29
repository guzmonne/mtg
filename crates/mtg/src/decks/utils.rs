use crate::prelude::*;

/// Convert mtg_core DeckCard to CLI DeckCard
fn convert_core_deck_card_to_cli(core_card: &mtg_core::DeckCard) -> super::DeckCard {
    super::DeckCard {
        quantity: core_card.quantity,
        name: core_card.name.clone(),
        set_code: core_card.set_code.clone(),
        collector_number: core_card.collector_number.clone(),
        card_details: core_card
            .card_details
            .as_ref()
            .map(crate::scryfall::convert_core_card_to_cli),
    }
}

/// Convert CLI DeckCard to mtg_core DeckCard
fn convert_cli_deck_card_to_core(cli_card: &super::DeckCard) -> mtg_core::DeckCard {
    mtg_core::DeckCard {
        quantity: cli_card.quantity,
        name: cli_card.name.clone(),
        set_code: cli_card.set_code.clone(),
        collector_number: cli_card.collector_number.clone(),
        card_details: None, // We'll fetch this separately
    }
}

/// Convert mtg_core DeckList to CLI DeckList
pub fn convert_core_deck_list_to_cli(core_deck_list: &mtg_core::DeckList) -> super::DeckList {
    super::DeckList {
        main_deck: core_deck_list
            .main_deck
            .iter()
            .map(convert_core_deck_card_to_cli)
            .collect(),
        sideboard: core_deck_list
            .sideboard
            .iter()
            .map(convert_core_deck_card_to_cli)
            .collect(),
    }
}

/// Convert CLI DeckList to mtg_core DeckList
fn convert_cli_deck_list_to_core(cli_deck_list: &super::DeckList) -> mtg_core::DeckList {
    mtg_core::DeckList {
        main_deck: cli_deck_list
            .main_deck
            .iter()
            .map(convert_cli_deck_card_to_core)
            .collect(),
        sideboard: cli_deck_list
            .sideboard
            .iter()
            .map(convert_cli_deck_card_to_core)
            .collect(),
    }
}

/// Fetch card details for all cards in a deck list using the global Scryfall client
pub async fn fetch_card_details_with_global(
    deck_list: super::DeckList,
    global: &crate::Global,
) -> Result<super::DeckList> {
    let scryfall_client = global.create_scryfall_client()?;
    let core_deck_list = convert_cli_deck_list_to_core(&deck_list);
    let core_deck_with_details =
        mtg_core::decks::utils::fetch_card_details(core_deck_list, &scryfall_client).await?;
    Ok(convert_core_deck_list_to_cli(&core_deck_with_details))
}

/// Calculate deck stats using mtg_core but with CLI types
pub fn calculate_deck_stats_cli(deck_list: &super::DeckList) -> Result<mtg_core::DeckStats> {
    let core_deck_list = convert_cli_deck_list_to_core(deck_list);
    mtg_core::calculate_deck_stats(&core_deck_list)
}

/// Convert mtg_core ParsedDeck to CLI DeckList
pub fn convert_parsed_deck_to_cli_deck_list(parsed_deck: &mtg_core::ParsedDeck) -> super::DeckList {
    super::DeckList {
        main_deck: parsed_deck
            .main_deck
            .iter()
            .map(convert_core_deck_card_to_cli)
            .collect(),
        sideboard: parsed_deck
            .sideboard
            .iter()
            .map(convert_core_deck_card_to_cli)
            .collect(),
    }
}
