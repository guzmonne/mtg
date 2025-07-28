use crate::prelude::*;
use mtg_core::{GathererCard, GathererClient, GathererSearchParams, GathererSearchResponse};
use prettytable::{Cell, Row};

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

pub async fn run(app: App, global: crate::Global) -> Result<()> {
    let client = global.create_gatherer_client()?;

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
            let params = GathererSearchParams {
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
                page,
            };

            search_cards(&client, &params, pretty).await
        }
        SubCommands::Card { name, pretty } => get_card(&client, &name, pretty).await,
    }
}

async fn search_cards(
    client: &GathererClient,
    params: &GathererSearchParams,
    pretty: bool,
) -> Result<()> {
    let response = client.search(params).await.map_err(|e| eyre!(e))?;

    if pretty {
        display_pretty_results(&response, params)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&response)?);
    }

    Ok(())
}

async fn get_card(client: &GathererClient, name: &str, pretty: bool) -> Result<()> {
    let card = client.get_card(name).await.map_err(|e| eyre!(e))?;

    if pretty {
        display_single_card_details(&card)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&card)?);
    }

    Ok(())
}

fn display_pretty_results(
    response: &GathererSearchResponse,
    params: &GathererSearchParams,
) -> Result<()> {
    let total_pages = response.total_pages.unwrap_or(1);
    let current_page = response.page_index.unwrap_or(1);
    let total_items = response.total_items.unwrap_or(0);
    let current_items = response.current_item_count.unwrap_or(0);

    // Create table with clean format (space-separated columns)
    let mut table = new_table();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Type"),
        Cell::new("Cost"),
        Cell::new("Set"),
        Cell::new("Rarity"),
        Cell::new("P/T/L"),
    ]));

    if let Some(items) = &response.items {
        for item in items {
            let name = item.name.as_deref().unwrap_or("Unknown");
            let type_line = item.type_line.as_deref().unwrap_or("Unknown");
            let mana_cost = item.mana_cost.as_deref().unwrap_or("");
            let set_name = item.set_name.as_deref().unwrap_or("Unknown");
            let rarity = item.rarity.as_deref().unwrap_or("Unknown");

            // Handle power/toughness/loyalty
            let pt_loyalty = if let Some(loyalty_val) = item.loyalty {
                loyalty_val.to_string()
            } else if let (Some(p), Some(t)) = (&item.power, &item.toughness) {
                format!("{p}/{t}")
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
    aeprintln!();
    aeprintln!(
        "Found {total_items} cards (showing {current_items} on page {current_page} of {total_pages})"
    );

    // Show pagination commands if needed
    if total_pages > 1 {
        aeprintln!();
        aeprintln!("Pagination commands:");

        // Build base command from current search parameters
        let mut base_cmd = "mtg gatherer search".to_string();

        if let Some(name) = &params.name {
            base_cmd.push_str(&format!(" --name \"{name}\""));
        }
        if let Some(rules) = &params.rules {
            base_cmd.push_str(&format!(" --rules \"{rules}\""));
        }
        if let Some(card_type) = &params.card_type {
            base_cmd.push_str(&format!(" --card-type \"{card_type}\""));
        }
        if let Some(subtype) = &params.subtype {
            base_cmd.push_str(&format!(" --subtype \"{subtype}\""));
        }
        if let Some(supertype) = &params.supertype {
            base_cmd.push_str(&format!(" --supertype \"{supertype}\""));
        }
        if let Some(mana_cost) = &params.mana_cost {
            base_cmd.push_str(&format!(" --mana-cost \"{mana_cost}\""));
        }
        if let Some(set) = &params.set {
            base_cmd.push_str(&format!(" --set \"{set}\""));
        }
        if let Some(rarity) = &params.rarity {
            base_cmd.push_str(&format!(" --rarity \"{rarity}\""));
        }
        if let Some(artist) = &params.artist {
            base_cmd.push_str(&format!(" --artist \"{artist}\""));
        }
        if let Some(power) = &params.power {
            base_cmd.push_str(&format!(" --power \"{power}\""));
        }
        if let Some(toughness) = &params.toughness {
            base_cmd.push_str(&format!(" --toughness \"{toughness}\""));
        }
        if let Some(loyalty) = &params.loyalty {
            base_cmd.push_str(&format!(" --loyalty \"{loyalty}\""));
        }
        if let Some(flavor) = &params.flavor {
            base_cmd.push_str(&format!(" --flavor \"{flavor}\""));
        }
        if let Some(colors) = &params.colors {
            base_cmd.push_str(&format!(" --colors \"{colors}\""));
        }
        if let Some(format) = &params.format {
            base_cmd.push_str(&format!(" --format \"{format}\""));
        }
        if let Some(language) = &params.language {
            base_cmd.push_str(&format!(" --language \"{language}\""));
        }

        if current_page > 1 {
            aeprintln!("Previous page: {base_cmd} --page {}", current_page - 1);
        }
        if current_page < total_pages {
            aeprintln!("Next page: {base_cmd} --page {}", current_page + 1);
        }
        aeprintln!("Jump to page: {base_cmd} --page <PAGE_NUMBER>");
    }

    Ok(())
}

fn display_single_card_details(card: &GathererCard) -> Result<()> {
    let mut table = new_table();

    // Card name
    if let Some(name) = &card.name {
        table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(name)]));
    }

    // Mana cost
    if let Some(mana_cost) = &card.mana_cost {
        if !mana_cost.is_empty() {
            table.add_row(Row::new(vec![Cell::new("Mana Cost"), Cell::new(mana_cost)]));
        }
    }

    // Type line
    if let Some(type_line) = &card.type_line {
        table.add_row(Row::new(vec![Cell::new("Type"), Cell::new(type_line)]));
    }

    // Oracle text
    if let Some(oracle_text) = &card.oracle_text {
        if !oracle_text.is_empty() {
            let decoded_text = decode_html_entities(oracle_text);
            table.add_row(Row::new(vec![
                Cell::new("Oracle Text"),
                Cell::new(&decoded_text),
            ]));
        }
    }

    // Power/Toughness
    if let (Some(p), Some(t)) = (&card.power, &card.toughness) {
        table.add_row(Row::new(vec![
            Cell::new("Power/Toughness"),
            Cell::new(&format!("{p}/{t}")),
        ]));
    }

    // Loyalty
    if let Some(loyalty) = card.loyalty {
        table.add_row(Row::new(vec![
            Cell::new("Loyalty"),
            Cell::new(&loyalty.to_string()),
        ]));
    }

    // Set
    if let Some(set_name) = &card.set_name {
        table.add_row(Row::new(vec![Cell::new("Set"), Cell::new(set_name)]));
    }

    // Rarity
    if let Some(rarity) = &card.rarity {
        table.add_row(Row::new(vec![Cell::new("Rarity"), Cell::new(rarity)]));
    }

    // Artist
    if let Some(artist) = &card.artist {
        table.add_row(Row::new(vec![Cell::new("Artist"), Cell::new(artist)]));
    }

    // Flavor text
    if let Some(flavor_text) = &card.flavor_text {
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
