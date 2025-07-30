pub mod cache;
pub mod companion;
pub mod decks;
pub mod gatherer;
pub mod scryfall;

// Re-export the ScryfallClient for easy access from the binary
pub use scryfall::{
    AdvancedSearchParams as ScryfallAdvancedSearchParams,
    AutocompleteResponse as ScryfallAutocompleteResponse, Card as ScryfallCard, ScryfallClient,
    ScryfallClientBuilder, ScryfallClientConfig, SearchParams as ScryfallSearchParams,
    SearchResponse as ScryfallSearchResponse,
};

// Re-export the GathererClient for easy access from the binary
pub use gatherer::{
    Card as GathererCard, GathererClient, SearchParams as GathererSearchParams,
    SearchResponse as GathererSearchResponse,
};

// Re-export cache types for easy access
pub use cache::{CacheStore, DiskCache, DiskCacheBuilder};

// Re-export deck types for easy access
pub use decks::{
    calculate_deck_stats, generate_short_hash, parse_deck_list,
    ranked::{
        ContentfulFields, ContentfulItem, ContentfulItemSys, ContentfulItemWithId,
        ContentfulResponse, ContentfulSys, ParsedDecksResponse, RankedDecksClient,
        RankedListParams, RankedListResponse,
    },
    DeckCard, DeckList, DeckStats, ParsedDeck,
};

// Re-export companion types for easy access
pub use companion::{
    event_parser::EventParser as CompanionEventParser,
    player_parser::PlayerEventParser as CompanionPlayerEventParser,
    types::{
        format_mana_cost, to_camel_case, zone_to_string, DraftState, GameAction, MatchState,
        ParsedEvent, PlayerEvent, PlayerInfo, RawLogEvent,
    },
};
