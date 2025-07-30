use super::events::EnhancedPlayerEvent;
use crate::prelude::*;
use mtg_core::{GameAction, MatchState, ParsedEvent};

pub struct EventDisplay {}

impl EventDisplay {
    pub fn new(_colors: bool) -> Self {
        Self {}
    }

    pub fn display_parsed_event(&self, event: &ParsedEvent) -> Result<()> {
        match event {
            ParsedEvent::UserAuthenticated {
                user_id,
                display_name,
            } => {
                println!("🔐 User authenticated: {} ({})", display_name, user_id);
            }

            ParsedEvent::MatchStarted(match_state) => {
                self.display_match_start(match_state)?;
            }

            ParsedEvent::MatchEnded(match_state) => {
                self.display_match_end(match_state)?;
            }

            ParsedEvent::TurnChange {
                turn,
                active_player,
            } => {
                println!("🔄 Turn {}: Player {} is active", turn, active_player);
            }

            ParsedEvent::LifeChange {
                player,
                old_life,
                new_life,
            } => {
                let change = *new_life as i32 - *old_life as i32;
                let change_str = if change > 0 {
                    format!("+{}", change)
                } else {
                    change.to_string()
                };
                println!(
                    "❤️  Player {} life: {} → {} ({})",
                    player, old_life, new_life, change_str
                );
            }

            ParsedEvent::GameAction(action) => {
                self.display_game_action(action)?;
            }

            ParsedEvent::CardPlayed {
                player,
                card_name,
                zone_from,
                zone_to,
            } => {
                println!(
                    "🃏 Player {} played {} ({} → {})",
                    player, card_name, zone_from, zone_to
                );
            }

            ParsedEvent::Mulligan {
                player,
                from_size,
                to_size,
            } => {
                println!(
                    "🔄 Player {} mulligan: {} → {} cards",
                    player, from_size, to_size
                );
            }

            ParsedEvent::DraftPack {
                pack_number,
                pick_number,
                cards,
            } => {
                println!(
                    "📦 Draft Pack {}.{}: {} cards available",
                    pack_number + 1,
                    pick_number + 1,
                    cards.len()
                );
            }

            ParsedEvent::DraftPick { card_id } => {
                println!("🎯 Draft Pick: Card ID {}", card_id);
            }

            ParsedEvent::DraftCompleted => {
                println!("✅ Draft completed!");
            }

            ParsedEvent::DeckSubmitted { deck_id, deck_name } => match (deck_id, deck_name) {
                (Some(id), Some(name)) => println!("🃏 Deck submitted: {} ({})", name, id),
                (Some(id), None) => println!("🃏 Deck submitted: {}", id),
                (None, Some(name)) => println!("🃏 Deck submitted: {}", name),
                (None, None) => println!("🃏 Deck submitted"),
            },
        }
        Ok(())
    }

