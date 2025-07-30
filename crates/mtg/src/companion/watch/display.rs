use super::types::*;
use crate::companion::parse::events::EnhancedPlayerEvent;
use crate::prelude::*;
use chrono::Local;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use std::io::{self, Write};

pub struct MatchDisplay {
    show_detailed_actions: bool,
    use_colors: bool,
}

impl MatchDisplay {
    pub fn new() -> Self {
        Self {
            show_detailed_actions: true,
            use_colors: true,
        }
    }

    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    pub fn with_detailed_actions(mut self, show_detailed: bool) -> Self {
        self.show_detailed_actions = show_detailed;
        self
    }

    // Enhanced display for real-time events with timestamps
    pub fn display_enhanced_event(
        &self,
        event: &EnhancedPlayerEvent,
        player_names: &std::collections::HashMap<u64, String>,
    ) -> Result<()> {
        let timestamp = Local::now().format("%H:%M:%S");

        match event {
            EnhancedPlayerEvent::MatchStarted { match_id, players } => {
                println!("\n🎮 ═══════════════════════════════════════════");
                println!("🎮 [{}] MATCH STARTED: {}", timestamp, match_id);
                println!("🎮 ═══════════════════════════════════════════");
                for (seat_id, name, platform) in players {
                    println!(
                        "👤 [{}] Player {}: {} ({})",
                        timestamp, seat_id, name, platform
                    );
                }
                println!();
            }

            EnhancedPlayerEvent::GameState {
                turn,
                active_player,
            } => {
                if let (Some(turn_num), Some(active)) = (turn, active_player) {
                    let player_name = player_names
                        .get(active)
                        .map(|s| s.as_str())
                        .unwrap_or("Unknown");
                    println!(
                        "🔄 [{}] Turn {}: {} is active",
                        timestamp, turn_num, player_name
                    );
                }
            }

            EnhancedPlayerEvent::Mulligan { decision } => {
                println!("🤲 [{}] {}", timestamp, decision);
            }

            EnhancedPlayerEvent::CardPlayed { action, zones: _ } => {
                println!("🎴 [{}] {}", timestamp, action);
            }

            EnhancedPlayerEvent::SpellCast { mana_cost } => {
                println!("✨ [{}] {}", timestamp, mana_cost);
            }

            EnhancedPlayerEvent::ActionTaken { action_type } => {
                // Enhanced formatting for combat actions
                if action_type.contains("🗡️")
                    || action_type.contains("⚔️")
                    || action_type.contains("💥")
                    || action_type.contains("❤️")
                {
                    println!("⚔️ [{}] {}", timestamp, action_type);
                } else if action_type == "Attackers declared" {
                    println!("⚔️ [{}] Attackers declared", timestamp);
                } else if action_type.contains("New turn started") {
                    println!("🔄 [{}] New turn started", timestamp);
                } else {
                    println!("🎮 [{}] {}", timestamp, action_type);
                }
            }

            EnhancedPlayerEvent::PhaseChange { phase, step } => {
                if let Some(step_name) = step {
                    println!("🕐 [{}] {} Phase - {}", timestamp, phase, step_name);
                } else {
                    println!("🕐 [{}] {} Phase", timestamp, phase);
                }
            }

            EnhancedPlayerEvent::LifeChange { player, life_total } => {
                let default_name = format!("Player {}", player);
                let player_name = player_names
                    .get(player)
                    .map(|s| s.as_str())
                    .unwrap_or(&default_name);
                println!("❤️  [{}] {} life: {}", timestamp, player_name, life_total);
            }

            EnhancedPlayerEvent::Attackers { count } => {
                println!("⚔️  [{}] {} creature(s) attacking", timestamp, count);
            }

            EnhancedPlayerEvent::Blockers { count } => {
                println!("🛡️  [{}] {} creature(s) blocking", timestamp, count);
            }

            EnhancedPlayerEvent::CombatSequence {
                phase,
                step,
                attacking_creatures,
                damage_dealt,
                life_changes,
            } => {
                println!("⚔️ [{}] Combat: {} - {}", timestamp, phase, step);
                for creature in attacking_creatures {
                    println!("   🗡️ {} attacks", creature);
                }
                if let Some(damage) = damage_dealt {
                    println!("   💥 {} damage dealt", damage);
                }
                for (player, life) in life_changes {
                    let default_name = format!("Player {}", player);
                    let player_name = player_names
                        .get(player)
                        .map(|s| s.as_str())
                        .unwrap_or(&default_name);
                    println!("   ❤️ {} life: {}", player_name, life);
                }
            }

            EnhancedPlayerEvent::DeckInfo {
                name,
                format,
                card_count,
            } => {
                println!(
                    "🃏 [{}] Deck loaded: {} ({}) - {} cards",
                    timestamp, name, format, card_count
                );
            }

            EnhancedPlayerEvent::PriorityPass => {
                println!("⏭️  [{}] Priority passed", timestamp);
            }

            EnhancedPlayerEvent::TargetSelection { target_id } => {
                println!(
                    "🎯 [{}] Target selected: Instance ID {}",
                    timestamp, target_id
                );
            }

            EnhancedPlayerEvent::CounterChange {
                counter_type,
                amount,
            } => {
                println!(
                    "🔢 [{}] Counter added: {} {} counter(s)",
                    timestamp, amount, counter_type
                );
            }

            EnhancedPlayerEvent::ManaPaid => {
                println!("💎 [{}] Mana paid for spell/ability", timestamp);
            }

            EnhancedPlayerEvent::CardRevealed { count } => {
                println!("👁️  [{}] {} card(s) revealed", timestamp, count);
            }

            EnhancedPlayerEvent::CardDrawn => {
                println!("📤 [{}] Card drawn", timestamp);
            }

            EnhancedPlayerEvent::AbilityActivated => {
                println!("⚡ [{}] Ability activated", timestamp);
            }

            EnhancedPlayerEvent::PermanentTapped { tapped } => {
                if *tapped {
                    println!("⤵️ [{}] Permanent tapped", timestamp);
                } else {
                    println!("⤴️ [{}] Permanent untapped", timestamp);
                }
            }

            EnhancedPlayerEvent::SpellResolution { grp_id } => {
                println!(
                    "🔮 [{}] Spell/ability resolving (GRP ID: {})",
                    timestamp, grp_id
                );
            }

            EnhancedPlayerEvent::UIMessage | EnhancedPlayerEvent::GameEvent => {
                // Skip these for cleaner output
            }
        }
        Ok(())
    }

