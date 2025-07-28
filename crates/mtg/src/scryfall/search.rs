use crate::cache::CacheManager;
use crate::prelude::*;
use prettytable::{Cell, Row};

use super::{
    display_single_card_details, enhance_error_message, format_color_identity_query,
    format_color_query, format_comparison, parse_scryfall_card_response, parse_scryfall_response,
    Card, List,
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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL with query parameters
    let url = "https://api.scryfall.com/cards/search".to_string();
    let mut query_params = vec![
        ("q", params.query.clone()),
        ("page", params.page.to_string()),
        ("order", params.order.clone()),
        ("dir", params.dir.clone()),
        ("unique", params.unique.clone()),
    ];

    if params.include_extras {
        query_params.push(("include_extras", "true".to_string()));
    }
    if params.include_multilingual {
        query_params.push(("include_multilingual", "true".to_string()));
    }
    if params.include_variations {
        query_params.push(("include_variations", "true".to_string()));
    }

    // Generate cache key
    let cache_key = CacheManager::hash_request(&(&url, &query_params));

    if global.verbose {
        println!("Search query: {}", params.query);
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let response: Response = serde_json::from_value(cached_response.data)?;
        if params.pretty {
            display_pretty_results(&response, &params)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
    }

    // Build the full URL with query parameters
    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let full_url = format!("{url}?{query_string}");

    if global.verbose {
        println!("Request URL: {full_url}");
    }

    let response = client.get(&full_url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    if global.verbose {
        println!("Response length: {} characters", response_text.len());
    }

    // Handle CSV response
    if params.csv {
        println!("{response_text}");
        return Ok(());
    }

    // Parse the response
    let search_response = parse_scryfall_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&search_response)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    if params.pretty {
        display_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

fn display_pretty_results(response: &Response, params: &Params) -> Result<()> {
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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL with query parameters
    let url = "https://api.scryfall.com/cards/search".to_string();
    let mut query_params = vec![
        ("q", params.query.clone()),
        ("page", params.page.to_string()),
        ("order", params.order.clone()),
        ("dir", params.dir.clone()),
        ("unique", params.unique.clone()),
    ];

    if params.include_extras {
        query_params.push(("include_extras", "true".to_string()));
    }
    if params.include_multilingual {
        query_params.push(("include_multilingual", "true".to_string()));
    }
    if params.include_variations {
        query_params.push(("include_variations", "true".to_string()));
    }

    // Generate cache key
    let cache_key = CacheManager::hash_request(&(&url, &query_params));

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let response: Response = serde_json::from_value(cached_response.data)?;
        return Ok(response);
    }

    // Build the full URL with query parameters
    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let full_url = format!("{url}?{query_string}");

    let response = client.get(&full_url).send().await?;

    let response_text = response.text().await?;

    // Parse the response
    let search_response = parse_scryfall_response_with_query(&response_text, &params.query)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&search_response)?)
        .await?;

    Ok(search_response)
}

