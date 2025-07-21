use crate::cache::CacheManager;
use crate::prelude::*;
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, clap::Parser)]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Search for Magic cards using Gatherer advanced search
    Search {
        /// Card name to search for
        #[clap(long, short)]
        name: Option<String>,

        /// Rules text to search for
        #[clap(long)]
        rules: Option<String>,

        /// Card type (e.g., "Creature", "Instant", "Creature,Enchantment" for OR, "Creature+Legendary" for AND)
        #[clap(long, short = 't')]
        card_type: Option<String>,

        /// Card subtype (e.g., "Human", "Wizard", "Human,Wizard" for OR, "Human+Soldier" for AND)
        #[clap(long, short = 's')]
        subtype: Option<String>,

        /// Card supertype (e.g., "Legendary", "Snow", "Legendary,Snow" for OR)
        #[clap(long)]
        supertype: Option<String>,

        /// Mana cost (e.g., "{2}{U}", "1W(B/G)(W/P)")
        #[clap(long, short = 'm')]
        mana_cost: Option<String>,

        /// Set name (e.g., "Magic: The Gathering—FINAL FANTASY")
        #[clap(long)]
        set: Option<String>,

        /// Rarity (Common, Uncommon, Rare, Mythic)
        #[clap(long, short)]
        rarity: Option<String>,

        /// Artist name
        #[clap(long, short)]
        artist: Option<String>,

        /// Power value or range (e.g., "5", "5-10")
        #[clap(long, short)]
        power: Option<String>,

        /// Toughness value or range (e.g., "2", "2-5")
        #[clap(long)]
        toughness: Option<String>,

        /// Loyalty value or range (e.g., "3", "3-6")
        #[clap(long)]
        loyalty: Option<String>,

        /// Flavor text to search for
        #[clap(long)]
        flavor: Option<String>,

        /// Colors (e.g., "W", "U", "B", "R", "G", "!RBW" for not these colors)
        #[clap(long, short)]
        colors: Option<String>,

        /// Format legality (e.g., "Legal:Standard", "Banned:Modern", "Legal:Standard,Banned:Modern")
        #[clap(long, short = 'f')]
        format: Option<String>,

        /// Language (e.g., "English", "Japanese", "French", "German", "Spanish", "Italian")
        #[clap(long, short = 'l')]
        language: Option<String>,

        /// Display results in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,

        /// Page number for pagination (default: 1)
        #[clap(long, default_value = "1")]
        page: u32,
    },
    /// Get a specific Magic card by name
    Card {
        /// Card name to fetch
        name: String,

        /// Display result in a formatted table instead of JSON
        #[clap(long)]
        pretty: bool,
    },
}

#[derive(Debug, Serialize)]
struct SearchRequest {
    #[serde(rename = "searchTerm")]
    search_term: String,
    #[serde(rename = "cardName")]
    card_name: String,
    rules: String,
    #[serde(rename = "instanceSuperType")]
    instance_super_type: String,
    #[serde(rename = "instanceType")]
    instance_type: String,
    #[serde(rename = "instanceSubtype")]
    instance_subtype: String,
    colors: String,
    #[serde(rename = "commanderColor")]
    commander_color: String,
    #[serde(rename = "manaCost")]
    mana_cost: String,
    #[serde(rename = "formatLegalities")]
    format_legalities: String,
    #[serde(rename = "setName")]
    set_name: String,
    #[serde(rename = "rarityName")]
    rarity_name: String,
    #[serde(rename = "artistName")]
    artist_name: String,
    power: String,
    toughness: String,
    loyalty: String,
    #[serde(rename = "flavorText")]
    flavor_text: String,
    language: String,
    #[serde(rename = "cardPrints")]
    card_prints: String,
    #[serde(rename = "extraCards")]
    extra_cards: String,
    page: String,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            search_term: "$undefined".to_string(),
            card_name: "$undefined".to_string(),
            rules: "$undefined".to_string(),
            instance_super_type: "$undefined".to_string(),
            instance_type: "$undefined".to_string(),
            instance_subtype: "$undefined".to_string(),
            colors: "$undefined".to_string(),
            commander_color: "$undefined".to_string(),
            mana_cost: "$undefined".to_string(),
            format_legalities: "$undefined".to_string(),
            set_name: "$undefined".to_string(),
            rarity_name: "$undefined".to_string(),
            artist_name: "$undefined".to_string(),
            power: "$undefined".to_string(),
            toughness: "$undefined".to_string(),
            loyalty: "$undefined".to_string(),
            flavor_text: "$undefined".to_string(),
            language: "eq~English~en-us".to_string(),
            card_prints: "$undefined".to_string(),
            extra_cards: "$undefined".to_string(),
            page: "$undefined".to_string(),
        }
    }
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    match app.command {
        SubCommands::Search {
            name,
            rules,
            card_type,
            subtype,
            supertype,
            mana_cost,
            set,
            rarity,
            artist,
            power,
            toughness,
            loyalty,
            flavor,
            colors,
            format,
            language,
            pretty,
            page,
        } => {
            search_cards(
                SearchParams {
                    name,
                    rules,
                    card_type,
                    subtype,
                    supertype,
                    mana_cost,
                    set,
                    rarity,
                    artist,
                    power,
                    toughness,
                    loyalty,
                    flavor,
                    colors,
                    format,
                    language,
                    pretty,
                    page,
                },
                global,
            )
            .await
        }
        SubCommands::Card { name, pretty } => get_card(&name, pretty, global).await,
    }
}