    // Display a compact match status summary
    #[allow(dead_code)]
    pub fn display_match_status(
        &self,
        player_names: &std::collections::HashMap<u64, String>,
        current_turn: Option<u64>,
        active_player: Option<u64>,
    ) -> Result<()> {
        let timestamp = Local::now().format("%H:%M:%S");

        if let (Some(turn), Some(active)) = (current_turn, active_player) {
            let default_name = format!("Player {}", active);
            let player_name = player_names
                .get(&active)
                .map(|s| s.as_str())
                .unwrap_or(&default_name);

            println!("\n📊 [{}] ═══ MATCH STATUS ═══", timestamp);
            println!("🔄 Turn: {} | Active: {}", turn, player_name);

            // Show all known players
            if !player_names.is_empty() {
                println!("👥 Players:");
                for (seat_id, name) in player_names {
                    let status = if *seat_id == active {
                        "🟢 Active"
                    } else {
                        "⚪ Waiting"
                    };
                    println!("   {} Player {}: {}", status, seat_id, name);
                }
            }
            println!("═══════════════════════════════════\n");
        }
        Ok(())
    }

    pub fn display_match_start(&self, match_state: &MatchState) -> Result<()> {
        println!("\n🎮 ═══════════════════════════════════════════");
        println!("🎮 MATCH STARTED!");
        println!("🎮 ═══════════════════════════════════════════");

        // Display player matchup
        if match_state.players.len() >= 2 {
            println!(
                "\n⚔️  {} vs {}",
                match_state.players[0].screen_name, match_state.players[1].screen_name
            );
        }

        println!("\nMatch ID: {}", match_state.match_id);

        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // Header
        table.set_header(vec!["Player", "Life", "Seat"]);

        for player in &match_state.players {
            let mut row = vec![
                Cell::new(&player.screen_name),
                Cell::new(player.life_total),
                Cell::new(player.seat_id),
            ];

            if self.use_colors {
                // Color code players
                let color = if player.seat_id == 1 {
                    Color::Blue
                } else {
                    Color::Red
                };
                row[0] = row[0].clone().fg(color);
            }

            table.add_row(row);
        }

        println!("{}", table);
        println!("\n🎯 Good luck, have fun!");
        println!("Match ID: {}", match_state.match_id);
        println!("Started: {}", match_state.started_at.format("%H:%M:%S"));
        self.print_separator("")?;

        Ok(())
    }

    pub fn display_turn_change(
        &self,
        turn: u32,
        active_player: usize,
        players: &[Player],
    ) -> Result<()> {
        let player_name = players
            .get(active_player)
            .map(|p| p.screen_name.as_str())
            .unwrap_or("Unknown");

        let header = format!("Turn {} - {}", turn, player_name);
        self.print_turn_separator(&header)?;

        Ok(())
    }

