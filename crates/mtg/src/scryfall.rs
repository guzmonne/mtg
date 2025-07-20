use crate::prelude::*;
use crate::cache::CacheManager;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use prettytable::{Table, Row, Cell, format};

#[derive(Debug, clap::Parser)]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Search for Magic cards using Scryfall advanced search
    Search {
        /// Search query using Scryfall syntax (e.g., "c:red t:creature", "Lightning Bolt")
        query: String,

        /// Display results in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Page number for pagination (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,

        /// Sort order (name, cmc, power, toughness, artist, set, released, rarity, usd, tix, eur)
        #[clap(long, default_value = "name")]
        order: String,

        /// Sort direction (auto, asc, desc)
        #[clap(long, default_value = "auto")]
        dir: String,

        /// Include extra cards (tokens, emblems, etc.)
        #[clap(long)]
        include_extras: bool,

        /// Include multilingual cards
        #[clap(long)]
        include_multilingual: bool,

        /// Include variations (different printings)
        #[clap(long)]
        include_variations: bool,

        /// Unique mode (cards, prints, art)
        #[clap(long, default_value = "cards")]
        unique: String,
    },

    /// Get a specific Magic card by exact name
    Card {
        /// Card name to fetch
        name: String,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Set code to get specific printing (optional)
        #[clap(long, short)]
        set: Option<String>,
    },

    /// Search for cards with advanced filters
    Advanced {
        /// Card name (partial matching allowed)
        #[clap(long, short)]
        name: Option<String>,

        /// Oracle text to search for
        #[clap(long, short)]
        oracle: Option<String>,

        /// Card type (e.g., "creature", "instant")
        #[clap(long, short = 't')]
        card_type: Option<String>,

        /// Colors (e.g., "w", "wu", "wubrg")
        #[clap(long, short)]
        colors: Option<String>,

        /// Color identity for commander (e.g., "w", "wu")
        #[clap(long)]
        identity: Option<String>,

        /// Mana cost (e.g., "{2}{U}", "3")
        #[clap(long, short = 'm')]
        mana: Option<String>,

        /// Mana value/CMC (e.g., "3", ">=4", "<2")
        #[clap(long)]
        mv: Option<String>,

        /// Power (e.g., "2", ">=3", "*")
        #[clap(long, short)]
        power: Option<String>,

        /// Toughness (e.g., "2", ">=3", "*")
        #[clap(long)]
        toughness: Option<String>,

        /// Loyalty (e.g., "3", ">=4")
        #[clap(long)]
        loyalty: Option<String>,

        /// Set code (e.g., "ktk", "war")
        #[clap(long, short)]
        set: Option<String>,

        /// Rarity (common, uncommon, rare, mythic)
        #[clap(long, short)]
        rarity: Option<String>,

        /// Artist name
        #[clap(long, short)]
        artist: Option<String>,

        /// Flavor text to search for
        #[clap(long)]
        flavor: Option<String>,

        /// Format legality (e.g., "standard", "modern", "legacy")
        #[clap(long, short = 'f')]
        format: Option<String>,

        /// Language (e.g., "en", "ja", "de")
        #[clap(long, short = 'l')]
        language: Option<String>,

        /// Display results in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Page number for pagination (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,

        /// Sort order
        #[clap(long, default_value = "name")]
        order: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ScryfallSearchResponse {
    object: String,
    total_cards: u32,
    has_more: bool,
    next_page: Option<String>,
    data: Vec<ScryfallCard>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScryfallErrorResponse {
    object: String,
    code: String,
    status: u16,
    details: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScryfallCard {
    object: String,
    id: String,
    oracle_id: Option<String>,
    multiverse_ids: Option<Vec<u32>>,
    mtgo_id: Option<u32>,
    arena_id: Option<u32>,
    tcgplayer_id: Option<u32>,
    cardmarket_id: Option<u32>,
    name: String,
    lang: String,
    released_at: String,
    uri: String,
    scryfall_uri: String,
    layout: String,
    highres_image: bool,
    image_status: String,
    image_uris: Option<Value>,
    mana_cost: Option<String>,
    cmc: f64,
    type_line: String,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    keywords: Option<Vec<String>>,
    legalities: Value,
    games: Vec<String>,
    reserved: bool,
    foil: bool,
    nonfoil: bool,
    finishes: Vec<String>,
    oversized: bool,
    promo: bool,
    reprint: bool,
    variation: bool,
    set_id: String,
    set: String,
    set_name: String,
    set_type: String,
    set_uri: String,
    set_search_uri: String,
    scryfall_set_uri: String,
    rulings_uri: String,
    prints_search_uri: String,
    collector_number: String,
    digital: bool,
    rarity: String,
    flavor_text: Option<String>,
    card_back_id: Option<String>,
    artist: Option<String>,
    artist_ids: Option<Vec<String>>,
    illustration_id: Option<String>,
    border_color: String,
    frame: String,
    security_stamp: Option<String>,
    full_art: bool,
    textless: bool,
    booster: bool,
    story_spotlight: bool,
    edhrec_rank: Option<u32>,
    penny_rank: Option<u32>,
    prices: Option<Value>,
    related_uris: Option<Value>,
    purchase_uris: Option<Value>,
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    match app.command {
        SubCommands::Search {
            query,
            pretty,
            page,
            order,
            dir,
            include_extras,
            include_multilingual,
            include_variations,
            unique,
        } => {
            search_cards(SearchParams {
                query,
                pretty,
                page,
                order,
                dir,
                include_extras,
                include_multilingual,
                include_variations,
                unique,
            }, global).await
        }
        SubCommands::Card { name, pretty, set } => {
            get_card(&name, pretty, set.as_deref(), global).await
        }
        SubCommands::Advanced {
            name,
            oracle,
            card_type,
            colors,
            identity,
            mana,
            mv,
            power,
            toughness,
            loyalty,
            set,
            rarity,
            artist,
            flavor,
            format,
            language,
            pretty,
            page,
            order,
        } => {
            advanced_search(AdvancedSearchParams {
                name,
                oracle,
                card_type,
                colors,
                identity,
                mana,
                mv,
                power,
                toughness,
                loyalty,
                set,
                rarity,
                artist,
                flavor,
                format,
                language,
                pretty,
                page,
                order,
            }, global).await
        }
    }
}

struct SearchParams {
    query: String,
    pretty: bool,
    page: u32,
    order: String,
    dir: String,
    include_extras: bool,
    include_multilingual: bool,
    include_variations: bool,
    unique: String,
}

fn parse_scryfall_response(response_text: &str) -> Result<ScryfallSearchResponse> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;
    
    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response
            let error_response: ScryfallErrorResponse = serde_json::from_str(response_text)?;
            return Err(crate::error::Error::Generic(format!(
                "Scryfall API error ({}): {}", 
                error_response.code, 
                error_response.details
            )).into());
        }
    }
    
    // Parse as search response
    let search_response: ScryfallSearchResponse = serde_json::from_str(response_text)?;
    Ok(search_response)
}

