use crate::prelude::*;

#[derive(Debug, clap::Parser)]
#[command(name = "cards")]
#[command(about = "Search and retrieve Magic: The Gathering cards")]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// List cards with optional filtering
    #[clap(name = "list")]
    List(ListOptions),

    /// Get a specific card by ID or multiverseid
    #[clap(name = "get")]
    Get(GetOptions),

    /// Search cards by name (supports partial and exact matching)
    #[clap(name = "search")]
    Search(SearchOptions),
}

#[derive(Debug, clap::Args, Clone)]
pub struct ListOptions {
    /// Card name (partial match)
    #[clap(long)]
    name: Option<String>,

    /// Card layout (normal, split, flip, double-faced, etc.)
    #[clap(long)]
    layout: Option<String>,

    /// Converted mana cost
    #[clap(long)]
    cmc: Option<u32>,

    /// Card colors (comma-separated for AND, pipe-separated for OR)
    #[clap(long)]
    colors: Option<String>,

    /// Card color identity
    #[clap(long)]
    color_identity: Option<String>,

    /// Card type
    #[clap(long)]
    card_type: Option<String>,

    /// Card supertypes
    #[clap(long)]
    supertypes: Option<String>,

    /// Card types
    #[clap(long)]
    types: Option<String>,

    /// Card subtypes
    #[clap(long)]
    subtypes: Option<String>,

    /// Card rarity (Common, Uncommon, Rare, Mythic Rare, etc.)
    #[clap(long)]
    rarity: Option<String>,

    /// Set code
    #[clap(long)]
    set: Option<String>,

    /// Set name
    #[clap(long)]
    set_name: Option<String>,

    /// Oracle text
    #[clap(long)]
    text: Option<String>,

    /// Flavor text
    #[clap(long)]
    flavor: Option<String>,

    /// Artist name
    #[clap(long)]
    artist: Option<String>,

    /// Card number
    #[clap(long)]
    number: Option<String>,

    /// Power (for creatures)
    #[clap(long)]
    power: Option<String>,

    /// Toughness (for creatures)
    #[clap(long)]
    toughness: Option<String>,

    /// Loyalty (for planeswalkers)
    #[clap(long)]
    loyalty: Option<String>,

    /// Language
    #[clap(long)]
    language: Option<String>,

    /// Game format
    #[clap(long)]
    game_format: Option<String>,

    /// Legality (Legal, Banned, Restricted)
    #[clap(long)]
    legality: Option<String>,

    /// Page number for pagination
    #[clap(long, default_value = "1")]
    page: u32,

    /// Number of results per page (max 100)
    #[clap(long, default_value = "20")]
    page_size: u32,

    /// Field to order results by
    #[clap(long)]
    order_by: Option<String>,

    /// Get random cards
    #[clap(long)]
    random: bool,

    /// Filter by field availability
    #[clap(long)]
    contains: Option<String>,
}

#[derive(Debug, clap::Args, Clone)]
pub struct GetOptions {
    /// Card ID or multiverseid
    id: String,
}

#[derive(Debug, clap::Args, Clone)]
pub struct SearchOptions {
    /// Card name to search for
    name: String,

    /// Use exact name matching (wrap in quotes)
    #[clap(long)]
    exact: bool,

    /// Language for foreign name search
    #[clap(long)]
    language: Option<String>,

    /// Page number for pagination
    #[clap(long, default_value = "1")]
    page: u32,

    /// Number of results per page (max 100)
    #[clap(long, default_value = "20")]
    page_size: u32,
}

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .build()?;

    if global.verbose {
        aprintln!("MTG API Base URL: {}", global.api_base_url);
        aprintln!("Request Timeout: {}s", global.timeout);
        aprintln!();
    }

    match app.command {
        Commands::List(options) => list_cards(client, &global.api_base_url, options).await,
        Commands::Get(options) => get_card(client, &global.api_base_url, options).await,
        Commands::Search(options) => search_cards(client, &global.api_base_url, options).await,
    }
}

