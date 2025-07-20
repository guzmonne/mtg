# MTG CLI - Scryfall API Enhancement Plan (WIP)

## 📋 **Project Overview**

This document outlines the comprehensive enhancement plan for the MTG CLI's Scryfall API integration. The goal is to transform our current basic implementation into a production-ready, feature-complete Magic: The Gathering card API client while maintaining full backward compatibility and existing cache behavior.

### **Current State**
- ✅ Basic card search with caching
- ✅ Advanced search parameter building  
- ✅ Single card lookup functionality
- ✅ Pretty table display output
- ✅ JSON response handling
- ✅ MCP tool integration

### **Target State**
- 🎯 Complete Scryfall API feature parity
- 🎯 Multi-format output support (JSON, CSV, text, images)
- 🎯 Comprehensive image caching system
- 🎯 Full multi-faced card support (DFCs, split cards, etc.)
- 🎯 Set management and browsing
- 🎯 Production-ready error handling and rate limiting
- 🎯 Enhanced caching with multiple data types

---

## 🏗️ **Implementation Phases**

### **Phase 1: Foundation & Core Data Structures** 
*Priority: HIGH | Estimated: 2-3 weeks*

#### **Task 1.1: Enhanced Error Handling System**
**Files to modify:** `src/error.rs`, `src/scryfall.rs`

**Subtasks:**
- [ ] Create comprehensive `ScryfallError` struct with all API error fields
- [ ] Implement error parsing for `status`, `code`, `details`, `type`, `warnings`
- [ ] Add structured error handling in all API functions
- [ ] Create error display formatting for CLI output
- [ ] Add warning collection and display system
- [ ] Update existing error handling to use new system

**Data Structures:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ScryfallError {
    pub object: String,           // Always "error"
    pub status: u16,              // HTTP status code  
    pub code: String,             // Computer-friendly error code
    pub details: String,          // Human-readable explanation
    pub error_type: Option<String>, // Additional context
    pub warnings: Option<Vec<String>>, // Non-fatal warnings
}

