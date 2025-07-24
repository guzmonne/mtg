use crate::prelude::*;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use super::{
    extract_timestamp_from_line, get_all_log_files_sorted, get_default_log_path, ArenaData,
    InventoryEntry,
};

pub async fn deck_info_in_logs() -> Result<(ArenaData, PathBuf, usize)> {
    let log_dir = get_default_log_path()?;
    let log_files = get_all_log_files_sorted(&log_dir)?;

    aeprintln!(
        "🔍 Searching for deck information across {} log files...",
        log_files.len()
    );

    for (file_index, log_file) in log_files.iter().enumerate() {
        aeprintln!(
            "📁 Checking file {} of {}: {:?}",
            file_index + 1,
            log_files.len(),
            log_file.file_name().unwrap()
        );

        match parse_all_inventory_entries_from_file(log_file) {
            Ok(entries) => {
                aeprintln!("   Found {} inventory entries in this file", entries.len());

                // Check each inventory entry for deck information
                for (entry_index, entry) in entries.iter().enumerate() {
                    if has_deck_information(&entry.data) {
                        aeprintln!(
                            "✅ Found deck information in inventory entry {} (line {})",
                            entry_index + 1,
                            entry.line_number
                        );
                        return Ok((entry.data.clone(), log_file.clone(), entry.line_number));
                    }
                }

                aeprintln!("   No deck information found in any inventory entries");
            }
            Err(e) => {
                aeprintln!("   ⚠️  Error parsing file: {}", e);
            }
        }
    }

    Err(eyre!("No deck information found in any log files"))
}

fn parse_all_inventory_entries_from_file(file_path: &Path) -> Result<Vec<InventoryEntry>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if let Some(data) = parse_inventory_info_json_from_line(&line) {
            // Only collect entries that have inventory info
            if data.inventory_info.is_some() {
                let timestamp = extract_timestamp_from_line(&line);
                entries.push(InventoryEntry {
                    data,
                    timestamp,
                    line_number: line_number + 1,
                });
            }
        }
    }

    // Sort by timestamp, newest first
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(entries)
}

fn has_deck_information(data: &ArenaData) -> bool {
    if let Some(decks) = &data.decks {
        !decks.is_empty()
    } else {
        false
    }
}

pub fn parse_inventory_info_json_from_line(line: &str) -> Option<ArenaData> {
    // Look for JSON objects in the line
    if let Some(start) = line.find('{') {
        let json_str = &line[start..];

        // Try to parse as our expected structure
        if let Ok(data) = serde_json::from_str::<ArenaData>(json_str) {
            // Check if this JSON has any of the fields we're interested in
            if data.inventory_info.is_some()
                || data.deck_summaries_v2.is_some()
                || data.decks.is_some()
            {
                return Some(data);
            }
        }
    }
    None
}
