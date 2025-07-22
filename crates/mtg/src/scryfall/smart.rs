use crate::prelude::*;

use super::{detect_query_intent, validate_and_suggest_query, QueryIntent};

pub async fn run(query: &str, pretty: bool, page: u32, global: crate::Global) -> Result<()> {
    let query = query.trim();

    // Validate query first
    if let Err(validation_error) = validate_and_suggest_query(query) {
        aeprintln!("{validation_error}");
        return Ok(());
    }

    // Try to detect what kind of query this is
    if let Some(intent) = detect_query_intent(query) {
        match intent {
            QueryIntent::ExactCardName(name) => {
                if global.verbose {
                    aeprintln!("Auto-detected: Exact card name lookup for '{name}'");
                }
                super::search::by_name(&name, pretty, None, global).await
            }
            QueryIntent::SetCollector(set, collector) => {
                if global.verbose {
                    aeprintln!("Auto-detected: Set/collector lookup for {set} {collector}",);
                }
                super::search::by_collector(&set, &collector, None, pretty, global).await
            }
            QueryIntent::ArenaId(id) => {
                if global.verbose {
                    aeprintln!("Auto-detected: Arena ID lookup for {id}");
                }
                super::search::by_arena_id(id, pretty, global).await
            }
            QueryIntent::MtgoId(id) => {
                if global.verbose {
                    aeprintln!("Auto-detected: MTGO ID lookup for {id}");
                }
                super::search::by_mtgo_id(id, pretty, global).await
            }
            QueryIntent::ScryfallId(id) => {
                if global.verbose {
                    aeprintln!("Auto-detected: Scryfall UUID lookup");
                }
                super::search::by_id(&id, pretty, global).await
            }
            QueryIntent::SearchQuery(search_query) => {
                if global.verbose {
                    aeprintln!("Auto-detected: Search query '{search_query}'");
                }
                super::search::run(
                    super::search::Params {
                        query: search_query,
                        pretty,
                        page,
                        order: "name".to_string(),
                        dir: "auto".to_string(),
                        include_extras: false,
                        include_multilingual: false,
                        include_variations: false,
                        unique: "cards".to_string(),
                        csv: false,
                    },
                    global,
                )
                .await
            }
        }
    } else {
        // Fallback to search if we can't detect intent
        if global.verbose {
            aeprintln!("Could not auto-detect intent, falling back to search");
        }
        super::search::run(
            super::search::Params {
                query: query.to_string(),
                pretty,
                page,
                order: "name".to_string(),
                dir: "auto".to_string(),
                include_extras: false,
                include_multilingual: false,
                include_variations: false,
                unique: "cards".to_string(),
                csv: false,
            },
            global,
        )
        .await
    }
}