    pub fn display_life_change(
        &self,
        player_idx: usize,
        old_life: i32,
        new_life: i32,
        players: &[Player],
    ) -> Result<()> {
        let player_name = players
            .get(player_idx)
            .map(|p| p.screen_name.as_str())
            .unwrap_or("Unknown");

        let change = new_life - old_life;
        let change_str = if change > 0 {
            format!("+{}", change)
        } else {
            change.to_string()
        };

        let timestamp = chrono::Utc::now().format("%H:%M:%S");

        let mut message = format!(
            "[{}] {} life changed by {} ({} → {})",
            timestamp, player_name, change_str, old_life, new_life
        );

        if self.use_colors {
            if change > 0 {
                message = format!("\x1b[32m{}\x1b[0m", message); // Green for life gain
            } else if change < 0 {
                message = format!("\x1b[31m{}\x1b[0m", message); // Red for life loss
            }
        }

        println!("{}", message);
        Ok(())
    }

    pub fn display_game_action(&self, action: &GameAction, players: &[Player]) -> Result<()> {
        if !self.show_detailed_actions {
            return Ok(());
        }

        let player_name = players
            .get(action.player)
            .map(|p| p.screen_name.as_str())
            .unwrap_or("Unknown");

        let timestamp = action.timestamp.format("%H:%M:%S");

        let mut message = format!("[{}] {} {}", timestamp, player_name, action.description);

        if self.use_colors {
            let color = match action.action_type {
                ActionType::Play | ActionType::Cast => Color::Yellow,
                ActionType::Attack => Color::Red,
                ActionType::Block => Color::Blue,
                ActionType::Draw => Color::Green,
                ActionType::Discard => Color::Magenta,
                _ => Color::White,
            };

            // Apply color to the action description part
            let colored_desc = format!(
                "\x1b[{}m{}\x1b[0m",
                self.color_to_ansi(color),
                action.description
            );
            message = format!("[{}] {} {}", timestamp, player_name, colored_desc);
        }

        println!("{}", message);
        Ok(())
    }

    pub fn display_card_played(
        &self,
        player_idx: usize,
        card_name: &str,
        zone_from: &Zone,
        zone_to: &Zone,
        players: &[Player],
    ) -> Result<()> {
        let player_name = players
            .get(player_idx)
            .map(|p| p.screen_name.as_str())
            .unwrap_or("Unknown");

        let timestamp = chrono::Utc::now().format("%H:%M:%S");
        let zone_desc = self.format_zone_transition(zone_from, zone_to);

        let mut message = format!(
            "[{}] {} plays {} ({})",
            timestamp, player_name, card_name, zone_desc
        );

        if self.use_colors {
            message = format!("\x1b[33m{}\x1b[0m", message); // Yellow for card plays
        }

        println!("{}", message);
        Ok(())
    }

    pub fn display_mulligan(
        &self,
        player_idx: usize,
        from_size: usize,
        to_size: usize,
        players: &[Player],
    ) -> Result<()> {
        let player_name = players
            .get(player_idx)
            .map(|p| p.screen_name.as_str())
            .unwrap_or("Unknown");

        let timestamp = chrono::Utc::now().format("%H:%M:%S");

        let action = if to_size < from_size {
            "mulligans"
        } else {
            "keeps"
        };

        let mut message = format!(
            "[{}] {} {} to {} cards",
            timestamp, player_name, action, to_size
        );

        if self.use_colors {
            message = format!("\x1b[36m{}\x1b[0m", message); // Cyan for mulligans
        }

        println!("{}", message);
        Ok(())
    }