fn parse_scryfall_card_response(response_text: &str) -> Result<ScryfallCard> {
    // First, try to parse as a generic JSON value to check the object type
    let json_value: serde_json::Value = serde_json::from_str(response_text)?;
    
    if let Some(object_type) = json_value.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            // Parse as error response
            let error_response: ScryfallErrorResponse = serde_json::from_str(response_text)?;
            return Err(crate::error::Error::Generic(format!(
                "Scryfall API error ({}): {}", 
                error_response.code, 
                error_response.details
            )).into());
        }
    }
    
    // Parse as card response
    let card: ScryfallCard = serde_json::from_str(response_text)?;
    Ok(card)
}

struct AdvancedSearchParams {
    name: Option<String>,
    oracle: Option<String>,
    card_type: Option<String>,
    colors: Option<String>,
    identity: Option<String>,
    mana: Option<String>,
    mv: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    set: Option<String>,
    rarity: Option<String>,
    artist: Option<String>,
    flavor: Option<String>,
    format: Option<String>,
    language: Option<String>,
    pretty: bool,
    page: u32,
    order: String,
}

fn build_advanced_query(params: &AdvancedSearchParams) -> String {
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
        query_parts.push(format!("type:{}", card_type));
    }

    if let Some(colors) = &params.colors {
        query_parts.push(format!("color:{}", colors));
    }

    if let Some(identity) = &params.identity {
        query_parts.push(format!("identity:{}", identity));
    }

    if let Some(mana) = &params.mana {
        query_parts.push(format!("mana:{}", mana));
    }

    if let Some(mv) = &params.mv {
        query_parts.push(format!("manavalue:{}", mv));
    }

    if let Some(power) = &params.power {
        query_parts.push(format!("power:{}", power));
    }

    if let Some(toughness) = &params.toughness {
        query_parts.push(format!("toughness:{}", toughness));
    }

    if let Some(loyalty) = &params.loyalty {
        query_parts.push(format!("loyalty:{}", loyalty));
    }

    if let Some(set) = &params.set {
        query_parts.push(format!("set:{}", set));
    }

    if let Some(rarity) = &params.rarity {
        query_parts.push(format!("rarity:{}", rarity));
    }

    if let Some(artist) = &params.artist {
        query_parts.push(format!("artist:\"{}\"", artist.replace('"', "\\\"")));
    }

    if let Some(flavor) = &params.flavor {
        query_parts.push(format!("flavor:\"{}\"", flavor.replace('"', "\\\"")));
    }

    if let Some(format) = &params.format {
        query_parts.push(format!("format:{}", format));
    }

    if let Some(language) = &params.language {
        query_parts.push(format!("lang:{}", language));
    }

    query_parts.join(" ")
}

