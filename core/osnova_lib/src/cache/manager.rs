//! # Component Cache Manager
//!
//! Local cache for downloaded components with LRU eviction.
//!
//! This module provides:
//! - LRU (Least Recently Used) eviction policy
//! - Configurable cache size limits
//! - Platform-specific cache directories
//! - Thread-safe operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use osnova_lib::cache::CacheManager;
//! use osnova_lib::platform::paths::get_component_cache_dir;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let cache_dir = get_component_cache_dir()?;
//! let max_size = 500 * 1024 * 1024; // 500MB
//!
//! let cache = CacheManager::new(cache_dir, max_size)?;
//!
//! // Store component
//! let data = b"component data...";
//! cache.store("my-component-v1.0.0", data).await?;
//!
//! // Retrieve component
//! if let Some(data) = cache.get("my-component-v1.0.0").await? {
//!     println!("Cache hit! {} bytes", data.len());
//! }
//! # Ok(())
//! # }
//! ```

use crate::error::{OsnovaError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cache entry metadata
#[derive(Clone, Debug)]
struct CacheEntry {
    /// File path in cache directory
    path: PathBuf,
    /// Size in bytes
    size: usize,
    /// Last access timestamp (for LRU)
    last_accessed: u64,
}

/// Component cache manager with LRU eviction
///
/// Manages a local cache of downloaded components with automatic
/// eviction when the cache size exceeds the configured limit.
#[derive(Clone)]
pub struct CacheManager {
    /// Base cache directory
    cache_dir: PathBuf,
    /// Maximum cache size in bytes
    max_size: usize,
    /// Cache entries metadata
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Current cache size in bytes
    current_size: Arc<RwLock<usize>>,
}

impl CacheManager {
    /// Create a new cache manager
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Base directory for cache storage
    /// * `max_size` - Maximum cache size in bytes
    ///
    /// # Returns
    ///
    /// * `Ok(CacheManager)` - Successfully created cache manager
    /// * `Err(OsnovaError::Storage)` - Failed to create cache directory
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use osnova_lib::cache::CacheManager;
    /// use osnova_lib::platform::paths::get_cache_dir;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = CacheManager::new(get_cache_dir()?, 500 * 1024 * 1024)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<P: AsRef<Path>>(cache_dir: P, max_size: usize) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();

        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).map_err(|e| {
            OsnovaError::Storage(format!("Failed to create cache directory: {}", e))
        })?;

        // Load existing cache entries
        let (entries, current_size) = Self::load_cache_index(&cache_dir)?;

        Ok(Self {
            cache_dir,
            max_size,
            entries: Arc::new(RwLock::new(entries)),
            current_size: Arc::new(RwLock::new(current_size)),
        })
    }

    /// Store data in the cache
    ///
    /// Stores data under the given key. If the cache is full, evicts
    /// least recently used entries to make space.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the cached data
    /// * `data` - Data to store
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// cache.store("component-v1.0.0", data).await?;
    /// ```
    pub async fn store(&self, key: &str, data: &[u8]) -> Result<()> {
        let data_size = data.len();

        // Evict entries if necessary
        self.evict_if_needed(data_size).await?;

        // Write data to file
        let file_path = self.cache_dir.join(Self::sanitize_key(key));
        tokio::fs::write(&file_path, data)
            .await
            .map_err(|e| OsnovaError::Storage(format!("Failed to write cache file: {}", e)))?;

        // Update metadata
        let entry = CacheEntry {
            path: file_path,
            size: data_size,
            last_accessed: Self::current_timestamp(),
        };

        let mut entries = self.entries.write().await;
        let mut current_size = self.current_size.write().await;

        entries.insert(key.to_string(), entry);
        *current_size += data_size;

        Ok(())
    }

    /// Get data from the cache
    ///
    /// Retrieves data for the given key and updates its LRU timestamp.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the cached data
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Vec<u8>))` - Cache hit, returns data
    /// * `Ok(None)` - Cache miss, key not found
    /// * `Err(OsnovaError)` - Failed to read cache file
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(data) = cache.get("component-v1.0.0").await? {
    ///     println!("Found {} bytes", data.len());
    /// }
    /// ```
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut entries = self.entries.write().await;

        if let Some(entry) = entries.get_mut(key) {
            // Update last accessed time
            entry.last_accessed = Self::current_timestamp();

            // Read file
            let data = tokio::fs::read(&entry.path)
                .await
                .map_err(|e| OsnovaError::Storage(format!("Failed to read cache file: {}", e)))?;

            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Remove a specific entry from the cache
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the cached data
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// cache.remove("old-component").await?;
    /// ```
    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut entries = self.entries.write().await;
        let mut current_size = self.current_size.write().await;

        if let Some(entry) = entries.remove(key) {
            // Delete file
            if let Err(e) = tokio::fs::remove_file(&entry.path).await {
                // Log error but don't fail the operation
                eprintln!("Warning: Failed to delete cache file: {}", e);
            }

            // Update size
            *current_size = current_size.saturating_sub(entry.size);
        }

        Ok(())
    }

    /// Clear all cached data
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// cache.clear().await?;
    /// ```
    pub async fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().await;
        let mut current_size = self.current_size.write().await;

        // Delete all files
        for entry in entries.values() {
            if let Err(e) = tokio::fs::remove_file(&entry.path).await {
                eprintln!("Warning: Failed to delete cache file: {}", e);
            }
        }

        // Clear metadata
        entries.clear();
        *current_size = 0;

        Ok(())
    }

    /// Get current cache size in bytes
    pub fn current_size(&self) -> usize {
        // Safe to use blocking read since this is a simple counter
        match self.current_size.try_read() {
            Ok(guard) => *guard,
            Err(_) => 0, // Return 0 if locked (rare case)
        }
    }

    /// Get maximum cache size in bytes
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Evict entries if needed to make space for new data
    async fn evict_if_needed(&self, required_size: usize) -> Result<()> {
        let current_size = *self.current_size.read().await;

        if current_size + required_size <= self.max_size {
            return Ok(()); // No eviction needed
        }

        let mut entries = self.entries.write().await;
        let mut current_size_guard = self.current_size.write().await;

        // Sort entries by last accessed (oldest first)
        let mut sorted_entries: Vec<_> = entries.iter().collect();
        sorted_entries.sort_by_key(|(_, entry)| entry.last_accessed);

        // Evict oldest entries until we have enough space
        let target_size = self.max_size - required_size;
        let mut evicted_size = 0;
        let mut keys_to_remove = Vec::new();

        for (key, entry) in sorted_entries {
            if *current_size_guard - evicted_size <= target_size {
                break;
            }

            keys_to_remove.push(key.clone());
            evicted_size += entry.size;

            // Delete file
            if let Err(e) = tokio::fs::remove_file(&entry.path).await {
                eprintln!("Warning: Failed to delete cache file during eviction: {}", e);
            }
        }

        // Remove from metadata
        for key in keys_to_remove {
            entries.remove(&key);
        }

        *current_size_guard -= evicted_size;

        Ok(())
    }

    /// Load existing cache index from disk
    fn load_cache_index(cache_dir: &Path) -> Result<(HashMap<String, CacheEntry>, usize)> {
        let mut entries = HashMap::new();
        let mut total_size = 0;

        // Read all files in cache directory
        if let Ok(read_dir) = fs::read_dir(cache_dir) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let path = entry.path();
                        let size = metadata.len() as usize;
                        let file_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();

                        let cache_entry = CacheEntry {
                            path: path.clone(),
                            size,
                            last_accessed: Self::current_timestamp(),
                        };

                        entries.insert(file_name, cache_entry);
                        total_size += size;
                    }
                }
            }
        }

        Ok((entries, total_size))
    }

    /// Sanitize key to be filesystem-safe
    fn sanitize_key(key: &str) -> String {
        key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
    }

    /// Get current timestamp in seconds since epoch
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_key() {
        assert_eq!(CacheManager::sanitize_key("simple-key"), "simple-key");
        assert_eq!(
            CacheManager::sanitize_key("key/with\\slashes"),
            "key_with_slashes"
        );
        assert_eq!(
            CacheManager::sanitize_key("key:with*special?chars"),
            "key_with_special_chars"
        );
    }
}
