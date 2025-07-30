use super::display::MatchDisplay;
use super::types::RawLogEvent;
use crate::companion::parse::events::{parse_player_json_line, EnhancedPlayerEvent};
use crate::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

pub struct PlayerEventParser {
    // Track player names and seat mappings
    player_names: HashMap<u64, String>,
    // Track current match ID
    current_match_id: Option<String>,
    // Track game state for context
    current_turn: Option<u64>,
    active_player: Option<u64>,
    // Enhanced display for better formatting
    display: MatchDisplay,
}

impl PlayerEventParser {
    pub fn new() -> Self {
        Self {
            player_names: HashMap::new(),
            current_match_id: None,
            current_turn: None,
            active_player: None,
            display: MatchDisplay::new()
                .with_colors(true)
                .with_detailed_actions(true),
        }
    }

    pub fn parse_player_event(&mut self, event: RawLogEvent) -> Result<()> {
        // Use the enhanced parsing logic from parse_player_json_line
        if let Ok(Some(enhanced_event)) = parse_player_json_line(&event.raw_data) {
            self.display_enhanced_event(&enhanced_event)?;
        } else {
            // Fallback to legacy parsing for compatibility
            match event.event_name.as_str() {
                "PlayerMatchStarted" => self.handle_match_started(&event.raw_data)?,
                "PlayerGameState" => self.handle_game_state(&event.raw_data)?,
                "PlayerMulligan" => self.handle_mulligan(&event.raw_data)?,
                "PlayerCardPlayed" => self.handle_card_played(&event.raw_data)?,
                "PlayerSpellCast" => self.handle_spell_cast(&event.raw_data)?,
                "PlayerTargetSelection" => self.handle_target_selection(&event.raw_data)?,
                "PlayerCounterChange" => self.handle_counter_change(&event.raw_data)?,
                "PlayerManaPaid" => self.handle_mana_paid(&event.raw_data)?,
                "PlayerCardRevealed" => self.handle_card_revealed(&event.raw_data)?,
                "PlayerCardDrawn" => self.handle_card_drawn(&event.raw_data)?,
                "PlayerAbilityActivated" => self.handle_ability_activated(&event.raw_data)?,
                "PlayerPermanentTapped" => self.handle_permanent_tapped(&event.raw_data)?,
                "PlayerActionTaken" => self.handle_action_taken(&event.raw_data)?,
                "PlayerPhaseChange" => self.handle_phase_change(&event.raw_data)?,
                "PlayerLifeChange" => self.handle_life_change(&event.raw_data)?,
                "PlayerAttackers" => self.handle_attackers(&event.raw_data)?,
                "PlayerBlockers" => self.handle_blockers(&event.raw_data)?,
                "PlayerSpellResolution" => self.handle_spell_resolution(&event.raw_data)?,
                "PlayerDeckInfo" => self.handle_deck_info(&event.raw_data)?,
                "PlayerPriorityPass" => self.handle_priority_pass(&event.raw_data)?,
                "PlayerUIMessage" => self.handle_ui_message(&event.raw_data)?,
                "PlayerGameEvent" => self.handle_game_event(&event.raw_data)?,
                _ => {
                    // Unknown event type, skip silently
                }
            }
        }
        Ok(())
    }

    // Enhanced event display using the new rich parsing
    fn display_enhanced_event(&mut self, event: &EnhancedPlayerEvent) -> Result<()> {
        // Update player names for match started events
        if let EnhancedPlayerEvent::MatchStarted { players, .. } = event {
            for (seat_id, name, _) in players {
                self.player_names.insert(*seat_id, name.clone());
            }
        }

        // Update turn tracking for game state events
        if let EnhancedPlayerEvent::GameState {
            turn,
            active_player,
        } = event
        {
            if let (Some(turn_num), Some(active)) = (turn, active_player) {
                if self.current_turn != Some(*turn_num) {
                    self.current_turn = Some(*turn_num);
                    self.active_player = Some(*active);
                }
            }
        }

        // Use the enhanced display with timestamps
        self.display
            .display_enhanced_event(event, &self.player_names)?;
        Ok(())
    }