fn display_pretty_results(response: &ScryfallSearchResponse, params: &SearchParams) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
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
            format!("{}/{}", power, toughness)
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
    eprintln!();
    eprintln!("Found {} cards (showing {} on page {})", 
             response.total_cards, response.data.len(), params.page);

    if response.has_more {
        eprintln!();
        eprintln!("Pagination commands:");
        
        let mut base_cmd = format!("mtg scryfall search \"{}\"", params.query);
        
        if params.page > 1 {
            eprintln!("Previous page: {} --page {}", base_cmd, params.page - 1);
        }
        eprintln!("Next page: {} --page {}", base_cmd, params.page + 1);
        eprintln!("Jump to page: {} --page <PAGE_NUMBER>", base_cmd);
    }

    Ok(())
}

fn display_advanced_pretty_results(response: &ScryfallSearchResponse, params: &AdvancedSearchParams) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
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
            format!("{}/{}", power, toughness)
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
    eprintln!();
    eprintln!("Found {} cards (showing {} on page {})", 
             response.total_cards, response.data.len(), params.page);

    if response.has_more {
        eprintln!();
        eprintln!("Next page available - use --page {} to continue", params.page + 1);
    }

    Ok(())
}

fn display_single_card_details(card: &ScryfallCard) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    
    // Card name
    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(&card.name)]));
    
    // Mana cost
    if let Some(mana_cost) = &card.mana_cost {
        if !mana_cost.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Mana Cost"), Cell::new(mana_cost)]));
        }
    }
    
    // Mana value
    if card.cmc > 0.0 {
        table.add_row(Row::new(vec![Cell::new("Mana Value"), Cell::new(&card.cmc.to_string())]));
    }
    
    // Type line
    table.add_row(Row::new(vec![Cell::new("Type"), Cell::new(&card.type_line)]));
    
    // Oracle text
    if let Some(oracle_text) = &card.oracle_text {
        if !oracle_text.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Oracle Text"), Cell::new(oracle_text)]));
        }
    }
    
    // Power/Toughness
    if let (Some(power), Some(toughness)) = (&card.power, &card.toughness) {
        table.add_row(Row::new(vec![Cell::new("Power/Toughness"), Cell::new(&format!("{}/{}", power, toughness))]));
    }
    
    // Loyalty
    if let Some(loyalty) = &card.loyalty {
        table.add_row(Row::new(vec![Cell::new("Loyalty"), Cell::new(loyalty)]));
    }
    
    // Set
    table.add_row(Row::new(vec![Cell::new("Set"), Cell::new(&format!("{} ({})", card.set_name, card.set.to_uppercase()))]));
    
    // Rarity
    table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new(&card.rarity)]));
    
    // Artist
    if let Some(artist) = &card.artist {
        table.add_row(Row::new(vec![Cell::new("Artist"), Cell::new(artist)]));
    }
    
    // Flavor text
    if let Some(flavor_text) = &card.flavor_text {
        if !flavor_text.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Flavor Text"), Cell::new(flavor_text)]));
        }
    }
    
    // Collector number
    table.add_row(Row::new(vec![Cell::new("Collector Number"), Cell::new(&card.collector_number)]));
    
    // Legalities (show a few key formats)
    if let Some(legalities) = card.legalities.as_object() {
        let mut legal_formats = Vec::new();
        let key_formats = ["standard", "pioneer", "modern", "legacy", "vintage", "commander"];
        
        for format in &key_formats {
            if let Some(status) = legalities.get(*format).and_then(|v| v.as_str()) {
                if status == "legal" {
                    legal_formats.push(format.to_string());
                }
            }
        }
        
        if !legal_formats.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Legal In"), Cell::new(&legal_formats.join(", "))]));
        }
    }
    
    table.printstd();
    Ok(())
}

