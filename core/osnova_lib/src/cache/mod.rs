//! # Component Cache Module
//!
//! Local caching for downloaded components with LRU eviction.
//!
//! This module provides:
//! - Cache manager with configurable size limits
//! - LRU (Least Recently Used) eviction policy
//! - Platform-specific cache directories
//! - Thread-safe operations
//!
//! ## Platform Cache Locations
//!
//! - Linux: `~/.cache/osnova/components/`
//! - macOS: `~/Library/Caches/Osnova/components/`
//! - Windows: `%LOCALAPPDATA%\Osnova\Cache\components\`
//! - Android: `getExternalCacheDir()/components/`
//! - iOS: `Library/Caches/components/`
//!
//! ## Example
//!
//! ```rust,no_run
//! use osnova_lib::cache::CacheManager;
//! use osnova_lib::platform::paths::get_component_cache_dir;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let cache_dir = get_component_cache_dir()?;
//! let max_size = 500 * 1024 * 1024; // 500MB
//!
//! let cache = CacheManager::new(cache_dir, max_size)?;
//! # Ok(())
//! # }
//! ```

pub mod manager;

pub use manager::CacheManager;
