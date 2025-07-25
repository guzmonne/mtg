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
const POLL_INTERVAL_MS: u64 = 500; // Check more frequently
const EVENT_PREFIX: &str = "[UnityCrossThreadLogger]";

pub struct LogTailer {
    file_path: PathBuf,
    state_manager: StateManager,
    buffer: String,
    current_event: Option<String>,
}

impl LogTailer {
    pub async fn new(file_path: impl AsRef<Path>, from_beginning: bool) -> Result<Self> {
        let mut state_manager = StateManager::new()?;
        let file_path = file_path.as_ref().to_path_buf();

        // Load existing state for this file
        state_manager.load_state(&file_path, from_beginning).await?;

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
        aeprintln!("Starting to tail log file: {}", self.file_path.display());
        aeprintln!(
            "Resuming from position: {}",
            self.state_manager.current_state().bytes_read
        );

        let mut last_status = std::time::Instant::now();
        let status_interval = Duration::from_secs(30);

        loop {
            let events = self.read_new_events().await?;

            for event in events {
                if let Err(e) = callback(event) {
                    aeprintln!("Error processing event: {}", e);
                }
            }

            // Show periodic status message
            if last_status.elapsed() > status_interval {
                let current_pos = self.state_manager.current_state().bytes_read;
                aeprintln!("⏳ Still watching... (position: {})", current_pos);
                last_status = std::time::Instant::now();
            }

            // Check for new log files (Arena creates new files periodically)
            if let Ok(newest_file) =
                crate::companion::parse::find_newest_log_file(self.file_path.parent().unwrap())
            {
                if newest_file != self.file_path {
                    aeprintln!("Switching to newer log file: {}", newest_file.display());

                    // Save current state before switching
                    self.save_state().await?;

                    // Switch to new file
                    self.file_path = newest_file;
                    self.state_manager
                        .load_state(&self.file_path, false)
                        .await?;
                    self.buffer.clear();
                    self.current_event = None;
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
            aeprintln!("Log file was truncated, restarting from beginning");
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
        let mut cursor = 0;

        // If we have a partial event from previous chunk, try to complete it
        if let Some(current_event) = self.current_event.take() {
            if let Some(line_end) = self.buffer.find('\n') {
                let mut completed_event = current_event;
                completed_event.push_str(&self.buffer[..line_end]);

                // Try to parse the completed event
                if let Some(event) = self.parse_event_line(&completed_event)? {
                    events.push(event);
                }

                cursor = line_end + 1;
            } else {
                // Still incomplete, add entire buffer to current event
                let mut updated_event = current_event;
                updated_event.push_str(&self.buffer);
                self.current_event = Some(updated_event);
                self.buffer.clear();
                return Ok(events);
            }
        }

        // Look for new events in the remaining buffer
        while cursor < self.buffer.len() {
            if let Some(prefix_pos) = self.buffer[cursor..].find(EVENT_PREFIX) {
                let event_start = cursor + prefix_pos;

                // Find the end of this event (next event prefix or end of buffer)
                let next_event_pos = self.buffer[event_start + EVENT_PREFIX.len()..]
                    .find(EVENT_PREFIX)
                    .map(|pos| event_start + EVENT_PREFIX.len() + pos);

                let event_end = next_event_pos.unwrap_or(self.buffer.len());
                let event_content = &self.buffer[event_start..event_end];

                // Check if this event is complete (ends with newline or is at buffer end)
                if event_content.ends_with('\n') || next_event_pos.is_some() {
                    if let Some(event) = self.parse_event_line(event_content)? {
                        events.push(event);
                    }
                    cursor = event_end;
                } else {
                    // Incomplete event, save it for next chunk
                    self.current_event = Some(event_content.to_string());
                    cursor = self.buffer.len();
                }
            } else {
                // No more events in buffer
                break;
            }
        }

        // Remove processed content from buffer
        if cursor > 0 {
            self.buffer.drain(..cursor);
        }

        Ok(events)
    }

    fn parse_event_line(&self, content: &str) -> Result<Option<RawLogEvent>> {
        let content = content.trim();

        // Must start with our event prefix
        if !content.starts_with(EVENT_PREFIX) {
            return Ok(None);
        }

        // Split content into lines
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Ok(None);
        }

        // Check for STATE CHANGED pattern first
        if lines[0].contains("STATE CHANGED") {
            // Parse state change: [500799] [UnityCrossThreadLogger]STATE CHANGED {"old":"Playing","new":"MatchCompleted"}
            if let Some(json_start) = lines[0].find('{') {
                let json_data = lines[0][json_start..].to_string();
                return Ok(Some(RawLogEvent {
                    timestamp: Some(Utc::now()),
                    event_name: "StateChanged".to_string(),
                    raw_data: json_data,
                }));
            }
        }

        // Find the line with the arrow (could be first or second line)
        let mut arrow_line_idx = None;
        let mut arrow_out = None;
        let mut arrow_in = None;

        for (idx, line) in lines.iter().enumerate() {
            if line.contains("==> ") {
                arrow_out = line.find("==> ");
                arrow_line_idx = Some(idx);
                break;
            } else if line.contains("<== ") {
                arrow_in = line.find("<== ");
                arrow_line_idx = Some(idx);
                break;
            }
        }

        if let Some(line_idx) = arrow_line_idx {
            let arrow_line = lines[line_idx];
            let arrow_pos = arrow_out.or(arrow_in).unwrap();
            let is_outgoing = arrow_out.is_some();
            let after_arrow = &arrow_line[arrow_pos + 4..];

            // For incoming events, format is: EventName(guid) with JSON on next line
            // For outgoing events, format is: EventName {"json": "data"}
            let (event_name, raw_data) = if !is_outgoing {
                // Incoming event - look for parentheses
                let event_name = if let Some(paren_pos) = after_arrow.find('(') {
                    after_arrow[..paren_pos].trim().to_string()
                } else {
                    after_arrow.trim().to_string()
                };

                // For incoming events, JSON data is on the next line after the arrow line
                // Skip empty lines and find the JSON data
                let mut json_data = String::new();
                for line in lines.iter().skip(line_idx + 1) {
                    let line = line.trim();
                    if !line.is_empty() && (line.starts_with('{') || line.starts_with('[')) {
                        json_data = line.to_string();
                        break;
                    }
                }

                (event_name, json_data)
            } else {
                // Outgoing event - look for space or brace
                if let Some(space_pos) = after_arrow.find(' ') {
                    let event_name = after_arrow[..space_pos].to_string();
                    let raw_data = after_arrow[space_pos..].trim().to_string();
                    (event_name, raw_data)
                } else if let Some(brace_pos) = after_arrow.find('{') {
                    let event_name = after_arrow[..brace_pos].trim().to_string();
                    let raw_data = after_arrow[brace_pos..].to_string();
                    (event_name, raw_data)
                } else {
                    // Just event name, no data
                    (after_arrow.to_string(), String::new())
                }
            };

            // Parse timestamp (basic parsing, could be improved)
            let timestamp = Some(Utc::now()); // For now, use current time

            return Ok(Some(RawLogEvent {
                timestamp,
                event_name,
                raw_data,
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
    async fn test_log_tailer_basic() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Initial line")?;
        temp_file.flush()?;

        let mut tailer = LogTailer::new(temp_file.path(), false).await?;

        // Add new content
        writeln!(temp_file, "[UnityCrossThreadLogger]1/1/2025 12:00:00 PM")?;
        writeln!(temp_file, "==> TestEvent")?;
        writeln!(temp_file, "{{\"test\": \"data\"}}")?;
        temp_file.flush()?;

        // Read new events
        let events = tailer.read_new_events().await?;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_name, "TestEvent");

        Ok(())
    }
}
