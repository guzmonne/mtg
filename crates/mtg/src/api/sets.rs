use crate::prelude::*;

#[derive(Debug, clap::Parser)]
#[command(name = "sets")]
#[command(about = "Browse Magic: The Gathering sets and generate booster packs")]
pub struct SetsCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// List all sets with optional filtering
    #[clap(name = "list")]
    List(ListOptions),

    /// Get a specific set by code
    #[clap(name = "get")]
    Get(GetOptions),

    /// Generate a booster pack for a specific set
    #[clap(name = "booster")]
    Booster(BoosterOptions),
}

#[derive(Debug, clap::Args, Clone)]
pub struct ListOptions {
    /// Set name (supports pipe-separated list for OR matching)
    #[clap(long)]
    name: Option<String>,

    /// Block name (supports pipe-separated list for OR matching)
    #[clap(long)]
    block: Option<String>,

    /// Page number for pagination
    #[clap(long, default_value = "1")]
    page: u32,

    /// Number of results per page (max 100)
    #[clap(long, default_value = "20")]
    page_size: u32,
}

#[derive(Debug, clap::Args, Clone)]
pub struct GetOptions {
    /// Set code (e.g., "ktk" for Khans of Tarkir)
    code: String,
}

#[derive(Debug, clap::Args, Clone)]
pub struct BoosterOptions {
    /// Set code to generate booster pack for
    code: String,
}

