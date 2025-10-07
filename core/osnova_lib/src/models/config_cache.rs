//! Configuration and cache models for Osnova applications
//!
//! This module provides AppConfiguration and AppCache types which manage:
//! - Per-user application configuration settings
//! - Per-user application cache data
//! - Encryption at rest for user data
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::models::config_cache::{AppConfiguration, AppCache};
//! use serde_json::json;
//!
//! // Create application configuration
//! let mut config = AppConfiguration::new("app-id", "user-id");
//! config.set_setting("theme", json!("dark"));
//!
//! // Create application cache
//! let cache = AppCache::new("app-id", "user-id", vec![1, 2, 3]);
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Application configuration scoped to a specific user
///
/// Each user can have their own configuration settings for each application.
/// Configuration is encrypted at rest using cocoon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfiguration {
    /// Application ID (FK -> OsnovaApplication.id)
    app_id: String,

    /// User ID (scoped to RootIdentity)
    user_id: String,

    /// Configuration settings (key-value pairs)
    settings: HashMap<String, Value>,

    /// Unix timestamp when configuration was last updated
    updated_at: u64,
}

impl AppConfiguration {
    /// Create a new application configuration
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::config_cache::AppConfiguration;
    ///
    /// let config = AppConfiguration::new("app-123", "user-456");
    /// assert_eq!(config.app_id(), "app-123");
    /// assert_eq!(config.user_id(), "user-456");
    /// ```
    pub fn new(app_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            user_id: user_id.into(),
            settings: HashMap::new(),
            updated_at: Self::current_timestamp(),
        }
    }

    /// Create configuration with specific timestamp (for testing/imports)
    pub fn with_timestamp(
        app_id: impl Into<String>,
        user_id: impl Into<String>,
        updated_at: u64,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            user_id: user_id.into(),
            settings: HashMap::new(),
            updated_at,
        }
    }

    /// Create configuration with initial settings
    pub fn with_settings(
        app_id: impl Into<String>,
        user_id: impl Into<String>,
        settings: HashMap<String, Value>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            user_id: user_id.into(),
            settings,
            updated_at: Self::current_timestamp(),
        }
    }

    /// Get the application ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the user ID
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get all settings
    pub fn settings(&self) -> &HashMap<String, Value> {
        &self.settings
    }

    /// Get a specific setting by key
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::config_cache::AppConfiguration;
    /// use serde_json::json;
    ///
    /// let mut config = AppConfiguration::new("app-123", "user-456");
    /// config.set_setting("theme", json!("dark"));
    ///
    /// assert_eq!(config.get_setting("theme"), Some(&json!("dark")));
    /// assert_eq!(config.get_setting("missing"), None);
    /// ```
    pub fn get_setting(&self, key: &str) -> Option<&Value> {
        self.settings.get(key)
    }

    /// Set a configuration setting
    ///
    /// Updates the `updated_at` timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::config_cache::AppConfiguration;
    /// use serde_json::json;
    ///
    /// let mut config = AppConfiguration::new("app-123", "user-456");
    /// config.set_setting("theme", json!("dark"));
    /// config.set_setting("fontSize", json!(14));
    ///
    /// assert_eq!(config.settings().len(), 2);
    /// ```
    pub fn set_setting(&mut self, key: impl Into<String>, value: Value) {
        self.settings.insert(key.into(), value);
        self.updated_at = Self::current_timestamp();
    }

    /// Remove a configuration setting
    ///
    /// Updates the `updated_at` timestamp if the key existed.
    pub fn remove_setting(&mut self, key: &str) -> Option<Value> {
        let result = self.settings.remove(key);
        if result.is_some() {
            self.updated_at = Self::current_timestamp();
        }
        result
    }

    /// Clear all settings
    ///
    /// Updates the `updated_at` timestamp.
    pub fn clear_settings(&mut self) {
        self.settings.clear();
        self.updated_at = Self::current_timestamp();
    }

    /// Get the last updated timestamp
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }

    /// Get current Unix timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch")
            .as_secs()
    }
}

/// Application cache scoped to a specific user
///
/// Each user can have their own cache data for each application.
/// Cache data is regenerable and encrypted at rest using cocoon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppCache {
    /// Application ID (FK -> OsnovaApplication.id)
    app_id: String,

    /// User ID (scoped to RootIdentity)
    user_id: String,

    /// Opaque cache data (regenerable)
    entries: Vec<u8>,

    /// Unix timestamp when cache was last updated
    updated_at: u64,
}