pub struct SearchParams {
    pub name: Option<String>,
    pub rules: Option<String>,
    pub card_type: Option<String>,
    pub subtype: Option<String>,
    pub supertype: Option<String>,
    pub mana_cost: Option<String>,
    pub set: Option<String>,
    pub rarity: Option<String>,
    pub artist: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub flavor: Option<String>,
    pub colors: Option<String>,
    pub format: Option<String>,
    pub language: Option<String>,
    pub pretty: bool,
    pub page: u32,
}

fn parse_server_action_response(response: &str) -> Result<Option<Value>> {
    // Next.js server action responses have format:
    // 0:{"a":"$@1","f":"","b":"..."}
    // 1:{"apiVersion":"1","method":"CardData.search","data":{...}}

    for line in response.lines() {
        if let Some(colon_pos) = line.find(':') {
            let json_part = &line[colon_pos + 1..];

            // Try to parse this line as JSON
            if let Ok(parsed) = serde_json::from_str::<Value>(json_part) {
                // Look for the card data in the response
                if let Some(data) = parsed.get("data") {
                    return Ok(Some(data.clone()));
                }
                // If this line contains card data directly, return it
                if parsed.get("apiVersion").is_some() || parsed.get("kind").is_some() {
                    return Ok(Some(parsed));
                }
            }
        }
    }

    // If no structured data found, try parsing the entire response as JSON
    match serde_json::from_str::<Value>(response) {
        Ok(json) => Ok(Some(json)),
        Err(_) => Ok(None),
    }
}

fn decode_html_entities(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
        .replace("<i>", "") // Remove italic tags for terminal display
        .replace("</i>", "")
        .replace("<b>", "") // Remove bold tags
        .replace("</b>", "")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
}

