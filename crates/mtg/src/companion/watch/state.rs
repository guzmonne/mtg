use super::types::LogFileState;
use crate::prelude::*;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct StateManager {
    state_file_path: PathBuf,
    current_state: LogFileState,
}

impl StateManager {
    pub fn new() -> Result<Self> {
        let state_dir = dirs::cache_dir()
            .ok_or_else(|| eyre!("Could not determine cache directory"))?
            .join("mtg-cli");

        std::fs::create_dir_all(&state_dir)?;

        let state_file_path = state_dir.join("companion_watch_state.json");

        Ok(Self {
            state_file_path,
            current_state: LogFileState::default(),
        })
    }

    pub async fn load_state(
        &mut self,
        log_file_path: &Path,
        from_beginning: bool,
    ) -> Result<LogFileState> {
        if !from_beginning && self.state_file_path.exists() {
            match fs::read_to_string(&self.state_file_path).await {
                Ok(content) => {
                    match serde_json::from_str::<LogFileState>(&content) {
                        Ok(state) => {
                            // Only use saved state if it's for the same file
                            if state.file_path == log_file_path.to_string_lossy() {
                                // Verify the file still exists and hasn't been truncated
                                if let Ok(metadata) = fs::metadata(log_file_path).await {
                                    if metadata.len() >= state.bytes_read {
                                        self.current_state = state.clone();
                                        return Ok(state);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            aeprintln!("Warning: Could not parse state file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    aeprintln!("Warning: Could not read state file: {}", e);
                }
            }
        }

        // Create new state for this file
        let new_state = LogFileState {
            file_path: log_file_path.to_string_lossy().to_string(),
            bytes_read: 0,
            ..Default::default()
        };

        self.current_state = new_state.clone();
        Ok(new_state)
    }

    pub async fn save_state(&mut self, state: LogFileState) -> Result<()> {
        self.current_state = state.clone();

        let content = serde_json::to_string_pretty(&state)?;
        fs::write(&self.state_file_path, content).await?;

        Ok(())
    }

    pub fn current_state(&self) -> &LogFileState {
        &self.current_state
    }

    pub fn update_position(&mut self, bytes_read: u64) {
        self.current_state.bytes_read = bytes_read;
    }
}
