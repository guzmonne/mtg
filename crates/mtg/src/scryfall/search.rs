use crate::prelude::*;
use prettytable::{Cell, Row};

use super::{
    convert_core_card_to_cli, convert_core_response_to_cli, display_single_card_details, Card, List,
};

/// Type alias for backward compatibility
pub type Response = List<Card>;

pub struct Params {
    pub query: String,
    pub pretty: bool,
    pub page: u32,
    pub order: String,
    pub dir: String,
    pub include_extras: bool,
    pub include_multilingual: bool,
    pub include_variations: bool,
    pub unique: String,
    pub csv: bool,
}

pub struct AdvancedParams {
    pub name: Option<String>,
    pub oracle: Option<String>,
    pub card_type: Option<String>,
    pub colors: Option<String>,
    pub identity: Option<String>,
    pub mana: Option<String>,
    pub mv: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub set: Option<String>,
    pub rarity: Option<String>,
    pub artist: Option<String>,
    pub flavor: Option<String>,
    pub format: Option<String>,
    pub language: Option<String>,
    pub pretty: bool,
    pub page: u32,
    pub order: String,
    pub dir: String,
    pub include_extras: bool,
    pub include_multilingual: bool,
    pub include_variations: bool,
    pub unique: String,
}

pub async fn run(params: Params, global: crate::Global) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    // Handle CSV response - use raw API call for CSV
    if params.csv {
        let mut query_params = vec![
            ("q".to_string(), params.query.clone()),
            ("page".to_string(), params.page.to_string()),
            ("order".to_string(), params.order.clone()),
            ("dir".to_string(), params.dir.clone()),
            ("unique".to_string(), params.unique.clone()),
            ("format".to_string(), "csv".to_string()),
        ];

        if params.include_extras {
            query_params.push(("include_extras".to_string(), "true".to_string()));
        }
        if params.include_multilingual {
            query_params.push(("include_multilingual".to_string(), "true".to_string()));
        }
        if params.include_variations {
            query_params.push(("include_variations".to_string(), "true".to_string()));
        }

        let csv_response = client
            .get_raw_with_params("cards/search", query_params)
            .await?;
        println!("{csv_response}");
        return Ok(());
    }

    // Build search parameters for mtg_core
    let search_params = mtg_core::scryfall::SearchParams {
        q: params.query.clone(),
        unique: Some(params.unique.clone()),
        order: Some(params.order.clone()),
        dir: Some(params.dir.clone()),
        include_extras: if params.include_extras {
            Some(true)
        } else {
            None
        },
        include_multilingual: if params.include_multilingual {
            Some(true)
        } else {
            None
        },
        include_variations: if params.include_variations {
            Some(true)
        } else {
            None
        },
        page: Some(params.page),
    };

    if global.verbose {
        println!("Search query: {}", params.query);
    }

    // Use mtg_core client to search
    let core_response = client.search_cards(search_params).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    if params.pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

pub fn display_pretty_results(response: &Response, params: &Params) -> Result<()> {
    let mut table = new_table();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Cost"),
        Cell::new("Type"),
        Cell::new("Set"),
        Cell::new("Rarity"),
        Cell::new("P/T/L"),
    ]));

    for card in &response.data {
        let mana_cost = card.mana_cost.as_deref().unwrap_or("");
        let pt_loyalty = if let Some(loyalty) = &card.loyalty {
            loyalty.clone()
        } else if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
            format!("{power}/{toughness}")
        } else {
            "-".to_string()
        };

        table.add_row(Row::new(vec![
            Cell::new(&card.name),
            Cell::new(mana_cost),
            Cell::new(&card.type_line),
            Cell::new(&card.set_name),
            Cell::new(&card.rarity),
            Cell::new(&pt_loyalty),
        ]));
    }

    table.printstd();

    // Display pagination summary
    aeprintln!();
    aeprintln!(
        "Found {} cards (showing {} on page {})",
        response.total_cards.unwrap_or(response.data.len() as u32),
        response.data.len(),
        params.page
    );

    if response.has_more {
        aeprintln!();
        aeprintln!("Pagination commands:");

        let base_cmd = format!("mtg scryfall search \"{}\"", params.query);

        if params.page > 1 {
            aeprintln!("Previous page: {base_cmd} --page {}", params.page - 1);
        }
        aeprintln!("Next page: {base_cmd} --page {}", params.page + 1);
        aeprintln!("Jump to page: {base_cmd} --page <PAGE_NUMBER>");
    }

    Ok(())
}

