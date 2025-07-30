use crate::prelude::*;

mod async_processor;
mod display;
mod parser;
mod player_parser;
mod player_tailer;
mod resolver;
mod state;
mod tailer;
mod types;

use async_processor::{AsyncProcessor, AsyncTask};
use display::MatchDisplay;
use parser::{EventParser, ParsedEvent};
use player_parser::PlayerEventParser;
use player_tailer::PlayerLogTailer;
use resolver::CardResolver;
use tokio::sync::mpsc;

use tailer::LogTailer;
use types::RawLogEvent;

pub struct Params {
    pub log_path: Option<String>,
    pub filter: Option<Vec<String>>,
    pub format: String,
    pub from_beginning: bool,
    pub verbose: bool,
}

pub async fn run(params: Params) -> Result<()> {
    // Determine main log file path
    let log_path = if let Some(path) = params.log_path {
        let path_buf = std::path::PathBuf::from(path);
        if path_buf.is_dir() {
            // If it's a directory, find the newest log file in it
            crate::companion::parse::find_newest_log_file(&path_buf)?
        } else {
            // If it's a file, use it directly
            path_buf
        }
    } else {
        // Use default log directory and find newest file
        let log_dir = crate::companion::parse::get_default_log_path()?;
        crate::companion::parse::find_newest_log_file(&log_dir)?
    };

    // Try to get Player.log path
    let player_log_path: Option<std::path::PathBuf> =
        crate::companion::parse::get_player_log_path().ok();

    aeprintln!("Watching MTG Arena log file: {}", log_path.display());
    if let Some(ref player_path) = player_log_path {
        aeprintln!("Also watching Player.log file: {}", player_path.display());
    } else {
        aeprintln!("Player.log not found - continuing with main log only");
    }
    aeprintln!("Press Ctrl+C to stop watching...\n");

    // Create channel for async tasks
    let (tx, rx) = mpsc::channel::<AsyncTask>(100);

    // Spawn async processor
    let processor = AsyncProcessor::new(rx);
    let processor_handle = tokio::spawn(async move {
        if let Err(e) = processor.run().await {
            eprintln!("Async processor error: {}", e);
        }
    });

    // Initialize components
    let mut tailer = LogTailer::new(&log_path, params.from_beginning).await?;
    let mut parser = EventParser::new();
    let mut resolver = CardResolver::new();
    let display = MatchDisplay::new()
        .with_colors(params.format == "pretty")
        .with_detailed_actions(true);

    // Clone parameters for player log processing
    let tx_clone = tx.clone();
    let filter_clone = params.filter.clone();
    let verbose_clone = params.verbose;

    // Start Player.log processing in a separate task if available
    let player_handle = if let Some(player_path) = player_log_path {
        let player_path_clone = player_path.clone();
        let from_beginning = params.from_beginning;

        Some(tokio::spawn(async move {
            match PlayerLogTailer::new(&player_path_clone, from_beginning).await {
                Ok(mut player_tailer) => {
                    let mut player_parser = PlayerEventParser::new();

                    if let Err(e) = player_tailer
                        .tail_with_callback(|raw_event| {
                            process_player_event(
                                raw_event,
                                &mut player_parser,
                                &filter_clone,
                                verbose_clone,
                            )
                        })
                        .await
                    {
                        aeprintln!("Player.log processing error: {}", e);
                    }
                }
                Err(e) => {
                    aeprintln!("Failed to initialize Player.log tailer: {}", e);
                }
            }
        }))
    } else {
        None
    };

    // Start main log tailing and processing events
    let result = tailer
        .tail_with_callback(|raw_event| {
            process_raw_event(
                raw_event,
                &mut parser,
                &mut resolver,
                &display,
                &params.filter,
                params.verbose,
                &tx,
            )
        })
        .await;

    // Clean up
    drop(tx); // Close the channel
    let _ = processor_handle.await;

    // Clean up player log task if it exists
    if let Some(handle) = player_handle {
        handle.abort();
    }

    result
}

fn process_raw_event(
    raw_event: RawLogEvent,
    parser: &mut EventParser,
    resolver: &mut CardResolver,
    display: &MatchDisplay,
    filter: &Option<Vec<String>>,
    verbose: bool,
    async_tx: &mpsc::Sender<AsyncTask>,
) -> Result<()> {
    if verbose {
        aeprintln!("🔍 Processing event: {}", raw_event.event_name);
    }

    // Parse the raw event
    if let Some(event) = parser.parse_event(raw_event.clone(), async_tx.clone())? {
        if verbose {
            aeprintln!("🎯 Parsed event: {:?}", std::mem::discriminant(&event));
        }

        // Apply filters if specified
        if let Some(filters) = filter {
            if !should_show_event(&event, filters) {
                if verbose {
                    aeprintln!("🚫 Event filtered out");
                }
                return Ok(());
            }
        }

        // Handle the parsed event
        handle_event(event, parser, resolver, display)?;
    } else if verbose {
        aeprintln!(
            "❌ No event parsed from raw event: {}",
            raw_event.event_name
        );
    }

    Ok(())
}

