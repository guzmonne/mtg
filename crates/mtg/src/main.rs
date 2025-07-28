#![allow(clippy::large_enum_variant)]
#![allow(clippy::uninlined_format_args)]

use crate::prelude::*;
use clap::Parser;
use std::io::Write;

mod api;
mod cache;
// Companion module is only available on macOS and Windows since MTG Arena
// is not available on Linux platforms
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod companion;
mod completions;
mod decks;
mod error;
mod gatherer;
mod mcp;
mod prelude;
mod scryfall;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = "Magic The Gathering API CLI")]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,

    #[clap(flatten)]
    global: Global,
}

#[derive(Debug, Clone, clap::Args)]
pub struct Global {
    /// MTG API Base URL
    #[clap(
        long,
        env = "MTG_API_BASE_URL",
        global = true,
        default_value = "https://api.magicthegathering.io/v1"
    )]
    pub api_base_url: String,

    /// Request timeout in seconds
    #[clap(long, env = "MTG_TIMEOUT", global = true, default_value = "30")]
    pub timeout: u64,

    /// Whether to display additional information
    #[clap(long, env = "MTG_VERBOSE", global = true, default_value = "false")]
    pub verbose: bool,

    /// Scryfall API Base URL
    #[clap(
        long,
        env = "SCRYFALL_API_BASE_URL",
        global = true,
        default_value = "https://api.scryfall.com"
    )]
    pub scryfall_base_url: String,

    /// Custom User-Agent for Scryfall requests
    #[clap(long, env = "SCRYFALL_USER_AGENT", global = true)]
    pub scryfall_user_agent: Option<String>,

    /// Rate limit delay between Scryfall requests in milliseconds
    #[clap(
        long,
        env = "SCRYFALL_RATE_LIMIT_MS",
        global = true,
        default_value = "100"
    )]
    pub scryfall_rate_limit_ms: u64,

    /// Disable caching for all requests
    #[clap(long, global = true)]
    pub no_cache: bool,

    /// Clear cache before running command
    #[clap(long, global = true)]
    pub clear_cache: bool,

    /// Use custom cache directory
    #[clap(long, env = "MTG_CACHE_DIR", global = true, value_name = "PATH")]
    pub cache_dir: Option<std::path::PathBuf>,

    /// Set cache TTL in hours
    #[clap(long, env = "MTG_CACHE_TTL_HOURS", global = true, default_value = "24")]
    pub cache_ttl_hours: u64,
}

impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}

impl Global {
    /// Create a new Global configuration with sensible defaults
    pub fn new() -> Self {
        Self {
            api_base_url: "https://api.magicthegathering.io/v1".to_string(),
            verbose: false,
            timeout: 30,
            scryfall_base_url: "https://api.scryfall.com".to_string(),
            scryfall_user_agent: None,
            scryfall_rate_limit_ms: 100,
            no_cache: false,
            clear_cache: false,
            cache_dir: None,
            cache_ttl_hours: 24,
        }
    }

    /// Create a configured ScryfallClient from global options
    pub fn create_scryfall_client(&self) -> Result<mtg_core::ScryfallClient> {
        let mut builder = mtg_core::ScryfallClient::builder()
            .base_url(&self.scryfall_base_url)
            .timeout_secs(self.timeout)
            .verbose(self.verbose)
            .rate_limit_delay_ms(Some(self.scryfall_rate_limit_ms))
            .enable_cache(!self.no_cache);

        if let Some(user_agent) = &self.scryfall_user_agent {
            builder = builder.user_agent(user_agent);
        }

        if let Some(ref cache_dir) = self.cache_dir {
            builder = builder.cache_path(cache_dir);
        }

        if !self.no_cache {
            builder = builder.cache_ttl_secs(self.cache_ttl_hours * 3600);
        }

        builder.build()
    }

    /// Create a configured GathererClient from global options
    pub fn create_gatherer_client(&self) -> Result<mtg_core::GathererClient> {
        let mut builder = mtg_core::GathererClient::builder()
            .timeout_secs(self.timeout)
            .verbose(self.verbose)
            .enable_cache(!self.no_cache)
            .cache_ttl_hours(self.cache_ttl_hours);

        if let Some(ref cache_dir) = self.cache_dir {
            builder = builder.cache_dir(cache_dir);
        }

        builder.build().map_err(|e| eyre!(e))
    }
}

#[derive(Debug, clap::Parser)]
pub enum McpCommands {
    /// Start MCP server with STDIO transport (default)
    Stdio,

    /// Start MCP server with SSE transport (HTTP endpoints)
    Sse {
        /// Host to bind to
        #[clap(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[clap(long, default_value = "3000")]
        port: u16,
    },
}