impl AppCache {
    /// Create a new application cache
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    /// * `entries` - Cache data as bytes
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::config_cache::AppCache;
    ///
    /// let cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);
    /// assert_eq!(cache.app_id(), "app-123");
    /// assert_eq!(cache.user_id(), "user-456");
    /// assert_eq!(cache.entries(), &[1, 2, 3]);
    /// ```
    pub fn new(
        app_id: impl Into<String>,
        user_id: impl Into<String>,
        entries: Vec<u8>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            user_id: user_id.into(),
            entries,
            updated_at: Self::current_timestamp(),
        }
    }

    /// Create cache with specific timestamp (for testing/imports)
    pub fn with_timestamp(
        app_id: impl Into<String>,
        user_id: impl Into<String>,
        entries: Vec<u8>,
        updated_at: u64,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            user_id: user_id.into(),
            entries,
            updated_at,
        }
    }

    /// Get the application ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the user ID
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get the cache entries
    pub fn entries(&self) -> &[u8] {
        &self.entries
    }

    /// Update the cache entries
    ///
    /// Updates the `updated_at` timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::config_cache::AppCache;
    ///
    /// let mut cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);
    /// cache.update_entries(vec![4, 5, 6]);
    ///
    /// assert_eq!(cache.entries(), &[4, 5, 6]);
    /// ```
    pub fn update_entries(&mut self, entries: Vec<u8>) {
        self.entries = entries;
        self.updated_at = Self::current_timestamp();
    }

    /// Clear the cache entries
    ///
    /// Updates the `updated_at` timestamp.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.updated_at = Self::current_timestamp();
    }

    /// Get the cache size in bytes
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the last updated timestamp
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }

    /// Get current Unix timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch")
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_app_configuration_new() {
        let config = AppConfiguration::new("app-123", "user-456");

        assert_eq!(config.app_id(), "app-123");
        assert_eq!(config.user_id(), "user-456");
        assert_eq!(config.settings().len(), 0);
        assert!(config.updated_at() > 0);
    }

    #[test]
    fn test_app_configuration_with_timestamp() {
        let config = AppConfiguration::with_timestamp("app-123", "user-456", 1000);

        assert_eq!(config.updated_at(), 1000);
    }

    #[test]
    fn test_app_configuration_with_settings() {
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), json!("dark"));

        let config = AppConfiguration::with_settings("app-123", "user-456", settings);

        assert_eq!(config.settings().len(), 1);
        assert_eq!(config.get_setting("theme"), Some(&json!("dark")));
    }

    #[test]
    fn test_app_configuration_set_setting() {
        let mut config = AppConfiguration::new("app-123", "user-456");

        config.set_setting("theme", json!("dark"));
        config.set_setting("fontSize", json!(14));

        assert_eq!(config.settings().len(), 2);
        assert_eq!(config.get_setting("theme"), Some(&json!("dark")));
        assert_eq!(config.get_setting("fontSize"), Some(&json!(14)));
    }

    #[test]
    fn test_app_configuration_get_missing_setting() {
        let config = AppConfiguration::new("app-123", "user-456");

        assert_eq!(config.get_setting("missing"), None);
    }

    #[test]
    fn test_app_configuration_remove_setting() {
        let mut config = AppConfiguration::new("app-123", "user-456");
        config.set_setting("theme", json!("dark"));

        let removed = config.remove_setting("theme");
        assert_eq!(removed, Some(json!("dark")));
        assert_eq!(config.settings().len(), 0);

        // Removing again returns None
        let removed = config.remove_setting("theme");
        assert_eq!(removed, None);
    }

    #[test]
    fn test_app_configuration_clear_settings() {
        let mut config = AppConfiguration::new("app-123", "user-456");
        config.set_setting("theme", json!("dark"));
        config.set_setting("fontSize", json!(14));

        assert_eq!(config.settings().len(), 2);

        config.clear_settings();
        assert_eq!(config.settings().len(), 0);
    }

    #[test]
    fn test_app_configuration_serialization() {
        let mut config = AppConfiguration::with_timestamp("app-123", "user-456", 1000);
        config.set_setting("theme", json!("dark"));

        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: AppConfiguration = serde_json::from_str(&json)
            .expect("Failed to deserialize");

        assert_eq!(config.app_id(), deserialized.app_id());
        assert_eq!(config.user_id(), deserialized.user_id());
        assert_eq!(config.settings(), deserialized.settings());
    }

    #[test]
    fn test_app_cache_new() {
        let cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);

        assert_eq!(cache.app_id(), "app-123");
        assert_eq!(cache.user_id(), "user-456");
        assert_eq!(cache.entries(), &[1, 2, 3]);
        assert_eq!(cache.size(), 3);
        assert!(!cache.is_empty());
        assert!(cache.updated_at() > 0);
    }

    #[test]
    fn test_app_cache_with_timestamp() {
        let cache = AppCache::with_timestamp("app-123", "user-456", vec![1, 2, 3], 1000);

        assert_eq!(cache.updated_at(), 1000);
    }

    #[test]
    fn test_app_cache_update_entries() {
        let mut cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);

        cache.update_entries(vec![4, 5, 6, 7]);

        assert_eq!(cache.entries(), &[4, 5, 6, 7]);
        assert_eq!(cache.size(), 4);
    }

    #[test]
    fn test_app_cache_clear() {
        let mut cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);

        cache.clear();

        assert_eq!(cache.entries(), &[] as &[u8]);
        assert_eq!(cache.size(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_app_cache_is_empty() {
        let cache = AppCache::new("app-123", "user-456", vec![]);

        assert!(cache.is_empty());

        let cache_with_data = AppCache::new("app-123", "user-456", vec![1]);
        assert!(!cache_with_data.is_empty());
    }

    #[test]
    fn test_app_cache_serialization() {
        let cache = AppCache::with_timestamp("app-123", "user-456", vec![1, 2, 3], 1000);

        let json = serde_json::to_string(&cache).expect("Failed to serialize");
        let deserialized: AppCache = serde_json::from_str(&json)
            .expect("Failed to deserialize");

        assert_eq!(cache, deserialized);
    }

    #[test]
    fn test_app_cache_clone() {
        let cache = AppCache::new("app-123", "user-456", vec![1, 2, 3]);
        let cloned = cache.clone();

        assert_eq!(cache, cloned);
    }

    #[test]
    fn test_app_configuration_clone() {
        let mut config = AppConfiguration::new("app-123", "user-456");
        config.set_setting("theme", json!("dark"));

        let cloned = config.clone();
        assert_eq!(config, cloned);
    }
}