pub async fn json(params: Params, global: crate::Global) -> Result<Response> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    // Build search parameters for mtg_core
    let search_params = mtg_core::scryfall::SearchParams {
        q: params.query.clone(),
        unique: Some(params.unique.clone()),
        order: Some(params.order.clone()),
        dir: Some(params.dir.clone()),
        include_extras: if params.include_extras {
            Some(true)
        } else {
            None
        },
        include_multilingual: if params.include_multilingual {
            Some(true)
        } else {
            None
        },
        include_variations: if params.include_variations {
            Some(true)
        } else {
            None
        },
        page: Some(params.page),
    };

    // Use mtg_core client to search
    let core_response = client.search_cards(search_params).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    Ok(search_response)
}

pub async fn advanced_json(params: AdvancedParams, global: crate::Global) -> Result<Response> {
    let query = build_advanced_query(&params);

    if query.is_empty() {
        return Err(
            crate::error::Error::Generic("No search parameters provided".to_string()).into(),
        );
    }

    let search_params = Params {
        query,
        pretty: false,
        page: params.page,
        order: params.order.clone(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    json(search_params, global).await
}

pub fn build_advanced_query(params: &AdvancedParams) -> String {
    let mut query_parts = Vec::new();

    if let Some(name) = &params.name {
        if name.contains(' ') || name.contains('"') {
            query_parts.push(format!("name:\"{}\"", name.replace('"', "\\\"")));
        } else {
            query_parts.push(name.clone());
        }
    }

    if let Some(oracle) = &params.oracle {
        query_parts.push(format!("oracle:\"{}\"", oracle.replace('"', "\\\"")));
    }

    if let Some(card_type) = &params.card_type {
        query_parts.push(format!("type:{card_type}"));
    }

    if let Some(colors) = &params.colors {
        query_parts.push(format!("color:{colors}"));
    }

    if let Some(identity) = &params.identity {
        query_parts.push(format!("identity:{identity}"));
    }

    if let Some(mana) = &params.mana {
        query_parts.push(format!("mana:{mana}"));
    }

    if let Some(mv) = &params.mv {
        query_parts.push(format!("manavalue:{mv}"));
    }

    if let Some(power) = &params.power {
        query_parts.push(format!("power:{power}"));
    }

    if let Some(toughness) = &params.toughness {
        query_parts.push(format!("toughness:{toughness}"));
    }

    if let Some(loyalty) = &params.loyalty {
        query_parts.push(format!("loyalty:{loyalty}"));
    }

    if let Some(set) = &params.set {
        query_parts.push(format!("set:{set}"));
    }

    if let Some(rarity) = &params.rarity {
        query_parts.push(format!("rarity:{rarity}"));
    }

    if let Some(artist) = &params.artist {
        query_parts.push(format!("artist:\"{}\"", artist.replace('"', "\\\"")));
    }

    if let Some(flavor) = &params.flavor {
        query_parts.push(format!("flavor:\"{}\"", flavor.replace('"', "\\\"")));
    }

    if let Some(format) = &params.format {
        query_parts.push(format!("format:{format}"));
    }

    if let Some(language) = &params.language {
        query_parts.push(format!("lang:{language}"));
    }

    query_parts.join(" ")
}

pub async fn advanced(params: AdvancedParams, global: crate::Global) -> Result<()> {
    let query = build_advanced_query(&params);

    if query.is_empty() {
        return Err(
            crate::error::Error::Generic("No search parameters provided".to_string()).into(),
        );
    }

    if global.verbose {
        println!("Built query: {query}");
    }

    let search_params = Params {
        query,
        pretty: params.pretty,
        page: params.page,
        order: params.order.clone(),
        dir: params.dir.clone(),
        include_extras: params.include_extras,
        include_multilingual: params.include_multilingual,
        include_variations: params.include_variations,
        unique: params.unique.clone(),
        csv: false,
    };

    // Use the existing search_cards function
    run(search_params, global).await
}

pub async fn by_name(
    name: &str,
    pretty: bool,
    set_code: Option<&str>,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card: {name}");
        if let Some(set) = set_code {
            println!("In set: {set}");
        }
    }

    // Use mtg_core client to get card by name
    let core_card = client.get_card_named(name, set_code).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_collector(
    set_code: &str,
    collector_number: &str,
    lang: Option<&str>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by collector: {set_code} #{collector_number}");
        if let Some(language) = lang {
            println!("Language: {language}");
        }
    }

    // Use mtg_core client to get card by collector number
    let core_card = client
        .get_card_by_collector(set_code, collector_number, lang)
        .await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_arena_id(arena_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by Arena ID: {arena_id}");
    }

    // Use mtg_core client to get card by Arena ID
    let core_card = client.get_card_by_arena_id(arena_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_id(id: &str, pretty: bool, global: crate::Global) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by ID: {id}");
    }

    // Use mtg_core client to get card by ID
    let core_card = client.get_card_by_id(id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_mtgo_id(mtgo_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by MTGO ID: {mtgo_id}");
    }

    // Use mtg_core client to get card by MTGO ID
    let core_card = client.get_card_by_mtgo_id(mtgo_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_cardmarket_id(
    cardmarket_id: u32,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by Cardmarket ID: {cardmarket_id}");
    }

    // Use mtg_core client to get card by Cardmarket ID
    let core_card = client.get_card_by_cardmarket_id(cardmarket_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_tcgplayer_id(tcgplayer_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by TCGPlayer ID: {tcgplayer_id}");
    }

    // Use mtg_core client to get card by TCGPlayer ID
    let core_card = client.get_card_by_tcgplayer_id(tcgplayer_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_multiverse_id(
    multiverse_id: u32,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        println!("Looking up card by Multiverse ID: {multiverse_id}");
    }

    // Use mtg_core client to get card by Multiverse ID
    let core_card = client.get_card_by_multiverse_id(multiverse_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

// TODO: Use this when we make the event parser async
#[allow(dead_code)]
pub async fn get_card_by_arena_id(arena_id: u32) -> Result<Card> {
    let global = crate::Global::new();

    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    // Use mtg_core client to get card by Arena ID
    let core_card = client.get_card_by_arena_id(arena_id).await?;

    // Convert to CLI type
    let card = convert_core_card_to_cli(&core_card);

    Ok(card)
}

pub async fn commanders(
    identity: Option<String>,
    mana_value: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        aeprintln!(
            "Searching for commanders with identity: {:?}, mana value: {:?}",
            identity,
            mana_value
        );
    }

    // Use mtg_core client to search commanders
    let core_response = client.search_commanders(identity, mana_value).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    let params = Params {
        query: "commanders".to_string(), // For display purposes
        pretty,
        page: 1,
        order: "name".to_string(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    if pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

pub async fn planeswalkers(
    color: Option<String>,
    loyalty: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        aeprintln!(
            "Searching for planeswalkers with color: {:?}, loyalty: {:?}, format: {:?}",
            color,
            loyalty,
            format
        );
    }

    // Use mtg_core client to search planeswalkers
    let core_response = client.search_planeswalkers(color, loyalty, format).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    let params = Params {
        query: "planeswalkers".to_string(), // For display purposes
        pretty,
        page: 1,
        order: "name".to_string(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    if pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

pub async fn sorceries(
    color: Option<String>,
    mana_value: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        aeprintln!(
            "Searching for sorceries with color: {:?}, mana value: {:?}, format: {:?}",
            color,
            mana_value,
            format
        );
    }

    // Use mtg_core client to search sorceries
    let core_response = client.search_sorceries(color, mana_value, format).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    let params = Params {
        query: "sorceries".to_string(), // For display purposes
        pretty,
        page: 1,
        order: "name".to_string(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    if pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

pub async fn instants(
    color: Option<String>,
    mana_value: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        aeprintln!(
            "Searching for instants with color: {:?}, mana value: {:?}, format: {:?}",
            color,
            mana_value,
            format
        );
    }

    // Use mtg_core client to search instants
    let core_response = client.search_instants(color, mana_value, format).await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    let params = Params {
        query: "instants".to_string(), // For display purposes
        pretty,
        page: 1,
        order: "name".to_string(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    if pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

pub async fn creatures(
    color: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    mana_value: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    // Create mtg_core client with proper configuration
    let client = mtg_core::scryfall::ScryfallClient::builder()
        .timeout_secs(global.timeout)
        .verbose(global.verbose)
        .build()?;

    if global.verbose {
        aeprintln!("Searching for creatures with color: {:?}, power: {:?}, toughness: {:?}, mana value: {:?}, format: {:?}", 
                  color, power, toughness, mana_value, format);
    }

    // Use mtg_core client to search creatures
    let core_response = client
        .search_creatures(color, power, toughness, mana_value, format)
        .await?;

    // Convert to CLI types
    let search_response = convert_core_response_to_cli(&core_response);

    let params = Params {
        query: "creatures".to_string(), // For display purposes
        pretty,
        page: 1,
        order: "name".to_string(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
        csv: false,
    };

    if pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}
