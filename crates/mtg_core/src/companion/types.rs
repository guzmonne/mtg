use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RawLogEvent {
    pub timestamp: Option<DateTime<Utc>>,
    pub event_name: String,
    pub raw_data: String,
}

#[derive(Debug, Clone, Default)]
pub struct MatchState {
    pub match_id: Option<String>,
    pub game_number: u32,
    pub players: Vec<PlayerInfo>,
    pub turn_number: u32,
    pub active_player: u32,
    pub phase: String,
    pub step: String,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub winner: Option<u32>,
    pub match_type: String,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub seat_id: u32,
    pub screen_name: String,
    pub life_total: u32,
    pub starting_life: u32,
    pub deck_id: Option<String>,
    pub deck_name: Option<String>,
    pub mulligans: u32,
    pub cards_drawn: u32,
    pub lands_played: u32,
    pub spells_cast: u32,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            seat_id: 0,
            screen_name: "Unknown".to_string(),
            life_total: 20,
            starting_life: 20,
            deck_id: None,
            deck_name: None,
            mulligans: 0,
            cards_drawn: 0,
            lands_played: 0,
            spells_cast: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DraftState {
    pub draft_id: Option<String>,
    pub set_code: String,
    pub pack_number: u32,
    pub pick_number: u32,
    pub picks: Vec<u32>, // Card IDs
    pub available_cards: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct GameAction {
    pub player: u32,
    pub action_type: String,
    pub target: Option<String>,
    pub cost: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum ParsedEvent {
    UserAuthenticated {
        user_id: String,
        display_name: String,
    },
    MatchStarted(MatchState),
    MatchEnded(MatchState),
    TurnChange {
        turn: u32,
        active_player: u32,
    },
    LifeChange {
        player: u32,
        old_life: u32,
        new_life: u32,
    },
    GameAction(GameAction),
    CardPlayed {
        player: u32,
        card_name: String,
        zone_from: String,
        zone_to: String,
    },
    Mulligan {
        player: u32,
        from_size: u32,
        to_size: u32,
    },
    DraftPack {
        pack_number: u32,
        pick_number: u32,
        cards: Vec<u32>,
    },
    DraftPick {
        card_id: u32,
    },
    DraftCompleted,
    DeckSubmitted {
        deck_id: Option<String>,
        deck_name: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    TargetSelection { target_id: u64, action: String },
    CounterChange { counter_type: String, amount: u64 },
    ManaPaid,
    CardRevealed { count: usize },
    CardDrawn,
    AbilityActivated,
    PermanentTapped { tapped: bool },
    ActionTaken { action_type: String },
    PhaseChange { phase: String },
    LifeChange { player: u64, life_total: u64 },
    UIMessage,
    GameEvent,
}

// Shared utility functions
pub fn format_mana_cost(mana_cost: &[HashMap<String, serde_json::Value>]) -> String {
    let mut result = String::new();

    for cost in mana_cost {
        if let Some(colors) = cost.get("color") {
            if let Some(colors_array) = colors.as_array() {
                for color in colors_array {
                    if let Some(color_str) = color.as_str() {
                        let symbol = match color_str {
                            "ManaColor_White" => "W",
                            "ManaColor_Blue" => "U",
                            "ManaColor_Black" => "B",
                            "ManaColor_Red" => "R",
                            "ManaColor_Green" => "G",
                            "ManaColor_Generic" => "C",
                            _ => "?",
                        };
                        result.push_str(symbol);
                    }
                }
            }
        }

        if let Some(count) = cost.get("count") {
            if let Some(count_num) = count.as_u64() {
                if count_num > 1 {
                    result = format!("{count_num}{result}");
                }
            }
        }
    }

    if result.is_empty() {
        "0".to_string()
    } else {
        format!("{{{result}}}")
    }
}

pub fn zone_to_string(zone_id: u32) -> String {
    match zone_id {
        23 => "Hand".to_string(),
        28 => "Battlefield".to_string(),
        29 => "Graveyard".to_string(),
        24 => "Library".to_string(),
        27 => "Stack".to_string(),
        26 => "Exile".to_string(),
        18 => "Revealed".to_string(),
        30 => "Limbo".to_string(),
        _ => format!("Zone{zone_id}"),
    }
}

pub fn to_camel_case(snake_case: &str) -> String {
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