    pub fn display_player_event(&self, event: &EnhancedPlayerEvent) -> Result<()> {
        match event {
            EnhancedPlayerEvent::MatchStarted { match_id, players } => {
                println!("🎮 Match started: {}", match_id);
                for (seat_id, name, platform) in players {
                    println!("   Player {}: {} ({})", seat_id, name, platform);
                }
            }

            EnhancedPlayerEvent::GameState {
                turn,
                active_player,
            } => {
                if let (Some(turn), Some(active)) = (turn, active_player) {
                    println!("🔄 Turn {}: Player {} is active", turn, active);
                }
            }

            EnhancedPlayerEvent::Mulligan { decision } => {
                // Enhanced mulligan display with emojis
                if decision.contains("🤲") || decision.contains("🔄") {
                    // Decision already has emoji
                    println!("{}", decision);
                } else {
                    let decision_text = match decision.as_str() {
                        "Kept hand" => "🤲 Kept hand",
                        "Mulliganed" => "🔄 Mulliganed",
                        "MulliganOption_AcceptHand" => "🤲 Kept hand",
                        "MulliganOption_Mulligan" => "🔄 Mulliganed",
                        _ => &format!("🤲 {}", decision),
                    };
                    println!("{}", decision_text);
                }
            }

            EnhancedPlayerEvent::CardPlayed { action, zones: _ } => {
                // Enhanced formatting for different card actions
                if action.contains("📥")
                    || action.contains("🏔️")
                    || action.contains("🎯")
                    || action.contains("🔄")
                {
                    // Action already has emoji, just print it
                    println!("{}", action);
                } else if action.contains("Played") && action.contains("→") {
                    println!("🏔️ {}", action);
                } else if action.contains("Drew a card") {
                    println!("📥 Drew a card");
                } else if action.contains("Cast") {
                    println!("🎯 Cast a spell");
                } else {
                    println!("🎴 {}", action);
                }
            }

            EnhancedPlayerEvent::SpellCast { mana_cost } => {
                println!("✨ Spell cast for mana cost: {}", mana_cost);
            }

            EnhancedPlayerEvent::TargetSelection { target_id } => {
                println!("🎯 Target selected: Instance ID {}", target_id);
            }

            EnhancedPlayerEvent::CounterChange {
                counter_type,
                amount,
            } => {
                println!("🔢 Counter added: {} {} counter(s)", amount, counter_type);
            }

            EnhancedPlayerEvent::ManaPaid => {
                println!("💎 Mana paid for spell/ability");
            }

            EnhancedPlayerEvent::CardRevealed { count } => {
                println!("👁️  {} card(s) revealed", count);
            }

            EnhancedPlayerEvent::CardDrawn => {
                println!("📤 Card drawn");
            }

            EnhancedPlayerEvent::AbilityActivated => {
                println!("⚡ Ability activated");
            }

            EnhancedPlayerEvent::PermanentTapped { tapped } => {
                if *tapped {
                    println!("🔄 Permanent tapped");
                } else {
                    println!("🔄 Permanent untapped");
                }
            }

            EnhancedPlayerEvent::ActionTaken { action_type } => {
                // Enhanced formatting for specific action types
                if action_type.contains("🗡️")
                    || action_type.contains("⚔️")
                    || action_type.contains("💥")
                    || action_type.contains("❤️")
                {
                    // Combat-related actions get special formatting
                    println!("{}", action_type);
                } else if action_type == "Attackers declared" {
                    println!("⚔️ Attackers declared");
                } else if action_type.contains("New turn started") {
                    println!("🔄 New turn started");
                } else {
                    println!("🎮 {}", action_type);
                }
            }

            EnhancedPlayerEvent::PhaseChange { phase, step } => {
                if let Some(step) = step {
                    println!("🕐 {} Phase - {}", phase, step);
                } else {
                    println!("🕐 {} Phase", phase);
                }
            }

            EnhancedPlayerEvent::LifeChange { player, life_total } => {
                println!("❤️  Player {} life: {}", player, life_total);
            }

            EnhancedPlayerEvent::Attackers { count } => {
                println!("⚔️  {} creature(s) attacking", count);
            }

            EnhancedPlayerEvent::Blockers { count } => {
                println!("🛡️  {} creature(s) blocking", count);
            }

            EnhancedPlayerEvent::SpellResolution { grp_id } => {
                println!("🔮 Spell/ability resolving (GRP ID: {})", grp_id);
            }

            EnhancedPlayerEvent::DeckInfo {
                name,
                format,
                card_count,
            } => {
                println!(
                    "🃏 Deck loaded: {} ({}) - {} cards",
                    name, format, card_count
                );
            }

            EnhancedPlayerEvent::PriorityPass => {
                println!("⏭️  Priority passed");
            }

            EnhancedPlayerEvent::UIMessage => {
                // Skip UI messages as they're not very interesting
            }

            EnhancedPlayerEvent::GameEvent => {
                // Generic game event, skip for now
            }

            EnhancedPlayerEvent::CombatSequence {
                phase,
                step,
                attacking_creatures,
                damage_dealt,
                life_changes,
            } => {
                println!("⚔️ Combat: {} - {}", phase, step);
                if !attacking_creatures.is_empty() {
                    for creature in attacking_creatures {
                        println!("   🗡️ {} attacks", creature);
                    }
                }
                if let Some(damage) = damage_dealt {
                    println!("   💥 {} damage dealt", damage);
                }
                for (player, life) in life_changes {
                    println!("   ❤️ Player {} life: {}", player, life);
                }
            }
        }
        Ok(())
    }

    fn display_match_start(&self, match_state: &MatchState) -> Result<()> {
        println!("🎮 Match Started");
        if let Some(match_id) = &match_state.match_id {
            println!("   Match ID: {}", match_id);
        }
        if !match_state.format.is_empty() {
            println!("   Format: {}", match_state.format);
        }
        if !match_state.players.is_empty() {
            println!("   Players:");
            for player in &match_state.players {
                println!(
                    "     {} (Seat {}): {} life",
                    player.screen_name, player.seat_id, player.life_total
                );
            }
        }
        Ok(())
    }

