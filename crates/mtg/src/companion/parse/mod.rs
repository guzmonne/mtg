use crate::cache::CacheManager;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

mod find;
mod print;
mod utils;

pub(crate) use utils::{find_newest_log_file, get_default_log_path};

pub struct Params {
    pub file: String,
    pub analyze: Option<String>,
    pub pretty: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct ArenaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    inventory_info: Option<InventoryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deck_summaries_v2: Option<Vec<DeckSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decks: Option<HashMap<String, Deck>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_messages: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_cosmetics: Option<PreferredCosmetics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_page_achievements: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deck_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timewalk_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_definitions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kill_switch_notification: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_metadata_info: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_periodic_rewards: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct InventoryInfo {
    seq_id: Option<u32>,
    changes: Option<Vec<Value>>,
    gems: Option<u32>,
    gold: Option<u32>,
    total_vault_progress: Option<u32>,
    wc_track_position: Option<u32>,
    wild_card_commons: Option<u32>,
    wild_card_un_commons: Option<u32>,
    wild_card_rares: Option<u32>,
    wild_card_mythics: Option<u32>,
    custom_tokens: Option<HashMap<String, u32>>,
    boosters: Option<Vec<Value>>,
    vouchers: Option<HashMap<String, Value>>,
    prize_walls_unlocked: Option<Vec<Value>>,
    cosmetics: Option<Cosmetics>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Cosmetics {
    art_styles: Option<Vec<Value>>,
    avatars: Option<Vec<Value>>,
    pets: Option<Vec<Value>>,
    sleeves: Option<Vec<Value>>,
    emotes: Option<Vec<Value>>,
    titles: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct DeckSummary {
    deck_id: String,
    name: String,
    attributes: Option<Vec<DeckAttribute>>,
    description: Option<String>,
    deck_tile_id: Option<u32>,
    deck_art_id: Option<u32>,
    format_legalities: Option<HashMap<String, bool>>,
    preferred_cosmetics: Option<PreferredCosmetics>,
    deck_validation_summaries: Option<Vec<Value>>,
    unowned_cards: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DeckAttribute {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct PreferredCosmetics {
    avatar: Option<Value>,
    sleeve: Option<Value>,
    pet: Option<String>,
    title: Option<String>,
    emotes: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Deck {
    main_deck: Vec<CardEntry>,
    reduced_sideboard: Option<Vec<CardEntry>>,
    sideboard: Option<Vec<CardEntry>>,
    command_zone: Option<Vec<CardEntry>>,
    companions: Option<Vec<CardEntry>>,
    card_skins: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CardEntry {
    card_id: u32,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CombinedDeckInfo {
    id: String,
    name: String,
    format: String,
    main_deck_count: usize,
    sideboard_count: usize,
    deck_content: Deck,
    format_legalities: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
struct InventoryEntry {
    data: ArenaData,
    timestamp: SystemTime,
    line_number: usize,
}

fn get_all_log_files_sorted(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut log_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
            log_files.push(path);
        }
    }

    // Sort by modification time, newest first
    log_files.sort_by(|a, b| {
        let a_time = a
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let b_time = b
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        b_time.cmp(&a_time)
    });

    if log_files.is_empty() {
        return Err(eyre!("No log files found in directory"));
    }

    Ok(log_files)
}

fn extract_timestamp_from_line(line: &str) -> SystemTime {
    // Try to extract timestamp from log line format
    // MTG Arena logs typically have timestamps at the beginning
    // Format is usually something like: [UnityCrossThreadLogger]2024-01-15 14:30:25.123
    if let Some(bracket_end) = line.find(']') {
        let after_bracket = &line[bracket_end + 1..];
        if let Some(space_pos) = after_bracket.find(' ') {
            let date_part = &after_bracket[..space_pos];
            let time_part = &after_bracket[space_pos + 1..];

            // Try to parse the timestamp
            if let Some(dot_pos) = time_part.find('.') {
                let time_without_ms = &time_part[..dot_pos];
                let datetime_str = format!("{} {}", date_part, time_without_ms);

                // Try to parse as a standard datetime format
                if let Ok(parsed_time) =
                    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                {
                    let utc_time = parsed_time.and_utc().timestamp();
                    return SystemTime::UNIX_EPOCH
                        + std::time::Duration::from_secs(utc_time as u64);
                }
            }
        }
    }

    // Fallback to current time if parsing fails
    SystemTime::now()
}

async fn find_inventory_in_logs() -> Result<(ArenaData, PathBuf)> {
    let log_dir = get_default_log_path()?;
    let log_files = get_all_log_files_sorted(&log_dir)?;

    aeprintln!(
        "Searching for inventory data across {} log files...",
        log_files.len()
    );

    for (index, log_file) in log_files.iter().enumerate() {
        aeprintln!(
            "Checking file {} of {}: {:?}",
            index + 1,
            log_files.len(),
            log_file.file_name().unwrap()
        );

        let file = fs::File::open(log_file)?;
        let reader = BufReader::new(file);
        let mut last_arena_data: Option<ArenaData> = None;

        for line in reader.lines() {
            let line = line?;
            if let Some(data) = find::parse_inventory_info_json_from_line(&line) {
                // Only update if this data has inventory info
                if data.inventory_info.is_some() {
                    last_arena_data = Some(data);
                }
            }
        }

        if let Some(data) = last_arena_data {
            if data.inventory_info.is_some() {
                aeprintln!(
                    "✅ Found inventory data in: {:?}",
                    log_file.file_name().unwrap()
                );
                return Ok((data, log_file.clone()));
            }
        }
    }

    Err(eyre!("No inventory data found in any log files"))
}

fn get_format_abbreviation(format: &str) -> &'static str {
    match format {
        // Standard formats
        "Standard" => "STD",
        "TraditionalStandard" => "TST",
        "Standard_EarlyAccess" => "SEA",
        "Traditional_Standard_EarlyAccess" => "TSE",
        "StandardArtisan" => "STA",
        "StandardPauper" => "STP",
        "StandardImmortalSun" => "SIS",
        "StandardTreasures" => "STR",
        "StandardGiantMonsters" => "SGM",
        "StandardCascade" => "SCA",
        "StandardStaircase" => "SSC",
        "100CardStandard" => "S100",

        // Alchemy formats
        "Alchemy" => "ALC",
        "TraditionalAlchemy" => "TAL",
        "FutureAlchemy" => "FAL",
        "AlchemyArtisan" => "ALA",

        // Historic formats
        "Historic" => "HIS",
        "TraditionalHistoric" => "THI",
        "HistoricBrawl" => "HBR",
        "HistoricArtisan" => "HAR",
        "HistoricPauper" => "HPA",
        "HistoricSingleton" => "HSI",
        "HistoricSingleton100" => "H100",
        "HistoricShakeup" => "HS1",
        "HistoricShakeup2" => "HS2",
        "HistoricShakeup3" => "HS3",
        "Historic_WithoutStandard" => "HWS",
        "RebalancedHistoric" => "RHI",
        "RetroHistoric" => "RTH",
        "PreHistoricTEST" => "PHT",
        "HistoricNBL" => "HNB",
        "HistoricCascadeAndChimil" => "HCC",
        "HistoricAprilFoolLimited" => "HAF",
        "TreasuredHistoricSingleton" => "THS",
        "HistoricSingletonNoBans" => "HSN",

        // Explorer formats
        "Explorer" => "EXP",
        "TraditionalExplorer" => "TEX",

        // Timeless formats
        "Timeless" => "TIM",
        "TraditionalTimeless" => "TTI",

        // Brawl formats
        "Brawl" => "BRW",
        "DirectGameBrawl" => "DBR",
        "DirectGameBrawlRebalanced" => "DBB",
        "ArtisanBrawl" => "ABR",
        "CascadeBrawl" => "CBR",
        "STXBrawl" => "SBR",
        "LTRBrawl" => "LBR",

        // Limited formats
        "Draft" => "DRF",
        "Sealed" => "SEA",
        "Draft_Rebalanced" => "DRR",
        "Sealed_Rebalanced" => "SER",

        // Direct Game formats
        "DirectGame" => "DIG",
        "DirectGameAlchemy" => "DGA",
        "DirectGameLimited" => "DGL",
        "DirectGameLimitedRebalanced" => "DGR",

        // Set-specific formats
        "GRN" => "GRN",
        "RNA" => "RNA",
        "WAR" => "WAR",
        "M20" => "M20",
        "ELD" => "ELD",
        "THB" => "THB",
        "IKO" => "IKO",
        "M21" => "M21",
        "ZNR" => "ZNR",
        "KHM" => "KHM",
        "STX" => "STX",
        "AFR" => "AFR",
        "MID" => "MID",
        "VOW" => "VOW",
        "NEO" => "NEO",
        "SNC" => "SNC",
        "DMU" => "DMU",
        "BRO" => "BRO",
        "ONE" => "ONE",
        "MOM" => "MOM",
        "WOE" => "WOE",
        "LCI" => "LCI",
        "MKM" => "MKM",
        "OTJ" => "OTJ",
        "BLB" => "BLB",
        "DSK" => "DSK",
        "FDN" => "FDN",
        "FIN" => "FIN",
        "MH3" => "MH3",
        "LTR" => "LTR",
        "KTK" => "KTK",

        // Other formats
        "Singleton" => "SIN",
        "Pauper" => "PAU",
        "Cascade" => "CAS",
        "CascadeSingleton" => "CSI",
        "Pandemonium" => "PAN",
        "GiantMonsters" => "GIM",
        "Gladiator" => "GLA",
        "TreasureSingleton" => "TRS",
        "Tripleton" => "TRI",
        "AllZeroes" => "AZR",
        "IdentityTest" => "IDT",
        "TCConstructed" => "TCC",
        "TCSingleton" => "TCS",
        "Renewal" => "REN",
        "Renewal_ZNRTest" => "RZT",
        "ArtisanHistoric_Achievement" => "AHA",
        "ArtisanFuture" => "AFU",
        "ArtisanStandard" => "AST",
        "PlaneswalkerParty" => "PWP",
        "HarvestBash" => "HVB",
        "MoreBans" => "MBN",
        "LastCall" => "LCA",
        "3Sets" => "3ST",
        "Standard_Recent" => "SRC",

        // Quest/Event formats
        "QE_RuleBreakers_NoIndvQuota" => "QRB",
        "QE_IndvQuotaPri_RarityQuotas_UBorUR" => "QIQ",
        "QE_2Mythic_1Sideboard_3Yarok" => "QMY",
        "QE_HeavyWeight" => "QHW",
        "QE_2ArtifactCommanders" => "QAC",
        "QE_RedCreaturesOnly_NoMythics" => "QRC",

        // Historic Brawl with allowlist
        "HistoricBrawlWithAllowList" => "HBA",
        _ if format.starts_with("HistoricBrawlWithAllowList_") => "HBA",

        // Default to UNK for unknown formats
        _ => "UNK",
    }
}

pub async fn run(params: Params) -> Result<()> {
    // Special handling for deck analysis
    if params.analyze == Some("decks".to_string())
        && (params.file.is_empty() || params.file == "latest")
    {
        match find::deck_info_in_logs().await {
            Ok((data, log_file, line_number)) => {
                println!("\n🎮 MTG Arena Deck Information\n");
                println!(
                    "Source: {:?} (line {})\n",
                    log_file.file_name().unwrap(),
                    line_number
                );

                if let Some(decks) = &data.decks {
                    if let Some(summaries) = &data.deck_summaries_v2 {
                        if params.pretty {
                            print::combined_decks(summaries, decks);
                        } else {
                            println!("{}", serde_json::to_string_pretty(decks)?);
                        }
                    } else {
                        // We have decks but no summaries
                        if params.pretty {
                            println!(
                                "Found {} deck definitions but no deck summaries",
                                decks.len()
                            );
                            for (id, deck) in decks {
                                println!("\nDeck ID: {}", id);
                                println!("Main deck cards: {}", deck.main_deck.len());
                                if let Some(sideboard) = &deck.sideboard {
                                    println!("Sideboard cards: {}", sideboard.len());
                                }
                            }
                        } else {
                            println!("{}", serde_json::to_string_pretty(decks)?);
                        }
                    }
                } else {
                    aprintln!("No deck data found");
                }
                return Ok(());
            }
            Err(_) => {
                println!("\n❌ No deck information found in any log files");
                println!("\n💡 To get deck information:");
                println!("   1. Play a few games in MTG Arena");
                println!("   2. Visit your deck collection in the game");
                println!("   3. Consider enabling advanced logging from the settings menu");
                println!("   4. Try running this command again after some gameplay");
                return Ok(());
            }
        }
    }

    // Special handling for inventory analysis
    if params.analyze == Some("inventory".to_string())
        && (params.file.is_empty() || params.file == "latest")
    {
        match find_inventory_in_logs().await {
            Ok((data, log_file)) => {
                println!("\n🎮 MTG Arena Inventory Data\n");
                println!("Source: {:?}\n", log_file.file_name().unwrap());

                if let Some(inventory) = &data.inventory_info {
                    if params.pretty {
                        print::inventory(inventory);
                    } else {
                        println!("{}", serde_json::to_string_pretty(inventory)?);
                    }
                } else {
                    aprintln!("No inventory data found");
                }
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    let file_path = if params.file.is_empty() || params.file == "latest" {
        // Find the newest log file
        let log_dir = get_default_log_path()?;
        let newest = find_newest_log_file(&log_dir)?;
        aeprintln!("Using newest log file: {:?}", newest.file_name().unwrap());
        newest
    } else {
        PathBuf::from(&params.file)
    };

    if !file_path.exists() {
        return Err(eyre!("File not found: {:?}", file_path));
    }

    // Open file and read line by line (to handle large files)
    let file = fs::File::open(&file_path)?;
    let reader = BufReader::new(file);

    let mut last_arena_data: Option<ArenaData> = None;
    let mut line_count = 0;

    aeprintln!("Scanning log file for Arena data...");

    for line in reader.lines() {
        line_count += 1;

        if line_count % 10000 == 0 {
            aeprintln!("Processed {} lines...", line_count);
        }

        let line = line?;

        // Try to parse JSON from this line
        if let Some(data) = find::parse_inventory_info_json_from_line(&line) {
            last_arena_data = Some(data);
        }
    }

    aeprintln!("Finished scanning {} lines", line_count);

    if let Some(data) = last_arena_data {
        // Only show pretty output if no analyze option is specified or if pretty flag is set
        if params.analyze.is_none() || params.pretty {
            println!("\n🎮 MTG Arena Data Summary\n");

            if let Some(inventory) = &data.inventory_info {
                print::inventory(inventory);
            }

            let mut combined_decks = Vec::new();
            if let Some(summaries) = &data.deck_summaries_v2 {
                if let Some(decks) = &data.decks {
                    combined_decks = print::combined_decks(summaries, decks);
                } else {
                    // If we only have summaries but no deck contents
                    aeprintln!("Note: Deck summaries found but no deck contents available");
                }
            }

            // Cache the combined deck information
            if !combined_decks.is_empty() {
                match CacheManager::new() {
                    Ok(cache) => {
                        let cache_data = serde_json::json!({
                            "timestamp": std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                            "source_file": file_path.to_string_lossy(),
                            "deck_count": combined_decks.len(),
                            "decks": combined_decks
                        });

                        let hash = "arena_decks_combined";
                        match cache.set(hash, cache_data).await {
                            Ok(_) => aprintln!("\n✅ Cached combined deck information"),
                            Err(e) => aprintln!("\n⚠️  Failed to cache deck information: {}", e),
                        }
                    }
                    Err(e) => aprintln!("\n⚠️  Failed to initialize cache: {}", e),
                }
            }

            // Print other available data indicators
            println!("\n=== Other Data Available ===");
            if data.preferred_cosmetics.is_some() {
                println!("✓ Preferred Cosmetics");
            }
            if data.home_page_achievements.is_some() {
                println!("✓ Home Page Achievements");
            }
            if data.token_definitions.is_some() {
                println!("✓ Token Definitions");
            }
            if data.client_periodic_rewards.is_some() {
                println!("✓ Client Periodic Rewards");
            }
        }

        // If analysis type was specified, provide more detailed output
        if let Some(analysis_type) = params.analyze {
            match analysis_type.as_str() {
                "inventory" => {
                    if let Some(inv) = &data.inventory_info {
                        if params.pretty {
                            // Already printed above in pretty format
                        } else {
                            println!("\n=== Detailed Inventory JSON ===");
                            println!("{}", serde_json::to_string_pretty(inv)?);
                        }
                    } else {
                        aprintln!("No inventory data found in this file");
                    }
                }
                "decks" => {
                    if let Some(decks) = &data.decks {
                        if params.pretty {
                            // Decks are already shown in pretty format above
                        } else {
                            println!("\n=== Detailed Decks JSON ===");
                            println!("{}", serde_json::to_string_pretty(decks)?);
                        }
                    } else {
                        aprintln!("No deck data found in this file");
                    }
                }
                "full" => {
                    if params.pretty {
                        // All data already shown in pretty format above
                    } else {
                        println!("\n=== Full Arena Data JSON ===");
                        println!("{}", serde_json::to_string_pretty(&data)?);
                    }
                }
                _ => {
                    aprintln!(
                        "Unknown analysis type: {}. Available: inventory, decks, full",
                        analysis_type
                    );
                }
            }
        }
    } else {
        aprintln!("No Arena data found in the log file");
    }

    Ok(())
}
