use crate::prelude::*;

mod async_processor;
mod display;
mod parser;
mod resolver;
mod state;
mod tailer;
mod types;

use async_processor::{AsyncProcessor, AsyncTask};
use display::MatchDisplay;
use parser::{EventParser, ParsedEvent};
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
    // Determine log file path
    let log_path = if let Some(path) = params.log_path {
        std::path::PathBuf::from(path)
    } else {
        // Use default log directory and find newest file
        let log_dir = crate::companion::parse::get_default_log_path()?;
        crate::companion::parse::find_newest_log_file(&log_dir)?
    };

    aeprintln!("Watching MTG Arena log file: {}", log_path.display());
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

    // Start tailing and processing events
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