fn parse_scryfall_response_with_query(response_text: &str, query: &str) -> Result<Response> {
    match parse_scryfall_response(response_text) {
        Ok(response) => Ok(response),
        Err(e) => {
            let enhanced_error = enhance_error_message(&e.to_string(), query);
            Err(eyre!("{}", enhanced_error))
        }
    }
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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL for named card lookup
    let url = if let Some(set) = set_code {
        format!(
            "https://api.scryfall.com/cards/named?exact={}&set={}",
            urlencoding::encode(name),
            urlencoding::encode(set)
        )
    } else {
        format!(
            "https://api.scryfall.com/cards/named?exact={}",
            urlencoding::encode(name)
        )
    };

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card: {name}");
        if let Some(set) = set_code {
            println!("In set: {set}");
        }
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = if let Some(language) = lang {
        format!(
            "https://api.scryfall.com/cards/{}/{}/{}",
            urlencoding::encode(set_code),
            urlencoding::encode(collector_number),
            urlencoding::encode(language)
        )
    } else {
        format!(
            "https://api.scryfall.com/cards/{}/{}",
            urlencoding::encode(set_code),
            urlencoding::encode(collector_number)
        )
    };

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by collector: {set_code} #{collector_number}",);
        if let Some(language) = lang {
            println!("Language: {language}");
        }
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_arena_id(arena_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/arena/{arena_id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by Arena ID: {arena_id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_id(id: &str, pretty: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/{id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by ID: {id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_mtgo_id(mtgo_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/mtgo/{mtgo_id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by MTGO ID: {mtgo_id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/cardmarket/{cardmarket_id}",);

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by Cardmarket ID: {cardmarket_id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

pub async fn by_tcgplayer_id(tcgplayer_id: u32, pretty: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/tcgplayer/{tcgplayer_id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by TCGPlayer ID: {tcgplayer_id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

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
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/multiverse/{multiverse_id}",);

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    if global.verbose {
        println!("Looking up card by Multiverse ID: {multiverse_id}");
        println!("Cache key: {cache_key}");
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card: Card = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {url}");
    }

    let response = client.get(&url).send().await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    if global.verbose {
        println!("Response cached");
    }

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
    let cache_manager = CacheManager::new()?;
    let global = crate::Global {
        api_base_url: "https://api.magicthegathering.io/v1".to_string(),
        timeout: 30,
        verbose: false,
        scryfall_base_url: "https://api.scryfall.com".to_string(),
        scryfall_user_agent: None,
        scryfall_rate_limit_ms: 100,
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    let url = format!("https://api.scryfall.com/cards/arena/{arena_id}");

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        let card: Card = serde_json::from_value(cached_response.data)?;
        return Ok(card);
    }

    let response = client.get(&url).send().await?;
    let response_text = response.text().await?;

    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;

    // Cache the successful response
    cache_manager
        .set(&cache_key, serde_json::to_value(&card)?)
        .await?;

    Ok(card)
}

pub async fn commanders(
    identity: Option<String>,
    mana_value: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    let mut query_parts = vec![
        "t:legendary".to_string(),
        "t:creature".to_string(),
        "is:commander".to_string(),
    ];

    if let Some(id) = identity {
        query_parts.push(format_color_identity_query(&id));
    }

    if let Some(mv) = mana_value {
        query_parts.push(format!("mv{}", format_comparison(&mv)));
    }

    let query = query_parts.join(" ");

    if global.verbose {
        aeprintln!("Generated commander search query: {query}");
    }

    run(
        Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
}

pub async fn planeswalkers(
    color: Option<String>,
    loyalty: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    let mut query_parts = vec!["t:planeswalker".to_string()];

    if let Some(c) = color {
        query_parts.push(format_color_query(&c));
    }

    if let Some(l) = loyalty {
        query_parts.push(format!("loy{}", format_comparison(&l)));
    }

    if let Some(f) = format {
        query_parts.push(format!("f:{f}"));
    }

    let query = query_parts.join(" ");

    if global.verbose {
        aeprintln!("Generated planeswalker search query: {query}");
    }

    run(
        Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
}

pub async fn sorceries(
    color: Option<String>,
    mana_value: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    let mut query_parts = vec!["t:sorcery".to_string()];

    if let Some(c) = color {
        query_parts.push(format_color_query(&c));
    }

    if let Some(mv) = mana_value {
        query_parts.push(format!("mv{}", format_comparison(&mv)));
    }

    if let Some(f) = format {
        query_parts.push(format!("f:{f}"));
    }

    let query = query_parts.join(" ");

    if global.verbose {
        aeprintln!("Generated sorcery search query: {query}");
    }

    run(
        Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
}

pub async fn instants(
    color: Option<String>,
    mana_value: Option<String>,
    format: Option<String>,
    pretty: bool,
    global: crate::Global,
) -> Result<()> {
    let mut query_parts = vec!["t:instant".to_string()];

    if let Some(c) = color {
        query_parts.push(format_color_query(&c));
    }

    if let Some(mv) = mana_value {
        query_parts.push(format!("mv{}", format_comparison(&mv)));
    }

    if let Some(f) = format {
        query_parts.push(format!("f:{f}"));
    }

    let query = query_parts.join(" ");

    if global.verbose {
        aeprintln!("Generated instant search query: {query}");
    }

    run(
        Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
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
    let mut query_parts = vec!["t:creature".to_string()];

    if let Some(c) = color {
        query_parts.push(format_color_query(&c));
    }

    if let Some(p) = power {
        query_parts.push(format!("pow{}", format_comparison(&p)));
    }

    if let Some(t) = toughness {
        query_parts.push(format!("tou{}", format_comparison(&t)));
    }

    if let Some(mv) = mana_value {
        query_parts.push(format!("mv{}", format_comparison(&mv)));
    }

    if let Some(f) = format {
        query_parts.push(format!("f:{f}"));
    }

    let query = query_parts.join(" ");

    if global.verbose {
        aeprintln!("Generated creature search query: {query}");
    }

    run(
        Params {
            query,
            pretty,
            page: 1,
            order: "name".to_string(),
            dir: "auto".to_string(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            unique: "cards".to_string(),
            csv: false,
        },
        global,
    )
    .await
}
