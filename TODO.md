# MTG MCP Server Improvement Tasks

## High Priority Tasks

### 1. Create Custom Prompts

**Reason**: AI assistants currently must construct all queries from scratch. Pre-built prompts would streamline common workflows and provide better user experience.

**Implementation**:

```rust
// In crates/mtg/src/mcp/prompts.rs (new file)
use mcp_core::types::{Prompt, PromptArgument};

pub fn deck_building_prompt() -> Prompt {
    Prompt {
        name: "deck_builder".to_string(),
        description: Some("Interactive deck building assistant for Magic: The Gathering".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "format".to_string(),
                description: Some("Target format (standard, modern, legacy, commander, etc.)".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "archetype".to_string(),
                description: Some("Deck archetype (aggro, control, midrange, combo)".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "budget".to_string(),
                description: Some("Budget constraint in USD".to_string()),
                required: Some(false),
            },
        ]),
    }
}

pub fn card_search_prompt() -> Prompt {
    Prompt {
        name: "card_searcher".to_string(),
        description: Some("Advanced card search with natural language queries".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "description".to_string(),
                description: Some("Natural language description of desired card".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "format".to_string(),
                description: Some("Format legality requirement".to_string()),
                required: Some(false),
            },
        ]),
    }
}

pub fn synergy_finder_prompt() -> Prompt {
    Prompt {
        name: "synergy_finder".to_string(),
        description: Some("Find cards that synergize with your existing cards".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "cards".to_string(),
                description: Some("Comma-separated list of card names".to_string()),
                required: Some(true),
            },
            PromptArgument {
                name: "format".to_string(),
                description: Some("Format constraint".to_string()),
                required: Some(false),
            },
        ]),
    }
}

// In crates/mtg/src/mcp/mod.rs
mod prompts;

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    let server_protocol = Server::builder(/* ... */)
        .set_capabilities(ServerCapabilities {
            tools: Some(ToolCapabilities::default()),
            prompts: Some(PromptCapabilities::default()),
            ..Default::default()
        })
        .register_prompt(prompts::deck_building_prompt())
        .register_prompt(prompts::card_search_prompt())
        .register_prompt(prompts::synergy_finder_prompt())
        // ... existing tools
        .build();
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/prompts.rs` (new)
- `crates/mtg/src/mcp/mod.rs`

**Success Criteria**:

- [ ] 3+ prompts registered and functional
- [ ] Prompts work correctly in Claude Desktop
- [ ] Documentation updated with prompt examples
- [ ] Prompts provide meaningful assistance for common workflows
- [ ] Tests verify prompt registration and functionality

### 3. Implement Resources

**Reason**: Resources allow AI assistants to browse and explore data without specific queries, enabling more natural data discovery workflows. The cards resource must be carefully implemented with search/pagination to avoid context overflow with 20,000+ cards.

**Implementation**:

```rust
// In crates/mtg/src/mcp/resources.rs (new file)
use mcp_core::types::{Resource, ResourceContents, TextResourceContents};
use serde_json::json;

pub fn cards_resource() -> Resource {
    Resource {
        uri: "mtg://cards".to_string(),
        name: "MTG Cards Database".to_string(),
        description: Some("Magic: The Gathering card database with search capabilities. Use query parameters: ?query=<scryfall_query>&page=<number>&limit=<number>".to_string()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
    }
}

pub fn sets_resource() -> Resource {
    Resource {
        uri: "mtg://sets".to_string(),
        name: "MTG Sets Database".to_string(),
        description: Some("All Magic sets from Alpha to present with metadata".to_string()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
    }
}

pub fn types_resource() -> Resource {
    Resource {
        uri: "mtg://types".to_string(),
        name: "MTG Types System".to_string(),
        description: Some("Card types, subtypes, supertypes, and format information".to_string()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
    }
}

pub async fn handle_cards_resource(uri: &str) -> Result<ResourceContents> {
    // Parse URI for query parameters: mtg://cards?query=c:red&page=1&limit=50
    let url = url::Url::parse(&format!("http://dummy.com{}", &uri[5..]))?; // Remove "mtg://"
    let query_pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();

    if let Some(search_query) = query_pairs.get("query") {
        // Use search query to filter cards via Scryfall
        let page = query_pairs.get("page")
            .and_then(|p| p.parse::<u32>().ok())
            .unwrap_or(1);
        let limit = query_pairs.get("limit")
            .and_then(|l| l.parse::<u32>().ok())
            .unwrap_or(25)
            .min(175); // Scryfall max

        // Execute search with pagination
        let search_results = execute_paginated_search(search_query, page, limit).await?;

        let response_data = json!({
            "description": "Magic: The Gathering Cards Database - Search Results",
            "query": search_query,
            "page": page,
            "limit": limit,
            "total_cards": search_results.total_cards,
            "has_more": search_results.has_more,
            "cards": search_results.data,
            "next_page": if search_results.has_more {
                Some(format!("mtg://cards?query={}&page={}&limit={}",
                    urlencoding::encode(search_query), page + 1, limit))
            } else { None }
        });

        Ok(ResourceContents::Text(TextResourceContents {
            text: serde_json::to_string_pretty(&response_data)?,
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
        }))
    } else {
        // Return overview with usage instructions - NO ACTUAL CARD DATA
        let overview_data = json!({
            "description": "Magic: The Gathering Cards Database",
            "total_cards": "20000+",
            "usage": {
                "search": "Add ?query=<scryfall_query> to search cards",
                "pagination": "Add &page=<number>&limit=<number> for pagination",
                "examples": [
                    "mtg://cards?query=c:red t:creature",
                    "mtg://cards?query=Lightning Bolt",
                    "mtg://cards?query=f:standard r:rare&limit=10"
                ]
            },
            "supported_queries": [
                "Card names: 'Lightning Bolt'",
                "Colors: 'c:red', 'c:wu'",
                "Types: 't:creature', 't:instant'",
                "Formats: 'f:standard', 'f:modern'",
                "Mana cost: 'mana=3', 'mana>=4'",
                "Complex: 'c:red t:creature mana<=3 f:standard'"
            ],
            "formats": ["standard", "modern", "legacy", "vintage", "commander"]
        });

        Ok(ResourceContents::Text(TextResourceContents {
            text: serde_json::to_string_pretty(&overview_data)?,
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
        }))
    }
}

async fn execute_paginated_search(query: &str, page: u32, limit: u32) -> Result<SearchResults> {
    // Implementation that uses existing Scryfall search functionality
    // with proper pagination support
    let global = crate::Global {
        api_base_url: "https://api.scryfall.com".to_string(),
        verbose: false,
        timeout: 30,
    };

    // Use existing scryfall search with pagination
    crate::scryfall::search_cards_paginated(query, page, limit, global).await
}

// Similar handlers for sets and types...

// In crates/mtg/src/mcp/mod.rs
mod resources;

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    let server_protocol = Server::builder(/* ... */)
        .set_capabilities(ServerCapabilities {
            tools: Some(ToolCapabilities::default()),
            resources: Some(ResourceCapabilities::default()),
            ..Default::default()
        })
        .register_resource(resources::cards_resource(), resources::handle_cards_resource)
        .register_resource(resources::sets_resource(), resources::handle_sets_resource)
        .register_resource(resources::types_resource(), resources::handle_types_resource)
        // ... existing tools
        .build();
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/resources.rs` (new)
- `crates/mtg/src/mcp/mod.rs`

**Success Criteria**:

- [ ] 3 resources registered and accessible
- [ ] Cards resource supports query parameters and pagination
- [ ] Cards resource never returns full database (context safety)
- [ ] Resources work in Claude Desktop
- [ ] Clear usage instructions and examples provided
- [ ] Pagination works correctly with next_page links
- [ ] Search queries are properly validated and executed
- [ ] Tests verify resource functionality and context limits