async fn list_cards(client: reqwest::Client, base_url: &str, options: ListOptions) -> Result<()> {
    let url = f!("{}/cards", base_url);
    let mut request = client.get(&url);

    // Add all the optional parameters
    if let Some(name) = &options.name {
        request = request.query(&[("name", name)]);
    }
    if let Some(layout) = &options.layout {
        request = request.query(&[("layout", layout)]);
    }
    if let Some(cmc) = &options.cmc {
        request = request.query(&[("cmc", cmc)]);
    }
    if let Some(colors) = &options.colors {
        request = request.query(&[("colors", colors)]);
    }
    if let Some(color_identity) = &options.color_identity {
        request = request.query(&[("colorIdentity", color_identity)]);
    }
    if let Some(card_type) = &options.card_type {
        request = request.query(&[("type", card_type)]);
    }
    if let Some(supertypes) = &options.supertypes {
        request = request.query(&[("supertypes", supertypes)]);
    }
    if let Some(types) = &options.types {
        request = request.query(&[("types", types)]);
    }
    if let Some(subtypes) = &options.subtypes {
        request = request.query(&[("subtypes", subtypes)]);
    }
    if let Some(rarity) = &options.rarity {
        request = request.query(&[("rarity", rarity)]);
    }
    if let Some(set) = &options.set {
        request = request.query(&[("set", set)]);
    }
    if let Some(set_name) = &options.set_name {
        request = request.query(&[("setName", set_name)]);
    }
    if let Some(text) = &options.text {
        request = request.query(&[("text", text)]);
    }
    if let Some(flavor) = &options.flavor {
        request = request.query(&[("flavor", flavor)]);
    }
    if let Some(artist) = &options.artist {
        request = request.query(&[("artist", artist)]);
    }
    if let Some(number) = &options.number {
        request = request.query(&[("number", number)]);
    }
    if let Some(power) = &options.power {
        request = request.query(&[("power", power)]);
    }
    if let Some(toughness) = &options.toughness {
        request = request.query(&[("toughness", toughness)]);
    }
    if let Some(loyalty) = &options.loyalty {
        request = request.query(&[("loyalty", loyalty)]);
    }
    if let Some(language) = &options.language {
        request = request.query(&[("language", language)]);
    }
    if let Some(game_format) = &options.game_format {
        request = request.query(&[("gameFormat", game_format)]);
    }
    if let Some(legality) = &options.legality {
        request = request.query(&[("legality", legality)]);
    }
    if let Some(order_by) = &options.order_by {
        request = request.query(&[("orderBy", order_by)]);
    }
    if let Some(contains) = &options.contains {
        request = request.query(&[("contains", contains)]);
    }

    request = request.query(&[("page", &options.page.to_string())]);
    request = request.query(&[("pageSize", &options.page_size.to_string())]);

    if options.random {
        request = request.query(&[("random", "true")]);
    }

    let response = request.send().await?;

    // Check rate limiting headers
    if let Some(remaining) = response.headers().get("Ratelimit-Remaining") {
        if let Ok(remaining_str) = remaining.to_str() {
            if let Ok(remaining_count) = remaining_str.parse::<u32>() {
                if remaining_count < 100 {
                    aprintln!("Warning: Only {} API requests remaining", remaining_count);
                }
            }
        }
    }

    let json: serde_json::Value = response.json().await?;

    if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        let mut table = new_table();
        table.set_titles(prettytable::row!["Name", "Set", "Type", "Rarity", "CMC"]);

        for card in cards {
            let name = card.get("name").and_then(|n| n.as_str()).unwrap_or("N/A");
            let set = card.get("set").and_then(|s| s.as_str()).unwrap_or("N/A");
            let card_type = card.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
            let rarity = card.get("rarity").and_then(|r| r.as_str()).unwrap_or("N/A");
            let cmc = card
                .get("cmc")
                .and_then(|c| c.as_u64())
                .map(|c| c.to_string())
                .unwrap_or("N/A".to_string());

            table.add_row(prettytable::row![name, set, card_type, rarity, cmc]);
        }

        aprintln!("{}", table.to_string());
        aprintln!("Showing {} cards", cards.len());
    } else {
        aprintln!("No cards found");
    }

    Ok(())
}

