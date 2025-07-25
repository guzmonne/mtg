#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MatchState {
    pub match_id: String,
    pub players: Vec<Player>,
    pub current_turn: u32,
    pub active_player: usize,
    pub phase: GamePhase,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub result: Option<MatchResult>,
    pub actions: Vec<GameAction>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub seat_id: u32,
    pub screen_name: String,
    pub life_total: i32,
    pub hand_size: usize,
    pub initial_hand: Vec<CardInstance>,
    pub mulligans: Vec<MulliganInfo>,
    pub deck_cards: Vec<CardInstance>,
}

#[derive(Debug, Clone)]
pub struct CardInstance {
    pub instance_id: u32,
    pub grp_id: u32,
    pub card_info: Option<CardInfo>,
    pub zone: Zone,
}

#[derive(Debug, Clone)]
pub struct CardInfo {
    pub name: String,
    pub mana_cost: String,
    pub type_line: String,
    pub oracle_text: String,
}

#[derive(Debug, Clone)]
pub struct MulliganInfo {
    pub from_hand_size: usize,
    pub to_hand_size: usize,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GameAction {
    pub timestamp: DateTime<Utc>,
    pub player: usize,
    pub action_type: ActionType,
    pub description: String,
    pub life_change: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum GamePhase {
    Beginning,
    Main1,
    Combat,
    Main2,
    End,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Zone {
    Hand,
    Library,
    Graveyard,
    Battlefield,
    Exile,
    Stack,
    Command,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Play,
    Cast,
    Draw,
    Discard,
    Mulligan,
    Attack,
    Block,
    Activate,
    Trigger,
    LifeChange,
    PhaseChange,
    TurnChange,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub winner: Option<usize>,
    pub reason: String,
    pub duration_seconds: u64,
}

// Arena log event structures - this is the top-level structure in the logs
#[derive(Debug, Deserialize)]
pub struct ArenaLogEvent {
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<u32>,
    pub timestamp: Option<String>,
    #[serde(rename = "greToClientEvent")]
    pub gre_to_client_event: Option<GreToClientEvent>,
    #[serde(rename = "matchGameRoomStateChangedEvent")]
    pub match_game_room_state_changed_event: Option<MatchGameRoomStateChangedEvent>,
    #[serde(rename = "MatchesV3")]
    pub matches_v3: Option<MatchesV3>,
}

#[derive(Debug, Deserialize)]
pub struct GreToClientEvent {
    #[serde(rename = "greToClientMessages")]
    pub gre_to_client_messages: Vec<GreToClientMessage>,
}

#[derive(Debug, Deserialize)]
pub struct GreToClientMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "gameStateMessage")]
    pub game_state_message: Option<GameStateMessage>,
}

#[derive(Debug, Deserialize)]
pub struct GameStateMessage {
    #[serde(rename = "gameInfo")]
    pub game_info: Option<GameInfo>,
    pub players: Option<Vec<ArenaPlayer>>,
    #[serde(rename = "turnInfo")]
    pub turn_info: Option<TurnInfo>,
    pub zones: Option<Vec<ArenaZone>>,
    #[serde(rename = "gameObjects")]
    pub game_objects: Option<Vec<ArenaGameObject>>,
    pub actions: Option<Vec<ArenaAction>>,
}

