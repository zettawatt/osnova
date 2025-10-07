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
//! ```rust,ignore
//! use osnova_lib::cache::{CacheManager, get_platform_cache_dir};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let cache_dir = get_platform_cache_dir()?;
//!     let max_size = 500 * 1024 * 1024; // 500MB
//!
//!     let cache = CacheManager::new(cache_dir, max_size)?;
//!
//!     // Store component
//!     cache.store("my-component-v1.0.0", component_data).await?;
//!
//!     // Retrieve component
//!     if let Some(data) = cache.get("my-component-v1.0.0").await? {
//!         println!("Cache hit!");
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod manager;

pub use manager::CacheManager;

use crate::error::{OsnovaError, Result};
use std::path::PathBuf;

/// Get platform-specific cache directory for components
///
/// Returns the appropriate cache directory based on the current platform:
/// - Linux: `~/.cache/osnova/components/`
/// - macOS: `~/Library/Caches/Osnova/components/`
/// - Windows: `%LOCALAPPDATA%\Osnova\Cache\components\`
///
/// # Returns
///
/// * `Ok(PathBuf)` - Platform-specific cache directory
/// * `Err(OsnovaError::Storage)` - Failed to determine cache directory
///
/// # Example
///
/// ```rust,ignore
/// let cache_dir = get_platform_cache_dir()?;
/// println!("Cache directory: {}", cache_dir.display());
/// ```
pub fn get_platform_cache_dir() -> Result<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| OsnovaError::Storage("HOME environment variable not set".to_string()))?;
        Ok(PathBuf::from(home).join(".cache/osnova/components"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| OsnovaError::Storage("HOME environment variable not set".to_string()))?;
        Ok(PathBuf::from(home).join("Library/Caches/Osnova/components"))
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
            OsnovaError::Storage("LOCALAPPDATA environment variable not set".to_string())
        })?;
        Ok(PathBuf::from(local_app_data).join("Osnova\\Cache\\components"))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        // Fallback for other platforms (Android, iOS, etc.)
        let current_dir = std::env::current_dir()
            .map_err(|e| OsnovaError::Storage(format!("Failed to get current directory: {}", e)))?;
        Ok(current_dir.join("cache/components"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_cache_dir() {
        // Test that we can get a platform cache directory
        let result = get_platform_cache_dir();
        assert!(result.is_ok());

        let cache_dir = result.unwrap();
        assert!(cache_dir.to_string_lossy().contains("osnova")
            || cache_dir.to_string_lossy().contains("Osnova")
            || cache_dir.to_string_lossy().contains("cache"));
    }
}