#[derive(Debug, clap::Parser)]
pub enum CacheCommands {
    /// Show cache statistics
    Stats {
        /// Show statistics for specific prefix only
        #[clap(long)]
        prefix: Option<String>,
    },

    /// Clear cached data
    Clear {
        /// Only clear specific prefix
        #[clap(long)]
        prefix: Option<String>,

        /// Confirm clearing without prompt
        #[clap(long, short = 'y')]
        yes: bool,
    },

    /// Clean old cache entries
    Clean {
        /// Remove entries older than N hours
        #[clap(long, default_value = "24")]
        older_than_hours: u64,

        /// Target size in MB to clean cache down to
        #[clap(long)]
        target_size_mb: Option<u64>,
    },

    /// List cache entries
    List {
        /// Filter by prefix
        #[clap(long)]
        prefix: Option<String>,

        /// Show detailed information
        #[clap(long, short = 'v')]
        verbose: bool,
    },
}

#[derive(Debug, clap::Parser)]
pub enum SubCommands {
    /// Access the MTG API directly
    Api {
        #[command(subcommand)]
        command: crate::api::ApiCommands,
    },

    /// Search cards using Wizards' Gatherer advanced search
    Gatherer(crate::gatherer::App),

    /// Search cards using Scryfall advanced search
    Scryfall(crate::scryfall::App),

    /// Generate shell completions
    Completions(crate::completions::App),

    /// Analyze Magic: The Gathering deck lists
    Decks(crate::decks::App),

    /// Manage cache data
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },

    /// Start Model Context Protocol server for AI integration (defaults to STDIO)
    Mcp {
        #[command(subcommand)]
        command: Option<McpCommands>,
    },

    /// Track and analyze MTG Arena log files
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Companion(crate::companion::App),
}

/// Execute cache management commands
async fn execute_cache_command(cmd: CacheCommands, global: &Global) -> Result<()> {
    use mtg_core::cache::DiskCache;

    let cache = if let Some(ref cache_dir) = global.cache_dir {
        DiskCache::builder()
            .base_path(cache_dir)
            .prefix("scryfall")
            .build()?
    } else {
        DiskCache::builder().prefix("scryfall").build()?
    };

    match cmd {
        CacheCommands::Stats { prefix } => {
            let stats = cache.stats(prefix.as_deref()).await?;
            println!("Cache Statistics:");
            println!("  Total files: {}", stats.total_files);
            println!(
                "  Total size: {:.2} MB",
                stats.total_size as f64 / 1_048_576.0
            );

            if !stats.prefixes.is_empty() {
                println!("  Prefixes: {}", stats.prefixes.join(", "));
            }
        }

        CacheCommands::Clear { prefix, yes } => {
            if !yes {
                print!("Are you sure you want to clear the cache? [y/N] ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled");
                    return Ok(());
                }
            }

            if let Some(prefix) = prefix {
                let report = cache.clean_prefix(&prefix).await?;
                println!(
                    "Cleared {} entries with prefix '{}' ({:.2} MB)",
                    report.removed_count,
                    prefix,
                    report.freed_bytes as f64 / 1_048_576.0
                );
            } else {
                let report = cache.clean_all().await?;
                println!(
                    "Cleared {} cache entries ({:.2} MB)",
                    report.removed_count,
                    report.freed_bytes as f64 / 1_048_576.0
                );
            }
        }

        CacheCommands::Clean {
            older_than_hours,
            target_size_mb,
        } => {
            let older_than = std::time::Duration::from_secs(older_than_hours * 3600);

            if let Some(target_mb) = target_size_mb {
                let target_bytes = target_mb * 1_048_576;
                let report = cache.clean_to_size_limit(target_bytes, None).await?;
                println!(
                    "Cleaned {} entries ({:.2} MB)",
                    report.removed_count,
                    report.freed_bytes as f64 / 1_048_576.0
                );
            } else {
                let report = cache.clean_older_than(older_than, None).await?;
                println!(
                    "Cleaned {} entries older than {} hours ({:.2} MB)",
                    report.removed_count,
                    older_than_hours,
                    report.freed_bytes as f64 / 1_048_576.0
                );
            }
        }

        CacheCommands::List { prefix, verbose: _ } => {
            let prefixes = cache.list_prefixes().await?;

            if prefixes.is_empty() {
                println!("No cache prefixes found");
                return Ok(());
            }

            println!("Cache prefixes:");
            for prefix_name in prefixes {
                if let Some(ref filter_prefix) = prefix {
                    if !prefix_name.contains(filter_prefix) {
                        continue;
                    }
                }
                let stats = cache.stats(Some(&prefix_name)).await?;
                println!(
                    "  {} ({} files, {:.2} MB)",
                    prefix_name,
                    stats.total_files,
                    stats.total_size as f64 / 1_048_576.0
                );
            }
        }
    }

    Ok(())
}