#[derive(Debug, Deserialize)]
pub struct GameInfo {
    #[serde(rename = "matchID")]
    pub match_id: Option<String>,
    #[serde(rename = "gameNumber")]
    pub game_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ArenaPlayer {
    #[serde(rename = "controllerSeatId")]
    pub controller_seat_id: u32,
    #[serde(rename = "lifeTotal")]
    pub life_total: i32,
    #[serde(rename = "systemSeatNumber")]
    pub system_seat_number: u32,
}

#[derive(Debug, Deserialize)]
pub struct TurnInfo {
    #[serde(rename = "turnNumber")]
    pub turn_number: u32,
    #[serde(rename = "activePlayer")]
    pub active_player: u32,
    pub phase: Option<String>,
    pub step: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArenaZone {
    #[serde(rename = "zoneId")]
    pub zone_id: u32,
    #[serde(rename = "type")]
    pub zone_type: String,
    #[serde(rename = "ownerSeatId")]
    pub owner_seat_id: Option<u32>,
    #[serde(rename = "objectInstanceIds")]
    pub object_instance_ids: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct ArenaGameObject {
    #[serde(rename = "instanceId")]
    pub instance_id: u32,
    #[serde(rename = "grpId")]
    pub grp_id: u32,
    #[serde(rename = "type")]
    pub object_type: Option<String>,
    #[serde(rename = "zoneId")]
    pub zone_id: Option<u32>,
    #[serde(rename = "ownerSeatId")]
    pub owner_seat_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ArenaAction {
    #[serde(rename = "actionType")]
    pub action_type: Option<String>,
    #[serde(rename = "instanceId")]
    pub instance_id: Option<u32>,
    #[serde(rename = "grpId")]
    pub grp_id: Option<u32>,
    #[serde(rename = "seatId")]
    pub seat_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MatchGameRoomStateChangedEvent {
    #[serde(rename = "gameRoomInfo")]
    pub game_room_info: Option<GameRoomInfo>,
}

#[derive(Debug, Deserialize)]
pub struct GameRoomInfo {
    #[serde(rename = "finalMatchResult")]
    pub final_match_result: Option<FinalMatchResult>,
}

#[derive(Debug, Deserialize)]
pub struct FinalMatchResult {
    #[serde(rename = "winningTeamId")]
    pub winning_team_id: Option<u32>,
    #[serde(rename = "resultType")]
    pub result_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchesV3 {
    #[serde(rename = "MatchId")]
    pub match_id: String,
    #[serde(rename = "PlayerInfos")]
    pub player_infos: Vec<PlayerInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerInfo {
    #[serde(rename = "ScreenName")]
    pub screen_name: String,
    #[serde(rename = "RankingClass")]
    pub ranking_class: Option<String>,
    #[serde(rename = "RankingTier")]
    pub ranking_tier: Option<u32>,
    #[serde(rename = "SystemSeatId")]
    pub system_seat_id: u32,
}

impl Default for MatchState {
    fn default() -> Self {
        Self {
            match_id: String::new(),
            players: Vec::new(),
            current_turn: 0,
            active_player: 0,
            phase: GamePhase::Unknown,
            started_at: Utc::now(),
            ended_at: None,
            result: None,
            actions: Vec::new(),
        }
    }
}

impl From<&str> for Zone {
    fn from(zone_type: &str) -> Self {
        match zone_type {
            "ZoneType_Hand" => Zone::Hand,
            "ZoneType_Library" => Zone::Library,
            "ZoneType_Graveyard" => Zone::Graveyard,
            "ZoneType_Battlefield" => Zone::Battlefield,
            "ZoneType_Exile" => Zone::Exile,
            "ZoneType_Stack" => Zone::Stack,
            "ZoneType_Command" => Zone::Command,
            _ => Zone::Unknown,
        }
    }
}

impl From<&str> for GamePhase {
    fn from(phase: &str) -> Self {
        match phase {
            "Phase_Beginning" => GamePhase::Beginning,
            "Phase_Main1" => GamePhase::Main1,
            "Phase_Combat" => GamePhase::Combat,
            "Phase_Main2" => GamePhase::Main2,
            "Phase_End" => GamePhase::End,
            _ => GamePhase::Unknown,
        }
    }
}

impl From<&str> for ActionType {
    fn from(action_type: &str) -> Self {
        match action_type {
            "ActionType_Play" => ActionType::Play,
            "ActionType_Cast" => ActionType::Cast,
            "ActionType_Draw" => ActionType::Draw,
            "ActionType_Discard" => ActionType::Discard,
            "ActionType_Mulligan" => ActionType::Mulligan,
            "ActionType_Attack" => ActionType::Attack,
            "ActionType_Block" => ActionType::Block,
            "ActionType_Activate" => ActionType::Activate,
            "ActionType_Trigger" => ActionType::Trigger,
            _ => ActionType::Unknown,
        }
    }
}

// State management types
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LogFileState {
    pub file_path: String,
    pub bytes_read: u64,
    pub last_timestamp: Option<DateTime<Utc>>,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub current_match_id: Option<String>,
}

// Authentication event types
#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
    #[serde(rename = "authenticateResponse")]
    pub authenticate_response: Option<AuthenticateData>,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticateData {
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    #[serde(rename = "screenName")]
    pub screen_name: Option<String>,
}

// Draft event types
#[derive(Debug, Deserialize)]
pub struct DraftPack {
    #[serde(rename = "PackCards")]
    pub pack_cards: Option<String>, // Comma-separated card IDs
    #[serde(rename = "SelfPack")]
    pub self_pack: Option<u32>,
    #[serde(rename = "SelfPick")]
    pub self_pick: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct DraftPick {
    #[serde(rename = "IsPickingCompleted")]
    pub is_picking_completed: Option<bool>,
    #[serde(rename = "PickedCard")]
    pub picked_card: Option<u32>,
}

#[derive(Debug, Default, Clone)]
pub struct DraftState {
    pub pack_number: u32,
    pub pick_number: u32,
    pub is_drafting: bool,
    pub current_pack: Vec<u32>,
    pub picked_cards: Vec<u32>,
}

// Deck event types
#[derive(Debug, Deserialize)]
pub struct DeckSubmission {
    #[serde(rename = "deckId")]
    pub deck_id: Option<String>,
    #[serde(rename = "deckName")]
    pub deck_name: Option<String>,
    #[serde(rename = "mainDeck")]
    pub main_deck: Option<HashMap<String, u32>>,
    #[serde(rename = "sideboard")]
    pub sideboard: Option<HashMap<String, u32>>,
    #[serde(rename = "commandZoneGRPIds")]
    pub command_zone_grp_ids: Option<Vec<u32>>,
}

// Enhanced event parsing types
#[derive(Debug, Clone)]
pub struct RawLogEvent {
    pub timestamp: Option<DateTime<Utc>>,
    pub event_name: String,
    pub raw_data: String,
}

#[derive(Debug)]
pub struct ParsedLogData {
    pub event_type: String,
    pub data: serde_json::Value,
}

// Event extraction strategies
#[derive(Debug)]
pub enum ExtractionStrategy {
    Direct,         // Direct JSON parsing
    Payload,        // Extract from 'payload' field
    PayloadCapital, // Extract from 'Payload' field
    Request,        // Extract from 'request' field
    CamelCase,      // Match event name in camelCase
}

// Zone ID mappings based on JS implementation
pub const ZONE_STACK: u32 = 27;
pub const ZONE_BATTLEFIELD: u32 = 28;
pub const ZONE_HAND_P1: u32 = 31;
pub const ZONE_HAND_P2: u32 = 35;
pub const ZONE_LIBRARY: u32 = 36;
pub const ZONE_GRAVEYARD: u32 = 37;
pub const ZONE_EXILE: u32 = 38;

impl From<u32> for Zone {
    fn from(zone_id: u32) -> Self {
        match zone_id {
            ZONE_HAND_P1 | ZONE_HAND_P2 => Zone::Hand,
            ZONE_LIBRARY => Zone::Library,
            ZONE_GRAVEYARD => Zone::Graveyard,
            ZONE_BATTLEFIELD => Zone::Battlefield,
            ZONE_EXILE => Zone::Exile,
            ZONE_STACK => Zone::Stack,
            _ => Zone::Unknown,
        }
    }
}