fn display_pretty_results(data: &Value, params: &SearchParams) -> Result<()> {
    let total_pages = data.get("totalPages").and_then(|v| v.as_u64()).unwrap_or(1);
    let current_page = data.get("pageIndex").and_then(|v| v.as_u64()).unwrap_or(1);
    let total_items = data.get("totalItems").and_then(|v| v.as_u64()).unwrap_or(0);
    let current_items = data
        .get("currentItemCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // Create table with clean format (space-separated columns)
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Type"),
        Cell::new("Cost"),
        Cell::new("Set"),
        Cell::new("Rarity"),
        Cell::new("P/T/L"),
    ]));

    if let Some(items) = data.get("items").and_then(|v| v.as_array()) {
        for item in items {
            let name = item
                .get("instanceName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let type_line = item
                .get("instanceTypeLine")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let mana_cost = item
                .get("instanceManaText")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let set_name = item
                .get("setName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let rarity = item
                .get("rarityName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            // Handle power/toughness/loyalty
            let power = item.get("oraclePower").and_then(|v| v.as_str());
            let toughness = item.get("oracleToughness").and_then(|v| v.as_str());
            let loyalty = item.get("calculatedLoyalty").and_then(|v| v.as_u64());

            let pt_loyalty = if let Some(loyalty_val) = loyalty {
                loyalty_val.to_string()
            } else if let (Some(p), Some(t)) = (power, toughness) {
                format!("{}/{}", p, t)
            } else {
                "-".to_string()
            };

            table.add_row(Row::new(vec![
                Cell::new(name),
                Cell::new(type_line),
                Cell::new(mana_cost),
                Cell::new(set_name),
                Cell::new(rarity),
                Cell::new(&pt_loyalty),
            ]));
        }
    }

    table.printstd();

    // Display pagination summary to stderr
    eprintln!();
    eprintln!(
        "Found {} cards (showing {} on page {} of {})",
        total_items, current_items, current_page, total_pages
    );

    // Show pagination commands if needed
    if total_pages > 1 {
        eprintln!();
        eprintln!("Pagination commands:");

        // Build base command from current search parameters
        let mut base_cmd = "mtg gatherer search".to_string();

        if let Some(name) = &params.name {
            base_cmd.push_str(&format!(" --name \"{}\"", name));
        }
        if let Some(rules) = &params.rules {
            base_cmd.push_str(&format!(" --rules \"{}\"", rules));
        }
        if let Some(card_type) = &params.card_type {
            base_cmd.push_str(&format!(" --card-type \"{}\"", card_type));
        }
        if let Some(subtype) = &params.subtype {
            base_cmd.push_str(&format!(" --subtype \"{}\"", subtype));
        }
        if let Some(supertype) = &params.supertype {
            base_cmd.push_str(&format!(" --supertype \"{}\"", supertype));
        }
        if let Some(mana_cost) = &params.mana_cost {
            base_cmd.push_str(&format!(" --mana-cost \"{}\"", mana_cost));
        }
        if let Some(set) = &params.set {
            base_cmd.push_str(&format!(" --set \"{}\"", set));
        }
        if let Some(rarity) = &params.rarity {
            base_cmd.push_str(&format!(" --rarity \"{}\"", rarity));
        }
        if let Some(artist) = &params.artist {
            base_cmd.push_str(&format!(" --artist \"{}\"", artist));
        }
        if let Some(power) = &params.power {
            base_cmd.push_str(&format!(" --power \"{}\"", power));
        }
        if let Some(toughness) = &params.toughness {
            base_cmd.push_str(&format!(" --toughness \"{}\"", toughness));
        }
        if let Some(loyalty) = &params.loyalty {
            base_cmd.push_str(&format!(" --loyalty \"{}\"", loyalty));
        }
        if let Some(flavor) = &params.flavor {
            base_cmd.push_str(&format!(" --flavor \"{}\"", flavor));
        }
        if let Some(colors) = &params.colors {
            base_cmd.push_str(&format!(" --colors \"{}\"", colors));
        }
        if let Some(format) = &params.format {
            base_cmd.push_str(&format!(" --format \"{}\"", format));
        }
        if let Some(language) = &params.language {
            base_cmd.push_str(&format!(" --language \"{}\"", language));
        }
        if params.pretty {
            base_cmd.push_str(" --pretty");
        }

        if current_page > 1 {
            eprintln!("Previous page: {} --page {}", base_cmd, current_page - 1);
        }
        if current_page < total_pages {
            eprintln!("Next page: {} --page {}", base_cmd, current_page + 1);
        }
        eprintln!("Jump to page: {} --page <PAGE_NUMBER>", base_cmd);
    }

    Ok(())
}

fn card_name_to_url_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace(
            [
                ',', '\'', ':', '!', '?', '(', ')', '[', ']', '{', '}', '/', '\\', '"',
            ],
            "",
        )
        .replace('&', "and")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

fn format_query_with_operators(query: &str) -> String {
    // Handle comma-separated values as OR operations
    // Handle + separated values as AND operations
    if query.contains(',') || query.contains('+') {
        let parts: Vec<&str> = if query.contains(',') {
            query.split(',').collect()
        } else {
            query.split('+').collect()
        };

        let operator = if query.contains(',') { "~OR~" } else { "~AND~" };

        parts
            .iter()
            .map(|part| format!("eq~{}", part.trim()))
            .collect::<Vec<String>>()
            .join(&format!(",{},", operator))
    } else {
        format!("eq~{}", query)
    }
}

fn display_single_card_pretty(html: &str) -> Result<()> {
    // Parse HTML to extract card information
    // This is a simplified parser - in a real implementation you might want to use a proper HTML parser
    let mut table = Table::new();

    // Extract card name
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html[start..].find("</title>") {
            let title = &html[start + 7..start + end];
            if let Some(card_name) = title.split(" - ").next() {
                table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(card_name)]));
            }
        }
    }

    // Extract mana cost
    if let Some(start) = html.find("Mana Cost:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(cost_start) = line.find("</td><td>") {
                if let Some(cost_end) = line[cost_start + 9..].find("</td>") {
                    let mana_cost = &line[cost_start + 9..cost_start + 9 + cost_end];
                    let clean_cost = mana_cost.replace("<img", "").replace("/>", "");
                    if !clean_cost.trim().is_empty() {
                        table.add_row(Row::new(vec![
                            Cell::new("Mana Cost"),
                            Cell::new(&clean_cost),
                        ]));
                    }
                }
            }
        }
    }

    // Extract type line
    if let Some(start) = html.find("Type:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(type_start) = line.find("</td><td>") {
                if let Some(type_end) = line[type_start + 9..].find("</td>") {
                    let type_line = &line[type_start + 9..type_start + 9 + type_end];
                    table.add_row(Row::new(vec![Cell::new("Type"), Cell::new(type_line)]));
                }
            }
        }
    }

    // Extract oracle text
    if let Some(start) = html.find("Oracle Text:") {
        if let Some(line_end) = html[start..].find("</tr>") {
            let line = &html[start..start + line_end];
            if let Some(text_start) = line.find("</td><td>") {
                if let Some(text_end) = line[text_start + 9..].find("</td>") {
                    let oracle_text = &line[text_start + 9..text_start + 9 + text_end];
                    let clean_text = oracle_text.replace("<br>", "\n").replace("<br/>", "\n");
                    table.add_row(Row::new(vec![
                        Cell::new("Oracle Text"),
                        Cell::new(&clean_text),
                    ]));
                }
            }
        }
    }

    // Extract power/toughness
    if let Some(start) = html.find("P/T:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(pt_start) = line.find("</td><td>") {
                if let Some(pt_end) = line[pt_start + 9..].find("</td>") {
                    let pt = &line[pt_start + 9..pt_start + 9 + pt_end];
                    table.add_row(Row::new(vec![Cell::new("P/T"), Cell::new(pt)]));
                }
            }
        }
    }

    // Extract loyalty
    if let Some(start) = html.find("Loyalty:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(loyalty_start) = line.find("</td><td>") {
                if let Some(loyalty_end) = line[loyalty_start + 9..].find("</td>") {
                    let loyalty = &line[loyalty_start + 9..loyalty_start + 9 + loyalty_end];
                    table.add_row(Row::new(vec![Cell::new("Loyalty"), Cell::new(loyalty)]));
                }
            }
        }
    }

    table.printstd();
    Ok(())
}

