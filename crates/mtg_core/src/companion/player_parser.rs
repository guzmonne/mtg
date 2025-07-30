use super::types::*;
use color_eyre::Result;
use serde_json::Value;

pub struct PlayerEventParser {
    // Track player-specific state if needed
}

impl Default for PlayerEventParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerEventParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_player_event(&mut self, event: RawLogEvent) -> Result<Option<PlayerEvent>> {
        match event.event_name.as_str() {
            "PlayerTargetSelection" => self.handle_target_selection(&event.raw_data),
            "PlayerCounterChange" => self.handle_counter_change(&event.raw_data),
            "PlayerManaPaid" => Ok(Some(PlayerEvent::ManaPaid)),
            "PlayerCardRevealed" => self.handle_card_revealed(&event.raw_data),
            "PlayerCardDrawn" => Ok(Some(PlayerEvent::CardDrawn)),
            "PlayerAbilityActivated" => Ok(Some(PlayerEvent::AbilityActivated)),
            "PlayerPermanentTapped" => self.handle_permanent_tapped(&event.raw_data),
            "PlayerActionTaken" => self.handle_action_taken(&event.raw_data),
            "PlayerPhaseChange" => self.handle_phase_change(&event.raw_data),
            "PlayerLifeChange" => self.handle_life_change(&event.raw_data),
            "PlayerUIMessage" => Ok(Some(PlayerEvent::UIMessage)),
            "PlayerGameEvent" => Ok(Some(PlayerEvent::GameEvent)),
            _ => Ok(None), // Unknown event type
        }
    }

    fn handle_target_selection(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(select_targets) = json.get("selectTargetsResp") {
                if let Some(target) = select_targets.get("target") {
                    if let Some(targets) = target.get("targets").and_then(|t| t.as_array()) {
                        for target_obj in targets {
                            if let Some(target_id) = target_obj
                                .get("targetInstanceId")
                                .and_then(|id| id.as_u64())
                            {
                                let action = target_obj
                                    .get("legalAction")
                                    .and_then(|a| a.as_str())
                                    .unwrap_or("Select")
                                    .to_string();

                                return Ok(Some(PlayerEvent::TargetSelection {
                                    target_id,
                                    action,
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_counter_change(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
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

                                    return Ok(Some(PlayerEvent::CounterChange {
                                        counter_type: counter_name.to_string(),
                                        amount,
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_card_revealed(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(zones) = json.get("zones").and_then(|z| z.as_array()) {
                for zone in zones {
                    if let Some(zone_type) = zone.get("type").and_then(|t| t.as_str()) {
                        if zone_type == "ZoneType_Revealed" {
                            if let Some(object_ids) =
                                zone.get("objectInstanceIds").and_then(|ids| ids.as_array())
                            {
                                return Ok(Some(PlayerEvent::CardRevealed {
                                    count: object_ids.len(),
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_permanent_tapped(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
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
                                                return Ok(Some(PlayerEvent::PermanentTapped {
                                                    tapped: tapped == 1,
                                                }));
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

    fn handle_action_taken(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
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
                                                return Ok(Some(PlayerEvent::ActionTaken {
                                                    action_type: action_name.to_string(),
                                                }));
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

    fn handle_phase_change(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
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

                                for detail in details {
                                    if let Some(key) = detail.get("key").and_then(|k| k.as_str()) {
                                        if key == "phase" {
                                            phase = detail
                                                .get("valueInt32")
                                                .and_then(|v| v.as_array())
                                                .and_then(|arr| arr.first())
                                                .and_then(|v| v.as_u64());
                                        }
                                    }
                                }

                                if let Some(phase) = phase {
                                    let phase_name = match phase {
                                        1 => "Beginning",
                                        2 => "Main",
                                        3 => "Combat",
                                        4 => "Main 2",
                                        5 => "End",
                                        _ => "Unknown",
                                    };
                                    return Ok(Some(PlayerEvent::PhaseChange {
                                        phase: phase_name.to_string(),
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_life_change(&mut self, raw_data: &str) -> Result<Option<PlayerEvent>> {
        if let Ok(json) = serde_json::from_str::<Value>(raw_data) {
            if let Some(players) = json.get("players").and_then(|p| p.as_array()) {
                for player in players {
                    if let Some(life_total) = player.get("lifeTotal").and_then(|l| l.as_u64()) {
                        if let Some(seat_number) =
                            player.get("systemSeatNumber").and_then(|s| s.as_u64())
                        {
                            return Ok(Some(PlayerEvent::LifeChange {
                                player: seat_number,
                                life_total,
                            }));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