async fn search_cards(params: SearchParams, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL with query parameters
    let mut url = "https://api.scryfall.com/cards/search".to_string();
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
        println!("Cache key: {}", cache_key);
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }
        
        let response: ScryfallSearchResponse = serde_json::from_value(cached_response.data)?;
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
    let full_url = format!("{}?{}", url, query_string);

    if global.verbose {
        println!("Request URL: {}", full_url);
    }

    let response = client
        .get(&full_url)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;
    
    if global.verbose {
        println!("Response length: {} characters", response_text.len());
    }

    // Parse the response
    let search_response = parse_scryfall_response(&response_text)?;
    
    // Cache the successful response
    cache_manager.set(&cache_key, serde_json::to_value(&search_response)?).await?;
    
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

async fn advanced_search(params: AdvancedSearchParams, global: crate::Global) -> Result<()> {
    let query = build_advanced_query(&params);
    
    if query.is_empty() {
        return Err(crate::error::Error::Generic("No search parameters provided".to_string()).into());
    }

    if global.verbose {
        println!("Built query: {}", query);
    }

    let search_params = SearchParams {
        query,
        pretty: params.pretty,
        page: params.page,
        order: params.order.clone(),
        dir: "auto".to_string(),
        include_extras: false,
        include_multilingual: false,
        include_variations: false,
        unique: "cards".to_string(),
    };

    let cache_manager = CacheManager::new()?;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL with query parameters
    let url = "https://api.scryfall.com/cards/search";
    let query_params = vec![
        ("q", search_params.query.clone()),
        ("page", search_params.page.to_string()),
        ("order", search_params.order.clone()),
        ("unique", search_params.unique.clone()),
    ];

    // Generate cache key
    let cache_key = CacheManager::hash_request(&(&url, &query_params));
    
    if global.verbose {
        println!("Cache key: {}", cache_key);
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }
        
        let response: ScryfallSearchResponse = serde_json::from_value(cached_response.data)?;
        if params.pretty {
            display_advanced_pretty_results(&response, &params)?;
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
    let full_url = format!("{}?{}", url, query_string);

    if global.verbose {
        println!("Request URL: {}", full_url);
    }

    let response = client
        .get(&full_url)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;
    
    // Parse the response
    let search_response = parse_scryfall_response(&response_text)?;
    
    // Cache the successful response
    cache_manager.set(&cache_key, serde_json::to_value(&search_response)?).await?;
    
    if global.verbose {
        println!("Response cached");
    }
    
    if params.pretty {
        display_advanced_pretty_results(&search_response, &params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&search_response)?);
    }

    Ok(())
}

async fn get_card(name: &str, pretty: bool, set_code: Option<&str>, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("mtg-cli/1.0")
        .build()?;

    // Build URL for named card lookup
    let url = if let Some(set) = set_code {
        format!("https://api.scryfall.com/cards/named?exact={}&set={}", 
                urlencoding::encode(name), urlencoding::encode(set))
    } else {
        format!("https://api.scryfall.com/cards/named?exact={}", 
                urlencoding::encode(name))
    };

    // Generate cache key
    let cache_key = CacheManager::hash_request(&url);
    
    if global.verbose {
        println!("Looking up card: {}", name);
        if let Some(set) = set_code {
            println!("In set: {}", set);
        }
        println!("Cache key: {}", cache_key);
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }
        
        let card: ScryfallCard = serde_json::from_value(cached_response.data)?;
        if pretty {
            display_single_card_details(&card)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
        println!("Request URL: {}", url);
    }

    let response = client
        .get(&url)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
    }

    let response_text = response.text().await?;
    
    // Parse the response
    let card = parse_scryfall_card_response(&response_text)?;
    
    // Cache the successful response
    cache_manager.set(&cache_key, serde_json::to_value(&card)?).await?;
    
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