    fn display_match_end(&self, match_state: &MatchState) -> Result<()> {
        println!("🏁 Match Ended");
        if let Some(winner) = match_state.winner {
            println!("   Winner: Player {}", winner);
        }
        if !match_state.players.is_empty() {
            println!("   Final State:");
            for player in &match_state.players {
                println!(
                    "     {} (Seat {}): {} life",
                    player.screen_name, player.seat_id, player.life_total
                );
            }
        }
        Ok(())
    }

    fn display_game_action(&self, action: &GameAction) -> Result<()> {
        println!(
            "⚡ Player {} {}: {}",
            action.player, action.action_type, action.description
        );
        if let Some(target) = &action.target {
            println!("   Target: {}", target);
        }
        if let Some(cost) = &action.cost {
            println!("   Cost: {}", cost);
        }
        Ok(())
    }

    pub fn display_summary(
        &self,
        main_events: &[ParsedEvent],
        player_events: &[EnhancedPlayerEvent],
        total_lines: usize,
    ) -> Result<()> {
        println!("\n📊 Parse Summary");
        println!("   Total lines processed: {}", total_lines);
        println!("   Main log events found: {}", main_events.len());
        println!("   Player log events found: {}", player_events.len());

        if !main_events.is_empty() || !player_events.is_empty() {
            println!("\n📋 Event Breakdown:");

            // Count main event types
            let mut main_counts = std::collections::HashMap::new();
            for event in main_events {
                let event_type = match event {
                    ParsedEvent::UserAuthenticated { .. } => "Authentication",
                    ParsedEvent::MatchStarted(_) => "Match Start",
                    ParsedEvent::MatchEnded(_) => "Match End",
                    ParsedEvent::TurnChange { .. } => "Turn Change",
                    ParsedEvent::LifeChange { .. } => "Life Change",
                    ParsedEvent::GameAction(_) => "Game Action",
                    ParsedEvent::CardPlayed { .. } => "Card Played",
                    ParsedEvent::Mulligan { .. } => "Mulligan",
                    ParsedEvent::DraftPack { .. } => "Draft Pack",
                    ParsedEvent::DraftPick { .. } => "Draft Pick",
                    ParsedEvent::DraftCompleted => "Draft Complete",
                    ParsedEvent::DeckSubmitted { .. } => "Deck Submitted",
                };
                *main_counts.entry(event_type).or_insert(0) += 1;
            }

            for (event_type, count) in main_counts {
                println!("   {}: {}", event_type, count);
            }

            // Count player event types
            let mut player_counts = std::collections::HashMap::new();
            for event in player_events {
                let event_type = match event {
                    EnhancedPlayerEvent::MatchStarted { .. } => "Match Started",
                    EnhancedPlayerEvent::GameState { .. } => "Game State",
                    EnhancedPlayerEvent::Mulligan { .. } => "Mulligan",
                    EnhancedPlayerEvent::CardPlayed { .. } => "Card Played",
                    EnhancedPlayerEvent::SpellCast { .. } => "Spell Cast",
                    EnhancedPlayerEvent::TargetSelection { .. } => "Target Selection",
                    EnhancedPlayerEvent::CounterChange { .. } => "Counter Change",
                    EnhancedPlayerEvent::ManaPaid => "Mana Paid",
                    EnhancedPlayerEvent::CardRevealed { .. } => "Card Revealed",
                    EnhancedPlayerEvent::CardDrawn => "Card Drawn",
                    EnhancedPlayerEvent::AbilityActivated => "Ability Activated",
                    EnhancedPlayerEvent::PermanentTapped { .. } => "Permanent Tapped",
                    EnhancedPlayerEvent::ActionTaken { .. } => "Action Taken",
                    EnhancedPlayerEvent::PhaseChange { .. } => "Phase Change",
                    EnhancedPlayerEvent::LifeChange { .. } => "Life Change",
                    EnhancedPlayerEvent::Attackers { .. } => "Attackers",
                    EnhancedPlayerEvent::Blockers { .. } => "Blockers",
                    EnhancedPlayerEvent::SpellResolution { .. } => "Spell Resolution",
                    EnhancedPlayerEvent::DeckInfo { .. } => "Deck Info",
                    EnhancedPlayerEvent::PriorityPass => "Priority Pass",
                    EnhancedPlayerEvent::UIMessage => "UI Message",
                    EnhancedPlayerEvent::GameEvent => "Game Event",
                    EnhancedPlayerEvent::CombatSequence { .. } => "Combat Sequence",
                };
                *player_counts.entry(event_type).or_insert(0) += 1;
            }

            for (event_type, count) in player_counts {
                println!("   Player {}: {}", event_type, count);
            }
        }

        Ok(())
    }
}