async fn get_card(name: &str, pretty: bool, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    // Use the existing search functionality to find the card
    let search_params = SearchParams {
        name: Some(name.to_string()),
        rules: None,
        card_type: None,
        subtype: None,
        supertype: None,
        mana_cost: None,
        set: None,
        rarity: None,
        artist: None,
        power: None,
        toughness: None,
        loyalty: None,
        flavor: None,
        colors: None,
        format: None,
        language: None, // Use default (English)
        pretty: false,  // Always get JSON first
        page: 1,
    };

    if global.verbose {
        println!("Searching for card '{}' using advanced search", name);
    }

    // Perform the search
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .build()?;

    let request = SearchRequest {
        card_name: name.to_string(),
        page: "1".to_string(),
        ..Default::default()
    };

    let payload = serde_json::json!(["$undefined", request, {}, true, 1]);

    // Generate cache key
    let url = "https://gatherer.wizards.com/advanced-search";
    let headers = vec![
        ("accept".to_string(), "text/x-component".to_string()),
        (
            "content-type".to_string(),
            "text/plain;charset=UTF-8".to_string(),
        ),
    ];
    let cache_key = CacheManager::hash_gatherer_search_request(url, &payload, &headers);

    if global.verbose {
        println!("Cache key: {}", cache_key);
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response for card '{}'", name);
        }

        let card_data = &cached_response.data;
        if let Some(items) = card_data.get("items").and_then(|v| v.as_array()) {
            if items.is_empty() {
                return Err(
                    crate::error::Error::Generic(format!("Card '{}' not found", name)).into(),
                );
            }

            // Get the first matching card (exact name match preferred)
            let mut best_match = &items[0];

            // Look for exact name match
            for item in items {
                if let Some(card_name) = item.get("instanceName").and_then(|v| v.as_str()) {
                    if card_name.eq_ignore_ascii_case(name) {
                        best_match = item;
                        break;
                    }
                }
            }

            if pretty {
                display_single_card_details(best_match)?;
            } else {
                println!("{}", serde_json::to_string_pretty(best_match)?);
            }
        } else {
            return Err(crate::error::Error::Generic(format!("Card '{}' not found", name)).into());
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss for card '{}', fetching from API", name);
    }

    let response = client
        .post("https://gatherer.wizards.com/advanced-search")
        .header("accept", "text/x-component")
        .header("accept-language", "en-US,en;q=0.9")
        .header("cache-control", "no-cache")
        .header("content-type", "text/plain;charset=UTF-8")
        .header("dnt", "1")
        .header("next-action", "7fdc558e6830828dfb95ae6a9638513cb4727e9b52")
        .header("next-router-state-tree", "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22en%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22advanced-search%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Fadvanced-search%22%2C%22refresh%22%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D%7D%2Cnull%2Cnull%5D")
        .header("origin", "https://gatherer.wizards.com")
        .header("pragma", "no-cache")
        .header("priority", "u=1, i")
        .header("referer", "https://gatherer.wizards.com/advanced-search")
        .header("sec-ch-ua", "\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .json(&payload)
        .send()
        .await?;

    let response_text = response.text().await?;

    if global.verbose {
        println!("Search response length: {} characters", response_text.len());
    }

    // Parse the response
    let parsed_data = parse_server_action_response(&response_text)?;

    if let Some(card_data) = parsed_data {
        // Cache the successful response
        cache_manager.set(&cache_key, card_data.clone()).await?;

        if global.verbose {
            println!("Response cached for card '{}'", name);
        }

        if let Some(items) = card_data.get("items").and_then(|v| v.as_array()) {
            if items.is_empty() {
                return Err(
                    crate::error::Error::Generic(format!("Card '{}' not found", name)).into(),
                );
            }

            // Get the first matching card (exact name match preferred)
            let mut best_match = &items[0];

            // Look for exact name match
            for item in items {
                if let Some(card_name) = item.get("instanceName").and_then(|v| v.as_str()) {
                    if card_name.eq_ignore_ascii_case(name) {
                        best_match = item;
                        break;
                    }
                }
            }

            if pretty {
                display_single_card_details(best_match)?;
            } else {
                println!("{}", serde_json::to_string_pretty(best_match)?);
            }
        } else {
            return Err(crate::error::Error::Generic(format!("Card '{}' not found", name)).into());
        }
    } else {
        return Err(crate::error::Error::Generic(format!("Card '{}' not found", name)).into());
    }

    Ok(())
}

fn display_single_card_details(card: &serde_json::Value) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    // Card name
    if let Some(name) = card.get("instanceName").and_then(|v| v.as_str()) {
        table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(name)]));
    }

    // Mana cost
    if let Some(mana_cost) = card.get("instanceManaText").and_then(|v| v.as_str()) {
        if !mana_cost.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Mana Cost"), Cell::new(mana_cost)]));
        }
    }

    // Type line
    if let Some(type_line) = card.get("instanceTypeLine").and_then(|v| v.as_str()) {
        table.add_row(Row::new(vec![Cell::new("Type"), Cell::new(type_line)]));
    }

    // Oracle text
    if let Some(oracle_text) = card.get("instanceText").and_then(|v| v.as_str()) {
        if !oracle_text.is_empty() {
            let decoded_text = decode_html_entities(oracle_text);
            table.add_row(Row::new(vec![
                Cell::new("Oracle Text"),
                Cell::new(&decoded_text),
            ]));
        }
    }

    // Power/Toughness
    let power = card.get("oraclePower").and_then(|v| v.as_str());
    let toughness = card.get("oracleToughness").and_then(|v| v.as_str());
    if let (Some(p), Some(t)) = (power, toughness) {
        table.add_row(Row::new(vec![
            Cell::new("Power/Toughness"),
            Cell::new(&format!("{}/{}", p, t)),
        ]));
    }

    // Loyalty
    if let Some(loyalty) = card.get("calculatedLoyalty").and_then(|v| v.as_u64()) {
        table.add_row(Row::new(vec![
            Cell::new("Loyalty"),
            Cell::new(&loyalty.to_string()),
        ]));
    }

    // Set
    if let Some(set_name) = card.get("setName").and_then(|v| v.as_str()) {
        table.add_row(Row::new(vec![Cell::new("Set"), Cell::new(set_name)]));
    }

    // Rarity
    if let Some(rarity) = card.get("rarityName").and_then(|v| v.as_str()) {
        table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new(rarity)]));
    }

    // Artist
    if let Some(artist) = card.get("artistName").and_then(|v| v.as_str()) {
        table.add_row(Row::new(vec![Cell::new("Artist"), Cell::new(artist)]));
    }

    // Flavor text
    if let Some(flavor_text) = card.get("flavorText").and_then(|v| v.as_str()) {
        if !flavor_text.is_empty() {
            let decoded_text = decode_html_entities(flavor_text);
            table.add_row(Row::new(vec![
                Cell::new("Flavor Text"),
                Cell::new(&decoded_text),
            ]));
        }
    }

    table.printstd();
    Ok(())
}

