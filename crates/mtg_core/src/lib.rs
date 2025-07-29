pub mod cache;
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
