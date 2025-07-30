use super::state::StateManager;
use super::types::RawLogEvent;
use crate::prelude::*;
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use tokio::time::sleep;

const CHUNK_SIZE: usize = 8192;
const POLL_INTERVAL_MS: u64 = 1000; // Check less frequently than main log
const PLAYER_LOG_PREFIX: &str = "[UnityCrossThreadLogger]";

pub struct PlayerLogTailer {
    file_path: PathBuf,
    state_manager: StateManager,
    buffer: String,
    current_event: Option<String>,
}

impl PlayerLogTailer {
    pub async fn new(file_path: impl AsRef<Path>, from_beginning: bool) -> Result<Self> {
        let mut state_manager = StateManager::new()?;
        let file_path = file_path.as_ref().to_path_buf();

        // Load existing state for this file (use different state key)
        let mut state_file_path = file_path.clone();
        state_file_path.set_extension("player_state");
        state_manager
            .load_state(&state_file_path, from_beginning)
            .await?;

        Ok(Self {
            file_path,
            state_manager,
            buffer: String::new(),
            current_event: None,
        })
    }

    pub async fn tail_with_callback<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(RawLogEvent) -> Result<()>,
    {
        aeprintln!(
            "Starting to tail Player.log file: {}",
            self.file_path.display()
        );
        aeprintln!(
            "Player.log position: {}",
            self.state_manager.current_state().bytes_read
        );

        loop {
            let events = self.read_new_events().await?;

            for event in events {
                if let Err(e) = callback(event) {
                    aeprintln!("Error processing Player.log event: {}", e);
                }
            }

            // Sleep to avoid busy waiting
            sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
        }
    }

    async fn read_new_events(&mut self) -> Result<Vec<RawLogEvent>> {
        let mut events = Vec::new();

        // Open file and seek to our position
        let mut file = File::open(&self.file_path).await?;
        let current_state = self.state_manager.current_state();
        let mut position = current_state.bytes_read;

        // Check if file was truncated
        let metadata = file.metadata().await?;
        if metadata.len() < position {
            aeprintln!("Player.log file was truncated, restarting from beginning");
            position = 0;
            self.buffer.clear();
            self.current_event = None;
        } else if metadata.len() == position {
            // No new content
            return Ok(events);
        }

        file.seek(SeekFrom::Start(position)).await?;

        // Read new content in chunks
        let mut chunk = vec![0u8; CHUNK_SIZE];
        loop {
            let bytes_read = file.read(&mut chunk).await?;
            if bytes_read == 0 {
                break;
            }

            // Convert chunk to string and add to buffer
            let chunk_str = String::from_utf8_lossy(&chunk[..bytes_read]);
            self.buffer.push_str(&chunk_str);
            position += bytes_read as u64;

            // Process complete events from buffer
            let new_events = self.extract_events_from_buffer()?;
            events.extend(new_events);
        }

        // Update position in state
        self.state_manager.update_position(position);
        self.save_state().await?;

        Ok(events)
    }

    fn extract_events_from_buffer(&mut self) -> Result<Vec<RawLogEvent>> {
        let mut events = Vec::new();
        let lines: Vec<&str> = self.buffer.lines().collect();
        let mut processed_lines = 0;

        // Process complete lines only
        for (i, line) in lines.iter().enumerate() {
            if line.contains(PLAYER_LOG_PREFIX) {
                // Check if this is a valuable event type
                if self.is_valuable_event(line) {
                    if let Some(event) = self.parse_player_log_line(line)? {
                        events.push(event);
                    }
                }
                processed_lines = i + 1;
            }
        }

        // Remove processed lines from buffer
        if processed_lines > 0 {
            let remaining_lines: Vec<&str> = lines.into_iter().skip(processed_lines).collect();
            self.buffer = remaining_lines.join("\n");
            if !self.buffer.is_empty() && !self.buffer.ends_with('\n') {
                self.buffer.push('\n');
            }
        }

        Ok(events)
    }

    fn is_valuable_event(&self, line: &str) -> bool {
        // Check for valuable event patterns from the documentation
        line.contains("matchGameRoomStateChangedEvent")
            || line.contains("ClientMessageType_MulliganResp")
            || line.contains("ClientMessageType_SelectTargetsResp")
            || line.contains("ClientMessageType_PerformActionResp")
            || line.contains("ClientMessageType_DeclareAttackersResp")
            || line.contains("GREMessageType_GameStateMessage")
            || line.contains("GREMessageType_SubmitAttackersResp")
            || line.contains("AnnotationType_ZoneTransfer")
            || line.contains("AnnotationType_CounterAdded")
            || line.contains("AnnotationType_ManaPaid")
            || line.contains("AnnotationType_CardDrawn")
            || line.contains("AnnotationType_AbilityInstanceCreated")
            || line.contains("AnnotationType_TappedUntappedPermanent")
            || line.contains("AnnotationType_UserActionTaken")
            || line.contains("AnnotationType_PhaseOrStepModified")
            || line.contains("AnnotationType_ResolutionStart")
            || line.contains("ZoneType_Revealed")
            || line.contains("EventGetCoursesV2")
            || line.contains("EventSetDeckV2")
            || line.contains("ActionType_Cast")
            || line.contains("ActionType_Pass")
            || line.contains("AttackState_Attacking")
            || line.contains("BlockState_Blocking")
            || line.contains("players") && line.contains("lifeTotal")
            || line.contains("ClientToGREUIMessage")
            || line.contains("GreToClientEvent")
    }

