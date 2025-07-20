use crate::prelude::*;

#[derive(Debug, clap::Parser)]
#[command(name = "types")]
#[command(about = "Get card types, subtypes, supertypes, and game formats")]
pub struct TypesCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// List all card types
    #[clap(name = "list")]
    List,

    /// List all card subtypes
    #[clap(name = "subtypes")]
    Subtypes,

    /// List all card supertypes
    #[clap(name = "supertypes")]
    Supertypes,

    /// List all game formats
    #[clap(name = "formats")]
    Formats,
}

impl TypesCommand {
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
            Commands::List => list_types(client, &api_base_url).await,
            Commands::Subtypes => list_subtypes(client, &api_base_url).await,
            Commands::Supertypes => list_supertypes(client, &api_base_url).await,
            Commands::Formats => list_formats(client, &api_base_url).await,
        }
    }
}

async fn list_types(client: reqwest::Client, base_url: &str) -> Result<()> {
    let url = f!("{}/types", base_url);
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(types) = json.get("types").and_then(|t| t.as_array()) {
        aprintln!("=== Card Types ===");
        aprintln!();

        let mut table = new_table();
        table.set_titles(prettytable::row!["Type"]);

        for card_type in types {
            if let Some(type_str) = card_type.as_str() {
                table.add_row(prettytable::row![type_str]);
            }
        }

        aprintln!("{}", table.to_string());
        aprintln!("Total: {} types", types.len());
    } else {
        aprintln!("No types found");
    }

    Ok(())
}

async fn list_subtypes(client: reqwest::Client, base_url: &str) -> Result<()> {
    let url = f!("{}/subtypes", base_url);
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(subtypes) = json.get("subtypes").and_then(|s| s.as_array()) {
        aprintln!("=== Card Subtypes ===");
        aprintln!();

        // Display in columns for better readability
        let subtypes_str: Vec<&str> = subtypes.iter().filter_map(|s| s.as_str()).collect();

        display_in_columns(&subtypes_str, 4);
        aprintln!();
        aprintln!("Total: {} subtypes", subtypes.len());
    } else {
        aprintln!("No subtypes found");
    }

    Ok(())
}

async fn list_supertypes(client: reqwest::Client, base_url: &str) -> Result<()> {
    let url = f!("{}/supertypes", base_url);
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(supertypes) = json.get("supertypes").and_then(|s| s.as_array()) {
        aprintln!("=== Card Supertypes ===");
        aprintln!();

        let mut table = new_table();
        table.set_titles(prettytable::row!["Supertype"]);

        for supertype in supertypes {
            if let Some(supertype_str) = supertype.as_str() {
                table.add_row(prettytable::row![supertype_str]);
            }
        }

        aprintln!("{}", table.to_string());
        aprintln!("Total: {} supertypes", supertypes.len());
    } else {
        aprintln!("No supertypes found");
    }

    Ok(())
}

async fn list_formats(client: reqwest::Client, base_url: &str) -> Result<()> {
    let url = f!("{}/formats", base_url);
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;

    if let Some(formats) = json.get("formats").and_then(|f| f.as_array()) {
        aprintln!("=== Game Formats ===");
        aprintln!();

        // Group formats by category for better organization
        let mut constructed = Vec::new();
        let mut limited = Vec::new();
        let mut blocks = Vec::new();
        let mut other = Vec::new();

        for format in formats {
            if let Some(format_str) = format.as_str() {
                if format_str.contains("Block") {
                    blocks.push(format_str);
                } else if [
                    "Standard",
                    "Modern",
                    "Legacy",
                    "Vintage",
                    "Commander",
                    "Extended",
                ]
                .contains(&format_str)
                {
                    constructed.push(format_str);
                } else if ["Limited", "Draft", "Sealed"].contains(&format_str) {
                    limited.push(format_str);
                } else {
                    other.push(format_str);
                }
            }
        }

        if !constructed.is_empty() {
            aprintln!("Constructed Formats:");
            for format in &constructed {
                aprintln!("  • {}", format);
            }
            aprintln!();
        }

        if !limited.is_empty() {
            aprintln!("Limited Formats:");
            for format in &limited {
                aprintln!("  • {}", format);
            }
            aprintln!();
        }

        if !blocks.is_empty() {
            aprintln!("Block Formats:");
            for format in &blocks {
                aprintln!("  • {}", format);
            }
            aprintln!();
        }

        if !other.is_empty() {
            aprintln!("Other Formats:");
            for format in &other {
                aprintln!("  • {}", format);
            }
            aprintln!();
        }

        aprintln!("Total: {} formats", formats.len());
    } else {
        aprintln!("No formats found");
    }

    Ok(())
}

fn display_in_columns(items: &[&str], columns: usize) {
    let rows = items.len().div_ceil(columns);

    for row in 0..rows {
        let mut line = String::new();
        for col in 0..columns {
            let index = row + col * rows;
            if index < items.len() {
                line.push_str(&f!("{:<20}", items[index]));
            }
        }
        aprintln!("{}", line.trim_end());
    }
}