fn extract_card_info_json(html: &str, url: &str) -> serde_json::Value {
    let mut card_info = serde_json::Map::new();

    // Extract card name
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html[start..].find("</title>") {
            let title = &html[start + 7..start + end];
            if let Some(card_name) = title.split(" - ").next() {
                card_info.insert(
                    "name".to_string(),
                    serde_json::Value::String(card_name.to_string()),
                );
            }
        }
    }

    // Extract mana cost
    if let Some(start) = html.find("Mana Cost:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(cost_start) = line.find("</td><td>") {
                if let Some(cost_end) = line[cost_start + 9..].find("</td>") {
                    let mana_cost = &line[cost_start + 9..cost_start + 9 + cost_end];
                    let clean_cost = mana_cost
                        .replace("<img", "")
                        .replace("/>", "")
                        .trim()
                        .to_string();
                    if !clean_cost.is_empty() {
                        card_info.insert(
                            "mana_cost".to_string(),
                            serde_json::Value::String(clean_cost),
                        );
                    }
                }
            }
        }
    }

    // Extract type line
    if let Some(start) = html.find("Type:") {
        if let Some(line_end) = html[start..].find('\n') {
            let line = &html[start..start + line_end];
            if let Some(type_start) = line.find("</td><td>") {
                if let Some(type_end) = line[type_start + 9..].find("</td>") {
                    let type_line = &line[type_start + 9..type_start + 9 + type_end];
                    card_info.insert(
                        "type_line".to_string(),
                        serde_json::Value::String(type_line.to_string()),
                    );
                }
            }
        }
    }

    // Extract oracle text
    if let Some(start) = html.find("Oracle Text:") {
        if let Some(line_end) = html[start..].find("</tr>") {
            let line = &html[start..start + line_end];
            if let Some(text_start) = line.find("</td><td>") {
                if let Some(text_end) = line[text_start + 9..].find("</td>") {
                    let oracle_text = &line[text_start + 9..text_start + 9 + text_end];
                    let clean_text = oracle_text.replace("<br>", "\n").replace("<br/>", "\n");
                    card_info.insert(
                        "oracle_text".to_string(),
                        serde_json::Value::String(clean_text),
                    );
                }
            }
        }
    }

    if !url.is_empty() {
        card_info.insert(
            "gatherer_url".to_string(),
            serde_json::Value::String(url.to_string()),
        );
    }

    serde_json::Value::Object(card_info)
}
pub async fn search_cards_json(
    params: SearchParams,
    global: crate::Global,
) -> Result<serde_json::Value> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .build()?;

    let mut request = SearchRequest::default();

    // Map CLI parameters to request fields (same logic as search_cards)
    if let Some(ref name) = params.name {
        request.card_name = name.clone();
    }
    if let Some(ref rules) = params.rules {
        request.rules = rules.clone();
    }
    if let Some(ref supertype) = params.supertype {
        request.instance_super_type = format_query_with_operators(supertype);
    }
    if let Some(ref card_type) = params.card_type {
        request.instance_type = format_query_with_operators(card_type);
    }
    if let Some(ref subtype) = params.subtype {
        request.instance_subtype = format_query_with_operators(subtype);
    }
    if let Some(ref mana_cost) = params.mana_cost {
        request.mana_cost = mana_cost.replace(" ", "_");
    }
    if let Some(ref set) = params.set {
        let escaped_set = set.replace(" ", "_").replace(":", "_");
        request.set_name = format!(
            "eq~{}~{}",
            escaped_set,
            set.chars().take(3).collect::<String>().to_uppercase()
        );
    }
    if let Some(ref rarity) = params.rarity {
        let rarity_code = match rarity.to_lowercase().as_str() {
            "common" => "C",
            "uncommon" => "U",
            "rare" => "R",
            "mythic" | "mythic rare" => "M",
            _ => rarity,
        };
        request.rarity_name = format!("eq~{}~{}", rarity, rarity_code);
    }
    if let Some(ref artist) = params.artist {
        request.artist_name = artist.clone();
    }
    if let Some(ref power) = params.power {
        request.power = power.replace("-", "_");
    }
    if let Some(ref toughness) = params.toughness {
        request.toughness = toughness.replace("-", "_");
    }
    if let Some(ref loyalty) = params.loyalty {
        request.loyalty = loyalty.replace("-", "_");
    }
    if let Some(ref flavor) = params.flavor {
        request.flavor_text = flavor.clone();
    }
    if let Some(ref colors) = params.colors {
        if colors.starts_with("!") || colors.starts_with("not ") {
            let clean_colors = colors
                .trim_start_matches('!')
                .trim_start_matches("not ")
                .trim();
            request.colors = format!("neq~{}", clean_colors.replace(",", "_"));
        } else {
            request.colors = colors.replace(",", "_");
        }
    }
    if let Some(ref format) = params.format {
        request.format_legalities = format.clone();
    }
    if let Some(ref language) = params.language {
        let lang_code = match language.to_lowercase().as_str() {
            "english" => "en-us",
            "japanese" => "ja-jp",
            "french" => "fr-fr",
            "german" => "de-de",
            "spanish" => "es-es",
            "italian" => "it-it",
            "portuguese" => "pt-br",
            "russian" => "ru-ru",
            "korean" => "ko-kr",
            "chinese simplified" | "simplified chinese" => "zh-cn",
            "chinese traditional" | "traditional chinese" => "zh-tw",
            _ => language,
        };
        request.language = format!("eq~{}~{}", language, lang_code);
    }

    request.page = params.page.to_string();

    let payload = serde_json::json!(["$undefined", request, {}, true, params.page]);

    let url = "https://gatherer.wizards.com/advanced-search";
    let headers = vec![
        ("accept".to_string(), "text/x-component".to_string()),
        (
            "content-type".to_string(),
            "text/plain;charset=UTF-8".to_string(),
        ),
    ];
    let cache_key = CacheManager::hash_gatherer_search_request(url, &payload, &headers);

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        return Ok(cached_response.data);
    }

    let response = client
        .post("https://gatherer.wizards.com/advanced-search")
        .header("accept", "text/x-component")
        .header("accept-language", "en-US,en;q=0.9")
        .header("cache-control", "no-cache")
        .header("content-type", "text/plain;charset=UTF-8")
        .header("dnt", "1")
        .header("next-action", "7fdc558e6830828dfb95ae6a9638513cb4727e9b52")
        .header("next-router-state-tree", "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22en%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22advanced-search%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Fadvanced-search%22%2C%22refresh%22%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D%7D%2Cnull%2Cnull%5D")
        .header("origin", "https://gatherer.wizards.com")
        .header("pragma", "no-cache")
        .header("priority", "u=1, i")
        .header("referer", "https://gatherer.wizards.com/advanced-search")
        .header("sec-ch-ua", "\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .json(&payload)
        .send()
        .await?;

    let response_text = response.text().await?;
    let parsed_data = parse_server_action_response(&response_text)?;

    if let Some(card_data) = parsed_data {
        cache_manager.set(&cache_key, card_data.clone()).await?;
        Ok(card_data)
    } else {
        Err(crate::error::Error::Generic("No card data found in response".to_string()).into())
    }
}