### 4. Add Advanced Analysis Tools

**Reason**: Provide unique value through AI-powered insights that go beyond simple data retrieval.

**Implementation**:

```rust
// In crates/mtg/src/mcp/analysis/ (new directory)
// crates/mtg/src/mcp/analysis/synergies.rs
use mcp_core::{tool_text_response, tools::ToolHandlerFn, types::{CallToolRequest, Tool}};
use serde_json::json;

pub struct SynergyFinder;

impl SynergyFinder {
    pub fn tool() -> Tool {
        Tool {
            name: "find_card_synergies".to_string(),
            description: Some("Discover cards that synergize with the provided cards based on mechanics, types, and interactions".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "card_names": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of card names to find synergies for"
                    },
                    "format": {
                        "type": "string",
                        "description": "Format constraint (standard, modern, legacy, commander)",
                        "enum": ["standard", "modern", "legacy", "vintage", "commander"]
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of synergistic cards to return",
                        "default": 10
                    }
                },
                "required": ["card_names"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                // Implementation logic:
                // 1. Get card details for input cards
                // 2. Analyze mechanics, types, keywords
                // 3. Search for cards with complementary abilities
                // 4. Score and rank synergies
                // 5. Return formatted results

                tool_text_response!("Synergy analysis implementation")
            })
        }
    }
}

// crates/mtg/src/mcp/analysis/improvements.rs
pub struct DeckImprover;

impl DeckImprover {
    pub fn tool() -> Tool {
        Tool {
            name: "suggest_deck_improvements".to_string(),
            description: Some("Analyze a deck and suggest improvements based on optimization goals".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "deck_list": {
                        "type": "string",
                        "description": "Deck list in standard format"
                    },
                    "optimization_goal": {
                        "type": "string",
                        "enum": ["speed", "consistency", "power", "budget"],
                        "description": "Primary optimization objective"
                    },
                    "format": {
                        "type": "string",
                        "description": "Target format"
                    }
                },
                "required": ["deck_list", "optimization_goal"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                // Implementation logic:
                // 1. Parse and analyze current deck
                // 2. Identify weaknesses based on goal
                // 3. Suggest card swaps/additions
                // 4. Provide reasoning for each suggestion

                tool_text_response!("Deck improvement analysis implementation")
            })
        }
    }
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/analysis/` (new directory)
- `crates/mtg/src/mcp/analysis/mod.rs` (new)
- `crates/mtg/src/mcp/analysis/synergies.rs` (new)
- `crates/mtg/src/mcp/analysis/improvements.rs` (new)
- `crates/mtg/src/mcp/mod.rs`

**Success Criteria**:

- [ ] Both analysis tools implemented and registered
- [ ] Tools provide meaningful, actionable insights
- [ ] Analysis considers format legality
- [ ] Results are well-formatted and explained
- [ ] Tools handle edge cases gracefully
- [ ] Tests verify analysis quality

### 5. Add Set Management Tools

**Reason**: Essential for comprehensive MTG data access. The CLI has robust set functionality that should be exposed via MCP.

**Implementation**:

```rust
// In crates/mtg/src/mcp/sets/ (new directory)
// crates/mtg/src/mcp/sets/list.rs
use mcp_core::{tool_text_response, tools::ToolHandlerFn, types::{CallToolRequest, Tool}};
use serde_json::json;

pub struct SetLister;

impl SetLister {
    pub fn tool() -> Tool {
        Tool {
            name: "list_sets".to_string(),
            description: Some("List Magic: The Gathering sets with comprehensive filtering options".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "set_type": {
                        "type": "string",
                        "description": "Filter by set type (core, expansion, masters, etc.)"
                    },
                    "released_after": {
                        "type": "string",
                        "description": "Filter sets released after this date (YYYY-MM-DD)"
                    },
                    "released_before": {
                        "type": "string",
                        "description": "Filter sets released before this date (YYYY-MM-DD)"
                    },
                    "block": {
                        "type": "string",
                        "description": "Filter by block name"
                    },
                    "digital_only": {
                        "type": "boolean",
                        "description": "Filter digital-only sets"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["table", "json"],
                        "description": "Output format",
                        "default": "table"
                    }
                }
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                // Reuse existing sets::list_sets functionality
                let global = crate::Global {
                    api_base_url: "https://api.scryfall.com".to_string(),
                    verbose: false,
                    timeout: 30,
                };

                // Parse parameters and call existing implementation
                match crate::sets::list_sets(params, global).await {
                    Ok(sets_list) => {
                        // Format response appropriately
                        tool_text_response!(format_sets_response(sets_list))
                    },
                    Err(e) => tool_text_response!(format!("Failed to list sets: {}", e)),
                }
            })
        }
    }
}

// crates/mtg/src/mcp/sets/info.rs
pub struct SetInfo;

impl SetInfo {
    pub fn tool() -> Tool {
        Tool {
            name: "get_set_info".to_string(),
            description: Some("Get detailed information about a specific Magic set".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "set_code": {
                        "type": "string",
                        "description": "3-letter set code (e.g., 'ktk', 'war', 'm21')"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["table", "json"],
                        "description": "Output format",
                        "default": "table"
                    }
                },
                "required": ["set_code"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                // Reuse existing sets::get_set_by_code functionality
                tool_text_response!("Set info implementation")
            })
        }
    }
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/sets/` (new directory)
- `crates/mtg/src/mcp/sets/mod.rs` (new)
- `crates/mtg/src/mcp/sets/list.rs` (new)
- `crates/mtg/src/mcp/sets/info.rs` (new)
- `crates/mtg/src/mcp/mod.rs`

**Success Criteria**:

- [ ] Both set tools implemented and registered
- [ ] Tools reuse existing CLI functionality
- [ ] Filtering options work correctly
- [ ] Output formatting is consistent
- [ ] Error handling is robust
- [ ] Tests verify set operations

### 6. Implement Batch Operations

**Reason**: Efficient multi-card retrieval reduces API calls and improves performance for bulk operations.

**Implementation**:

```rust
// In crates/mtg/src/mcp/batch.rs (new file)
use mcp_core::{tool_text_response, tools::ToolHandlerFn, types::{CallToolRequest, Tool}};
use serde_json::json;

pub struct BatchCardRetriever;

impl BatchCardRetriever {
    pub fn tool() -> Tool {
        Tool {
            name: "get_cards_batch".to_string(),
            description: Some("Retrieve multiple cards efficiently in a single request using various identifiers".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "identifiers": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string",
                                    "description": "Card name"
                                },
                                "set": {
                                    "type": "string",
                                    "description": "Set code"
                                },
                                "id": {
                                    "type": "string",
                                    "description": "Scryfall ID"
                                }
                            },
                            "anyOf": [
                                {"required": ["name"]},
                                {"required": ["id"]},
                                {"required": ["name", "set"]}
                            ]
                        },
                        "description": "Array of card identifiers (at least one identifier type required per card)",
                        "minItems": 1,
                        "maxItems": 75
                    },
                    "format": {
                        "type": "string",
                        "enum": ["detailed", "compact", "json"],
                        "description": "Output format",
                        "default": "detailed"
                    }
                },
                "required": ["identifiers"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                let empty_args = std::collections::HashMap::new();
                let args = request.arguments.as_ref().unwrap_or(&empty_args);

                if let Some(identifiers) = args.get("identifiers").and_then(|v| v.as_array()) {
                    // Use Scryfall's collection endpoint for batch retrieval
                    let mut results = Vec::new();

                    for identifier in identifiers {
                        // Process each identifier and collect results
                        // Handle different identifier types (name, id, name+set)
                    }

                    tool_text_response!(format_batch_results(results))
                } else {
                    tool_text_response!("Error: 'identifiers' parameter is required and must be an array.")
                }
            })
        }
    }
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/batch.rs` (new)
- `crates/mtg/src/mcp/mod.rs`