    // Handle match room state changed event - provides player info and match ID
    fn handle_match_started(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(match_event) = json.get("matchGameRoomStateChangedEvent") {
                if let Some(game_room_info) = match_event.get("gameRoomInfo") {
                    if let Some(config) = game_room_info.get("gameRoomConfig") {
                        // Extract match ID
                        if let Some(match_id) = config.get("matchId").and_then(|id| id.as_str()) {
                            self.current_match_id = Some(match_id.to_string());
                            println!("🎮 Match started: {}", match_id);
                        }

                        // Extract player information
                        if let Some(players) =
                            config.get("reservedPlayers").and_then(|p| p.as_array())
                        {
                            for player in players {
                                if let (Some(seat_id), Some(name)) = (
                                    player.get("systemSeatId").and_then(|s| s.as_u64()),
                                    player.get("playerName").and_then(|n| n.as_str()),
                                ) {
                                    self.player_names.insert(seat_id, name.to_string());
                                    if let Some(platform) =
                                        player.get("platformId").and_then(|p| p.as_str())
                                    {
                                        println!("👤 Player {}: {} ({})", seat_id, name, platform);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Handle initial game state with hand and library info
    fn handle_game_state(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(gre_event) = json.get("greToClientEvent") {
                if let Some(messages) = gre_event
                    .get("greToClientMessages")
                    .and_then(|m| m.as_array())
                {
                    for message in messages {
                        if let Some(msg_type) = message.get("type").and_then(|t| t.as_str()) {
                            if msg_type == "GREMessageType_GameStateMessage" {
                                if let Some(game_state) = message.get("gameStateMessage") {
                                    // Handle turn info
                                    if let Some(turn_info) = game_state.get("turnInfo") {
                                        if let Some(turn_num) =
                                            turn_info.get("turnNumber").and_then(|t| t.as_u64())
                                        {
                                            if self.current_turn != Some(turn_num) {
                                                self.current_turn = Some(turn_num);
                                                if let Some(active) = turn_info
                                                    .get("activePlayer")
                                                    .and_then(|a| a.as_u64())
                                                {
                                                    self.active_player = Some(active);
                                                    let player_name = self
                                                        .player_names
                                                        .get(&active)
                                                        .map(|s| s.as_str())
                                                        .unwrap_or("Unknown");
                                                    println!(
                                                        "🔄 Turn {}: {} is active",
                                                        turn_num, player_name
                                                    );
                                                }
                                            }
                                        }
                                    }

                                    // Handle player life totals
                                    if let Some(players) =
                                        game_state.get("players").and_then(|p| p.as_array())
                                    {
                                        for player in players {
                                            if let (Some(seat), Some(life)) = (
                                                player
                                                    .get("systemSeatNumber")
                                                    .and_then(|s| s.as_u64()),
                                                player.get("lifeTotal").and_then(|l| l.as_u64()),
                                            ) {
                                                let player_name = self
                                                    .player_names
                                                    .get(&seat)
                                                    .map(|s| s.as_str())
                                                    .unwrap_or("Unknown");

                                                // Check for mulligan pending
                                                if let Some(pending) = player
                                                    .get("pendingMessageType")
                                                    .and_then(|p| p.as_str())
                                                {
                                                    if pending == "ClientMessageType_MulliganResp" {
                                                        println!(
                                                            "🤔 {} is deciding on mulligan",
                                                            player_name
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Handle zones (hand, library, battlefield)
                                    if let Some(zones) =
                                        game_state.get("zones").and_then(|z| z.as_array())
                                    {
                                        for zone in zones {
                                            if let (Some(zone_type), Some(owner_seat)) = (
                                                zone.get("type").and_then(|t| t.as_str()),
                                                zone.get("ownerSeatId").and_then(|o| o.as_u64()),
                                            ) {
                                                if let Some(object_ids) = zone
                                                    .get("objectInstanceIds")
                                                    .and_then(|ids| ids.as_array())
                                                {
                                                    let player_name = self
                                                        .player_names
                                                        .get(&owner_seat)
                                                        .map(|s| s.as_str())
                                                        .unwrap_or("Unknown");

                                                    match zone_type {
                                                        "ZoneType_Hand" => {
                                                            println!(
                                                                "🃏 {} has {} cards in hand",
                                                                player_name,
                                                                object_ids.len()
                                                            );
                                                        }
                                                        "ZoneType_Library" => {
                                                            println!(
                                                                "📚 {} has {} cards in library",
                                                                player_name,
                                                                object_ids.len()
                                                            );
                                                        }
                                                        "ZoneType_Battlefield" => {
                                                            if !object_ids.is_empty() {
                                                                println!("⚔️  {} has {} permanents on battlefield", player_name, object_ids.len());
                                                            }
                                                        }
                                                        _ => {}
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
            }
        }
        Ok(())
    }

    // Handle mulligan decisions
    fn handle_mulligan(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(mulligan_resp) = json.get("mulliganResp") {
                if let Some(decision) = mulligan_resp.get("decision").and_then(|d| d.as_str()) {
                    let decision_text = match decision {
                        "MulliganOption_AcceptHand" => "kept their hand",
                        "MulliganOption_Mulligan" => "mulliganed",
                        _ => "made a decision",
                    };
                    println!("🤲 Player {}", decision_text);
                }
            }
        }
        Ok(())
    }

    // Handle card played/zone transfers
    fn handle_card_played(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_ZoneTransfer"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                let mut zone_src = None;
                                let mut zone_dest = None;
                                let mut category = None;

                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        match key {
                                            "zone_src" => {
                                                zone_src = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            "zone_dest" => {
                                                zone_dest = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            "category" => {
                                                category = detail
                                                    .get("valueString")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_str());
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                if let (Some(src), Some(dest), Some(cat)) =
                                    (zone_src, zone_dest, category)
                                {
                                    let action = match cat {
                                        "PlayLand" => "played a land",
                                        "CastSpell" => "cast a spell",
                                        _ => "moved a card",
                                    };
                                    println!("🎴 Player {} (zone {} → {})", action, src, dest);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Handle spell casting with mana cost
    fn handle_spell_cast(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(perform_action) = json.get("performActionResp") {
                if let Some(actions) = perform_action.get("actions").and_then(|a| a.as_array()) {
                    for action in actions {
                        if let Some(action_type) = action.get("actionType").and_then(|t| t.as_str())
                        {
                            if action_type == "ActionType_Cast" {
                                if let Some(mana_cost) =
                                    action.get("manaCost").and_then(|m| m.as_array())
                                {
                                    let mut cost_description = Vec::new();
                                    for cost in mana_cost {
                                        if let (Some(colors), Some(count)) = (
                                            cost.get("color").and_then(|c| c.as_array()),
                                            cost.get("count").and_then(|c| c.as_u64()),
                                        ) {
                                            for color in colors {
                                                if let Some(color_str) = color.as_str() {
                                                    let color_symbol = match color_str {
                                                        "ManaColor_White" => "W",
                                                        "ManaColor_Blue" => "U",
                                                        "ManaColor_Black" => "B",
                                                        "ManaColor_Red" => "R",
                                                        "ManaColor_Green" => "G",
                                                        "ManaColor_Generic" => &count.to_string(),
                                                        _ => "?",
                                                    };
                                                    cost_description
                                                        .push(format!("{}{}", count, color_symbol));
                                                }
                                            }
                                        }
                                    }
                                    if !cost_description.is_empty() {
                                        println!(
                                            "✨ Spell cast for mana cost: {}",
                                            cost_description.join("")
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_target_selection(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(select_targets) = json.get("selectTargetsResp") {
                if let Some(target) = select_targets.get("target") {
                    if let Some(targets) = target.get("targets").and_then(|t| t.as_array()) {
                        for target_obj in targets {
                            if let Some(target_id) = target_obj
                                .get("targetInstanceId")
                                .and_then(|id| id.as_u64())
                            {
                                println!("🎯 Target selected: Instance ID {}", target_id);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_counter_change(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_CounterAdded"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                let mut counter_type = None;
                                let mut amount = None;

                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        match key {
                                            "counter_type" => {
                                                counter_type = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            "transaction_amount" => {
                                                amount = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                if let (Some(counter_type), Some(amount)) = (counter_type, amount) {
                                    let counter_name = match counter_type {
                                        7 => "loyalty",
                                        1 => "+1/+1",
                                        2 => "-1/-1",
                                        _ => "unknown",
                                    };
                                    println!(
                                        "🔢 Counter added: {} {} counter(s)",
                                        amount, counter_name
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_mana_paid(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_ManaPaid"))
                        {
                            println!("💎 Mana paid for spell/ability");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_card_revealed(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(zones) = json.get("zones").and_then(|z| z.as_array()) {
                for zone in zones {
                    if let Some(zone_type) = zone.get("type").and_then(|t| t.as_str()) {
                        if zone_type == "ZoneType_Revealed" {
                            if let Some(object_ids) =
                                zone.get("objectInstanceIds").and_then(|ids| ids.as_array())
                            {
                                println!("👁️  {} card(s) revealed", object_ids.len());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_card_drawn(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_CardDrawn"))
                        {
                            println!("📤 Card drawn");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_ability_activated(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_AbilityInstanceCreated"))
                        {
                            println!("⚡ Ability activated");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_permanent_tapped(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_TappedUntappedPermanent"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        if key == "tapped" {
                                            if let Some(tapped) = detail
                                                .get("valueInt32")
                                                .and_then(|v| v.as_array())
                                                .and_then(|arr| arr.first())
                                                .and_then(|v| v.as_u64())
                                            {
                                                if tapped == 1 {
                                                    println!("🔄 Permanent tapped");
                                                } else {
                                                    println!("🔄 Permanent untapped");
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
        }
        Ok(())
    }

    fn handle_action_taken(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_UserActionTaken"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        if key == "actionType" {
                                            if let Some(action_type) = detail
                                                .get("valueInt32")
                                                .and_then(|v| v.as_array())
                                                .and_then(|arr| arr.first())
                                                .and_then(|v| v.as_u64())
                                            {
                                                let action_name = match action_type {
                                                    2 => "ability activation",
                                                    4 => "mana ability",
                                                    _ => "action",
                                                };
                                                println!("🎮 Player performed {}", action_name);
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
        Ok(())
    }

    // Handle attackers declaration
    fn handle_attackers(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(declare_attackers) = json.get("declareAttackersResp") {
                if let Some(auto_declare) = declare_attackers
                    .get("autoDeclare")
                    .and_then(|a| a.as_bool())
                {
                    if auto_declare {
                        if let Some(damage_recipient) =
                            declare_attackers.get("autoDeclareDamageRecipient")
                        {
                            if let Some(recipient_type) =
                                damage_recipient.get("type").and_then(|t| t.as_str())
                            {
                                if recipient_type == "DamageRecType_Player" {
                                    if let Some(target_seat) = damage_recipient
                                        .get("playerSystemSeatId")
                                        .and_then(|s| s.as_u64())
                                    {
                                        let target_name = self
                                            .player_names
                                            .get(&target_seat)
                                            .map(|s| s.as_str())
                                            .unwrap_or("Unknown");
                                        println!(
                                            "⚔️  Attackers declared targeting {}",
                                            target_name
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Also check for attacking creatures in game state
            if let Some(game_objects) = json.get("gameObjects").and_then(|g| g.as_array()) {
                let mut attacking_creatures = 0;
                for obj in game_objects {
                    if let Some(attack_state) = obj.get("attackState").and_then(|a| a.as_str()) {
                        if attack_state == "AttackState_Attacking" {
                            attacking_creatures += 1;
                        }
                    }
                }
                if attacking_creatures > 0 {
                    println!("⚔️  {} creature(s) attacking", attacking_creatures);
                }
            }
        }
        Ok(())
    }

    // Handle blockers declaration
    fn handle_blockers(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(game_objects) = json.get("gameObjects").and_then(|g| g.as_array()) {
                let mut blocking_creatures = 0;
                for obj in game_objects {
                    if let Some(block_state) = obj.get("blockState").and_then(|b| b.as_str()) {
                        if block_state == "BlockState_Blocking" {
                            blocking_creatures += 1;
                        }
                    }
                }
                if blocking_creatures > 0 {
                    println!("🛡️  {} creature(s) blocking", blocking_creatures);
                }
            }
        }
        Ok(())
    }

    // Handle spell/ability resolution
    fn handle_spell_resolution(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_ResolutionStart"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        if key == "grpid" {
                                            if let Some(grp_id) = detail
                                                .get("valueInt32")
                                                .and_then(|v| v.as_array())
                                                .and_then(|arr| arr.first())
                                                .and_then(|v| v.as_u64())
                                            {
                                                println!(
                                                    "🔮 Spell/ability resolving (GRP ID: {})",
                                                    grp_id
                                                );
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
        Ok(())
    }

    // Handle deck information
    fn handle_deck_info(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(courses) = json.get("Courses").and_then(|c| c.as_array()) {
                for course in courses {
                    if let Some(deck_summary) = course.get("CourseDeckSummary") {
                        if let (Some(deck_name), Some(format)) = (
                            deck_summary.get("Name").and_then(|n| n.as_str()),
                            deck_summary
                                .get("Attributes")
                                .and_then(|a| a.as_array())
                                .and_then(|attrs| {
                                    attrs.iter().find(|attr| {
                                        attr.get("name").and_then(|n| n.as_str()) == Some("Format")
                                    })
                                })
                                .and_then(|attr| attr.get("value").and_then(|v| v.as_str())),
                        ) {
                            println!("🃏 Deck loaded: {} ({})", deck_name, format);
                        }
                    }

                    if let Some(deck) = course.get("CourseDeck") {
                        if let Some(main_deck) = deck.get("MainDeck").and_then(|m| m.as_array()) {
                            let total_cards: u64 = main_deck
                                .iter()
                                .filter_map(|card| card.get("quantity").and_then(|q| q.as_u64()))
                                .sum();
                            println!("📊 Main deck: {} cards", total_cards);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Handle priority passing
    fn handle_priority_pass(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(perform_action) = json.get("performActionResp") {
                if let Some(actions) = perform_action.get("actions").and_then(|a| a.as_array()) {
                    for action in actions {
                        if let Some(action_type) = action.get("actionType").and_then(|t| t.as_str())
                        {
                            if action_type == "ActionType_Pass" {
                                println!("⏭️  Priority passed");
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_phase_change(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(annotations) = json.get("annotations").and_then(|a| a.as_array()) {
                for annotation in annotations {
                    if let Some(types) = annotation.get("type").and_then(|t| t.as_array()) {
                        if types
                            .iter()
                            .any(|t| t.as_str() == Some("AnnotationType_PhaseOrStepModified"))
                        {
                            if let Some(details) =
                                annotation.get("details").and_then(|d| d.as_array())
                            {
                                let mut phase = None;
                                let mut step = None;

                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        match key {
                                            "phase" => {
                                                phase = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            "step" => {
                                                step = detail
                                                    .get("valueInt32")
                                                    .and_then(|v| v.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|v| v.as_u64());
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                if let (Some(phase), Some(step)) = (phase, step) {
                                    let phase_name = match phase {
                                        1 => "Beginning",
                                        2 => "Main",
                                        3 => "Combat",
                                        4 => "Main 2",
                                        5 => "End",
                                        _ => "Unknown",
                                    };
                                    let step_name = match step {
                                        1 => "Upkeep",
                                        2 => "Draw",
                                        3 => "Begin Combat",
                                        4 => "Declare Attackers",
                                        5 => "Declare Blockers",
                                        6 => "Combat Damage",
                                        7 => "End Combat",
                                        8 => "End Step",
                                        9 => "Cleanup",
                                        _ => "",
                                    };
                                    if !step_name.is_empty() {
                                        println!("🕐 {} Phase - {}", phase_name, step_name);
                                    } else {
                                        println!("🕐 {} Phase", phase_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_life_change(&mut self, raw_data: &str) -> Result<()> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(players) = json.get("players").and_then(|p| p.as_array()) {
                for player in players {
                    if let Some(life_total) = player.get("lifeTotal").and_then(|l| l.as_u64()) {
                        if let Some(seat_number) =
                            player.get("systemSeatNumber").and_then(|s| s.as_u64())
                        {
                            println!("❤️  Player {} life: {}", seat_number, life_total);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_ui_message(&mut self, _raw_data: &str) -> Result<()> {
        // UI messages are usually not very interesting for gameplay tracking
        // Skip for now to avoid spam
        Ok(())
    }

    fn handle_game_event(&mut self, raw_data: &str) -> Result<()> {
        // This is a catch-all for other GreToClientEvent messages
        // We could parse more specific events here if needed
        if raw_data.contains("GREMessageType_GameStateMessage") {
            // Already handled by other specific handlers
        }
        Ok(())
    }
}