    pub fn display_match_end(&self, match_state: &MatchState) -> Result<()> {
        println!("\n🏁 ═══════════════════════════════════════════");
        println!("🏁 MATCH COMPLETE!");
        println!("🏁 ═══════════════════════════════════════════");

        if let Some(result) = &match_state.result {
            let winner_name = result
                .winner
                .and_then(|idx| match_state.players.get(idx))
                .map(|p| p.screen_name.as_str())
                .unwrap_or("Unknown");

            let loser_name = match_state
                .players
                .iter()
                .find(|p| Some(p.seat_id as usize) != result.winner)
                .map(|p| p.screen_name.as_str())
                .unwrap_or("Unknown");

            let duration = self.format_duration(result.duration_seconds);

            println!("\n🏆 {} defeats {}!", winner_name, loser_name);
            println!("📋 Victory by: {}", result.reason);
            println!("⏱️  Match duration: {}", duration);
            println!("🔢 Total turns: {}", match_state.current_turn);

            // Display final life totals
            println!("\n📊 Final Board State:");
            let mut table = Table::new();
            table.set_header(vec!["Player", "Final Life", "Status"]);

            for player in &match_state.players {
                let status = if Some(player.seat_id as usize) == result.winner {
                    "🏆 Winner"
                } else {
                    "❌ Defeated"
                };

                let mut row = vec![
                    Cell::new(&player.screen_name),
                    Cell::new(player.life_total),
                    Cell::new(status),
                ];

                if self.use_colors {
                    let color = if Some(player.seat_id as usize) == result.winner {
                        Color::Green
                    } else {
                        Color::Red
                    };
                    row[0] = row[0].clone().fg(color);
                    row[2] = row[2].clone().fg(color);
                }

                table.add_row(row);
            }

            println!("{}", table);

            // Show some match statistics if we have actions
            if !match_state.actions.is_empty() {
                println!("\n📈 Match Statistics:");
                let total_actions = match_state.actions.len();
                println!("   Total actions: {}", total_actions);

                // Count life changes
                let life_changes = match_state
                    .actions
                    .iter()
                    .filter(|a| matches!(a.action_type, ActionType::LifeChange))
                    .count();
                if life_changes > 0 {
                    println!("   Life changes: {}", life_changes);
                }
            }
        }

        println!("\n🎮 Thanks for playing!");
        self.print_separator("")?;
        Ok(())
    }

    fn print_separator(&self, title: &str) -> Result<()> {
        let width = 67;
        let separator = "═".repeat(width);

        if title.is_empty() {
            println!("{}", separator);
        } else {
            let padding = (width - title.len() - 2) / 2;
            let left_pad = " ".repeat(padding);
            let right_pad = " ".repeat(width - title.len() - 2 - padding);
            println!("{}", separator);
            println!("{}{}{}", left_pad, title, right_pad);
            println!("{}", separator);
        }

        io::stdout().flush()?;
        Ok(())
    }

    fn print_turn_separator(&self, title: &str) -> Result<()> {
        let width = 67;
        let separator = "─".repeat(width);

        println!("\n{}", title);
        println!("{}", separator);

        io::stdout().flush()?;
        Ok(())
    }

    fn format_zone_transition(&self, from: &Zone, to: &Zone) -> String {
        let from_str = self.zone_to_string(from);
        let to_str = self.zone_to_string(to);
        format!("{} → {}", from_str, to_str)
    }

    fn zone_to_string(&self, zone: &Zone) -> &str {
        match zone {
            Zone::Hand => "Hand",
            Zone::Library => "Library",
            Zone::Graveyard => "Graveyard",
            Zone::Battlefield => "Battlefield",
            Zone::Exile => "Exile",
            Zone::Stack => "Stack",
            Zone::Command => "Command",
            Zone::Unknown => "Unknown",
        }
    }

    fn format_duration(&self, seconds: u64) -> String {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        format!("{}:{:02}", minutes, remaining_seconds)
    }

    fn color_to_ansi(&self, color: Color) -> u8 {
        match color {
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            _ => 37,
        }
    }
}

impl Default for MatchDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn create_test_players() -> Vec<Player> {
        vec![
            Player {
                seat_id: 1,
                screen_name: "Alice".to_string(),
                life_total: 20,
                hand_size: 7,
                initial_hand: Vec::new(),
                mulligans: Vec::new(),
                deck_cards: Vec::new(),
            },
            Player {
                seat_id: 2,
                screen_name: "Bob".to_string(),
                life_total: 20,
                hand_size: 7,
                initial_hand: Vec::new(),
                mulligans: Vec::new(),
                deck_cards: Vec::new(),
            },
        ]
    }

    #[test]
    fn test_zone_transition_formatting() {
        let display = MatchDisplay::new();
        let result = display.format_zone_transition(&Zone::Hand, &Zone::Battlefield);
        assert_eq!(result, "Hand → Battlefield");
    }

    #[test]
    fn test_duration_formatting() {
        let display = MatchDisplay::new();
        assert_eq!(display.format_duration(65), "1:05");
        assert_eq!(display.format_duration(3661), "61:01");
        assert_eq!(display.format_duration(30), "0:30");
    }

    #[test]
    fn test_zone_to_string() {
        let display = MatchDisplay::new();
        assert_eq!(display.zone_to_string(&Zone::Hand), "Hand");
        assert_eq!(display.zone_to_string(&Zone::Battlefield), "Battlefield");
        assert_eq!(display.zone_to_string(&Zone::Unknown), "Unknown");
    }
}
