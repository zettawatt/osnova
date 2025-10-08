//! Platform-specific path utilities
//!
//! Provides cross-platform directory paths for data, cache, and config.
//!
//! ## Platform Directories
//!
//! ### Data Directory (`get_data_dir()`)
//!
//! | Platform | Location |
//! |----------|----------|
//! | Linux | `~/.local/share/osnova/` |
//! | macOS | `~/Library/Application Support/osnova/` |
//! | Windows | `%LOCALAPPDATA%\osnova\` |
//! | Android | `/data/data/com.osnova.app/files/` |
//! | iOS | `<app_sandbox>/Library/Application Support/` |
//!
//! ### Cache Directory (`get_cache_dir()`)
//!
//! | Platform | Location |
//! |----------|----------|
//! | Linux | `~/.cache/osnova/` |
//! | macOS | `~/Library/Caches/osnova/` |
//! | Windows | `%LOCALAPPDATA%\osnova\Cache\` |
//! | Android | `/data/data/com.osnova.app/cache/` |
//! | iOS | `<app_sandbox>/Library/Caches/` |
//!
//! ### Config Directory (`get_config_dir()`)
//!
//! | Platform | Location |
//! |----------|----------|
//! | Linux | `~/.config/osnova/` |
//! | macOS | `~/Library/Application Support/osnova/` |
//! | Windows | `%APPDATA%\osnova\` |
//! | Android | `/data/data/com.osnova.app/shared_prefs/` |
//! | iOS | `<app_sandbox>/Library/Preferences/` |
//!
//! ## Example
//!
//! ```rust,no_run
//! use osnova_lib::platform::paths::get_data_dir;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut db_path = get_data_dir()?;
//! db_path.push("identity.db");
//!
//! println!("Database: {}", db_path.display());
//! # Ok(())
//! # }
//! ```

use crate::error::{OsnovaError, Result};
use std::path::PathBuf;

/// Get application data directory
///
/// Returns platform-specific locations:
/// - Linux: `~/.local/share/osnova/`
/// - macOS: `~/Library/Application Support/osnova/`
/// - Windows: `%LOCALAPPDATA%\osnova\`
/// - Android: `/data/data/com.osnova.app/files/`
/// - iOS: `<app_sandbox>/Library/Application Support/`
///
/// # Returns
///
/// * `Ok(PathBuf)` - Platform-specific data directory
/// * `Err(OsnovaError::Storage)` - Failed to determine data directory
///
/// # Example
///
/// ```rust,no_run
/// use osnova_lib::platform::paths::get_data_dir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data_dir = get_data_dir()?;
/// println!("Data: {}", data_dir.display());
/// # Ok(())
/// # }
/// ```
pub fn get_data_dir() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or_else(|| {
        OsnovaError::Storage(
            "Failed to get data directory. Is this running in a container?".to_string(),
        )
    })?;
    path.push("osnova");
    Ok(path)
}

/// Get application cache directory
///
/// Returns platform-specific locations:
/// - Linux: `~/.cache/osnova/`
/// - macOS: `~/Library/Caches/osnova/`
/// - Windows: `%LOCALAPPDATA%\osnova\Cache\`
/// - Android: `/data/data/com.osnova.app/cache/`
/// - iOS: `<app_sandbox>/Library/Caches/`
///
/// # Returns
///
/// * `Ok(PathBuf)` - Platform-specific cache directory
/// * `Err(OsnovaError::Storage)` - Failed to determine cache directory
///
/// # Example
///
/// ```rust,no_run
/// use osnova_lib::platform::paths::get_cache_dir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let cache_dir = get_cache_dir()?;
/// println!("Cache: {}", cache_dir.display());
/// # Ok(())
/// # }
/// ```
pub fn get_cache_dir() -> Result<PathBuf> {
    let mut path = dirs::cache_dir().ok_or_else(|| {
        OsnovaError::Storage(
            "Failed to get cache directory. Is this running in a container?".to_string(),
        )
    })?;
    path.push("osnova");
    Ok(path)
}