/// Clear the cache directory
fn clear_cache(global: &Global) -> Result<()> {
    use mtg_core::cache::DiskCache;

    let cache = if let Some(ref cache_dir) = global.cache_dir {
        DiskCache::builder()
            .base_path(cache_dir)
            .prefix("scryfall")
            .build()?
    } else {
        DiskCache::builder().prefix("scryfall").build()?
    };

    tokio::runtime::Runtime::new()?.block_on(async {
        use mtg_core::cache::CacheStore;
        CacheStore::<String, String>::clear(&cache).await
    })?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let app = App::parse();

    // Clear cache if requested
    if app.global.clear_cache {
        clear_cache(&app.global)?;
        println!("Cache cleared");
    }

    match app.command {
        SubCommands::Api { command } => command.run().await,
        SubCommands::Gatherer(sub_app) => crate::gatherer::run(sub_app, app.global).await,
        SubCommands::Scryfall(sub_app) => crate::scryfall::run(sub_app, app.global).await,

        SubCommands::Completions(sub_app) => crate::completions::run(sub_app, app.global).await,
        SubCommands::Decks(sub_app) => crate::decks::run(sub_app, app.global).await,
        SubCommands::Cache { command } => execute_cache_command(command, &app.global).await,
        SubCommands::Mcp { command } => match command {
            Some(McpCommands::Stdio) | None => crate::mcp::run_mcp_server(app.global).await,
            Some(McpCommands::Sse { host, port }) => {
                crate::mcp::run_sse_server(app.global, host, port).await
            }
        },
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        SubCommands::Companion(sub_app) => crate::companion::run(sub_app, app.global).await,
    }
    .map_err(|err: color_eyre::eyre::Report| eyre!(err))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_new() {
        let global = Global::new();
        assert_eq!(global.api_base_url, "https://api.magicthegathering.io/v1");
        assert!(!global.verbose);
        assert_eq!(global.timeout, 30);
        assert_eq!(global.scryfall_base_url, "https://api.scryfall.com");
        assert!(global.scryfall_user_agent.is_none());
        assert_eq!(global.scryfall_rate_limit_ms, 100);
        assert!(!global.no_cache);
        assert!(!global.clear_cache);
        assert!(global.cache_dir.is_none());
        assert_eq!(global.cache_ttl_hours, 24);
    }

    #[test]
    fn test_global_default() {
        let global_new = Global::new();
        let global_default = Global::default();

        assert_eq!(global_new.api_base_url, global_default.api_base_url);
        assert_eq!(global_new.verbose, global_default.verbose);
        assert_eq!(global_new.timeout, global_default.timeout);
        assert_eq!(
            global_new.scryfall_base_url,
            global_default.scryfall_base_url
        );
        assert_eq!(
            global_new.scryfall_user_agent,
            global_default.scryfall_user_agent
        );
        assert_eq!(
            global_new.scryfall_rate_limit_ms,
            global_default.scryfall_rate_limit_ms
        );
        assert_eq!(global_new.no_cache, global_default.no_cache);
        assert_eq!(global_new.clear_cache, global_default.clear_cache);
        assert_eq!(global_new.cache_dir, global_default.cache_dir);
        assert_eq!(global_new.cache_ttl_hours, global_default.cache_ttl_hours);
    }

    #[test]
    fn test_scryfall_client_creation() {
        let global = Global::new();
        let client = global.create_scryfall_client();
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://api.scryfall.com");
        assert!(!client.is_verbose()); // Default is false
    }

    #[test]
    fn test_gatherer_client_creation() {
        let global = Global::new();
        let client = global.create_gatherer_client();
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(!client.is_verbose()); // Default is false
    }

    #[tokio::test]
    async fn test_scryfall_search_integration() {
        use mtg_core::ScryfallSearchParams;

        let global = Global::new();
        let client = global
            .create_scryfall_client()
            .expect("Failed to create client");

        // Test building an advanced query
        let query = client.build_advanced_query(&mtg_core::ScryfallAdvancedSearchParams {
            name: Some("Lightning Bolt".to_string()),
            card_type: Some("instant".to_string()),
            colors: Some("red".to_string()),
            ..Default::default()
        });

        assert!(query.contains("\"Lightning Bolt\""));
        assert!(query.contains("t:instant"));
        assert!(query.contains("c:r"));

        // Test search params creation
        let search_params = ScryfallSearchParams {
            q: query,
            unique: Some("cards".to_string()),
            order: Some("name".to_string()),
            ..Default::default()
        };

        // We don't actually make the API call in tests to avoid rate limiting
        // but we can verify the parameters are set correctly
        assert!(search_params.q.contains("Lightning Bolt"));
        assert_eq!(search_params.unique, Some("cards".to_string()));
        assert_eq!(search_params.order, Some("name".to_string()));
    }
}
