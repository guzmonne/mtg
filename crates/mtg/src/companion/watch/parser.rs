#![allow(dead_code)]

use super::types::*;
use crate::cache::CacheManager;
use crate::prelude::*;
use chrono::Utc;
use prettytable::{Cell, Row};
use serde_json::Value;
use std::collections::HashMap;

fn to_camel_case(snake_case: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in snake_case.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

// Calculate the probability of drawing at least one copy of a card in the opening hand
fn calculate_draw_probability(copies: u64, deck_size: u64, hand_size: u64) -> f64 {
    // Using hypergeometric distribution
    // P(at least 1) = 1 - P(exactly 0)
    // P(exactly 0) = C(deck_size - copies, hand_size) / C(deck_size, hand_size)

    if copies == 0 || copies > deck_size || hand_size > deck_size {
        return 0.0;
    }

    // Calculate using the formula: 1 - ((deck_size - copies) / deck_size) * ... * ((deck_size - copies - hand_size + 1) / (deck_size - hand_size + 1))
    let mut prob_none = 1.0;
    for i in 0..hand_size {
        prob_none *= (deck_size - copies - i) as f64 / (deck_size - i) as f64;
    }

    1.0 - prob_none
}

pub struct EventParser {
    current_match: Option<MatchState>,
    current_draft: Option<DraftState>,
    current_user: Option<(String, String)>, // (user_id, display_name)
    last_game_state: Option<Value>,         // Store the last game state for comparison
    game_objects: std::collections::HashMap<u32, GameObjectInfo>, // Track game objects by instance ID
}

#[derive(Debug, Clone)]
struct GameObjectInfo {
    grp_id: u32,
    owner: u32,
    zone: String,
    card_name: Option<String>,
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            current_match: None,
            current_draft: None,
            current_user: None,
            last_game_state: None,
            game_objects: std::collections::HashMap::new(),
        }
    }

    pub fn parse_event(
        &mut self,
        event: RawLogEvent,
        async_tx: tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<Option<ParsedEvent>> {
        match event.event_name.as_str() {
            // Business events (main event type in logs)
            "LogBusinessEvents" => self.handle_business_event(&event.raw_data),

            // Authentication events
            "Authenticate" => self.handle_authenticate_event(&event.raw_data),

            // Match events
            "GreToClientEvent" => self.handle_gre_event_raw(&event.raw_data, async_tx),
            "MatchGameRoomStateChangedEvent" => self.handle_match_state_change(&event.raw_data),

            // Draft events
            "HumanDraftPack" => self.handle_draft_pack(&event.raw_data),
            "HumanDraftPick" => self.handle_draft_pick(&event.raw_data),

            // Deck events
            "Event_GetCourseDeck" | "DeckMessage" | "DeckUpsertDeckV2" => {
                self.handle_deck_submission(&event.raw_data, async_tx)
            }
            "EventGetCoursesV2" => self.handle_courses_event(&event.raw_data),

            // Quest events
            "QuestGetQuests" => self.handle_quest_event(&event.raw_data),

            // Rank events
            "RankGetCombinedRankInfo" => self.handle_rank_info(&event.raw_data),

            // State change events
            "StateChanged" => self.handle_state_change(&event.raw_data),

            // Other events - silently ignore system events
            _ => Ok(None),
        }
    }

    fn extract_json_data(&self, raw_data: &str, event_name: &str) -> Result<Option<Value>> {
        let raw_data = raw_data.trim();

        // Strategy 1: Direct JSON parsing
        if raw_data.starts_with('{') {
            if let Ok(value) = serde_json::from_str::<Value>(raw_data) {
                return Ok(Some(value));
            }
        }

        // Strategy 2: Try to parse as JSON after cleaning
        let cleaned = raw_data
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("");

        if cleaned.starts_with('{') {
            if let Ok(value) = serde_json::from_str::<Value>(&cleaned) {
                return Ok(Some(value));
            }
        }

        // Strategy 3: Look for nested JSON in common fields
        if let Ok(outer) = serde_json::from_str::<Value>(raw_data) {
            // Check for 'payload' field
            if let Some(payload) = outer.get("payload") {
                if let Some(payload_str) = payload.as_str() {
                    if let Ok(inner) = serde_json::from_str::<Value>(payload_str) {
                        return Ok(Some(inner));
                    }
                }
                return Ok(Some(payload.clone()));
            }

            // Check for 'Payload' field (capital P)
            if let Some(payload) = outer.get("Payload") {
                if let Some(payload_str) = payload.as_str() {
                    if let Ok(inner) = serde_json::from_str::<Value>(payload_str) {
                        return Ok(Some(inner));
                    }
                }
                return Ok(Some(payload.clone()));
            }

            // Check for 'request' field
            if let Some(request) = outer.get("request") {
                if let Some(request_str) = request.as_str() {
                    if let Ok(inner) = serde_json::from_str::<Value>(request_str) {
                        return Ok(Some(inner));
                    }
                }
                return Ok(Some(request.clone()));
            }

            // Strategy 4: Look for camel-case event name
            let camel_case_name = to_camel_case(event_name);
            if let Some(event_data) = outer.get(&camel_case_name) {
                return Ok(Some(event_data.clone()));
            }

            // Return the outer JSON if no nested structure found
            return Ok(Some(outer));
        }

        Ok(None)
    }

    fn handle_authenticate_event(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "Authenticate")? {
            // Look for clientId and screenName
            let client_id = data
                .get("clientId")
                .or_else(|| {
                    data.get("authenticateResponse")
                        .and_then(|r| r.get("clientId"))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let screen_name = data
                .get("screenName")
                .or_else(|| {
                    data.get("authenticateResponse")
                        .and_then(|r| r.get("screenName"))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            if let (Some(user_id), Some(display_name)) = (client_id, screen_name) {
                self.current_user = Some((user_id.clone(), display_name.clone()));

                return Ok(Some(ParsedEvent::UserAuthenticated {
                    user_id,
                    display_name,
                }));
            }
        }
        Ok(None)
    }

    fn handle_business_event(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "LogBusinessEvents")? {
            // Business events have a nested structure with "request" field
            if let Some(request) = data.get("request") {
                if let Some(request_str) = request.as_str() {
                    // Parse the nested JSON in the request field
                    if let Ok(request_data) = serde_json::from_str::<Value>(request_str) {
                        // Check if this is a match analytics event (has MatchId and stats)
                        if request_data.get("MatchId").is_some()
                            && request_data.get("EventType").is_some()
                        {
                            // Check if it's a match result (EventType 4) or match analytics (EventType 7)
                            if let Some(event_type) =
                                request_data.get("EventType").and_then(|v| v.as_u64())
                            {
                                match event_type {
                                    4 => return self.handle_match_result(&request_data),
                                    7 => return self.handle_match_analytics(&request_data),
                                    _ => {}
                                }
                            }
                        }

                        if let Some(event_name) =
                            request_data.get("EventName").and_then(|v| v.as_str())
                        {
                            // Handle specific business event types that are game-relevant
                            match event_name {
                                "ClientConnected" => {
                                    if let Some(data) = request_data.get("Data") {
                                        if let (Some(account_id), Some(persona_id)) = (
                                            data.get("AccountId").and_then(|v| v.as_str()),
                                            data.get("PersonaId").and_then(|v| v.as_str()),
                                        ) {
                                            println!("\n🔐 Client Connected:");
                                            println!("  Account ID: {}", account_id);
                                            println!("  Persona ID: {}", persona_id);
                                        }
                                    }
                                }
                                "MatchStarted" => {
                                    println!("\n🎮 Match Started!");
                                    if let Some(data) = request_data.get("Data") {
                                        if let Ok(formatted) = serde_json::to_string_pretty(&data) {
                                            println!("{}", formatted);
                                        }
                                    }
                                }
                                "MatchCompleted" => {
                                    println!("\n🏁 Match Completed!");
                                    if let Some(data) = request_data.get("Data") {
                                        if let Ok(formatted) = serde_json::to_string_pretty(&data) {
                                            println!("{}", formatted);
                                        }
                                    }
                                }
                                "DraftPick" => {
                                    println!("\n🎯 Draft Pick!");
                                    if let Some(data) = request_data.get("Data") {
                                        if let Ok(formatted) = serde_json::to_string_pretty(&data) {
                                            println!("{}", formatted);
                                        }
                                    }
                                }
                                "DraftPack" => {
                                    println!("\n📦 Draft Pack!");
                                    if let Some(data) = request_data.get("Data") {
                                        if let Ok(formatted) = serde_json::to_string_pretty(&data) {
                                            println!("{}", formatted);
                                        }
                                    }
                                }
                                // Scene transitions (EventType 33)
                                _ if request_data.get("EventType").and_then(|v| v.as_u64())
                                    == Some(33) =>
                                {
                                    if let (Some(from), Some(to)) = (
                                        request_data.get("fromSceneName").and_then(|v| v.as_str()),
                                        request_data.get("toSceneName").and_then(|v| v.as_str()),
                                    ) {
                                        println!("\n🚪 Scene Change: {} → {}", from, to);
                                        if let Some(duration) =
                                            request_data.get("duration").and_then(|v| v.as_str())
                                        {
                                            println!("  Duration: {}", duration);
                                        }
                                    }
                                }
                                // Ignore system/telemetry events silently
                                "PreparingAssetsStep"
                                | "FileCleanupCheckStart"
                                | "FileCleanupCheckEnd"
                                | "ClientPerformanceMetrics"
                                | "SystemInfo"
                                | "GraphicsInfo" => {
                                    // These are just system telemetry, ignore silently
                                }
                                _ => {
                                    // For other business events, show them if they seem interesting
                                    if !event_name.contains("Performance")
                                        && !event_name.contains("Metric")
                                    {
                                        println!("\n📊 Business Event: {}", event_name);
                                        if let Some(data) = request_data.get("Data") {
                                            if let Ok(formatted) =
                                                serde_json::to_string_pretty(&data)
                                            {
                                                println!("{}", formatted);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_match_analytics(&mut self, data: &Value) -> Result<Option<ParsedEvent>> {
        println!("\n📊 Match Analytics:");

        if let Some(match_id) = data.get("MatchId").and_then(|v| v.as_str()) {
            println!("  🆔 Match ID: {}", match_id);
        }

        // Gameplay statistics
        println!("\n  ⏱️ Priority Timing:");
        if let Some(avg_time) = data
            .get("AveragePassPriorityWaitTimeInSeconds")
            .and_then(|v| v.as_str())
        {
            println!("    Average response time: {}s", avg_time);
        }
        if let Some(longest) = data
            .get("LongestPassPriorityWaitTimeInSeconds")
            .and_then(|v| v.as_str())
        {
            println!("    Longest response time: {}", longest);
        }
        if let Some(shortest) = data
            .get("ShortestPassPriorityWaitTimeInSeconds")
            .and_then(|v| v.as_str())
        {
            println!("    Shortest response time: {}", shortest);
        }

        println!("\n  🎯 Actions:");
        if let Some(priority_count) = data.get("ReceivedPriorityCount").and_then(|v| v.as_str()) {
            println!("    Times received priority: {}", priority_count);
        }
        if let Some(passed) = data.get("PassedPriorityCount").and_then(|v| v.as_str()) {
            println!("    Times passed priority: {}", passed);
        }
        if let Some(responded) = data
            .get("RespondedToPriorityCount")
            .and_then(|v| v.as_str())
        {
            println!("    Times responded: {}", responded);
        }

        println!("\n  🃏 Spells Cast:");
        if let Some(auto_pay) = data
            .get("SpellsCastWithAutoPayCount")
            .and_then(|v| v.as_str())
        {
            println!("    With auto-pay: {}", auto_pay);
        }
        if let Some(manual) = data
            .get("SpellsCastWithManualManaCount")
            .and_then(|v| v.as_str())
        {
            println!("    With manual mana: {}", manual);
        }
        if let Some(mixed) = data
            .get("SpellsCastWithMixedPayManaCount")
            .and_then(|v| v.as_str())
        {
            println!("    With mixed mana: {}", mixed);
        }

        println!("\n  📈 Board State Peaks:");
        if let Some(creatures) = data.get("MaxCreatures").and_then(|v| v.as_str()) {
            println!("    Max creatures: {}", creatures);
        }
        if let Some(lands) = data.get("MaxLands").and_then(|v| v.as_str()) {
            println!("    Max lands: {}", lands);
        }
        if let Some(artifacts) = data
            .get("MaxArtifactsAndEnchantments")
            .and_then(|v| v.as_str())
        {
            println!("    Max artifacts/enchantments: {}", artifacts);
        }

        Ok(None)
    }

    fn handle_match_result(&mut self, data: &Value) -> Result<Option<ParsedEvent>> {
        println!("\n🏁 Match Result Details:");

        if let Some(match_id) = data.get("MatchId").and_then(|v| v.as_str()) {
            println!("  🆔 Match ID: {}", match_id);
        }

        if let Some(event_id) = data.get("EventId").and_then(|v| v.as_str()) {
            println!("  🎮 Event: {}", event_id);
        }

        // Display result
        if let (
            Some(seat_id),
            Some(team_id),
            Some(winning_team),
            Some(winning_type),
            Some(winning_reason),
        ) = (
            data.get("SeatId").and_then(|v| v.as_u64()),
            data.get("TeamId").and_then(|v| v.as_u64()),
            data.get("WinningTeamId").and_then(|v| v.as_u64()),
            data.get("WinningType").and_then(|v| v.as_str()),
            data.get("WinningReason").and_then(|v| v.as_str()),
        ) {
            println!("\n  🏆 Result:");
            if team_id == winning_team {
                println!("    Winner: Player {} (Team {})", seat_id, team_id);
            } else {
                println!("    Loser: Player {} (Team {})", seat_id, team_id);
                println!("    Winner: Team {}", winning_team);
            }

            let reason_display = match winning_reason {
                "Concede" => "Concession",
                "Damage" => "Damage",
                "Timeout" => "Timeout",
                "DeckEmpty" => "Deck Empty",
                _ => winning_reason,
            };
            println!("    Victory type: {}", reason_display);
        }

        // Game statistics
        if let (Some(turns), Some(seconds)) = (
            data.get("TurnCount").and_then(|v| v.as_u64()),
            data.get("SecondsCount").and_then(|v| v.as_u64()),
        ) {
            println!("\n  ⏱️ Game Statistics:");
            let minutes = seconds / 60;
            let secs = seconds % 60;
            println!(
                "    Duration: {}:{:02} ({} seconds)",
                minutes, secs, seconds
            );
            println!("    Total turns: {}", turns);

            if let Some(starting_team) = data.get("StartingTeamId").and_then(|v| v.as_u64()) {
                println!("    Starting player: Team {}", starting_team);
            }
        }

        // Opening hands
        if let Some(starting_hand) = data.get("StartingHand").and_then(|v| v.as_array()) {
            println!("\n  🃏 Opening Hand:");
            println!("    Kept {} cards", starting_hand.len());

            if let Some(mulligans) = data.get("MulliganedHands").and_then(|v| v.as_array()) {
                if !mulligans.is_empty() {
                    println!("    Mulligans taken: {}", mulligans.len());
                } else {
                    println!("    No mulligans taken");
                }
            }
        }

        // Timer usage
        if let (Some(rope_shown), Some(rope_expired)) = (
            data.get("RopeShownCount").and_then(|v| v.as_u64()),
            data.get("RopeExpiredCount").and_then(|v| v.as_u64()),
        ) {
            if rope_shown > 0 || rope_expired > 0 {
                println!("\n  ⏰ Timer Usage:");
                println!("    Rope shown: {} times", rope_shown);
                println!("    Rope expired: {} times", rope_expired);
            }
        }

        Ok(None)
    }

    fn handle_courses_event(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "EventGetCoursesV2")? {
            println!("\n📚 Courses Event:");

            // Try to parse the courses array
            if let Some(courses) = data.get("Courses").and_then(|v| v.as_array()) {
                for course in courses {
                    if let (Some(name), Some(module)) = (
                        course.get("InternalEventName").and_then(|v| v.as_str()),
                        course.get("CurrentModule").and_then(|v| v.as_str()),
                    ) {
                        println!("  • {} - Status: {}", name, module);

                        // Show deck info if available
                        if let Some(deck_summary) = course.get("CourseDeckSummary") {
                            if let Some(deck_name) =
                                deck_summary.get("Name").and_then(|v| v.as_str())
                            {
                                println!("    Deck: {}", deck_name);
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_quest_event(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "QuestGetQuests")? {
            println!("\n🎯 Active Quests:");

            if let Some(quests) = data.get("quests").and_then(|v| v.as_array()) {
                if quests.is_empty() {
                    println!("  No active quests");
                } else {
                    for (i, quest) in quests.iter().enumerate() {
                        println!("\n  📋 Quest {}:", i + 1);

                        // Extract quest name from locKey
                        if let Some(loc_key) = quest.get("locKey").and_then(|v| v.as_str()) {
                            let quest_name = loc_key
                                .split('/')
                                .next_back()
                                .unwrap_or(loc_key)
                                .replace('_', " ");
                            println!("    Name: {}", quest_name);
                        }

                        // Progress
                        if let (Some(progress), Some(goal)) = (
                            quest.get("endingProgress").and_then(|v| v.as_u64()),
                            quest.get("goal").and_then(|v| v.as_u64()),
                        ) {
                            let percentage = (progress as f64 / goal as f64 * 100.0) as u32;
                            println!("    Progress: {}/{} ({}%)", progress, goal, percentage);

                            // Progress bar
                            let filled = (percentage / 5) as usize;
                            let empty = 20 - filled;
                            println!("    [{}{}]", "█".repeat(filled), "░".repeat(empty));
                        }

                        // Rewards
                        if let Some(chest) = quest.get("chestDescription") {
                            if let Some(header) = chest.get("headerLocKey").and_then(|v| v.as_str())
                            {
                                let reward_type = header
                                    .split('/')
                                    .next_back()
                                    .unwrap_or("Reward")
                                    .replace('_', " ");
                                println!("    Reward: {}", reward_type);

                                // Extract reward amounts
                                if let Some(params) =
                                    chest.get("locParams").and_then(|v| v.as_object())
                                {
                                    if let (Some(gold), Some(xp)) = (
                                        params.get("number1").and_then(|v| v.as_u64()),
                                        params.get("number2").and_then(|v| v.as_u64()),
                                    ) {
                                        println!("      💰 {} Gold", gold);
                                        println!("      ⭐ {} XP", xp);
                                    }
                                } else if let Some(quantity) =
                                    chest.get("quantity").and_then(|v| v.as_str())
                                {
                                    println!("      Amount: {}", quantity);
                                }
                            }
                        }

                        // Quest track
                        if let Some(track) = quest.get("questTrack").and_then(|v| v.as_str()) {
                            if track != "Default" {
                                println!("    Track: {}", track);
                            }
                        }
                    }
                }
            }

            if let Some(can_swap) = data.get("canSwap").and_then(|v| v.as_bool()) {
                println!(
                    "\n  🔄 Quest reroll available: {}",
                    if can_swap { "Yes" } else { "No" }
                );
            }
        }
        Ok(None)
    }

    fn handle_rank_info(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "RankGetCombinedRankInfo")? {
            println!("\n🏆 Rank Information:");

            // Constructed rank info
            if let (Some(season), Some(class), Some(level)) = (
                data.get("constructedSeasonOrdinal")
                    .and_then(|v| v.as_u64()),
                data.get("constructedClass").and_then(|v| v.as_str()),
                data.get("constructedLevel").and_then(|v| v.as_u64()),
            ) {
                println!("\n  📊 Constructed Rank:");
                println!("    Season: {}", season);
                println!("    Rank: {} {}", class, level);

                if let (Some(wins), Some(losses), Some(draws)) = (
                    data.get("constructedMatchesWon").and_then(|v| v.as_u64()),
                    data.get("constructedMatchesLost").and_then(|v| v.as_u64()),
                    data.get("constructedMatchesDrawn").and_then(|v| v.as_u64()),
                ) {
                    let total = wins + losses + draws;
                    let win_rate = if total > 0 {
                        (wins as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    println!(
                        "    Record: {}-{}-{} ({:.1}% win rate)",
                        wins, losses, draws, win_rate
                    );
                    println!("    Total matches: {}", total);
                }
            }

            // Limited rank info
            if let (Some(season), Some(level)) = (
                data.get("limitedSeasonOrdinal").and_then(|v| v.as_u64()),
                data.get("limitedLevel").and_then(|v| v.as_u64()),
            ) {
                println!("\n  🎲 Limited Rank:");
                println!("    Season: {}", season);

                // Limited uses numeric levels (Bronze 4 = 0, Bronze 3 = 1, etc.)
                let (rank_name, tier) = match level {
                    0..=3 => ("Bronze", 4 - (level % 4)),
                    4..=7 => ("Silver", 4 - (level % 4)),
                    8..=11 => ("Gold", 4 - (level % 4)),
                    12..=15 => ("Platinum", 4 - (level % 4)),
                    16..=19 => ("Diamond", 4 - (level % 4)),
                    _ => ("Mythic", 0),
                };

                if tier > 0 {
                    println!("    Rank: {} {}", rank_name, tier);
                } else {
                    println!("    Rank: {}", rank_name);
                }

                if let (Some(wins), Some(losses)) = (
                    data.get("limitedMatchesWon").and_then(|v| v.as_u64()),
                    data.get("limitedMatchesLost").and_then(|v| v.as_u64()),
                ) {
                    let total = wins + losses;
                    let win_rate = if total > 0 {
                        (wins as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    println!(
                        "    Record: {}-{} ({:.1}% win rate)",
                        wins, losses, win_rate
                    );
                    println!("    Total matches: {}", total);
                }
            }
        }
        Ok(None)
    }

    fn handle_gre_event_raw(
        &mut self,
        raw_data: &str,
        async_tx: tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<Option<ParsedEvent>> {
        if let Some(mut data) = self.extract_json_data(raw_data, "GreToClientEvent")? {
            // Check if the event is nested under "greToClientEvent" (lowercase)
            if let Some(event_data) = data.get("greToClientEvent") {
                data = event_data.clone();
            }

            // Process GRE messages
            if let Some(messages) = data.get("greToClientMessages").and_then(|v| v.as_array()) {
                for message in messages {
                    if let Some(msg_type) = message.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "GREMessageType_GameStateMessage" => {
                                if let Some(game_state_msg) = message.get("gameStateMessage") {
                                    self.process_game_state_message(game_state_msg, &async_tx)?;
                                }
                            }
                            "GREMessageType_TimerStateMessage" => {
                                if let Some(timer_msg) = message.get("timerStateMessage") {
                                    self.process_timer_state_message(timer_msg)?;
                                }
                            }
                            "GREMessageType_UIMessage" => {
                                if let Some(ui_msg) = message.get("uiMessage") {
                                    self.process_ui_message(ui_msg)?;
                                }
                            }
                            _ => {
                                // Other message types we might want to handle later
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_match_state_change(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        // First try to extract the JSON data
        if let Some(mut data) =
            self.extract_json_data(raw_data, "MatchGameRoomStateChangedEvent")?
        {
            // Check if the event is nested under "matchGameRoomStateChangedEvent" (lowercase)
            if let Some(event_data) = data.get("matchGameRoomStateChangedEvent") {
                data = event_data.clone();
            }

            // Now parse the match completion
            if let Some(game_room_info) = data.get("gameRoomInfo") {
                if let Some(state_type) = game_room_info.get("stateType").and_then(|v| v.as_str()) {
                    if state_type == "MatchGameRoomStateType_MatchCompleted" {
                        return self.handle_match_completed(game_room_info);
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_match_completed(&mut self, game_room_info: &Value) -> Result<Option<ParsedEvent>> {
        println!("\n⚔️ Match Completed:");

        // Extract player information
        if let Some(config) = game_room_info.get("gameRoomConfig") {
            if let Some(players) = config.get("reservedPlayers").and_then(|v| v.as_array()) {
                println!("\n  👥 Players:");
                for (i, player) in players.iter().enumerate() {
                    if let (Some(name), Some(seat_id)) = (
                        player.get("playerName").and_then(|v| v.as_str()),
                        player.get("systemSeatId").and_then(|v| v.as_u64()),
                    ) {
                        let platform = player
                            .get("platformId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");
                        println!(
                            "    Player {}: {} (Seat {}, {})",
                            i + 1,
                            name,
                            seat_id,
                            platform
                        );
                    }
                }
            }

            if let Some(match_id) = config.get("matchId").and_then(|v| v.as_str()) {
                println!("\n  🆔 Match ID: {}", match_id);
            }
        }

        // Extract match results
        if let Some(final_result) = game_room_info.get("finalMatchResult") {
            if let Some(result_list) = final_result.get("resultList").and_then(|v| v.as_array()) {
                for result in result_list {
                    if let (Some(scope), Some(winning_team)) = (
                        result.get("scope").and_then(|v| v.as_str()),
                        result.get("winningTeamId").and_then(|v| v.as_u64()),
                    ) {
                        match scope {
                            "MatchScope_Match" => {
                                println!("\n  🏆 Match Result:");

                                // Find the winning player
                                if let Some(config) = game_room_info.get("gameRoomConfig") {
                                    if let Some(players) =
                                        config.get("reservedPlayers").and_then(|v| v.as_array())
                                    {
                                        for player in players {
                                            if let (Some(team_id), Some(name)) = (
                                                player.get("teamId").and_then(|v| v.as_u64()),
                                                player.get("playerName").and_then(|v| v.as_str()),
                                            ) {
                                                if team_id == winning_team {
                                                    println!("    Winner: {} 🎉", name);
                                                } else {
                                                    println!("    Loser: {}", name);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            "MatchScope_Game" => {
                                // Individual game results - could track these if needed
                            }
                            _ => {}
                        }
                    }
                }
            }

            if let Some(reason) = final_result
                .get("matchCompletedReason")
                .and_then(|v| v.as_str())
            {
                let reason_display = match reason {
                    "MatchCompletedReasonType_Success" => "Normal completion",
                    "MatchCompletedReasonType_Timeout" => "Timeout",
                    "MatchCompletedReasonType_Concede" => "Concession",
                    _ => reason,
                };
                println!("    Completion reason: {}", reason_display);
            }
        }

        Ok(None)
    }

    fn process_game_state_message(
        &mut self,
        game_state_msg: &Value,
        async_tx: &tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<()> {
        // Check if this is a game state diff
        if let Some(state_type) = game_state_msg.get("type").and_then(|v| v.as_str()) {
            if state_type == "GameStateType_Diff" {
                // Process turn information
                if let Some(turn_info) = game_state_msg.get("turnInfo") {
                    self.display_turn_info(turn_info)?;
                }

                // Process game info for match end
                if let Some(game_info) = game_state_msg.get("gameInfo") {
                    if let Some(stage) = game_info.get("stage").and_then(|v| v.as_str()) {
                        if stage == "GameStage_GameOver" {
                            self.display_game_over(game_info)?;
                        }
                    }
                }

                // Process actions
                if let Some(actions) = game_state_msg.get("actions").and_then(|v| v.as_array()) {
                    if !actions.is_empty() {
                        println!("\n⚡ Actions:");
                        for action in actions {
                            self.display_action(action, async_tx)?;
                        }
                    }
                }

                // Process game objects diff
                if let Some(game_objects) =
                    game_state_msg.get("gameObjects").and_then(|v| v.as_array())
                {
                    self.process_game_objects(game_objects, async_tx)?;
                }

                // Process zones diff
                if let Some(zones) = game_state_msg.get("zones").and_then(|v| v.as_array()) {
                    self.process_zones_diff(zones)?;
                }

                // Process players diff
                if let Some(players) = game_state_msg.get("players").and_then(|v| v.as_array()) {
                    self.process_players_diff(players)?;
                }

                // Process timers if present
                if let Some(timers) = game_state_msg.get("timers").and_then(|v| v.as_array()) {
                    self.process_timers_in_game_state(timers)?;
                }

                // Process annotations - these tell us what actually happened
                if let Some(annotations) =
                    game_state_msg.get("annotations").and_then(|v| v.as_array())
                {
                    self.process_annotations(annotations, async_tx)?;
                }

                // Process persistent annotations
                if let Some(persist_annotations) = game_state_msg
                    .get("persistentAnnotations")
                    .and_then(|v| v.as_array())
                {
                    self.process_persistent_annotations(persist_annotations)?;
                }

                // Process deleted instances
                if let Some(deleted_ids) = game_state_msg
                    .get("diffDeletedInstanceIds")
                    .and_then(|v| v.as_array())
                {
                    self.process_deleted_instances(deleted_ids)?;
                }

                // Process deleted persistent annotations
                if let Some(deleted_ann_ids) = game_state_msg
                    .get("diffDeletedPersistentAnnotationIds")
                    .and_then(|v| v.as_array())
                {
                    self.process_deleted_annotations(deleted_ann_ids)?;
                }

                // Store this game state for future comparison
                self.last_game_state = Some(game_state_msg.clone());
            }
        }

        Ok(())
    }

    fn process_timer_state_message(&self, timer_msg: &Value) -> Result<()> {
        if let Some(seat_id) = timer_msg.get("seatId").and_then(|v| v.as_u64()) {
            if let Some(timers) = timer_msg.get("timers").and_then(|v| v.as_array()) {
                let mut active_timers = Vec::new();
                let mut warning_timers = Vec::new();

                for timer in timers {
                    if let (Some(timer_type), Some(duration)) = (
                        timer.get("type").and_then(|v| v.as_str()),
                        timer.get("durationSec").and_then(|v| v.as_u64()),
                    ) {
                        let is_running = timer
                            .get("running")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let elapsed = timer
                            .get("elapsedSec")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                        let warning_threshold = timer
                            .get("warningThresholdSec")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(30);

                        if is_running {
                            let remaining = duration.saturating_sub(elapsed);
                            let timer_name = match timer_type {
                                "TimerType_ActivePlayer" => "Active Player",
                                "TimerType_NonActivePlayer" => "Non-Active Player",
                                "TimerType_Inactivity" => "Inactivity",
                                "TimerType_Prologue" => "Prologue",
                                "TimerType_Epilogue" => "Epilogue",
                                "TimerType_Delay" => "Delay",
                                _ => timer_type,
                            };

                            if remaining <= warning_threshold {
                                warning_timers.push(format!(
                                    "⚠️ {} timer: {}s remaining",
                                    timer_name, remaining
                                ));
                            } else {
                                active_timers.push(format!(
                                    "{} timer: {}s remaining",
                                    timer_name, remaining
                                ));
                            }
                        }
                    }
                }

                if !active_timers.is_empty() || !warning_timers.is_empty() {
                    println!("\n⏱️ Timer Update - Player {}:", seat_id);
                    for timer in warning_timers {
                        println!("  {}", timer);
                    }
                    for timer in active_timers {
                        println!("  {}", timer);
                    }
                }
            }
        }
        Ok(())
    }

    fn process_ui_message(&self, ui_msg: &Value) -> Result<()> {
        // Handle UI messages like prompts, choices, etc.
        if let Some(prompt) = ui_msg.get("prompt") {
            if let Some(prompt_text) = prompt.get("promptText").and_then(|v| v.as_str()) {
                println!("\n💭 Game Prompt: {}", prompt_text);
            }
        }
        Ok(())
    }

    fn display_turn_info(&self, turn_info: &Value) -> Result<()> {
        if let (Some(phase), Some(turn), Some(active_player)) = (
            turn_info.get("phase").and_then(|v| v.as_str()),
            turn_info.get("turnNumber").and_then(|v| v.as_u64()),
            turn_info.get("activePlayer").and_then(|v| v.as_u64()),
        ) {
            let phase_display = match phase {
                "Phase_Beginning" => "Beginning",
                "Phase_Main1" => "Main 1",
                "Phase_Combat" => "Combat",
                "Phase_Main2" => "Main 2",
                "Phase_Ending" => "End",
                _ => phase,
            };

            // Check if we have a step
            let step_info = if let Some(step) = turn_info.get("step").and_then(|v| v.as_str()) {
                let step_display = match step {
                    "Step_Untap" => "Untap",
                    "Step_Upkeep" => "Upkeep",
                    "Step_Draw" => "Draw",
                    "Step_BeginCombat" => "Beginning of Combat",
                    "Step_DeclareAttack" => "Declare Attackers",
                    "Step_DeclareBlock" => "Declare Blockers",
                    "Step_FirstStrikeDamage" => "First Strike Damage",
                    "Step_CombatDamage" => "Combat Damage",
                    "Step_EndCombat" => "End of Combat",
                    "Step_End" => "End",
                    "Step_Cleanup" => "Cleanup",
                    _ => step,
                };
                format!(" - {}", step_display)
            } else {
                String::new()
            };

            println!(
                "\n📍 Turn {} - Player {}'s {}{}",
                turn, active_player, phase_display, step_info
            );

            // Show priority if different from active player
            if let Some(priority_player) = turn_info.get("priorityPlayer").and_then(|v| v.as_u64())
            {
                if priority_player != active_player {
                    println!("   Priority: Player {}", priority_player);
                }
            }

            // Show decision player if present
            if let Some(decision_player) = turn_info.get("decisionPlayer").and_then(|v| v.as_u64())
            {
                if decision_player > 0 {
                    println!("   Waiting for: Player {}", decision_player);
                }
            }
        }
        Ok(())
    }

    fn display_game_over(&self, game_info: &Value) -> Result<()> {
        println!("\n🏁 Game Over!");

        if let Some(results) = game_info.get("results").and_then(|v| v.as_array()) {
            for result in results {
                if let (Some(scope), Some(winning_team), Some(reason)) = (
                    result.get("scope").and_then(|v| v.as_str()),
                    result.get("winningTeamId").and_then(|v| v.as_u64()),
                    result.get("reason").and_then(|v| v.as_str()),
                ) {
                    let scope_display = match scope {
                        "MatchScope_Game" => "Game",
                        "MatchScope_Match" => "Match",
                        _ => scope,
                    };

                    let reason_display = match reason {
                        "ResultReason_Concede" => "Concession",
                        "ResultReason_Damage" => "Damage",
                        "ResultReason_Timeout" => "Timeout",
                        "ResultReason_DeckEmpty" => "Deck Empty",
                        _ => reason,
                    };

                    println!(
                        "  {} Winner: Player {} ({})",
                        scope_display, winning_team, reason_display
                    );
                }
            }
        }

        Ok(())
    }

    fn format_mana_cost(&self, mana_cost: &[Value]) -> String {
        let mut cost_str = String::new();
        for cost in mana_cost {
            if let (Some(colors), Some(count)) = (
                cost.get("color").and_then(|v| v.as_array()),
                cost.get("count").and_then(|v| v.as_u64()),
            ) {
                for color in colors {
                    if let Some(color_str) = color.as_str() {
                        let symbol = match color_str {
                            "ManaColor_White" => "W",
                            "ManaColor_Blue" => "U",
                            "ManaColor_Black" => "B",
                            "ManaColor_Red" => "R",
                            "ManaColor_Green" => "G",
                            "ManaColor_Colorless" => "C",
                            "ManaColor_Generic" => {
                                if count == 1 {
                                    "1"
                                } else {
                                    &count.to_string()
                                }
                            }
                            _ => "?",
                        };

                        if color_str == "ManaColor_Generic" && count > 1 {
                            cost_str.push_str(&format!("{{{}}}", count));
                        } else {
                            for _ in 0..count {
                                cost_str.push_str(&format!("{{{}}}", symbol));
                            }
                        }
                    }
                }
            }
        }
        cost_str
    }

    fn display_action(
        &self,
        action: &Value,
        async_tx: &tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<()> {
        if let (Some(seat_id), Some(action_data)) = (
            action.get("seatId").and_then(|v| v.as_u64()),
            action.get("action"),
        ) {
            if let Some(action_type) = action_data.get("actionType").and_then(|v| v.as_str()) {
                match action_type {
                    "ActionType_Activate_Mana" => {
                        if let (Some(instance_id), Some(ability_id)) = (
                            action_data.get("instanceId").and_then(|v| v.as_u64()),
                            action_data.get("abilityGrpId").and_then(|v| v.as_u64()),
                        ) {
                            // Look up the game object
                            if let Some(obj_info) = self.game_objects.get(&(instance_id as u32)) {
                                println!(
                                    "  🔴 Player {} taps object #{} for mana",
                                    seat_id, instance_id
                                );

                                // Send async request to get card details
                                if let Some(grp_id) = Some(obj_info.grp_id) {
                                    let _ = async_tx.try_send(crate::companion::watch::async_processor::AsyncTask::FetchCardByGrpId {
                                        grp_id,
                                        context: format!("Tapped for mana by Player {}", seat_id),
                                    });
                                }
                            } else {
                                println!(
                                    "  🔴 Player {} activates mana ability on #{}",
                                    seat_id, instance_id
                                );
                            }
                        }
                    }
                    "ActionType_Activate" => {
                        if let (Some(instance_id), Some(ability_id)) = (
                            action_data.get("instanceId").and_then(|v| v.as_u64()),
                            action_data.get("abilityGrpId").and_then(|v| v.as_u64()),
                        ) {
                            println!(
                                "  📜 Player {} activates ability {} on object #{}",
                                seat_id, ability_id, instance_id
                            );

                            // Display mana cost if present
                            if let Some(mana_cost) =
                                action_data.get("manaCost").and_then(|v| v.as_array())
                            {
                                let cost_str = self.format_mana_cost(mana_cost);
                                if !cost_str.is_empty() {
                                    println!("     Paying: {}", cost_str);
                                }
                            }

                            // Send async request to get card details
                            if let Some(obj_info) = self.game_objects.get(&(instance_id as u32)) {
                                let _ = async_tx.try_send(crate::companion::watch::async_processor::AsyncTask::FetchCardByGrpId {
                                    grp_id: obj_info.grp_id,
                                    context: format!("Ability activated by Player {}", seat_id),
                                });
                            }
                        }
                    }
                    "ActionType_Cast" => {
                        if let Some(grp_id) = action_data.get("grpId").and_then(|v| v.as_u64()) {
                            println!("  🎯 Player {} casts spell (grpId: {})", seat_id, grp_id);

                            // Send async request to get card details
                            let _ = async_tx.try_send(crate::companion::watch::async_processor::AsyncTask::FetchCardByGrpId {
                                grp_id: grp_id as u32,
                                context: format!("Cast by Player {}", seat_id),
                            });
                        }
                    }
                    _ => {
                        println!("  ❓ Player {} performs {}", seat_id, action_type);
                    }
                }
            }
        }
        Ok(())
    }

    fn process_game_objects(
        &mut self,
        game_objects: &[Value],
        async_tx: &tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<()> {
        for obj in game_objects {
            if let (Some(instance_id), Some(grp_id), Some(owner)) = (
                obj.get("instanceId").and_then(|v| v.as_u64()),
                obj.get("grpId").and_then(|v| v.as_u64()),
                obj.get("ownerSeatId").and_then(|v| v.as_u64()),
            ) {
                let zone = obj.get("zoneId").and_then(|v| v.as_u64()).unwrap_or(0);
                let zone_name = self.get_zone_name(zone);

                // Check if this is a new object or an update
                let is_new = !self.game_objects.contains_key(&(instance_id as u32));

                // Store object info
                self.game_objects.insert(
                    instance_id as u32,
                    GameObjectInfo {
                        grp_id: grp_id as u32,
                        owner: owner as u32,
                        zone: zone_name.to_string(),
                        card_name: None,
                    },
                );

                // If it's a new object in a visible zone, announce it
                if is_new && (zone == 3 || zone == 28 || zone == 5) {
                    // Extract card types for better display
                    let mut type_info = Vec::new();
                    if let Some(super_types) = obj.get("superTypes").and_then(|v| v.as_array()) {
                        for st in super_types {
                            if let Some(type_str) = st.as_str() {
                                type_info.push(type_str.replace("SuperType_", ""));
                            }
                        }
                    }
                    if let Some(card_types) = obj.get("cardTypes").and_then(|v| v.as_array()) {
                        for ct in card_types {
                            if let Some(type_str) = ct.as_str() {
                                type_info.push(type_str.replace("CardType_", ""));
                            }
                        }
                    }
                    if let Some(subtypes) = obj.get("subtypes").and_then(|v| v.as_array()) {
                        for st in subtypes {
                            if let Some(type_str) = st.as_str() {
                                type_info.push(type_str.replace("SubType_", ""));
                            }
                        }
                    }

                    let type_string = if type_info.is_empty() {
                        "Unknown".to_string()
                    } else {
                        type_info.join(" ")
                    };

                    println!(
                        "\n🆕 New object in {}: {} (Instance #{})",
                        zone_name, type_string, instance_id
                    );

                    // Fetch full card details
                    let _ = async_tx.try_send(
                        crate::companion::watch::async_processor::AsyncTask::FetchCardByGrpId {
                            grp_id: grp_id as u32,
                            context: format!("New in {} (Player {})", zone_name, owner),
                        },
                    );
                }
            }
        }
        Ok(())
    }

    fn process_zones_diff(&self, zones: &[Value]) -> Result<()> {
        // Process zone changes (cards moving between zones)
        for zone in zones {
            if let (Some(zone_id), Some(object_ids)) = (
                zone.get("zoneId").and_then(|v| v.as_u64()),
                zone.get("objectInstanceIds").and_then(|v| v.as_array()),
            ) {
                let zone_name = match zone_id {
                    1 => "Library",
                    2 => "Hand",
                    3 => "Battlefield",
                    4 => "Graveyard",
                    5 => "Stack",
                    6 => "Exile",
                    7 => "Command",
                    _ => "Unknown",
                };

                if !object_ids.is_empty() && zone_id == 3 {
                    // Something entered the battlefield
                    println!("\n🎭 Zone Change:");
                    println!("  {} objects entered {}", object_ids.len(), zone_name);
                }
            }
        }
        Ok(())
    }

    fn process_players_diff(&self, players: &[Value]) -> Result<()> {
        let mut has_changes = false;
        let mut status_lines = Vec::new();

        for player in players {
            if let (Some(seat), Some(life)) = (
                player.get("systemSeatNumber").and_then(|v| v.as_u64()),
                player.get("lifeTotal").and_then(|v| v.as_i64()),
            ) {
                let prev_life = if let Some(last_state) = &self.last_game_state {
                    last_state
                        .get("players")
                        .and_then(|v| v.as_array())
                        .and_then(|players| {
                            players.iter().find(|p| {
                                p.get("systemSeatNumber").and_then(|v| v.as_u64()) == Some(seat)
                            })
                        })
                        .and_then(|p| p.get("lifeTotal"))
                        .and_then(|v| v.as_i64())
                } else {
                    None
                };

                if let Some(prev) = prev_life {
                    if prev != life {
                        has_changes = true;
                        let diff = life - prev;
                        let symbol = if diff > 0 { "💚" } else { "💔" };
                        let change_str = if diff > 0 {
                            format!("+{}", diff)
                        } else {
                            diff.to_string()
                        };
                        status_lines.push(format!(
                            "  {} Player {}: {} → {} life ({})",
                            symbol, seat, prev, life, change_str
                        ));
                    }
                } else {
                    status_lines.push(format!("  Player {}: {} life", seat, life));
                }

                // Also check for status changes
                if let Some(status) = player.get("status").and_then(|v| v.as_str()) {
                    if status != "PlayerStatus_InGame" {
                        let status_display = match status {
                            "PlayerStatus_PendingLoss" => "⚠️ Pending Loss",
                            "PlayerStatus_Lost" => "❌ Lost",
                            "PlayerStatus_Won" => "🏆 Won",
                            _ => status,
                        };
                        status_lines.push(format!("    Status: {}", status_display));
                    }
                }
            }
        }

        if has_changes || !status_lines.is_empty() {
            println!("\n📊 Player Status Update:");
            for line in status_lines {
                println!("{}", line);
            }
        }

        Ok(())
    }

    fn process_timers_in_game_state(&self, timers: &[Value]) -> Result<()> {
        let mut active_timers = Vec::new();

        for timer in timers {
            if let (Some(timer_type), Some(duration), Some(is_running)) = (
                timer.get("type").and_then(|v| v.as_str()),
                timer.get("durationSec").and_then(|v| v.as_u64()),
                timer.get("running").and_then(|v| v.as_bool()),
            ) {
                if is_running {
                    let elapsed_sec = timer
                        .get("elapsedSec")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let elapsed_ms = timer.get("elapsedMs").and_then(|v| v.as_u64()).unwrap_or(0);
                    let total_elapsed_ms = elapsed_sec * 1000 + elapsed_ms;
                    let total_elapsed_sec = total_elapsed_ms as f64 / 1000.0;

                    let timer_name = match timer_type {
                        "TimerType_ActivePlayer" => "⏰ Active Player",
                        "TimerType_Inactivity" => "💤 Inactivity",
                        _ => timer_type,
                    };

                    let remaining = duration as f64 - total_elapsed_sec;
                    if remaining < 30.0 {
                        active_timers
                            .push(format!("{}: {:.1}s remaining ⚠️", timer_name, remaining));
                    } else {
                        active_timers
                            .push(format!("{}: {:.1}s elapsed", timer_name, total_elapsed_sec));
                    }
                }
            }
        }

        if !active_timers.is_empty() {
            println!("\n⏱️ Active Timers:");
            for timer in active_timers {
                println!("  {}", timer);
            }
        }

        Ok(())
    }

    fn process_annotations(
        &self,
        annotations: &[Value],
        async_tx: &tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<()> {
        for annotation in annotations {
            if let Some(ann_types) = annotation.get("type").and_then(|v| v.as_array()) {
                for ann_type in ann_types {
                    if let Some(type_str) = ann_type.as_str() {
                        match type_str {
                            "AnnotationType_ZoneTransfer" => {
                                self.handle_zone_transfer_annotation(annotation, async_tx)?;
                            }
                            "AnnotationType_UserActionTaken" => {
                                self.handle_user_action_annotation(annotation)?;
                            }
                            "AnnotationType_ObjectIdChanged" => {
                                self.handle_object_id_change_annotation(annotation)?;
                            }
                            "AnnotationType_DamageDealt" => {
                                self.handle_damage_annotation(annotation)?;
                            }
                            "AnnotationType_LifeChanged" => {
                                self.handle_life_change_annotation(annotation)?;
                            }
                            "AnnotationType_PhaseOrStepModified" => {
                                self.handle_phase_step_annotation(annotation)?;
                            }
                            "AnnotationType_NewTurnStarted" => {
                                self.handle_new_turn_annotation(annotation)?;
                            }
                            "AnnotationType_TappedUntappedPermanent" => {
                                self.handle_tap_untap_annotation(annotation)?;
                            }
                            _ => {
                                // Other annotation types we might want to handle later
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_zone_transfer_annotation(
        &self,
        annotation: &Value,
        async_tx: &tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<()> {
        if let Some(affected_ids) = annotation.get("affectedIds").and_then(|v| v.as_array()) {
            if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
                let mut zone_src = 0;
                let mut zone_dest = 0;
                let mut category = String::new();

                for detail in details {
                    if let (Some(key), Some(value_type)) = (
                        detail.get("key").and_then(|v| v.as_str()),
                        detail.get("type").and_then(|v| v.as_str()),
                    ) {
                        match key {
                            "zone_src" => {
                                if let Some(values) =
                                    detail.get("valueInt32").and_then(|v| v.as_array())
                                {
                                    zone_src = values.first().and_then(|v| v.as_u64()).unwrap_or(0);
                                }
                            }
                            "zone_dest" => {
                                if let Some(values) =
                                    detail.get("valueInt32").and_then(|v| v.as_array())
                                {
                                    zone_dest =
                                        values.first().and_then(|v| v.as_u64()).unwrap_or(0);
                                }
                            }
                            "category" => {
                                if let Some(values) =
                                    detail.get("valueString").and_then(|v| v.as_array())
                                {
                                    category = values
                                        .first()
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                }
                            }
                            _ => {}
                        }
                    }
                }

                let zone_from = self.get_zone_name(zone_src);
                let zone_to = self.get_zone_name(zone_dest);

                for affected_id in affected_ids {
                    if let Some(instance_id) = affected_id.as_u64() {
                        println!("\n🎭 Zone Transfer: {}", category);

                        // Look up the game object
                        if let Some(obj_info) = self.game_objects.get(&(instance_id as u32)) {
                            println!(
                                "  Object #{} moved from {} to {}",
                                instance_id, zone_from, zone_to
                            );

                            // Send async request to get card details
                            let context = match category.as_str() {
                                "PlayLand" => format!("Land played from {}", zone_from),
                                "CastSpell" => format!("Spell cast from {}", zone_from),
                                "Resolve" => format!("Spell resolved to {}", zone_to),
                                "Discard" => format!("Discarded from {}", zone_from),
                                "Draw" => format!("Drawn from {}", zone_from),
                                _ => format!("Moved from {} to {}", zone_from, zone_to),
                            };

                            let _ = async_tx.try_send(crate::companion::watch::async_processor::AsyncTask::FetchCardByGrpId {
                                grp_id: obj_info.grp_id,
                                context,
                            });
                        } else {
                            println!(
                                "  Object #{} moved from {} to {}",
                                instance_id, zone_from, zone_to
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_user_action_annotation(&self, annotation: &Value) -> Result<()> {
        if let (Some(affector_id), Some(affected_ids)) = (
            annotation.get("affectorId").and_then(|v| v.as_u64()),
            annotation.get("affectedIds").and_then(|v| v.as_array()),
        ) {
            if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
                let mut action_type = 0;

                for detail in details {
                    if detail.get("key").and_then(|v| v.as_str()) == Some("actionType") {
                        if let Some(values) = detail.get("valueInt32").and_then(|v| v.as_array()) {
                            action_type = values.first().and_then(|v| v.as_u64()).unwrap_or(0);
                        }
                    }
                }

                let action_name = match action_type {
                    1 => "Cast",
                    2 => "Activate",
                    3 => "Play",
                    4 => "Attack",
                    5 => "Block",
                    _ => "Unknown",
                };

                println!(
                    "  👤 Player {} performed {} action",
                    affector_id, action_name
                );
            }
        }
        Ok(())
    }

    fn handle_object_id_change_annotation(&self, annotation: &Value) -> Result<()> {
        if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
            let mut orig_id = 0;
            let mut new_id = 0;

            for detail in details {
                if let (Some(key), Some(values)) = (
                    detail.get("key").and_then(|v| v.as_str()),
                    detail.get("valueInt32").and_then(|v| v.as_array()),
                ) {
                    match key {
                        "orig_id" => orig_id = values.first().and_then(|v| v.as_u64()).unwrap_or(0),
                        "new_id" => new_id = values.first().and_then(|v| v.as_u64()).unwrap_or(0),
                        _ => {}
                    }
                }
            }

            if orig_id > 0 && new_id > 0 {
                println!("  🔄 Object ID changed: #{} → #{}", orig_id, new_id);
            }
        }
        Ok(())
    }

    fn handle_damage_annotation(&self, annotation: &Value) -> Result<()> {
        if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
            let mut damage = 0;
            let mut source = 0;
            let mut target = 0;

            for detail in details {
                if let Some(key) = detail.get("key").and_then(|v| v.as_str()) {
                    if let Some(values) = detail.get("valueInt32").and_then(|v| v.as_array()) {
                        match key {
                            "damage" => {
                                damage = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                            }
                            "source" => {
                                source = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                            }
                            "target" => {
                                target = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                            }
                            _ => {}
                        }
                    }
                }
            }

            if damage > 0 {
                println!(
                    "  💥 {} damage dealt from #{} to #{}",
                    damage, source, target
                );
            }
        }
        Ok(())
    }

    fn handle_life_change_annotation(&self, annotation: &Value) -> Result<()> {
        if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
            let mut player = 0;
            let mut life_change = 0;

            for detail in details {
                if let Some(key) = detail.get("key").and_then(|v| v.as_str()) {
                    if let Some(values) = detail.get("valueInt32").and_then(|v| v.as_array()) {
                        match key {
                            "player" => {
                                player = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                            }
                            "lifeChange" => {
                                life_change = values.first().and_then(|v| v.as_i64()).unwrap_or(0)
                            }
                            _ => {}
                        }
                    }
                }
            }

            if life_change != 0 {
                let symbol = if life_change > 0 { "💚" } else { "💔" };
                let change_str = if life_change > 0 {
                    format!("+{}", life_change)
                } else {
                    life_change.to_string()
                };
                println!("  {} Player {} life {}", symbol, player, change_str);
            }
        }
        Ok(())
    }

    fn handle_phase_step_annotation(&self, annotation: &Value) -> Result<()> {
        if let Some(affected_ids) = annotation.get("affectedIds").and_then(|v| v.as_array()) {
            if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
                let mut phase = 0;
                let mut step = 0;

                for detail in details {
                    if let Some(key) = detail.get("key").and_then(|v| v.as_str()) {
                        if let Some(values) = detail.get("valueInt32").and_then(|v| v.as_array()) {
                            match key {
                                "phase" => {
                                    phase = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                                }
                                "step" => {
                                    step = values.first().and_then(|v| v.as_u64()).unwrap_or(0)
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let phase_name = match phase {
                    1 => "Beginning",
                    2 => "Main 1",
                    3 => "Combat",
                    4 => "Main 2",
                    5 => "Ending",
                    _ => "Unknown",
                };

                let step_name = match (phase, step) {
                    (1, 1) => "Untap",
                    (1, 2) => "Upkeep",
                    (1, 3) => "Draw",
                    (3, 1) => "Beginning of Combat",
                    (3, 2) => "Declare Attackers",
                    (3, 3) => "Declare Blockers",
                    (3, 4) => "First Strike Damage",
                    (3, 5) => "Damage",
                    (3, 6) => "End of Combat",
                    (5, 1) => "End",
                    (5, 10) => "Cleanup",
                    _ => "",
                };

                if !step_name.is_empty() {
                    println!("  📍 Phase/Step: {} - {}", phase_name, step_name);
                } else if phase > 0 {
                    println!("  📍 Phase: {}", phase_name);
                }
            }
        }
        Ok(())
    }

    fn handle_new_turn_annotation(&self, annotation: &Value) -> Result<()> {
        if let (Some(affector_id), Some(affected_ids)) = (
            annotation.get("affectorId").and_then(|v| v.as_u64()),
            annotation.get("affectedIds").and_then(|v| v.as_array()),
        ) {
            for affected_id in affected_ids {
                if let Some(player_id) = affected_id.as_u64() {
                    println!("\n🔄 New Turn Started!");
                    println!("  Active Player: Player {}", player_id);
                }
            }
        }
        Ok(())
    }

    fn handle_tap_untap_annotation(&self, annotation: &Value) -> Result<()> {
        if let Some(affected_ids) = annotation.get("affectedIds").and_then(|v| v.as_array()) {
            if let Some(details) = annotation.get("details").and_then(|v| v.as_array()) {
                let mut tapped = false;

                for detail in details {
                    if detail.get("key").and_then(|v| v.as_str()) == Some("tapped") {
                        if let Some(values) = detail.get("valueInt32").and_then(|v| v.as_array()) {
                            tapped = values.first().and_then(|v| v.as_u64()).unwrap_or(0) == 1;
                        }
                    }
                }

                for affected_id in affected_ids {
                    if let Some(instance_id) = affected_id.as_u64() {
                        if tapped {
                            println!("  ⚡ Object #{} tapped", instance_id);
                        } else {
                            println!("  ♻️ Object #{} untapped", instance_id);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn process_persistent_annotations(&self, annotations: &[Value]) -> Result<()> {
        for annotation in annotations {
            if let Some(ann_types) = annotation.get("type").and_then(|v| v.as_array()) {
                for ann_type in ann_types {
                    if let Some(type_str) = ann_type.as_str() {
                        match type_str {
                            "AnnotationType_EnteredZoneThisTurn" => {
                                if let Some(affected_ids) =
                                    annotation.get("affectedIds").and_then(|v| v.as_array())
                                {
                                    for id in affected_ids {
                                        if let Some(instance_id) = id.as_u64() {
                                            println!(
                                                "  ⏰ Object #{} entered this turn",
                                                instance_id
                                            );
                                        }
                                    }
                                }
                            }
                            "AnnotationType_ColorProduction" => {
                                // Track what colors a permanent can produce
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn process_deleted_instances(&mut self, deleted_ids: &[Value]) -> Result<()> {
        if !deleted_ids.is_empty() {
            println!("\n💀 Objects Removed:");
            for id in deleted_ids {
                if let Some(instance_id) = id.as_u64() {
                    // Look up what this object was
                    if let Some(obj_info) = self.game_objects.get(&(instance_id as u32)) {
                        println!("  Object #{} removed from {}", instance_id, obj_info.zone);
                    } else {
                        println!("  Object #{} removed from game", instance_id);
                    }

                    // Remove from our tracking
                    self.game_objects.remove(&(instance_id as u32));
                }
            }
        }
        Ok(())
    }

    fn process_deleted_annotations(&self, deleted_ann_ids: &[Value]) -> Result<()> {
        if !deleted_ann_ids.is_empty() {
            println!("\n🔚 Effects Ended:");
            for id in deleted_ann_ids {
                if let Some(ann_id) = id.as_u64() {
                    println!("  Persistent effect #{} ended", ann_id);
                }
            }
        }
        Ok(())
    }

    fn get_zone_name(&self, zone_id: u64) -> &'static str {
        match zone_id {
            1 => "Library",
            2 => "Hand",
            3 => "Battlefield",
            4 => "Graveyard",
            5 => "Stack",
            6 => "Exile",
            7 => "Command",
            28 => "Battlefield", // Sometimes battlefield has different IDs
            31 => "Hand",        // Sometimes hand has different IDs
            _ => "Unknown Zone",
        }
    }

    fn handle_state_change(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(state_data) = serde_json::from_str::<Value>(raw_data) {
            if let (Some(old_state), Some(new_state)) = (
                state_data.get("old").and_then(|v| v.as_str()),
                state_data.get("new").and_then(|v| v.as_str()),
            ) {
                println!("\n🔄 Game State Changed:");
                println!("  From: {}", old_state);
                println!("  To: {}", new_state);

                // Handle specific state transitions
                match (old_state, new_state) {
                    ("Playing", "MatchCompleted") => {
                        println!("  📊 Match has ended!");
                    }
                    ("Idle", "Playing") => {
                        println!("  🎮 Match started!");
                    }
                    ("Matchmaking", "Playing") => {
                        println!("  🎯 Found opponent, starting match!");
                    }
                    ("Playing", "Sideboarding") => {
                        println!("  🔧 Entering sideboard phase");
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    fn handle_draft_pack(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "HumanDraftPack")? {
            if let Ok(draft_pack) = serde_json::from_value::<DraftPack>(data) {
                if let (Some(pack_cards), Some(pack_num), Some(pick_num)) = (
                    draft_pack.pack_cards,
                    draft_pack.self_pack,
                    draft_pack.self_pick,
                ) {
                    let card_ids: Vec<u32> = pack_cards
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();

                    // Update draft state
                    if self.current_draft.is_none() {
                        self.current_draft = Some(DraftState::default());
                    }

                    if let Some(ref mut draft) = self.current_draft {
                        draft.pack_number = pack_num;
                        draft.pick_number = pick_num;
                        draft.current_pack = card_ids.clone();
                        draft.is_drafting = true;
                    }

                    return Ok(Some(ParsedEvent::DraftPack {
                        pack_number: pack_num,
                        pick_number: pick_num,
                        cards: card_ids,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn handle_draft_pick(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Some(data) = self.extract_json_data(raw_data, "HumanDraftPick")? {
            if let Ok(draft_pick) = serde_json::from_value::<DraftPick>(data) {
                if let Some(is_completed) = draft_pick.is_picking_completed {
                    if is_completed {
                        if let Some(ref mut draft) = self.current_draft {
                            draft.is_drafting = false;
                        }
                        return Ok(Some(ParsedEvent::DraftCompleted));
                    }
                }

                if let Some(picked_card) = draft_pick.picked_card {
                    if let Some(ref mut draft) = self.current_draft {
                        draft.picked_cards.push(picked_card);
                    }
                    return Ok(Some(ParsedEvent::DraftPick {
                        card_id: picked_card,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn handle_deck_submission(
        &mut self,
        raw_data: &str,
        async_tx: tokio::sync::mpsc::Sender<crate::companion::watch::async_processor::AsyncTask>,
    ) -> Result<Option<ParsedEvent>> {
        // Try different event types that contain deck data
        let event_types = [
            "DeckSubmission",
            "Event_GetCourseDeck",
            "DeckMessage",
            "DeckUpsertDeckV2",
        ];

        for event_type in &event_types {
            if let Some(data) = self.extract_json_data(raw_data, event_type)? {
                // Some events have the deck in a "request" field, others might have it directly
                let deck_data =
                    if let Some(request_str) = data.get("request").and_then(|v| v.as_str()) {
                        // Try to parse the request field as JSON
                        serde_json::from_str::<Value>(request_str).ok()
                    } else if data.get("MainDeck").is_some() || data.get("Summary").is_some() {
                        // The data itself might be the deck
                        Some(data.clone())
                    } else {
                        None
                    };

                if let Some(deck) = deck_data {
                    // Display basic info immediately
                    self.display_deck_basic(&deck)?;

                    // Send async task to fetch full card data
                    let _ = async_tx.try_send(
                        crate::companion::watch::async_processor::AsyncTask::FetchDeckCards {
                            deck_data: deck.clone(),
                        },
                    );

                    return Ok(None);
                }
            }
        }

        // If we couldn't find deck data in expected format, just print what we have
        println!("\n🃏 Deck Event:");
        println!("  Raw data: {}", raw_data);

        Ok(None)
    }

    fn display_deck_basic(&self, deck_data: &Value) -> Result<()> {
        println!("\n🃏 Deck Event:");

        // Debug: Check what fields are present
        if std::env::var("MTG_DEBUG").is_ok() {
            if let Some(obj) = deck_data.as_object() {
                println!(
                    "  DEBUG: Available fields: {:?}",
                    obj.keys().collect::<Vec<_>>()
                );
            }
        }

        // Extract deck summary info
        if let Some(summary) = deck_data.get("Summary") {
            if let Some(name) = summary.get("Name").and_then(|v| v.as_str()) {
                println!("  📝 Name: {}", name);
            }

            if let Some(deck_id) = summary.get("DeckId").and_then(|v| v.as_str()) {
                println!("  🆔 ID: {}", deck_id);
            }

            // Extract format from attributes
            if let Some(attrs) = summary.get("Attributes").and_then(|v| v.as_array()) {
                for attr in attrs {
                    if let (Some(name), Some(value)) = (
                        attr.get("name").and_then(|v| v.as_str()),
                        attr.get("value").and_then(|v| v.as_str()),
                    ) {
                        if name == "Format" {
                            println!("  🎮 Format: {}", value);
                        }
                    }
                }
            }
        }

        // Extract main deck cards - check both at root level and inside "Deck" field
        let main_deck = deck_data
            .get("MainDeck")
            .and_then(|v| v.as_array())
            .or_else(|| {
                deck_data
                    .get("Deck")
                    .and_then(|d| d.get("MainDeck"))
                    .and_then(|v| v.as_array())
            });

        if let Some(main_deck) = main_deck {
            if main_deck.is_empty() {
                println!("\n  📋 Main Deck: Empty or not loaded");
            } else {
                println!("\n  📋 Main Deck ({} cards):", main_deck.len());

                // Group cards by arena_id and count
                let mut card_counts: std::collections::HashMap<u32, u32> =
                    std::collections::HashMap::new();
                for card in main_deck {
                    if let Some(arena_id) = card.get("cardId").and_then(|v| v.as_u64()) {
                        *card_counts.entry(arena_id as u32).or_insert(0) += 1;
                    }
                }

                // Display cards (without names for now)
                for (arena_id, count) in card_counts.iter() {
                    println!("    {}x Card #{}", count, arena_id);
                }
            }
        } else {
            println!("\n  📋 Main Deck: Not found in event data");
        }

        // Extract sideboard - check both at root level and inside "Deck" field
        let sideboard = deck_data
            .get("Sideboard")
            .and_then(|v| v.as_array())
            .or_else(|| {
                deck_data
                    .get("Deck")
                    .and_then(|d| d.get("Sideboard"))
                    .and_then(|v| v.as_array())
            });

        if let Some(sideboard) = sideboard {
            if !sideboard.is_empty() {
                println!("\n  📋 Sideboard ({} cards):", sideboard.len());

                let mut card_counts: std::collections::HashMap<u32, u32> =
                    std::collections::HashMap::new();
                for card in sideboard {
                    if let Some(arena_id) = card.get("cardId").and_then(|v| v.as_u64()) {
                        *card_counts.entry(arena_id as u32).or_insert(0) += 1;
                    }
                }

                for (arena_id, count) in card_counts.iter() {
                    println!("    {}x Card #{}", count, arena_id);
                }
            }
        }

        Ok(())
    }

    async fn display_deck_with_cards(&self, deck_data: &Value) -> Result<()> {
        println!("\n🃏 Deck Event:");

        let mut _deck_name = String::new();
        let mut _deck_format = String::new();

        // Extract deck summary info
        if let Some(summary) = deck_data.get("Summary") {
            if let Some(name) = summary.get("Name").and_then(|v| v.as_str()) {
                _deck_name = name.to_string();
                println!("  📝 Name: {}", name);
            }

            if let Some(deck_id) = summary.get("DeckId").and_then(|v| v.as_str()) {
                println!("  🆔 ID: {}", deck_id);
            }

            // Extract format from attributes
            if let Some(attrs) = summary.get("Attributes").and_then(|v| v.as_array()) {
                for attr in attrs {
                    if let (Some(name), Some(value)) = (
                        attr.get("name").and_then(|v| v.as_str()),
                        attr.get("value").and_then(|v| v.as_str()),
                    ) {
                        match name {
                            "Format" => {
                                _deck_format = value.to_string();
                                println!("  🎮 Format: {}", value);
                            }
                            "LastPlayed" => {
                                let cleaned = value.trim_matches('"');
                                println!("  ⏰ Last Played: {}", cleaned);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Extract action type
        if let Some(action) = deck_data.get("ActionType").and_then(|v| v.as_str()) {
            println!("  📋 Action: {}", action);
        }

        // Extract and display deck contents
        if let Some(deck) = deck_data.get("Deck") {
            if let Some(main_deck) = deck.get("MainDeck").and_then(|v| v.as_array()) {
                let total_cards: u64 = main_deck
                    .iter()
                    .filter_map(|card| card.get("quantity").and_then(|q| q.as_u64()))
                    .sum();

                println!("\n  📚 Main Deck ({} cards):", total_cards);

                // Collect all card IDs and quantities
                let mut card_entries: Vec<(u64, u64)> = Vec::new();
                for card in main_deck {
                    if let (Some(card_id), Some(quantity)) = (
                        card.get("cardId").and_then(|v| v.as_u64()),
                        card.get("quantity").and_then(|v| v.as_u64()),
                    ) {
                        card_entries.push((card_id, quantity));
                    }
                }

                // Fetch card names from Scryfall
                let card_map = self.fetch_card_names(&card_entries).await?;

                // Create table for deck display
                let mut table = new_table();
                table.add_row(Row::new(vec![
                    Cell::new("Card Name"),
                    Cell::new("Qty"),
                    Cell::new("Type"),
                    Cell::new("Cost"),
                    Cell::new("Draw %"),
                ]));

                // Separate lands and non-lands
                let mut lands: Vec<(u64, u64, &crate::scryfall::Card)> = Vec::new();
                let mut nonlands: Vec<(u64, u64, &crate::scryfall::Card)> = Vec::new();

                for (card_id, quantity) in &card_entries {
                    if let Some(card) = card_map.get(card_id) {
                        if card.type_line.to_lowercase().contains("land") {
                            lands.push((*card_id, *quantity, card));
                        } else {
                            nonlands.push((*card_id, *quantity, card));
                        }
                    }
                }

                // Sort non-lands by CMC, then by name
                nonlands.sort_by(|a, b| {
                    a.2.cmc
                        .partial_cmp(&b.2.cmc)
                        .unwrap()
                        .then(a.2.name.cmp(&b.2.name))
                });

                // Add non-lands to table
                for (_, quantity, card) in &nonlands {
                    let draw_prob = calculate_draw_probability(*quantity, total_cards, 7);
                    table.add_row(Row::new(vec![
                        Cell::new(&card.name),
                        Cell::new(&quantity.to_string()),
                        Cell::new(&card.type_line),
                        Cell::new(card.mana_cost.as_deref().unwrap_or("")),
                        Cell::new(&format!("{:.1}%", draw_prob * 100.0)),
                    ]));
                }

                // Add separator if we have both lands and non-lands
                if !nonlands.is_empty() && !lands.is_empty() {
                    table.add_row(Row::new(vec![
                        Cell::new("─────────────────────────────────────"),
                        Cell::new("───"),
                        Cell::new("─────────────────"),
                        Cell::new("──────"),
                        Cell::new("──────"),
                    ]));
                }

                // Sort lands by name
                lands.sort_by(|a, b| a.2.name.cmp(&b.2.name));

                // Add lands to table
                for (_, quantity, card) in &lands {
                    let draw_prob = calculate_draw_probability(*quantity, total_cards, 7);
                    table.add_row(Row::new(vec![
                        Cell::new(&card.name),
                        Cell::new(&quantity.to_string()),
                        Cell::new(&card.type_line),
                        Cell::new(""),
                        Cell::new(&format!("{:.1}%", draw_prob * 100.0)),
                    ]));
                }

                table.printstd();

                // Show mana curve for non-lands
                if !nonlands.is_empty() {
                    println!("\n  📊 Mana Curve:");
                    let mut curve: HashMap<u32, u32> = HashMap::new();
                    for (_, quantity, card) in &nonlands {
                        let cmc = card.cmc as u32;
                        *curve.entry(cmc).or_insert(0) += *quantity as u32;
                    }

                    let max_cmc = *curve.keys().max().unwrap_or(&0);
                    for cmc in 0..=max_cmc.min(7) {
                        let count = curve.get(&cmc).unwrap_or(&0);
                        let bar = "█".repeat(*count as usize);
                        println!("    {}: {} ({})", cmc, bar, count);
                    }
                    if max_cmc > 7 {
                        let high_cmc_count: u32 =
                            curve.iter().filter(|(k, _)| **k > 7).map(|(_, v)| v).sum();
                        println!(
                            "    8+: {} ({})",
                            "█".repeat(high_cmc_count as usize),
                            high_cmc_count
                        );
                    }
                }
            }

            // Show sideboard if present
            if let Some(sideboard) = deck.get("Sideboard").and_then(|v| v.as_array()) {
                if !sideboard.is_empty() {
                    println!("\n  📋 Sideboard ({} cards):", sideboard.len());

                    let mut sb_entries: Vec<(u64, u64)> = Vec::new();
                    for card in sideboard {
                        if let (Some(card_id), Some(quantity)) = (
                            card.get("cardId").and_then(|v| v.as_u64()),
                            card.get("quantity").and_then(|v| v.as_u64()),
                        ) {
                            sb_entries.push((card_id, quantity));
                        }
                    }

                    let sb_card_map = self.fetch_card_names(&sb_entries).await?;

                    let mut sb_cards: Vec<(u64, &crate::scryfall::Card)> = Vec::new();
                    for (card_id, quantity) in &sb_entries {
                        if let Some(card) = sb_card_map.get(card_id) {
                            sb_cards.push((*quantity, card));
                        }
                    }

                    // Sort by CMC then name
                    sb_cards.sort_by(|a, b| {
                        a.1.cmc
                            .partial_cmp(&b.1.cmc)
                            .unwrap()
                            .then(a.1.name.cmp(&b.1.name))
                    });

                    for (quantity, card) in sb_cards {
                        println!(
                            "    • {} {} {}",
                            quantity,
                            card.name,
                            card.mana_cost.as_deref().unwrap_or("")
                        );
                    }
                }
            }
        }

        Ok(())
    }

    async fn fetch_card_names(
        &self,
        card_entries: &[(u64, u64)],
    ) -> Result<HashMap<u64, crate::scryfall::Card>> {
        let cache_manager = CacheManager::new()?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("mtg-cli/1.0")
            .build()?;

        let mut card_map = HashMap::new();

        // Fetch each unique card
        let unique_ids: std::collections::HashSet<u64> =
            card_entries.iter().map(|(id, _)| *id).collect();

        for arena_id in unique_ids {
            let url = format!("https://api.scryfall.com/cards/arena/{}", arena_id);
            let cache_key = CacheManager::hash_request(&url);

            // Check cache first
            if let Some(cached_response) = cache_manager.get(&cache_key).await? {
                if let Ok(card) =
                    serde_json::from_value::<crate::scryfall::Card>(cached_response.data)
                {
                    card_map.insert(arena_id, card);
                    continue;
                }
            }

            // Fetch from API
            match client.get(&url).send().await {
                Ok(response) => {
                    if let Ok(text) = response.text().await {
                        if let Ok(card) = serde_json::from_str::<crate::scryfall::Card>(&text) {
                            // Cache the response
                            let _ = cache_manager
                                .set(&cache_key, serde_json::to_value(&card)?)
                                .await;
                            card_map.insert(arena_id, card);
                        }
                    }
                }
                Err(e) => {
                    // If we can't fetch, log the error and skip this card
                    eprintln!("Failed to fetch card {}: {}", arena_id, e);
                }
            }
        }

        Ok(card_map)
    }

    fn handle_gre_event(&mut self, gre_event: GreToClientEvent) -> Result<Option<ParsedEvent>> {
        println!("\n🎲 Game Rules Engine Event:");

        for message in gre_event.gre_to_client_messages {
            println!("  Message Type: {}", message.message_type);

            if let Some(game_state) = message.game_state_message {
                // Show game info
                if let Some(ref game_info) = game_state.game_info {
                    if let Some(ref match_id) = game_info.match_id {
                        println!("  Match ID: {}", match_id);
                    }
                    if let Some(game_num) = game_info.game_number {
                        println!("  Game Number: {}", game_num);
                    }
                }

                // Show turn info
                if let Some(ref turn_info) = game_state.turn_info {
                    println!(
                        "  Turn {}, Active Player: {}",
                        turn_info.turn_number, turn_info.active_player
                    );
                    if let Some(ref phase) = turn_info.phase {
                        println!("  Phase: {}", phase);
                    }
                }

                // Show player info
                if let Some(ref players) = game_state.players {
                    println!("  Players:");
                    for player in players {
                        println!(
                            "    • Seat {}: {} life",
                            player.controller_seat_id, player.life_total
                        );
                    }
                }

                // Show actions
                if let Some(ref actions) = game_state.actions {
                    if !actions.is_empty() {
                        println!("  Actions:");
                        for action in actions {
                            if let Some(ref action_type) = action.action_type {
                                println!("    • {}", action_type);
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn process_match_end(
        &mut self,
        state_change: MatchGameRoomStateChangedEvent,
    ) -> Result<Option<ParsedEvent>> {
        if let Some(ref mut match_state) = self.current_match {
            if let Some(game_room_info) = state_change.game_room_info {
                if let Some(final_result) = game_room_info.final_match_result {
                    match_state.ended_at = Some(Utc::now());

                    let duration = match_state
                        .ended_at
                        .unwrap()
                        .signed_duration_since(match_state.started_at)
                        .num_seconds() as u64;

                    let result = MatchResult {
                        winner: final_result.winning_team_id.map(|id| id as usize),
                        reason: final_result
                            .result_type
                            .unwrap_or_else(|| "Unknown".to_string()),
                        duration_seconds: duration,
                    };

                    match_state.result = Some(result.clone());

                    let completed_match = match_state.clone();
                    self.current_match = None;

                    return Ok(Some(ParsedEvent::MatchEnded(completed_match)));
                }
            }
        }
        Ok(None)
    }

    fn start_match_from_game_state(
        &mut self,
        match_id: String,
        game_state: &GameStateMessage,
    ) -> Result<Option<ParsedEvent>> {
        let mut players = Vec::new();

        // Extract player information from the game state
        if let Some(ref arena_players) = game_state.players {
            for arena_player in arena_players {
                players.push(Player {
                    seat_id: arena_player.controller_seat_id,
                    screen_name: format!("Player {}", arena_player.controller_seat_id), // We don't have screen names in game state
                    life_total: arena_player.life_total,
                    hand_size: 0, // Will be updated from zones
                    initial_hand: Vec::new(),
                    mulligans: Vec::new(),
                    deck_cards: Vec::new(),
                });
            }
        }

        // If no players found, create default players
        if players.is_empty() {
            players.push(Player {
                seat_id: 1,
                screen_name: "Player 1".to_string(),
                life_total: 20,
                hand_size: 0,
                initial_hand: Vec::new(),
                mulligans: Vec::new(),
                deck_cards: Vec::new(),
            });
            players.push(Player {
                seat_id: 2,
                screen_name: "Player 2".to_string(),
                life_total: 20,
                hand_size: 0,
                initial_hand: Vec::new(),
                mulligans: Vec::new(),
                deck_cards: Vec::new(),
            });
        }

        let (current_turn, active_player, phase) = if let Some(ref turn_info) = game_state.turn_info
        {
            // Convert seat ID to array index for active player
            let active_player_index = players
                .iter()
                .position(|p| p.seat_id == turn_info.active_player)
                .unwrap_or(0);

            (
                turn_info.turn_number,
                active_player_index,
                turn_info
                    .phase
                    .as_ref()
                    .map(|p| GamePhase::from(p.as_str()))
                    .unwrap_or(GamePhase::Unknown),
            )
        } else {
            (0, 0, GamePhase::Unknown)
        };

        let match_state = MatchState {
            match_id: match_id.clone(),
            players,
            current_turn,
            active_player,
            phase,
            started_at: Utc::now(),
            ended_at: None,
            result: None,
            actions: Vec::new(),
        };

        self.current_match = Some(match_state.clone());

        Ok(Some(ParsedEvent::MatchStarted(match_state)))
    }

    fn handle_match_start(&mut self, matches_v3: MatchesV3) -> Result<Option<ParsedEvent>> {
        let mut players = Vec::new();

        for player_info in matches_v3.player_infos {
            players.push(Player {
                seat_id: player_info.system_seat_id,
                screen_name: player_info.screen_name,
                life_total: 20, // Default starting life
                hand_size: 0,
                initial_hand: Vec::new(),
                mulligans: Vec::new(),
                deck_cards: Vec::new(),
            });
        }

        let match_state = MatchState {
            match_id: matches_v3.match_id.clone(),
            players,
            current_turn: 0,
            active_player: 0,
            phase: GamePhase::Unknown,
            started_at: Utc::now(),
            ended_at: None,
            result: None,
            actions: Vec::new(),
        };

        self.current_match = Some(match_state.clone());

        Ok(Some(ParsedEvent::MatchStarted(match_state)))
    }

    fn process_game_state(&mut self, game_state: GameStateMessage) -> Result<Option<ParsedEvent>> {
        let mut events = Vec::new();

        if let Some(ref mut match_state) = self.current_match {
            // Update player life totals
            if let Some(players) = game_state.players {
                for arena_player in players {
                    // Convert seat ID to array index before borrowing mutably
                    let player_index = match_state
                        .players
                        .iter()
                        .position(|p| p.seat_id == arena_player.controller_seat_id)
                        .unwrap_or(0);

                    if let Some(player) = match_state
                        .players
                        .iter_mut()
                        .find(|p| p.seat_id == arena_player.controller_seat_id)
                    {
                        if player.life_total != arena_player.life_total {
                            let old_life = player.life_total;
                            let life_change = arena_player.life_total - player.life_total;
                            player.life_total = arena_player.life_total;

                            let action = GameAction {
                                timestamp: Utc::now(),
                                player: player_index,
                                action_type: ActionType::LifeChange,
                                description: format!("Life changed by {}", life_change),
                                life_change: Some(life_change),
                            };

                            match_state.actions.push(action.clone());
                            events.push(ParsedEvent::LifeChange {
                                player: player_index,
                                old_life,
                                new_life: player.life_total,
                            });
                        }
                    }
                }
            }

            // Update turn information
            if let Some(turn_info) = game_state.turn_info {
                if match_state.current_turn != turn_info.turn_number {
                    match_state.current_turn = turn_info.turn_number;

                    // Convert seat ID to array index for active player
                    let active_player_index = match_state
                        .players
                        .iter()
                        .position(|p| p.seat_id == turn_info.active_player)
                        .unwrap_or(0);

                    match_state.active_player = active_player_index;

                    if let Some(phase_str) = turn_info.phase {
                        match_state.phase = GamePhase::from(phase_str.as_str());
                    }

                    let action = GameAction {
                        timestamp: Utc::now(),
                        player: active_player_index,
                        action_type: ActionType::TurnChange,
                        description: format!("Turn {} begins", turn_info.turn_number),
                        life_change: None,
                    };

                    match_state.actions.push(action);
                    events.push(ParsedEvent::TurnChange {
                        turn: turn_info.turn_number,
                        active_player: active_player_index,
                    });
                }
            }

            // Process actions (card plays, etc.)
            if let Some(actions) = game_state.actions {
                for arena_action in actions {
                    if let Some(action_type_str) = arena_action.action_type {
                        let action_type = ActionType::from(action_type_str.as_str());

                        let description = match action_type {
                            ActionType::Play => "Plays a card".to_string(),
                            ActionType::Cast => "Casts a spell".to_string(),
                            ActionType::Draw => "Draws a card".to_string(),
                            ActionType::Discard => "Discards a card".to_string(),
                            ActionType::Attack => "Declares attackers".to_string(),
                            ActionType::Block => "Declares blockers".to_string(),
                            _ => format!("Performs action: {}", action_type_str),
                        };

                        // Convert seat ID to array index
                        let player_index = if let Some(seat_id) = arena_action.seat_id {
                            match_state
                                .players
                                .iter()
                                .position(|p| p.seat_id == seat_id)
                                .unwrap_or(0)
                        } else {
                            0
                        };

                        let action = GameAction {
                            timestamp: Utc::now(),
                            player: player_index,
                            action_type,
                            description,
                            life_change: None,
                        };

                        match_state.actions.push(action.clone());
                        events.push(ParsedEvent::GameAction(action));
                    }
                }
            }
        }

        // Return the first event if any were generated
        Ok(events.into_iter().next())
    }

    pub fn current_match(&self) -> Option<&MatchState> {
        self.current_match.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum ParsedEvent {
    // Authentication events
    UserAuthenticated {
        user_id: String,
        display_name: String,
    },

    // Match events
    MatchStarted(MatchState),
    MatchEnded(MatchState),
    TurnChange {
        turn: u32,
        active_player: usize,
    },
    LifeChange {
        player: usize,
        old_life: i32,
        new_life: i32,
    },
    GameAction(GameAction),
    CardPlayed {
        player: usize,
        card_name: String,
        zone_from: Zone,
        zone_to: Zone,
    },
    Mulligan {
        player: usize,
        from_size: usize,
        to_size: usize,
    },

    // Draft events
    DraftPack {
        pack_number: u32,
        pick_number: u32,
        cards: Vec<u32>,
    },
    DraftPick {
        card_id: u32,
    },
    DraftCompleted,

    // Deck events
    DeckSubmitted {
        deck_id: Option<String>,
        deck_name: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_gre_event() {
        let mut parser = EventParser::new();

        // Create a channel for async tasks (we won't use it in the test)
        let (async_tx, _async_rx) = tokio::sync::mpsc::channel(100);

        // Test a simple timer state message
        let raw_event = RawLogEvent {
            timestamp: Some(Utc::now()),
            event_name: "GreToClientEvent".to_string(),
            raw_data: r#"{"greToClientMessages":[{"type":"GREMessageType_TimerStateMessage","timerStateMessage":{"timers":[{"timerId":1,"type":"TimerType_ActivePlayer","durationSec":30,"behavior":"TimerBehavior_TakeControl","warningThresholdSec":10,"elapsedSec":5.2,"elapsedMs":5200}]}}]}"#.to_string(),
        };

        let result = parser.parse_event(raw_event, async_tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("test_case"), "testCase");
        assert_eq!(to_camel_case("single"), "single");
    }
}