#[derive(Debug, thiserror::Error)]
pub enum ScryfallApiError {
    #[error("API Error ({code}): {details}")]
    ApiError { 
        code: String, 
        details: String, 
        status: u16,
        error_type: Option<String>,
        warnings: Vec<String>,
    },
    #[error("Rate limit exceeded: retry after {retry_after}s")]
    RateLimit { retry_after: u64 },
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Cache error: {0}")]  
    Cache(#[from] crate::cache::CacheError),
    #[error("Image error: {0}")]
    Image(#[from] ImageError),
}
```

#### **Task 1.2: Enhanced List Objects**
**Files to modify:** `src/scryfall.rs`

**Subtasks:**
- [ ] Replace current `ScryfallSearchResponse` with generic `ScryfallList<T>`
- [ ] Add support for `warnings` field in list responses
- [ ] Implement proper pagination state tracking
- [ ] Add `total_cards` handling for card lists
- [ ] Update all search functions to use new list structure
- [ ] Maintain backward compatibility for existing code

**Data Structures:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ScryfallList<T> {
    pub object: String,           // Always "list"
    pub data: Vec<T>,            // Array of requested objects
    pub has_more: bool,          // Pagination indicator
    pub next_page: Option<String>, // URI to next page
    pub total_cards: Option<u32>, // Total cards across all pages
    pub warnings: Option<Vec<String>>, // Non-fatal warnings
}

// Type aliases for backward compatibility
pub type ScryfallSearchResponse = ScryfallList<ScryfallCard>;
pub type ScryfallSetList = ScryfallList<ScryfallSet>;
```

#### **Task 1.3: Set Objects Implementation**
**Files to create:** `src/sets.rs`
**Files to modify:** `src/scryfall.rs`, `src/main.rs`

**Subtasks:**
- [ ] Create complete `ScryfallSet` struct with all API fields
- [ ] Implement set type enumeration with all 20+ types
- [ ] Add set lookup functions (by code, ID, TCGPlayer ID)
- [ ] Create set listing functionality
- [ ] Add set search capabilities
- [ ] Implement set icon downloading
- [ ] Add set-based card search

**Data Structures:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ScryfallSet {
    pub object: String,          // Always "set"
    pub id: String,              // UUID
    pub code: String,            // 3-6 letter set code
    pub mtgo_code: Option<String>, // MTGO-specific code
    pub arena_code: Option<String>, // Arena-specific code
    pub tcgplayer_id: Option<u32>, // TCGPlayer group ID
    pub name: String,            // English set name
    pub set_type: String,        // Classification
    pub released_at: Option<String>, // Release date
    pub block_code: Option<String>, // Block code
    pub block: Option<String>,   // Block name
    pub parent_set_code: Option<String>, // Parent set
    pub card_count: u32,         // Number of cards
    pub printed_size: Option<u32>, // Collector number denominator
    pub digital: bool,           // Video game only
    pub foil_only: bool,         // Only foil cards
    pub nonfoil_only: bool,      // Only nonfoil cards
    pub scryfall_uri: String,    // Scryfall website link
    pub uri: String,             // API link
    pub icon_svg_uri: String,    // Set icon SVG
    pub search_uri: String,      // Cards in set search URI
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetType {
    Core,           // Yearly core sets
    Expansion,      // Rotational expansion sets
    Masters,        // Reprint sets
    Alchemy,        // Arena Alchemy sets
    Masterpiece,    // Masterpiece series
    Arsenal,        // Commander gift sets
    FromTheVault,   // From the Vault sets
    Spellbook,      // Spellbook series
    PremiumDeck,    // Premium deck series
    DuelDeck,       // Duel decks
    DraftInnovation, // Special draft sets
    TreasureChest,  // MTGO treasure chests
    Commander,      // Commander precons
    Planechase,     // Planechase sets
    Archenemy,      // Archenemy sets
    Vanguard,       // Vanguard cards
    Funny,          // Un-sets and funny promos
    Starter,        // Starter/intro sets
    Box,            // Gift box sets
    Promo,          // Promotional cards
    Token,          // Token sets
    Memorabilia,    // Non-legal special cards
    Minigame,       // Minigame inserts
}
```

#### **Task 1.4: Enhanced Card Data Structures**
**Files to modify:** `src/scryfall.rs`

**Subtasks:**
- [ ] Expand `ScryfallCard` with missing API fields
- [ ] Add `CardFace` struct for multi-faced cards
- [ ] Implement `RelatedCard` for meld parts
- [ ] Create comprehensive `ImageUris` struct
- [ ] Add `Prices` struct for pricing data
- [ ] Implement `Legalities` struct
- [ ] Add frame effects and special properties
- [ ] Support printed text fields for non-English cards

**Data Structures:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CardFace {
    pub object: String,          // Always "card_face"
    pub name: String,            // Face name
    pub mana_cost: Option<String>, // Face mana cost
    pub type_line: String,       // Face type line
    pub oracle_text: Option<String>, // Face oracle text
    pub colors: Option<Vec<String>>, // Face colors
    pub power: Option<String>,   // Face power
    pub toughness: Option<String>, // Face toughness
    pub loyalty: Option<String>, // Face loyalty
    pub flavor_text: Option<String>, // Face flavor text
    pub illustration_id: Option<String>, // Face illustration ID
    pub image_uris: Option<ImageUris>, // Face images
    pub artist: Option<String>,  // Face artist
    pub artist_id: Option<String>, // Face artist ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedCard {
    pub object: String,          // Always "related_card"
    pub id: String,              // Related card ID
    pub component: String,       // Relationship type
    pub name: String,            // Related card name
    pub type_line: String,       // Related card type
    pub uri: String,             // Related card API URI
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUris {
    pub small: Option<String>,   // 146×204 thumbnail
    pub normal: Option<String>,  // 488×680 medium
    pub large: Option<String>,   // 672×936 large
    pub png: Option<String>,     // 745×1040 PNG
    pub art_crop: Option<String>, // Art only crop
    pub border_crop: Option<String>, // Border cropped
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub usd: Option<String>,     // USD price
    pub usd_foil: Option<String>, // USD foil price
    pub usd_etched: Option<String>, // USD etched price
    pub eur: Option<String>,     // EUR price
    pub eur_foil: Option<String>, // EUR foil price
    pub tix: Option<String>,     // MTGO tickets
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legalities {
    pub standard: Option<String>,
    pub future: Option<String>,
    pub historic: Option<String>,
    pub gladiator: Option<String>,
    pub pioneer: Option<String>,
    pub explorer: Option<String>,
    pub modern: Option<String>,
    pub legacy: Option<String>,
    pub pauper: Option<String>,
    pub vintage: Option<String>,
    pub penny: Option<String>,
    pub commander: Option<String>,
    pub oathbreaker: Option<String>,
    pub brawl: Option<String>,
    pub historicbrawl: Option<String>,
    pub alchemy: Option<String>,
    pub paupercommander: Option<String>,
    pub duel: Option<String>,
    pub oldschool: Option<String>,
    pub premodern: Option<String>,
    pub predh: Option<String>,
}

// Enhanced ScryfallCard with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct ScryfallCard {
    // Core identification
    pub object: String,
    pub id: String,
    pub oracle_id: Option<String>,
    pub multiverse_ids: Option<Vec<u32>>,
    pub mtgo_id: Option<u32>,
    pub arena_id: Option<u32>,
    pub tcgplayer_id: Option<u32>,
    pub cardmarket_id: Option<u32>,
    
    // Core card info
    pub name: String,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: String,
    
    // Multi-faced card support
    pub card_faces: Option<Vec<CardFace>>,
    pub all_parts: Option<Vec<RelatedCard>>,
    
    // Images
    pub highres_image: bool,
    pub image_status: String,
    pub image_uris: Option<ImageUris>,
    
    // Game data
    pub mana_cost: Option<String>,
    pub cmc: f64,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Vec<String>,
    pub keywords: Option<Vec<String>>,
    
    // Legality and formats
    pub legalities: Legalities,
    pub games: Vec<String>,
    pub reserved: bool,
    
    // Physical properties
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    
    // Set information
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: String,
    pub set_search_uri: String,
    pub scryfall_set_uri: String,
    pub rulings_uri: String,
    pub prints_search_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    
    // Flavor and art
    pub flavor_text: Option<String>,
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<String>>,
    pub illustration_id: Option<String>,
    
    // Frame and visual
    pub border_color: String,
    pub frame: String,
    pub frame_effects: Option<Vec<String>>,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    
    // Printed text (for non-English)
    pub printed_name: Option<String>,
    pub printed_type_line: Option<String>,
    pub printed_text: Option<String>,
    
    // Popularity and pricing
    pub edhrec_rank: Option<u32>,
    pub penny_rank: Option<u32>,
    pub prices: Option<Prices>,
    pub related_uris: Option<Value>,
    pub purchase_uris: Option<Value>,
    
    // Card back
    pub card_back_id: Option<String>,
}
```

---

### **Phase 2: Image Caching System**
*Priority: HIGH | Estimated: 2-3 weeks*

#### **Task 2.1: Image Cache Architecture**
**Files to create:** `src/image_cache.rs`
**Files to modify:** `src/cache.rs`, `Cargo.toml`

**Subtasks:**
- [ ] Design image cache directory structure
- [ ] Implement `ImageCache` struct with configuration
- [ ] Add image version enumeration (small, normal, large, png, etc.)
- [ ] Create face-specific image handling (front/back for DFCs)
- [ ] Implement cache size management and TTL
- [ ] Add image format validation
- [ ] Create cache metadata tracking

**Dependencies to add:**
```toml
[dependencies]
image = "0.24"           # Image format validation
tokio-fs = "0.1"         # Async file operations
sha2 = "0.10"            # Image integrity checking
```

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct ImageCache {
    cache_dir: PathBuf,          // Base cache directory
    max_size: u64,               // Maximum cache size in bytes
    ttl: Duration,               // Time-to-live for images
    config: ImageCacheConfig,
}

#[derive(Debug, Clone)]
pub struct ImageCacheConfig {
    pub max_api_cache_size: u64,     // Max API response cache
    pub max_image_cache_size: u64,   // Max image cache size
    pub image_ttl: Duration,         // Image TTL
    pub api_ttl: Duration,           // API response TTL
    pub cleanup_interval: Duration,   // Auto-cleanup frequency
    pub compression: bool,           // Compress cached data
    pub validate_images: bool,       // Validate image integrity
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageVersion {
    Small,      // 146×204 JPG thumbnail
    Normal,     // 488×680 JPG medium
    Large,      // 672×936 JPG large
    Png,        // 745×1040 PNG transparent
    BorderCrop, // 480×680 JPG cropped borders
    ArtCrop,    // Variable JPG art only
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageFace {
    Front,
    Back,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedImage {
    pub card_id: String,
    pub version: ImageVersion,
    pub face: Option<ImageFace>,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub cached_at: SystemTime,
    pub source_url: String,
    pub checksum: String,        // SHA-256 of image data
}
```

#### **Task 2.2: Image Download System**
**Files to modify:** `src/image_cache.rs`

**Subtasks:**
- [ ] Implement async image downloading with progress
- [ ] Add concurrent download management with rate limiting
- [ ] Create request deduplication (don't download same image twice)
- [ ] Implement HTTP 302 redirect handling for image URLs
- [ ] Add download retry logic with exponential backoff
- [ ] Create bandwidth-aware downloading
- [ ] Add image corruption detection and re-download

**Functions to implement:**
```rust
impl ImageCache {
    pub async fn get_image(&self, card_id: &str, version: ImageVersion, face: Option<ImageFace>) -> Result<PathBuf>;
    pub async fn download_image(&self, url: &str, card_id: &str, version: ImageVersion, face: Option<ImageFace>) -> Result<PathBuf>;
    pub async fn cache_card_images(&self, card: &ScryfallCard) -> Result<Vec<PathBuf>>;
    pub async fn cleanup_cache(&self) -> Result<CleanupStats>;
    pub fn get_cache_stats(&self) -> Result<CacheStats>;
    pub async fn validate_cache(&self) -> Result<ValidationReport>;
}
```

#### **Task 2.3: Cache Directory Management**
**Files to modify:** `src/image_cache.rs`

**Subtasks:**
- [ ] Create organized directory structure for images
- [ ] Implement cache size monitoring and enforcement
- [ ] Add LRU eviction for size management
- [ ] Create cache metadata persistence
- [ ] Implement atomic cache operations
- [ ] Add cache corruption recovery

**Directory Structure:**
```
~/.cache/mtg-cli/
├── api/                     # API response cache (existing)
│   └── responses/
├── images/                  # Image cache (new)
│   ├── cards/
│   │   ├── {card_id}/
│   │   │   ├── small.jpg
│   │   │   ├── normal.jpg
│   │   │   ├── large.jpg
│   │   │   ├── png.png
│   │   │   ├── border_crop.jpg
│   │   │   ├── art_crop.jpg
│   │   │   └── back_normal.jpg  # For DFCs
│   │   └── metadata.json    # Image cache metadata
│   └── sets/
│       └── {set_code}/
│           └── icon.svg
├── temp/                    # Temporary downloads
└── cache.db                 # Cache database (SQLite)
```

#### **Task 2.4: CLI Image Commands**
**Files to modify:** `src/scryfall.rs`, `src/main.rs`

**Subtasks:**
- [ ] Add `--download-images` flag to search commands
- [ ] Create `mtg scryfall images` subcommand group
- [ ] Implement image cache management commands
- [ ] Add image viewing integration (optional terminal display)
- [ ] Create cache statistics and cleanup commands

**New CLI Commands:**
```bash
# Download images for search results
mtg scryfall search "lightning bolt" --download-images --version large

# Image cache management
mtg scryfall images cache-stats
mtg scryfall images cleanup [--dry-run]
mtg scryfall images validate
mtg scryfall images clear [--confirm]

# Download specific card images
mtg scryfall images download <card_name> [--version <version>] [--face <front|back>]

# Set icon downloads
mtg scryfall images set-icon <set_code> [--output <path>]
```

---

### **Phase 3: Multi-Format Support & Advanced Features**
*Priority: MEDIUM | Estimated: 2-3 weeks*

#### **Task 3.1: Request Format System**
**Files to create:** `src/formats.rs`
**Files to modify:** `src/scryfall.rs`

**Subtasks:**
- [ ] Create `OutputFormat` enumeration
- [ ] Implement CSV export functionality
- [ ] Add text format support for plain text output
- [ ] Create image format request handling
- [ ] Add HTTP 302 redirect following for image/file formats
- [ ] Implement format-specific header parsing

**Data Structures:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Csv(CsvOptions),
    Text,
    Image(ImageVersion, Option<ImageFace>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CsvOptions {
    pub columns: Option<Vec<String>>,  // Custom column selection
    pub include_headers: bool,
    pub delimiter: char,
}

#[derive(Debug, Clone)]
pub struct FormatRequest {
    pub format: OutputFormat,
    pub pretty: bool,           // For JSON formatting
    pub version: Option<String>, // For image requests
    pub face: Option<String>,   // For DFC image requests
}
```

#### **Task 3.2: CSV Export System**
**Files to modify:** `src/formats.rs`

**Subtasks:**
- [ ] Implement card data to CSV conversion
- [ ] Add customizable column selection
- [ ] Create CSV header handling for pagination
- [ ] Add proper CSV escaping and formatting
- [ ] Implement streaming CSV for large datasets

**CSV Columns:**
```rust
pub enum CsvColumn {
    Name, ManaCost, Cmc, TypeLine, OracleText,
    Power, Toughness, Loyalty, Colors, ColorIdentity,
    Set, SetName, Rarity, Artist, FlavorText,
    CollectorNumber, Prices, Legalities, Keywords,
    Layout, FrameEffects, BorderColor, Frame,
    // ... all available card fields
}
```

#### **Task 3.3: Enhanced Pagination System**
**Files to create:** `src/pagination.rs`
**Files to modify:** `src/scryfall.rs`

**Subtasks:**
- [ ] Create pagination state management
- [ ] Implement auto-pagination for large result sets
- [ ] Add progress indicators for multi-page downloads
- [ ] Create resume capability for interrupted downloads
- [ ] Add memory-efficient streaming for large datasets
- [ ] Implement warning aggregation across pages

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct PaginationState {
    pub current_page: u32,
    pub total_cards: Option<u32>,
    pub has_more: bool,
    pub next_page_uri: Option<String>,
    pub warnings: Vec<String>,
    pub pages_fetched: u32,
    pub total_time: Duration,
}

#[derive(Debug, Clone)]
pub struct PaginationOptions {
    pub auto_paginate: bool,     // Fetch all pages automatically
    pub max_pages: Option<u32>,  // Limit total pages fetched
    pub page_size: Option<u32>,  // Cards per page (if supported)
    pub show_progress: bool,     // Display progress bar
    pub parallel_requests: u32,  // Concurrent page requests
}
```

---

### **Phase 4: Set Management & CLI Enhancement**
*Priority: MEDIUM | Estimated: 1-2 weeks*

#### **Task 4.1: Set Management Commands**
**Files to create:** `src/sets.rs`
**Files to modify:** `src/main.rs`, `src/scryfall.rs`

**Subtasks:**
- [ ] Implement set listing with filtering
- [ ] Add set lookup by code/ID/name
- [ ] Create set search functionality
- [ ] Add set-based card browsing
- [ ] Implement set icon downloading
- [ ] Create set type filtering and validation

**CLI Commands:**
```bash
# List all sets
mtg scryfall sets list [--type <type>] [--format table|json] [--released-after <date>]

# Get specific set information
mtg scryfall sets get <set_code> [--pretty] [--include-cards]

# Search sets by name/type
mtg scryfall sets search <query> [--type <type>] [--block <block>]

# List cards in a set
mtg scryfall sets cards <set_code> [--page <n>] [--pretty] [--download-images]

# Download set icon
mtg scryfall sets icon <set_code> [--output <path>] [--format svg|png]

# Set statistics
mtg scryfall sets stats <set_code>
```

#### **Task 4.2: Enhanced CLI User Experience**
**Files to modify:** `src/main.rs`, `src/scryfall.rs`

**Subtasks:**
- [ ] Add command autocompletion support
- [ ] Implement interactive mode for complex searches
- [ ] Create search result filtering and sorting
- [ ] Add output format selection for all commands
- [ ] Implement configuration file support
- [ ] Add verbose/debug output modes

**Configuration File:**
```toml
# ~/.config/mtg-cli/config.toml
[cache]
max_api_size = "100MB"
max_image_size = "1GB"
api_ttl = "1h"
image_ttl = "7d"
cleanup_interval = "1d"

[api]
timeout = 30
rate_limit = 50
retry_attempts = 3
user_agent = "mtg-cli/2.0"

[display]
default_format = "table"
page_size = 25
show_images = false
color_output = true

[images]
auto_download = false
default_version = "normal"
cache_all_versions = false
```

---

### **Phase 5: Production Readiness & Performance**
*Priority: MEDIUM | Estimated: 1-2 weeks*

#### **Task 5.1: Rate Limiting & HTTP Best Practices**
**Files to create:** `src/rate_limit.rs`
**Files to modify:** `src/scryfall.rs`

**Subtasks:**
- [ ] Implement rate limiting middleware (50-100 req/sec)
- [ ] Add exponential backoff for rate limit hits
- [ ] Create request queuing system
- [ ] Implement proper User-Agent strings
- [ ] Add request timeout handling
- [ ] Create connection pooling for efficiency

**Data Structures:**
```rust
#[derive(Debug)]
pub struct RateLimiter {
    requests_per_second: u32,
    burst_capacity: u32,
    current_tokens: Arc<Mutex<u32>>,
    last_refill: Arc<Mutex<Instant>>,
}

#[derive(Debug)]
pub struct RequestQueue {
    pending_requests: VecDeque<PendingRequest>,
    active_requests: u32,
    max_concurrent: u32,
}
```

#### **Task 5.2: Performance Optimization**
**Files to modify:** `src/cache.rs`, `src/scryfall.rs`

**Subtasks:**
- [ ] Implement response compression for cache
- [ ] Add connection pooling and keep-alive
- [ ] Create parallel request processing
- [ ] Optimize memory usage for large datasets
- [ ] Add request/response streaming
- [ ] Implement cache preloading strategies

#### **Task 5.3: Comprehensive Testing**
**Files to create:** `tests/integration/`, `tests/unit/`

**Subtasks:**
- [ ] Create unit tests for all new data structures
- [ ] Add integration tests with real API calls
- [ ] Implement cache behavior validation tests
- [ ] Create error handling and edge case tests
- [ ] Add performance benchmarks
- [ ] Create multi-format output validation tests

---

### **Phase 6: Documentation & Polish**
*Priority: LOW | Estimated: 1 week*

#### **Task 6.1: Documentation**
**Files to create:** `docs/api.md`, `docs/caching.md`, `docs/examples.md`
**Files to modify:** `README.md`

**Subtasks:**
- [ ] Update README with new features
- [ ] Create comprehensive API documentation
- [ ] Add caching system documentation
- [ ] Create usage examples and tutorials
- [ ] Document configuration options
- [ ] Add troubleshooting guide

#### **Task 6.2: Examples & Tutorials**
**Files to create:** `examples/`

**Subtasks:**
- [ ] Create basic usage examples
- [ ] Add advanced search examples
- [ ] Create image caching examples
- [ ] Add set management examples
- [ ] Create MCP integration examples

---

## 📊 **Success Metrics**

### **Functionality Metrics**
- [ ] All 20+ card layouts properly supported
- [ ] All Scryfall API endpoints accessible
- [ ] Multi-format output working (JSON, CSV, text, images)
- [ ] Image caching system operational
- [ ] Set management fully functional

### **Performance Metrics**
- [ ] Cache hit rate >80% for repeated queries
- [ ] API response time <2s for single requests
- [ ] Image download time <5s for normal-sized images
- [ ] Memory usage <100MB for typical operations
- [ ] Rate limiting compliance (no 429 errors)

### **Reliability Metrics**
- [ ] Error rate <1% with proper retry logic
- [ ] Cache corruption rate <0.1%
- [ ] Zero data loss during cache operations
- [ ] Graceful degradation when API unavailable
- [ ] 100% backward compatibility maintained

### **User Experience Metrics**
- [ ] Command completion time <500ms for cached results
- [ ] Clear error messages for all failure cases
- [ ] Intuitive CLI interface with helpful output
- [ ] Comprehensive help and documentation
- [ ] Zero breaking changes to existing workflows

---

## 🔄 **Migration Strategy**

### **Backward Compatibility**
- All existing CLI commands continue to work unchanged
- Existing cache format remains valid during transition
- JSON output structure maintains compatibility
- MCP tools continue functioning without modification
- Configuration migration handled automatically

### **Rollout Plan**
1. **Phase 1**: Core data structures (non-breaking changes)
2. **Phase 2**: Image caching (additive feature)
3. **Phase 3**: Multi-format support (additive feature)
4. **Phase 4**: Set management (new commands)
5. **Phase 5**: Performance optimization (transparent improvements)
6. **Phase 6**: Documentation and polish

### **Testing Strategy**
- Continuous integration testing at each phase
- Backward compatibility validation
- Performance regression testing
- Real API integration testing
- User acceptance testing with existing workflows

---

## 📋 **Task Tracking**

### **Phase 1 Progress** ✅ **COMPLETED**
- [x] Task 1.1: Enhanced Error Handling System ✅ **COMPLETED**
  - ✅ Created comprehensive `ScryfallError` struct with all API fields
  - ✅ Implemented `ScryfallApiError` enum with structured error types
  - ✅ Added warning collection and display system
  - ✅ Updated error parsing to use new comprehensive structures
- [x] Task 1.2: Enhanced List Objects ✅ **COMPLETED**
  - ✅ Replaced `ScryfallSearchResponse` with generic `ScryfallList<T>`
  - ✅ Added warnings field support to list objects
  - ✅ Updated pagination handling with optional total_cards field
  - ✅ Maintained backward compatibility with type alias
- [x] Task 1.3: Set Objects Implementation ✅ **COMPLETED**
  - ✅ Created complete `ScryfallSet` struct with 25+ API fields
  - ✅ Implemented `SetType` enum with all 23 documented set types
  - ✅ Added set lookup and listing functions with caching
  - ✅ Created CLI commands: `sets list`, `sets get`, `sets types`
  - ✅ Implemented pretty table displays for sets
- [x] Task 1.4: Enhanced Card Data Structures ✅ **SKIPPED**
  - ⚠️ Deferred to future phase - current card structure sufficient for Phase 1

### **Phase 2 Progress**
- [ ] Task 2.1: Image Cache Architecture
- [ ] Task 2.2: Image Download System
- [ ] Task 2.3: Cache Directory Management
- [ ] Task 2.4: CLI Image Commands

### **Phase 3 Progress**
- [ ] Task 3.1: Request Format System
- [ ] Task 3.2: CSV Export System
- [ ] Task 3.3: Enhanced Pagination System

### **Phase 4 Progress**
- [ ] Task 4.1: Set Management Commands
- [ ] Task 4.2: Enhanced CLI User Experience

### **Phase 5 Progress**
- [ ] Task 5.1: Rate Limiting & HTTP Best Practices
- [ ] Task 5.2: Performance Optimization
- [ ] Task 5.3: Comprehensive Testing

### **Phase 6 Progress**
- [ ] Task 6.1: Documentation
- [ ] Task 6.2: Examples & Tutorials

---

## 🎯 **Next Steps**

1. **Review and approve this plan** with stakeholders
2. **Set up development environment** with new dependencies
3. **Begin Phase 1, Task 1.1** (Enhanced Error Handling System)
4. **Create feature branch** for Phase 1 development
5. **Establish testing framework** for new features
6. **Set up CI/CD pipeline** for automated testing

---

*This document will be updated as implementation progresses. Each completed task should be marked with ✅ and include implementation notes.*