/// Get application config directory
///
/// Returns platform-specific locations:
/// - Linux: `~/.config/osnova/`
/// - macOS: `~/Library/Application Support/osnova/`
/// - Windows: `%APPDATA%\osnova\`
/// - Android: `/data/data/com.osnova.app/shared_prefs/`
/// - iOS: `<app_sandbox>/Library/Preferences/`
///
/// # Returns
///
/// * `Ok(PathBuf)` - Platform-specific config directory
/// * `Err(OsnovaError::Storage)` - Failed to determine config directory
///
/// # Example
///
/// ```rust,no_run
/// use osnova_lib::platform::paths::get_config_dir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config_dir = get_config_dir()?;
/// println!("Config: {}", config_dir.display());
/// # Ok(())
/// # }
/// ```
pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or_else(|| {
        OsnovaError::Storage(
            "Failed to get config directory. Is this running in a container?".to_string(),
        )
    })?;
    path.push("osnova");
    Ok(path)
}

/// Get component cache directory
///
/// Returns the cache subdirectory specifically for storing downloaded
/// application components (frontend and backend files).
///
/// # Returns
///
/// * `Ok(PathBuf)` - Platform-specific component cache directory
/// * `Err(OsnovaError::Storage)` - Failed to determine cache directory
///
/// # Example
///
/// ```rust,no_run
/// use osnova_lib::platform::paths::get_component_cache_dir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let comp_cache = get_component_cache_dir()?;
/// println!("Component cache: {}", comp_cache.display());
/// # Ok(())
/// # }
/// ```
pub fn get_component_cache_dir() -> Result<PathBuf> {
    let mut path = get_cache_dir()?;
    path.push("components");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_dir() {
        let path = get_data_dir().unwrap();
        assert!(path.ends_with("osnova"));

        // Verify platform-specific base
        #[cfg(target_os = "linux")]
        assert!(path.to_str().unwrap().contains(".local/share"));

        #[cfg(target_os = "macos")]
        assert!(path.to_str().unwrap().contains("Library/Application Support"));

        #[cfg(target_os = "windows")]
        {
            let path_str = path.to_str().unwrap();
            assert!(path_str.contains("AppData") && path_str.contains("Local"));
        }
    }

    #[test]
    fn test_get_cache_dir() {
        let path = get_cache_dir().unwrap();
        assert!(path.ends_with("osnova"));

        // Verify platform-specific base
        #[cfg(target_os = "linux")]
        assert!(path.to_str().unwrap().contains(".cache"));

        #[cfg(target_os = "macos")]
        assert!(path.to_str().unwrap().contains("Library/Caches"));

        #[cfg(target_os = "windows")]
        {
            let path_str = path.to_str().unwrap();
            assert!(path_str.contains("AppData") && path_str.contains("Local"));
        }
    }

    #[test]
    fn test_get_config_dir() {
        let path = get_config_dir().unwrap();
        assert!(path.ends_with("osnova"));

        // Verify platform-specific base
        #[cfg(target_os = "linux")]
        assert!(path.to_str().unwrap().contains(".config"));

        #[cfg(target_os = "macos")]
        assert!(path.to_str().unwrap().contains("Library/Application Support"));

        #[cfg(target_os = "windows")]
        {
            let path_str = path.to_str().unwrap();
            assert!(path_str.contains("AppData") && path_str.contains("Roaming"));
        }
    }

    #[test]
    fn test_get_component_cache_dir() {
        let path = get_component_cache_dir().unwrap();
        assert!(path.ends_with("components"));

        // Verify it's under cache directory
        let cache_dir = get_cache_dir().unwrap();
        assert!(path.starts_with(&cache_dir));
    }

    #[test]
    fn test_paths_are_different() {
        let data = get_data_dir().unwrap();
        let cache = get_cache_dir().unwrap();
        let _config = get_config_dir().unwrap();

        // On most platforms, these should be different directories
        // (On macOS, data and config are the same, which is fine)
        assert_ne!(data, cache);
    }
}