    fn parse_player_log_line(&self, line: &str) -> Result<Option<RawLogEvent>> {
        let line = line.trim();

        // Must contain our prefix
        if !line.contains(PLAYER_LOG_PREFIX) {
            return Ok(None);
        }

        // Extract timestamp if present
        let timestamp = Some(Utc::now()); // For now, use current time

        // Look for JSON data in the line
        if let Some(json_start) = line.find('{') {
            let json_data = &line[json_start..];

            // Try to determine event type from JSON content - prioritize more specific patterns
            let event_name = if json_data.contains("matchGameRoomStateChangedEvent") {
                "PlayerMatchStarted"
            } else if json_data.contains("ClientMessageType_MulliganResp") {
                "PlayerMulligan"
            } else if json_data.contains("ClientMessageType_DeclareAttackersResp")
                || json_data.contains("AttackState_Attacking")
            {
                "PlayerAttackers"
            } else if json_data.contains("BlockState_Blocking") {
                "PlayerBlockers"
            } else if json_data.contains("AnnotationType_ZoneTransfer") {
                "PlayerCardPlayed"
            } else if json_data.contains("ActionType_Cast") && json_data.contains("manaCost") {
                "PlayerSpellCast"
            } else if json_data.contains("ActionType_Pass") {
                "PlayerPriorityPass"
            } else if json_data.contains("AnnotationType_ResolutionStart") {
                "PlayerSpellResolution"
            } else if json_data.contains("EventGetCoursesV2")
                || json_data.contains("EventSetDeckV2")
            {
                "PlayerDeckInfo"
            } else if json_data.contains("ClientMessageType_SelectTargetsResp") {
                "PlayerTargetSelection"
            } else if json_data.contains("AnnotationType_CounterAdded") {
                "PlayerCounterChange"
            } else if json_data.contains("AnnotationType_ManaPaid") {
                "PlayerManaPaid"
            } else if json_data.contains("ZoneType_Revealed") {
                "PlayerCardRevealed"
            } else if json_data.contains("AnnotationType_CardDrawn") {
                "PlayerCardDrawn"
            } else if json_data.contains("AnnotationType_AbilityInstanceCreated") {
                "PlayerAbilityActivated"
            } else if json_data.contains("AnnotationType_TappedUntappedPermanent") {
                "PlayerPermanentTapped"
            } else if json_data.contains("AnnotationType_UserActionTaken") {
                "PlayerActionTaken"
            } else if json_data.contains("AnnotationType_PhaseOrStepModified") {
                "PlayerPhaseChange"
            } else if json_data.contains("GREMessageType_GameStateMessage") {
                "PlayerGameState"
            } else if json_data.contains("lifeTotal") {
                "PlayerLifeChange"
            } else if json_data.contains("ClientToGREUIMessage") {
                "PlayerUIMessage"
            } else if json_data.contains("GreToClientEvent") {
                "PlayerGameEvent"
            } else {
                "PlayerGenericEvent"
            };

            return Ok(Some(RawLogEvent {
                timestamp,
                event_name: event_name.to_string(),
                raw_data: json_data.to_string(),
            }));
        }

        Ok(None)
    }

    async fn save_state(&mut self) -> Result<()> {
        let current_state = self.state_manager.current_state().clone();
        self.state_manager.save_state(current_state).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_player_log_tailer_basic() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Initial line")?;
        temp_file.flush()?;

        let mut tailer = PlayerLogTailer::new(temp_file.path(), false).await?;

        // Add new content with Player.log format
        writeln!(temp_file, "[UnityCrossThreadLogger]7/28/2025 12:29:34 AM: Match to 7NUM5K7TBFDUPBPPRDNX433B5A: GreToClientEvent {{ \"transactionId\": \"test\", \"greToClientEvent\": {{ \"greToClientMessages\": [{{ \"type\": \"GREMessageType_GameStateMessage\", \"players\": [{{ \"lifeTotal\": 20 }}] }}] }} }}")?;
        temp_file.flush()?;

        // Read new events
        let events = tailer.read_new_events().await?;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_name, "PlayerGameState");

        Ok(())
    }
}