impl SetsCommand {
    pub async fn run(self) -> Result<()> {
        let api_base_url = std::env::var("MTG_API_BASE_URL")
            .unwrap_or_else(|_| "https://api.magicthegathering.io/v1".to_string());
        let timeout = std::env::var("MTG_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
        let verbose = std::env::var("MTG_VERBOSE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()?;

        if verbose {
            aprintln!("MTG API Base URL: {}", api_base_url);
            aprintln!("Request Timeout: {}s", timeout);
            aprintln!();
        }

        match self.command {
            Commands::List(options) => list_sets(client, &api_base_url, options).await,
            Commands::Get(options) => get_set(client, &api_base_url, options).await,
            Commands::Booster(options) => generate_booster(client, &api_base_url, options).await,
        }
    }
}

async fn list_sets(client: reqwest::Client, base_url: &str, options: ListOptions) -> Result<()> {
    let url = f!("{base_url}/sets");
    let mut request = client
        .get(&url)
        .query(&[("page", &options.page.to_string())])
        .query(&[("pageSize", &options.page_size.to_string())]);

    if let Some(name) = &options.name {
        request = request.query(&[("name", name)]);
    }
    if let Some(block) = &options.block {
        request = request.query(&[("block", block)]);
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

    if let Some(sets) = json.get("sets").and_then(|s| s.as_array()) {
        let mut table = new_table();
        table.set_titles(prettytable::row![
            "Code",
            "Name",
            "Type",
            "Block",
            "Release Date"
        ]);

        for set in sets {
            let code = set.get("code").and_then(|c| c.as_str()).unwrap_or("N/A");
            let name = set.get("name").and_then(|n| n.as_str()).unwrap_or("N/A");
            let set_type = set.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
            let block = set.get("block").and_then(|b| b.as_str()).unwrap_or("N/A");
            let release_date = set
                .get("releaseDate")
                .and_then(|r| r.as_str())
                .unwrap_or("N/A");

            table.add_row(prettytable::row![code, name, set_type, block, release_date]);
        }

        aprintln!("{}", table.to_string());
        aprintln!("Showing {} sets", sets.len());
    } else {
        aprintln!("No sets found");
    }

    Ok(())
}

async fn get_set(client: reqwest::Client, base_url: &str, options: GetOptions) -> Result<()> {
    let url = f!("{}/sets/{}", base_url, options.code);
    let response = client.get(&url).send().await?;

    if response.status() == 404 {
        aprintln!("Set '{}' not found", options.code);
        return Ok(());
    }

    let json: serde_json::Value = response.json().await?;

    if let Some(set) = json.get("set") {
        display_set_details(set);
    } else {
        aprintln!("Set not found");
    }

    Ok(())
}

async fn generate_booster(
    client: reqwest::Client,
    base_url: &str,
    options: BoosterOptions,
) -> Result<()> {
    let url = f!("{}/sets/{}/booster", base_url, options.code);
    let response = client.get(&url).send().await?;

    if response.status() == 404 {
        aprintln!(
            "Set '{}' not found or booster generation not available",
            options.code
        );
        return Ok(());
    }

    let json: serde_json::Value = response.json().await?;

    if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
        aprintln!("=== Booster Pack for {} ===", options.code.to_uppercase());
        aprintln!();

        let mut table = new_table();
        table.set_titles(prettytable::row!["Name", "Type", "Rarity", "Mana Cost"]);

        // Group cards by rarity for better display
        let mut rares = Vec::new();
        let mut uncommons = Vec::new();
        let mut commons = Vec::new();
        let mut others = Vec::new();

        for card in cards {
            let rarity = card
                .get("rarity")
                .and_then(|r| r.as_str())
                .unwrap_or("Unknown");
            match rarity {
                "Mythic Rare" | "Rare" => rares.push(card),
                "Uncommon" => uncommons.push(card),
                "Common" => commons.push(card),
                _ => others.push(card),
            }
        }

        // Display in order: Mythic/Rare, Uncommon, Common, Others
        for card_group in [&rares, &uncommons, &commons, &others] {
            for card in card_group {
                let name = card.get("name").and_then(|n| n.as_str()).unwrap_or("N/A");
                let card_type = card.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
                let rarity = card.get("rarity").and_then(|r| r.as_str()).unwrap_or("N/A");
                let mana_cost = card
                    .get("manaCost")
                    .and_then(|m| m.as_str())
                    .unwrap_or("N/A");

                table.add_row(prettytable::row![name, card_type, rarity, mana_cost]);
            }
        }

        aprintln!("{}", table.to_string());
        aprintln!("Generated booster pack with {} cards", cards.len());

        // Show rarity breakdown
        if !rares.is_empty() || !uncommons.is_empty() || !commons.is_empty() {
            aprintln!();
            aprintln!("Rarity breakdown:");
            if !rares.is_empty() {
                aprintln!("  Rare/Mythic: {}", rares.len());
            }
            if !uncommons.is_empty() {
                aprintln!("  Uncommon: {}", uncommons.len());
            }
            if !commons.is_empty() {
                aprintln!("  Common: {}", commons.len());
            }
            if !others.is_empty() {
                aprintln!("  Other: {}", others.len());
            }
        }
    } else {
        aprintln!("Failed to generate booster pack");
    }

    Ok(())
}

fn display_set_details(set: &serde_json::Value) {
    aprintln!("=== Set Details ===");

    if let Some(name) = set.get("name").and_then(|n| n.as_str()) {
        aprintln!("Name: {}", name);
    }

    if let Some(code) = set.get("code").and_then(|c| c.as_str()) {
        aprintln!("Code: {}", code);
    }

    if let Some(set_type) = set.get("type").and_then(|t| t.as_str()) {
        aprintln!("Type: {}", set_type);
    }

    if let Some(block) = set.get("block").and_then(|b| b.as_str()) {
        aprintln!("Block: {}", block);
    }

    if let Some(release_date) = set.get("releaseDate").and_then(|r| r.as_str()) {
        aprintln!("Release Date: {}", release_date);
    }

    if let Some(border) = set.get("border").and_then(|b| b.as_str()) {
        aprintln!("Border: {}", border);
    }

    if let Some(gatherer_code) = set.get("gathererCode").and_then(|g| g.as_str()) {
        aprintln!("Gatherer Code: {}", gatherer_code);
    }

    if let Some(magic_cards_info_code) = set.get("magicCardsInfoCode").and_then(|m| m.as_str()) {
        aprintln!("MagicCards.info Code: {}", magic_cards_info_code);
    }

    if let Some(online_only) = set.get("onlineOnly").and_then(|o| o.as_bool()) {
        if online_only {
            aprintln!("Online Only: Yes");
        }
    }

    if let Some(booster) = set.get("booster").and_then(|b| b.as_array()) {
        aprintln!("Booster Contents:");
        for (i, slot) in booster.iter().enumerate() {
            if let Some(slot_str) = slot.as_str() {
                aprintln!("  Slot {}: {}", i + 1, slot_str);
            } else if let Some(slot_array) = slot.as_array() {
                let slot_options: Vec<String> = slot_array
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                aprintln!("  Slot {}: {}", i + 1, slot_options.join(" or "));
            }
        }
    }
}