**Success Criteria**:

- [ ] Batch tool implemented and registered
- [ ] Supports multiple identifier types
- [ ] Handles up to 75 cards per request
- [ ] Efficient API usage (single request when possible)
- [ ] Clear error messages for invalid identifiers
- [ ] Tests verify batch functionality

### 7. Remove Redundant Tools

**Reason**: Simplify the API by removing tools with limited utility for AI assistants.

**Implementation**:

```rust
// In crates/mtg/src/mcp/mod.rs
pub async fn run_mcp_server(_global: crate::Global) -> Result<()> {
    let server_protocol = Server::builder(/* ... */)
        .register_tool(scryfall::named::Mcp::tool(), scryfall::named::Mcp::call())
        .register_tool(scryfall::id::Mcp::tool(), scryfall::id::Mcp::call())
        // REMOVE: .register_tool(scryfall::collector::Mcp::tool(), scryfall::collector::Mcp::call())
        // REMOVE: .register_tool(scryfall::random::Mcp::tool(), scryfall::random::Mcp::call())
        .register_tool(scryfall::autocomplete::Mcp::tool(), scryfall::autocomplete::Mcp::call())
        .register_tool(deck::analysis::Mcp::tool(), deck::analysis::Mcp::call())
        .register_tool(scryfall::search::Mcp::tool(), scryfall::search::Mcp::call())
        .build();
}
```

**Files to modify**:

- `crates/mtg/src/mcp/mod.rs`

**Files to remove**:

- `crates/mtg/src/mcp/scryfall/collector.rs`
- `crates/mtg/src/mcp/scryfall/random.rs`

**Success Criteria**:

- [ ] Redundant tools removed from server registration
- [ ] Tool files deleted
- [ ] Documentation updated to reflect changes
- [ ] No broken references remain
- [ ] Tests updated accordingly

## Medium Priority Tasks

### 1. Enhance Existing Tools

**Reason**: Improve usability and functionality of current tools with better options and error handling.

**Implementation**:

```rust
// Example enhancement for scryfall_search_cards
// In crates/mtg/src/mcp/scryfall/search.rs

impl Mcp {
    pub fn tool() -> Tool {
        Tool {
            name: "scryfall_search_cards".to_string(),
            description: Some("Search for Magic: The Gathering cards using Scryfall API with flexible query syntax and enhanced options".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    // ... existing properties ...
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results to return (1-175)",
                        "minimum": 1,
                        "maximum": 175,
                        "default": 25
                    },
                    "include_multilingual": {
                        "type": "boolean",
                        "description": "Include cards in other languages",
                        "default": false
                    },
                    "sort_direction": {
                        "type": "string",
                        "enum": ["auto", "asc", "desc"],
                        "description": "Sort direction",
                        "default": "auto"
                    }
                }
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        |request: CallToolRequest| {
            Box::pin(async move {
                // Enhanced error handling with suggestions
                match Self::execute_search(request).await {
                    Ok(result) => tool_text_response!(result),
                    Err(e) => {
                        let suggestion = Self::generate_error_suggestion(&e);
                        tool_text_response!(format!("Search failed: {}\n\nSuggestion: {}", e, suggestion))
                    }
                }
            })
        }
    }

    fn generate_error_suggestion(error: &Error) -> String {
        match error {
            Error::ScryfallApi(ScryfallApiError::NotFound { .. }) => {
                "Try using broader search terms or check spelling. Use scryfall_autocomplete_card_names for name suggestions.".to_string()
            },
            Error::ScryfallApi(ScryfallApiError::BadRequest { .. }) => {
                "Check your query syntax. Use simple terms like 'c:red t:creature' or card names.".to_string()
            },
            _ => "Try simplifying your search or check your network connection.".to_string()
        }
    }
}
```

**Files to modify**:

- `crates/mtg/src/mcp/scryfall/search.rs`
- `crates/mtg/src/mcp/scryfall/named.rs`
- `crates/mtg/src/mcp/deck/analysis.rs`
- `crates/mtg/src/mcp/scryfall/autocomplete.rs`

**Success Criteria**:

- [ ] All tools have enhanced parameter validation
- [ ] Improved error messages with actionable suggestions
- [ ] Better pagination support where applicable
- [ ] Result limiting options implemented
- [ ] Enhanced documentation for all parameters
- [ ] Tests verify enhancements work correctly

### 2. Performance Optimizations

**Reason**: Improve overall system performance and resource utilization for better user experience.

**Implementation**:

```rust
// In crates/mtg/src/mcp/performance.rs (new file)
use std::sync::Arc;
use tokio::sync::Semaphore;
use reqwest::Client;

pub struct PerformanceManager {
    client: Client,
    request_semaphore: Arc<Semaphore>,
    deduplication_cache: Arc<DashMap<String, Arc<tokio::sync::Mutex<Option<String>>>>>,
}

impl PerformanceManager {
    pub fn new() -> Self {
        let client = Client::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .gzip(true)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            request_semaphore: Arc::new(Semaphore::new(10)), // Limit concurrent requests
            deduplication_cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn deduplicated_request(&self, key: String, request_fn: impl Future<Output = Result<String>>) -> Result<String> {
        // Check if request is already in progress
        let mutex = self.deduplication_cache
            .entry(key.clone())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(None)))
            .clone();

        let mut guard = mutex.lock().await;

        if let Some(cached_result) = guard.as_ref() {
            return Ok(cached_result.clone());
        }

        // Execute request with semaphore limiting
        let _permit = self.request_semaphore.acquire().await?;
        let result = request_fn.await?;

        *guard = Some(result.clone());

        // Clean up after some time
        let cache_ref = self.deduplication_cache.clone();
        let key_clone = key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(60)).await;
            cache_ref.remove(&key_clone);
        });

        Ok(result)
    }
}

// In crates/mtg/src/mcp/mod.rs
mod performance;

pub async fn run_mcp_server(global: crate::Global) -> Result<()> {
    let performance_manager = Arc::new(performance::PerformanceManager::new());

    // Pass to all tools that need it
    let server_protocol = Server::builder(/* ... */)
        .register_tool(
            scryfall::search::Mcp::tool(),
            scryfall::search::Mcp::call_with_performance(performance_manager.clone())
        )
        // ... other tools
        .build();
}
```

**Files to create/modify**:

- `crates/mtg/src/mcp/performance.rs` (new)
- `crates/mtg/src/mcp/mod.rs`
- All tool implementation files

**Success Criteria**:

- [ ] Connection pooling implemented
- [ ] Request deduplication working
- [ ] Concurrent request limiting in place
- [ ] Response compression enabled
- [ ] Performance metrics available
- [ ] 20%+ improvement in response times under load
- [ ] Tests verify performance optimizations

## Implementation Order

1. **Caching Layer** - Foundation for performance improvements
2. **Remove Redundant Tools** - Simplify before adding complexity
3. **Custom Prompts** - High user impact, relatively simple
4. **Resources** - Enable new interaction patterns
5. **Set Management Tools** - Essential missing functionality
6. **Batch Operations** - Performance improvement for bulk operations
7. **Advanced Analysis Tools** - Complex but high value
8. **Tool Enhancements** - Polish existing functionality
9. **Performance Optimizations** - Final performance tuning

## Testing Strategy

Each task should include:

- Unit tests for core functionality
- Integration tests with MCP protocol
- Performance benchmarks where applicable
- Documentation updates
- Manual testing with Claude Desktop

## Success Metrics

- **Performance**: 50%+ improvement in cached response times
- **Functionality**: All documented tools working correctly
- **Usability**: Positive feedback from AI assistant interactions
- **Reliability**: <1% error rate under normal load
- **Documentation**: Complete coverage of all features