async fn get_card(client: reqwest::Client, base_url: &str, options: GetOptions) -> Result<()> {
    let url = f!("{}/cards/{}", base_url, options.id);
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(card) = json.get("card") {
        display_card_details(card);
    } else {
        aprintln!("Card not found");
    }

    Ok(())
}

async fn search_cards(
    client: reqwest::Client,
    base_url: &str,
    options: SearchOptions,
) -> Result<()> {
    let search_name = if options.exact {
        f!("\"{}\"", options.name)
    } else {
        options.name.clone()
    };

    let url = f!("{}/cards", base_url);
    let mut request = client
        .get(&url)
        .query(&[("name", &search_name)])
        .query(&[("page", &options.page.to_string())])
        .query(&[("pageSize", &options.page_size.to_string())]);

    if let Some(language) = &options.language {
        request = request.query(&[("language", language)]);
    }

    let response = request.send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        let mut table = new_table();
        table.set_titles(prettytable::row![
            "Name",
            "Set",
            "Type",
            "Rarity",
            "Mana Cost"
        ]);

        for card in cards {
            let name = card.get("name").and_then(|n| n.as_str()).unwrap_or("N/A");
            let set = card.get("set").and_then(|s| s.as_str()).unwrap_or("N/A");
            let card_type = card.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
            let rarity = card.get("rarity").and_then(|r| r.as_str()).unwrap_or("N/A");
            let mana_cost = card
                .get("manaCost")
                .and_then(|m| m.as_str())
                .unwrap_or("N/A");

            table.add_row(prettytable::row![name, set, card_type, rarity, mana_cost]);
        }

        aprintln!("{}", table.to_string());
        aprintln!("Found {} cards matching '{}'", cards.len(), options.name);
    } else {
        aprintln!("No cards found matching '{}'", options.name);
    }

    Ok(())
}

fn display_card_details(card: &serde_json::Value) {
    aprintln!("=== Card Details ===");

    if let Some(name) = card.get("name").and_then(|n| n.as_str()) {
        aprintln!("Name: {}", name);
    }

    if let Some(mana_cost) = card.get("manaCost").and_then(|m| m.as_str()) {
        aprintln!("Mana Cost: {}", mana_cost);
    }

    if let Some(cmc) = card.get("cmc").and_then(|c| c.as_u64()) {
        aprintln!("CMC: {}", cmc);
    }

    if let Some(card_type) = card.get("type").and_then(|t| t.as_str()) {
        aprintln!("Type: {}", card_type);
    }

    if let Some(rarity) = card.get("rarity").and_then(|r| r.as_str()) {
        aprintln!("Rarity: {}", rarity);
    }

    if let Some(set) = card.get("set").and_then(|s| s.as_str()) {
        aprintln!("Set: {}", set);
    }

    if let Some(text) = card.get("text").and_then(|t| t.as_str()) {
        aprintln!("Text: {}", text);
    }

    if let Some(power) = card.get("power").and_then(|p| p.as_str()) {
        if let Some(toughness) = card.get("toughness").and_then(|t| t.as_str()) {
            aprintln!("Power/Toughness: {}/{}", power, toughness);
        }
    }

    if let Some(loyalty) = card.get("loyalty").and_then(|l| l.as_str()) {
        aprintln!("Loyalty: {}", loyalty);
    }

    if let Some(artist) = card.get("artist").and_then(|a| a.as_str()) {
        aprintln!("Artist: {}", artist);
    }
}
