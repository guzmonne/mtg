use super::types::*;
// use crate::cache::{CacheStore, DiskCacheBuilder};
use color_eyre::Result;
use serde_json::Value;
use std::collections::HashMap;

pub struct EventParser {
    current_match: Option<MatchState>,
    current_draft: Option<DraftState>,
    current_user: Option<(String, String)>, // (user_id, display_name)
    last_game_state: Option<Value>,         // Store the last game state for comparison
    game_objects: HashMap<u32, GameObjectInfo>, // Track game objects by instance ID
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct GameObjectInfo {
    grp_id: u32,
    owner: u32,
    zone: String,
    card_name: Option<String>,
}

impl Default for EventParser {
    fn default() -> Self {
        Self::new()
    }
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            current_match: None,
            current_draft: None,
            current_user: None,
            last_game_state: None,
            game_objects: HashMap::new(),
        }
    }

    pub fn current_match(&self) -> Option<&MatchState> {
        self.current_match.as_ref()
    }

    fn get_player_name(&self, seat_id: u32) -> String {
        if let Some(ref match_state) = self.current_match {
            if let Some(player) = match_state.players.iter().find(|p| p.seat_id == seat_id) {
                return player.screen_name.clone();
            }
        }

        // Check if this is the current user
        if let Some((_, ref display_name)) = self.current_user {
            if seat_id == 1 {
                return display_name.clone();
            }
        }

        format!("Player {seat_id}")
    }

    pub fn parse_event(&mut self, event: RawLogEvent) -> Result<Option<ParsedEvent>> {
        match event.event_name.as_str() {
            "StateChanged" => self.handle_state_changed(&event.raw_data),
            "GreToClientEvent" => self.handle_gre_to_client_event(&event.raw_data),
            "ClientToGREMessage" => self.handle_client_to_gre_message(&event.raw_data),
            "ClientToMatchServiceMessage" => self.handle_client_to_match_service(&event.raw_data),
            "MatchServiceToClientMessage" => self.handle_match_service_to_client(&event.raw_data),
            "DraftPack" => self.handle_draft_pack(&event.raw_data),
            "DraftPick" => self.handle_draft_pick(&event.raw_data),
            "DraftCompleted" => Ok(Some(ParsedEvent::DraftCompleted)),
            "UserAuthenticated" => self.handle_user_authenticated(&event.raw_data),
            _ => Ok(None), // Unknown event type
        }
    }

    fn handle_state_changed(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let (Some(old_state), Some(new_state)) = (
                json.get("old").and_then(|v| v.as_str()),
                json.get("new").and_then(|v| v.as_str()),
            ) {
                match (old_state, new_state) {
                    ("Playing", "MatchCompleted") => {
                        if let Some(ref mut match_state) = self.current_match {
                            match_state.ended_at = Some(chrono::Utc::now());
                            return Ok(Some(ParsedEvent::MatchEnded(match_state.clone())));
                        }
                    }
                    (_, "Playing") => {
                        // Match started
                        let match_state = MatchState {
                            started_at: Some(chrono::Utc::now()),
                            ..Default::default()
                        };
                        self.current_match = Some(match_state.clone());
                        return Ok(Some(ParsedEvent::MatchStarted(match_state)));
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    fn handle_gre_to_client_event(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(gre_event) = json.get("greToClientEvent") {
                if let Some(messages) = gre_event
                    .get("greToClientMessages")
                    .and_then(|m| m.as_array())
                {
                    for message in messages {
                        if let Some(msg_type) = message.get("type").and_then(|t| t.as_str()) {
                            match msg_type {
                                "GREMessageType_GameStateMessage" => {
                                    return self.handle_game_state_message(message);
                                }
                                "GREMessageType_TimerStateMessage" => {
                                    // Handle timer updates if needed
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_game_state_message(&mut self, message: &Value) -> Result<Option<ParsedEvent>> {
        if let Some(game_state) = message.get("gameStateMessage") {
            // Store current game state for comparison
            self.last_game_state = Some(game_state.clone());

            // Check for turn changes
            if let Some(turn_info) = game_state.get("turnInfo") {
                if let (Some(turn_num), Some(active_player)) = (
                    turn_info.get("turnNumber").and_then(|t| t.as_u64()),
                    turn_info.get("activePlayer").and_then(|p| p.as_u64()),
                ) {
                    if let Some(ref mut match_state) = self.current_match {
                        if match_state.turn_number != turn_num as u32 {
                            match_state.turn_number = turn_num as u32;
                            match_state.active_player = active_player as u32;

                            if let Some(phase) = turn_info.get("phase").and_then(|p| p.as_str()) {
                                match_state.phase = phase.to_string();
                            }
                            if let Some(step) = turn_info.get("step").and_then(|s| s.as_str()) {
                                match_state.step = step.to_string();
                            }

                            return Ok(Some(ParsedEvent::TurnChange {
                                turn: turn_num as u32,
                                active_player: active_player as u32,
                            }));
                        }
                    }
                }
            }

            // Check for life changes
            if let Some(players) = game_state.get("players").and_then(|p| p.as_array()) {
                for player in players {
                    if let (Some(seat_id), Some(life_total)) = (
                        player.get("systemSeatNumber").and_then(|s| s.as_u64()),
                        player.get("lifeTotal").and_then(|l| l.as_u64()),
                    ) {
                        // Check if we need to add a new player first
                        let needs_new_player = if let Some(ref match_state) = self.current_match {
                            !match_state
                                .players
                                .iter()
                                .any(|p| p.seat_id == seat_id as u32)
                        } else {
                            false
                        };

                        if needs_new_player {
                            let screen_name = self.get_player_name(seat_id as u32);
                            let player_info = PlayerInfo {
                                seat_id: seat_id as u32,
                                life_total: life_total as u32,
                                starting_life: life_total as u32,
                                screen_name,
                                ..Default::default()
                            };

                            if let Some(ref mut match_state) = self.current_match {
                                match_state.players.push(player_info);
                            }
                        } else if let Some(ref mut match_state) = self.current_match {
                            if let Some(player_info) = match_state
                                .players
                                .iter_mut()
                                .find(|p| p.seat_id == seat_id as u32)
                            {
                                if player_info.life_total != life_total as u32 {
                                    let old_life = player_info.life_total;
                                    player_info.life_total = life_total as u32;

                                    return Ok(Some(ParsedEvent::LifeChange {
                                        player: seat_id as u32,
                                        old_life,
                                        new_life: life_total as u32,
                                    }));
                                }
                            }
                        }
                    }
                }
            }

            // Check for game objects and card plays
            if let Some(game_objects) = game_state.get("gameObjects").and_then(|g| g.as_array()) {
                for obj in game_objects {
                    if let Some(instance_id) = obj.get("instanceId").and_then(|i| i.as_u64()) {
                        let instance_id = instance_id as u32;

                        // Track game object
                        if let Some(grp_id) = obj.get("grpId").and_then(|g| g.as_u64()) {
                            let game_obj = GameObjectInfo {
                                grp_id: grp_id as u32,
                                owner: obj.get("ownerSeatId").and_then(|o| o.as_u64()).unwrap_or(0)
                                    as u32,
                                zone: obj
                                    .get("zoneId")
                                    .and_then(|z| z.as_u64())
                                    .map(|z| zone_to_string(z as u32))
                                    .unwrap_or_default(),
                                card_name: None, // Would need card database lookup
                            };
                            self.game_objects.insert(instance_id, game_obj);
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn handle_client_to_gre_message(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(payload) = json.get("payload") {
                if let Some("ClientMessageType_PerformActionResp") =
                    payload.get("type").and_then(|t| t.as_str())
                {
                    // Handle player actions
                    if let Some(actions) = payload
                        .get("performActionResp")
                        .and_then(|r| r.get("actions"))
                        .and_then(|a| a.as_array())
                    {
                        for action in actions {
                            if let Some(action_type) =
                                action.get("actionType").and_then(|t| t.as_str())
                            {
                                let game_action = GameAction {
                                    player: payload
                                        .get("systemSeatId")
                                        .and_then(|s| s.as_u64())
                                        .unwrap_or(0)
                                        as u32,
                                    action_type: action_type.to_string(),
                                    target: None,
                                    cost: None,
                                    description: format!("Player performed {action_type}"),
                                };
                                return Ok(Some(ParsedEvent::GameAction(game_action)));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_client_to_match_service(&mut self, _raw_data: &str) -> Result<Option<ParsedEvent>> {
        // Handle match service messages if needed
        Ok(None)
    }

    fn handle_match_service_to_client(&mut self, _raw_data: &str) -> Result<Option<ParsedEvent>> {
        // Handle match service responses if needed
        Ok(None)
    }

    fn handle_draft_pack(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(cards) = json.get("cards").and_then(|c| c.as_array()) {
                let card_ids: Vec<u32> = cards
                    .iter()
                    .filter_map(|c| c.as_u64().map(|id| id as u32))
                    .collect();

                if let Some(ref mut draft_state) = self.current_draft {
                    draft_state.available_cards = card_ids.clone();
                    return Ok(Some(ParsedEvent::DraftPack {
                        pack_number: draft_state.pack_number,
                        pick_number: draft_state.pick_number,
                        cards: card_ids,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn handle_draft_pick(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(card_id) = json.get("cardId").and_then(|c| c.as_u64()) {
                if let Some(ref mut draft_state) = self.current_draft {
                    draft_state.picks.push(card_id as u32);
                }
                return Ok(Some(ParsedEvent::DraftPick {
                    card_id: card_id as u32,
                }));
            }
        }
        Ok(None)
    }

    fn handle_user_authenticated(&mut self, raw_data: &str) -> Result<Option<ParsedEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let (Some(user_id), Some(display_name)) = (
                json.get("userId").and_then(|u| u.as_str()),
                json.get("displayName").and_then(|d| d.as_str()),
            ) {
                self.current_user = Some((user_id.to_string(), display_name.to_string()));
                return Ok(Some(ParsedEvent::UserAuthenticated {
                    user_id: user_id.to_string(),
                    display_name: display_name.to_string(),
                }));
            }
        }
        Ok(None)
    }
}