pub async fn search_cards(params: SearchParams, global: crate::Global) -> Result<()> {
    let cache_manager = CacheManager::new()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .build()?;

    let mut request = SearchRequest::default();

    // Map CLI parameters to request fields
    if let Some(ref name) = params.name {
        request.card_name = name.clone();
    }
    if let Some(ref rules) = params.rules {
        request.rules = rules.clone();
    }
    if let Some(ref supertype) = params.supertype {
        // Handle complex queries with AND/OR operators
        request.instance_super_type = format_query_with_operators(supertype);
    }
    if let Some(ref card_type) = params.card_type {
        // Handle complex queries with AND/OR operators
        request.instance_type = format_query_with_operators(card_type);
    }
    if let Some(ref subtype) = params.subtype {
        // Handle complex queries with AND/OR operators
        request.instance_subtype = format_query_with_operators(subtype);
    }
    if let Some(ref mana_cost) = params.mana_cost {
        // Mana cost format: "1W(B/G)(W/P)" -> "1_W_(B/G)_(W/P)"
        request.mana_cost = mana_cost.replace(" ", "_");
    }
    if let Some(ref set) = params.set {
        // Format set name with proper escaping for special characters
        let escaped_set = set.replace(" ", "_").replace(":", "_");
        request.set_name = format!(
            "eq~{}~{}",
            escaped_set,
            set.chars().take(3).collect::<String>().to_uppercase()
        );
    }
    if let Some(ref rarity) = params.rarity {
        let rarity_code = match rarity.to_lowercase().as_str() {
            "common" => "C",
            "uncommon" => "U",
            "rare" => "R",
            "mythic" | "mythic rare" => "M",
            _ => rarity,
        };
        request.rarity_name = format!("eq~{}~{}", rarity, rarity_code);
    }
    if let Some(ref artist) = params.artist {
        request.artist_name = artist.clone();
    }
    if let Some(ref power) = params.power {
        // Power/toughness can be ranges like "5_10"
        request.power = power.replace("-", "_");
    }
    if let Some(ref toughness) = params.toughness {
        // Toughness can be ranges like "2_5"
        request.toughness = toughness.replace("-", "_");
    }
    if let Some(ref loyalty) = params.loyalty {
        // Loyalty can be ranges like "3_6"
        request.loyalty = loyalty.replace("-", "_");
    }
    if let Some(ref flavor) = params.flavor {
        request.flavor_text = flavor.clone();
    }
    if let Some(ref colors) = params.colors {
        // Colors can have neq~ prefix for "not equal"
        if colors.starts_with("!") || colors.starts_with("not ") {
            let clean_colors = colors
                .trim_start_matches('!')
                .trim_start_matches("not ")
                .trim();
            request.colors = format!("neq~{}", clean_colors.replace(",", "_"));
        } else {
            request.colors = colors.replace(",", "_");
        }
    }
    if let Some(ref format) = params.format {
        // Format legalities have specific format: "Legal:Alchemy,Banned:Brawl"
        request.format_legalities = format.clone();
    }
    if let Some(ref language) = params.language {
        // Language format: "eq~English~en-us" for English, "eq~Japanese~ja-jp" for Japanese, etc.
        let lang_code = match language.to_lowercase().as_str() {
            "english" => "en-us",
            "japanese" => "ja-jp",
            "french" => "fr-fr",
            "german" => "de-de",
            "spanish" => "es-es",
            "italian" => "it-it",
            "portuguese" => "pt-br",
            "russian" => "ru-ru",
            "korean" => "ko-kr",
            "chinese simplified" | "simplified chinese" => "zh-cn",
            "chinese traditional" | "traditional chinese" => "zh-tw",
            _ => language, // Use as-is if not recognized
        };
        request.language = format!("eq~{}~{}", language, lang_code);
    }
    // Note: language defaults to "eq~English~en-us" if not specified

    // Set page number
    request.page = params.page.to_string();

    // Create the request payload as expected by the API
    let payload = serde_json::json!(["$undefined", request, {}, true, params.page]);

    if global.verbose {
        println!(
            "Request payload: {}",
            serde_json::to_string_pretty(&payload)?
        );
    }

    // Generate cache key
    let url = "https://gatherer.wizards.com/advanced-search";
    let headers = vec![
        ("accept".to_string(), "text/x-component".to_string()),
        (
            "content-type".to_string(),
            "text/plain;charset=UTF-8".to_string(),
        ),
    ];
    let cache_key = CacheManager::hash_gatherer_search_request(url, &payload, &headers);

    if global.verbose {
        println!("Cache key: {}", cache_key);
    }

    // Check cache first
    if let Some(cached_response) = cache_manager.get(&cache_key).await? {
        if global.verbose {
            println!("Using cached response");
        }

        let card_data = &cached_response.data;
        if params.pretty {
            display_pretty_results(card_data, &params)?;
        } else {
            println!("{}", serde_json::to_string_pretty(card_data)?);
        }
        return Ok(());
    }

    if global.verbose {
        println!("Cache miss, fetching from API");
    }

    let response = client
        .post("https://gatherer.wizards.com/advanced-search")
        .header("accept", "text/x-component")
        .header("accept-language", "en-US,en;q=0.9")
        .header("cache-control", "no-cache")
        .header("content-type", "text/plain;charset=UTF-8")
        .header("dnt", "1")
        .header("next-action", "7fdc558e6830828dfb95ae6a9638513cb4727e9b52")
        .header("next-router-state-tree", "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22en%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22advanced-search%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Fadvanced-search%22%2C%22refresh%22%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D%7D%2Cnull%2Cnull%5D")
        .header("origin", "https://gatherer.wizards.com")
        .header("pragma", "no-cache")
        .header("priority", "u=1, i")
        .header("referer", "https://gatherer.wizards.com/advanced-search")
        .header("sec-ch-ua", "\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .json(&payload)
        .send()
        .await?;

    if global.verbose {
        println!("Response status: {}", response.status());
        println!("Response headers: {:#?}", response.headers());
    }

    let response_text = response.text().await?;

    if global.verbose {
        println!("Raw response: {}", response_text);
    }

    // Parse Next.js server action response format
    let parsed_data = parse_server_action_response(&response_text)?;

    if let Some(card_data) = parsed_data {
        // Cache the successful response
        cache_manager.set(&cache_key, card_data.clone()).await?;

        if global.verbose {
            println!("Response cached");
        }

        if params.pretty {
            display_pretty_results(&card_data, &params)?;
        } else {
            println!("{}", serde_json::to_string_pretty(&card_data)?);
        }
    } else {
        println!("No card data found in response");
    }

    Ok(())
}