fn process_player_event(
    raw_event: RawLogEvent,
    player_parser: &mut PlayerEventParser,
    filter: &Option<Vec<String>>,
    verbose: bool,
) -> Result<()> {
    if verbose {
        aeprintln!("🔍 Processing Player.log event: {}", raw_event.event_name);
    }

    // Apply basic filtering for player events
    if let Some(filters) = filter {
        let should_show = match raw_event.event_name.as_str() {
            "PlayerMatchStarted" => filters.contains(&"match".to_string()),
            "PlayerLifeChange" => filters.contains(&"life".to_string()),
            "PlayerCardDrawn" | "PlayerCardPlayed" | "PlayerCardRevealed" => {
                filters.contains(&"cards".to_string())
            }
            "PlayerActionTaken"
            | "PlayerAbilityActivated"
            | "PlayerSpellCast"
            | "PlayerTargetSelection"
            | "PlayerManaPaid"
            | "PlayerPermanentTapped" => filters.contains(&"actions".to_string()),
            "PlayerPhaseChange" => filters.contains(&"turns".to_string()),
            "PlayerMulligan" => filters.contains(&"mulligans".to_string()),
            "PlayerAttackers" | "PlayerBlockers" => filters.contains(&"combat".to_string()),
            "PlayerDeckInfo" => filters.contains(&"deck".to_string()),
            "PlayerGameState"
            | "PlayerSpellResolution"
            | "PlayerCounterChange"
            | "PlayerPriorityPass" => filters.contains(&"game".to_string()),
            _ => true, // Show other events by default
        };

        if !should_show && !filters.is_empty() {
            if verbose {
                aeprintln!("🚫 Player event filtered out");
            }
            return Ok(());
        }
    }

    // Parse the player event
    if let Err(e) = player_parser.parse_player_event(raw_event) {
        if verbose {
            aeprintln!("❌ Error parsing player event: {}", e);
        }
    }

    Ok(())
}

fn should_show_event(event: &ParsedEvent, filters: &[String]) -> bool {
    // If no filters specified, show everything
    if filters.is_empty() {
        return true;
    }

    // Check if event matches any of the filters
    for filter in filters {
        let matches = match filter.as_str() {
            "auth" | "authentication" => {
                matches!(event, ParsedEvent::UserAuthenticated { .. })
            }
            "match" => {
                matches!(
                    event,
                    ParsedEvent::MatchStarted(_) | ParsedEvent::MatchEnded(_)
                )
            }
            "turns" => {
                matches!(event, ParsedEvent::TurnChange { .. })
            }
            "life" => {
                matches!(event, ParsedEvent::LifeChange { .. })
            }
            "actions" => {
                matches!(
                    event,
                    ParsedEvent::GameAction(_) | ParsedEvent::CardPlayed { .. }
                )
            }
            "cards" => {
                matches!(event, ParsedEvent::CardPlayed { .. })
            }
            "mulligans" => {
                matches!(event, ParsedEvent::Mulligan { .. })
            }
            "draft" => {
                matches!(
                    event,
                    ParsedEvent::DraftPack { .. }
                        | ParsedEvent::DraftPick { .. }
                        | ParsedEvent::DraftCompleted
                )
            }
            "deck" => {
                matches!(event, ParsedEvent::DeckSubmitted { .. })
            }
            "combat" => {
                // This filter is mainly for Player.log events
                false
            }
            "game" => {
                // This filter is mainly for Player.log events
                false
            }
            _ => true, // Unknown filter, show everything
        };

        if matches {
            return true;
        }
    }

    false
}

fn handle_event(
    event: ParsedEvent,
    parser: &mut EventParser,
    _resolver: &mut CardResolver,
    display: &MatchDisplay,
) -> Result<()> {
    // Most events are now handled directly in the parser with println!
    // This function is kept for compatibility but most cases won't be reached
    match event {
        ParsedEvent::UserAuthenticated {
            user_id,
            display_name,
        } => {
            println!("🔐 User authenticated: {} ({})", display_name, user_id);
        }

        ParsedEvent::MatchStarted(match_state) => {
            display.display_match_start(&match_state)?;
        }

        ParsedEvent::MatchEnded(match_state) => {
            display.display_match_end(&match_state)?;
        }

        ParsedEvent::TurnChange {
            turn,
            active_player,
        } => {
            if let Some(current_match) = parser.current_match() {
                display.display_turn_change(turn, active_player, &current_match.players)?;
            }
        }

        ParsedEvent::LifeChange {
            player,
            old_life,
            new_life,
        } => {
            if let Some(current_match) = parser.current_match() {
                display.display_life_change(player, old_life, new_life, &current_match.players)?;
            }
        }

        ParsedEvent::GameAction(action) => {
            if let Some(current_match) = parser.current_match() {
                display.display_game_action(&action, &current_match.players)?;
            }
        }

        ParsedEvent::CardPlayed {
            player,
            card_name,
            zone_from,
            zone_to,
        } => {
            if let Some(current_match) = parser.current_match() {
                display.display_card_played(
                    player,
                    &card_name,
                    &zone_from,
                    &zone_to,
                    &current_match.players,
                )?;
            }
        }

        ParsedEvent::Mulligan {
            player,
            from_size,
            to_size,
        } => {
            if let Some(current_match) = parser.current_match() {
                display.display_mulligan(player, from_size, to_size, &current_match.players)?;
            }
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

        ParsedEvent::DeckSubmitted { deck_id, deck_name } => {
            // This case shouldn't be reached anymore as we handle it in the parser
            match (deck_id, deck_name) {
                (Some(id), Some(name)) => println!("🃏 Deck submitted: {} ({})", name, id),
                (Some(id), None) => println!("🃏 Deck submitted: {}", id),
                (None, Some(name)) => println!("🃏 Deck submitted: {}", name),
                (None, None) => println!("🃏 Deck submitted"),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::MatchState;

    #[test]
    fn test_filter_matching() {
        let event = ParsedEvent::MatchStarted(MatchState::default());
        let filters = vec!["match".to_string()];
        assert!(should_show_event(&event, &filters));

        let filters = vec!["turns".to_string()];
        assert!(!should_show_event(&event, &filters));
    }

    #[test]
    fn test_empty_filters() {
        let event = ParsedEvent::MatchStarted(MatchState::default());
        let filters = vec![];
        assert!(should_show_event(&event, &filters)); // Empty filters should show everything
    }
}
