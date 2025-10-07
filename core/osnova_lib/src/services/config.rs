use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use crate::models::config_cache::{AppCache, AppConfiguration};
use crate::storage::{FileStorage, SqlStorage};

/// Configuration service for managing system and application settings
///
/// Provides OpenRPC methods:
/// - `config.getLauncherManifest` - Get configured launcher manifest address
/// - `config.setLauncherManifest` - Set launcher manifest address
/// - `config.setServer` - Configure server address for Client-Server mode
/// - `config.getAppConfig` - Get per-app configuration data
/// - `config.setAppConfig` - Update per-app configuration data
/// - `config.getAppCache` - Get per-app cache metadata
/// - `config.clearAppCache` - Clear cache for a specific app
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::ConfigService;
///
/// # fn example() -> anyhow::Result<()> {
/// let service = ConfigService::new("/path/to/storage")?;
///
/// // Set launcher manifest
/// service.set_launcher_manifest("xor://launcher-manifest-address")?;
///
/// // Get app configuration
/// let config = service.get_app_config("com.osnova.wallet", "user-123")?;
/// # Ok(())
/// # }
/// ```
pub struct ConfigService {
    file_storage: FileStorage,
    sql_storage: SqlStorage,
    system_config_path: PathBuf,
    encryption_key: [u8; 32],
}

/// System-wide configuration (launcher manifest, server address, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SystemConfig {
    /// Launcher manifest address (Autonomi XOR address)
    launcher_manifest: Option<String>,
    /// Server address for Client-Server mode
    server_address: Option<String>,
    /// Last updated timestamp
    updated_at: u64,
}

impl SystemConfig {
    fn new() -> Self {
        Self {
            launcher_manifest: None,
            server_address: None,
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

impl ConfigService {
    /// Create a new configuration service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    ///
    /// # Errors
    ///
    /// Returns an error if storage cannot be initialized
    pub fn new<P: Into<PathBuf>>(storage_path: P) -> Result<Self> {
        let storage_path = storage_path.into();
        let file_storage = FileStorage::new(&storage_path)?;
        let sql_storage = SqlStorage::new(storage_path.join("osnova.db"))?;
        let system_config_path = PathBuf::from("config/system.json");

        // Use a deterministic key for system config
        // TODO: In production, derive this from platform keystore
        let encryption_key = Self::derive_system_key();

        Ok(Self {
            file_storage,
            sql_storage,
            system_config_path,
            encryption_key,
        })
    }

    /// Get the configured launcher manifest address (OpenRPC: config.getLauncherManifest)
    ///
    /// Returns the Autonomi XOR address of the configured launcher manifest,
    /// or None if not configured.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// if let Some(manifest) = service.get_launcher_manifest()? {
    ///     println!("Launcher manifest: {}", manifest);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_launcher_manifest(&self) -> Result<Option<String>> {
        let config = self.load_system_config()?;
        Ok(config.launcher_manifest)
    }

    /// Set the launcher manifest address (OpenRPC: config.setLauncherManifest)
    ///
    /// Sets the Autonomi XOR address of the launcher manifest to use.
    /// This allows swapping between different launcher implementations.
    ///
    /// # Arguments
    ///
    /// * `manifest_address` - Autonomi XOR address of the launcher manifest
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// service.set_launcher_manifest("xor://launcher-manifest-address")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_launcher_manifest(&self, manifest_address: &str) -> Result<()> {
        let mut config = self.load_system_config()?;
        config.launcher_manifest = Some(manifest_address.to_string());
        config.update_timestamp();
        self.save_system_config(&config)?;
        Ok(())
    }

    /// Configure server address for Client-Server mode (OpenRPC: config.setServer)
    ///
    /// Sets the server address to use when running in Client-Server mode.
    ///
    /// # Arguments
    ///
    /// * `server_address` - Server address (e.g., "https://server.example.com")
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// service.set_server("https://my-server.example.com")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_server(&self, server_address: &str) -> Result<()> {
        let mut config = self.load_system_config()?;
        config.server_address = Some(server_address.to_string());
        config.update_timestamp();
        self.save_system_config(&config)?;
        Ok(())
    }

    /// Get server address
    ///
    /// Returns the configured server address, or None if not configured.
    pub fn get_server(&self) -> Result<Option<String>> {
        let config = self.load_system_config()?;
        Ok(config.server_address)
    }

    /// Get per-app configuration data (OpenRPC: config.getAppConfig)
    ///
    /// Returns the configuration settings for a specific app and user.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// let config = service.get_app_config("com.osnova.wallet", "user-123")?;
    /// if let Some(theme) = config.get_setting("theme") {
    ///     println!("Theme: {:?}", theme);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_app_config(&self, app_id: &str, user_id: &str) -> Result<AppConfiguration> {
        // Use a per-user encryption key derived from user_id
        // TODO: In production, derive from user's master key
        let encryption_key = Self::derive_user_config_key(user_id);

        match self.sql_storage.get_app_config(app_id, user_id, &encryption_key)? {
            Some(config) => Ok(config),
            None => Ok(AppConfiguration::new(app_id, user_id)),
        }
    }

    /// Update per-app configuration data (OpenRPC: config.setAppConfig)
    ///
    /// Updates the configuration settings for a specific app and user.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    /// * `settings` - Configuration settings to update (partial or full)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # use serde_json::json;
    /// # use std::collections::HashMap;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// let mut settings = HashMap::new();
    /// settings.insert("theme".to_string(), json!("dark"));
    /// settings.insert("language".to_string(), json!("en"));
    /// service.set_app_config("com.osnova.wallet", "user-123", settings)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_app_config(
        &self,
        app_id: &str,
        user_id: &str,
        settings: std::collections::HashMap<String, Value>,
    ) -> Result<()> {
        // Get existing config or create new one
        let mut config = self.get_app_config(app_id, user_id)?;

        // Update settings
        for (key, value) in settings {
            config.set_setting(&key, value);
        }

        // Use a per-user encryption key
        let encryption_key = Self::derive_user_config_key(user_id);

        // Save to database
        self.sql_storage.set_app_config(app_id, user_id, &config, &encryption_key)?;

        Ok(())
    }

    /// Get per-app cache metadata (OpenRPC: config.getAppCache)
    ///
    /// Returns metadata about the cache for a specific app and user.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// if let Some(cache) = service.get_app_cache("com.osnova.wallet", "user-123")? {
    ///     println!("Cache size: {} bytes", cache.data().len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_app_cache(&self, _app_id: &str, _user_id: &str) -> Result<Option<AppCache>> {
        // TODO: Implement app cache storage
        // For now, always return None
        Ok(None)
    }

    /// Clear cache for a specific app (OpenRPC: config.clearAppCache)
    ///
    /// Deletes all cache data for a specific app and user.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application identifier
    /// * `user_id` - User identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::ConfigService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = ConfigService::new("/tmp/storage")?;
    /// service.clear_app_cache("com.osnova.wallet", "user-123")?;
    /// println!("Cache cleared");
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear_app_cache(&self, _app_id: &str, _user_id: &str) -> Result<()> {
        // TODO: Implement app cache storage
        // For now, this is a no-op
        Ok(())
    }

    // Private helper methods

    /// Load system configuration from encrypted file storage
    fn load_system_config(&self) -> Result<SystemConfig> {
        if !self.file_storage.exists(&self.system_config_path) {
            return Ok(SystemConfig::new());
        }

        let encrypted_data = self
            .file_storage
            .read(&self.system_config_path, &self.encryption_key)
            .context("Failed to read system config")?;

        let config: SystemConfig = serde_json::from_slice(&encrypted_data)
            .context("Failed to deserialize system config")?;

        Ok(config)
    }

    /// Save system configuration to encrypted file storage
    fn save_system_config(&self, config: &SystemConfig) -> Result<()> {
        let config_json = serde_json::to_vec(config)
            .context("Failed to serialize system config")?;

        self.file_storage
            .write(&self.system_config_path, &config_json, &self.encryption_key)
            .context("Failed to write system config")?;

        Ok(())
    }

    /// Derive a deterministic encryption key for system config
    ///
    /// TODO: In production, integrate with platform keystore
    fn derive_system_key() -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-system-config-key-v1");
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_bytes());
        key
    }

    /// Derive a per-user encryption key for app configurations
    ///
    /// TODO: In production, derive from user's master key
    fn derive_user_config_key(user_id: &str) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-user-config-key-v1:");
        hasher.update(user_id.as_bytes());
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_bytes());
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_service() -> Result<(ConfigService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = ConfigService::new(temp_dir.path())?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_get_launcher_manifest_not_configured() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let manifest = service.get_launcher_manifest()?;
        assert!(manifest.is_none());

        Ok(())
    }

    #[test]
    fn test_set_and_get_launcher_manifest() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.set_launcher_manifest("xor://test-manifest-address")?;

        let manifest = service.get_launcher_manifest()?;
        assert_eq!(manifest, Some("xor://test-manifest-address".to_string()));

        Ok(())
    }

    #[test]
    fn test_set_and_get_server() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.set_server("https://server.example.com")?;

        let server = service.get_server()?;
        assert_eq!(server, Some("https://server.example.com".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_app_config_new() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let config = service.get_app_config("com.test.app", "user-123")?;

        assert_eq!(config.app_id(), "com.test.app");
        assert_eq!(config.user_id(), "user-123");
        assert!(config.settings().is_empty());

        Ok(())
    }

    #[test]
    fn test_set_and_get_app_config() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Create application first (required for foreign key)
        let app = crate::models::application::OsnovaApplication::new(
            "com.test.app",
            "Test App",
            "1.0.0",
            "https://icon.url",
            "Test application",
            vec![],
        )?;
        service.sql_storage.upsert_application(&app)?;

        let mut settings = std::collections::HashMap::new();
        settings.insert("theme".to_string(), serde_json::json!("dark"));
        settings.insert("language".to_string(), serde_json::json!("en"));

        service.set_app_config("com.test.app", "user-123", settings)?;

        let config = service.get_app_config("com.test.app", "user-123")?;
        assert_eq!(config.get_setting("theme"), Some(&serde_json::json!("dark")));
        assert_eq!(config.get_setting("language"), Some(&serde_json::json!("en")));

        Ok(())
    }

    #[test]
    fn test_update_app_config() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Create application first (required for foreign key)
        let app = crate::models::application::OsnovaApplication::new(
            "com.test.app",
            "Test App",
            "1.0.0",
            "https://icon.url",
            "Test application",
            vec![],
        )?;
        service.sql_storage.upsert_application(&app)?;

        // Set initial config
        let mut settings1 = std::collections::HashMap::new();
        settings1.insert("theme".to_string(), serde_json::json!("dark"));
        service.set_app_config("com.test.app", "user-123", settings1)?;

        // Update config with new settings
        let mut settings2 = std::collections::HashMap::new();
        settings2.insert("language".to_string(), serde_json::json!("en"));
        service.set_app_config("com.test.app", "user-123", settings2)?;

        // Verify both settings exist
        let config = service.get_app_config("com.test.app", "user-123")?;
        assert_eq!(config.get_setting("theme"), Some(&serde_json::json!("dark")));
        assert_eq!(config.get_setting("language"), Some(&serde_json::json!("en")));

        Ok(())
    }

    #[test]
    fn test_app_config_per_user() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Create application first (required for foreign key)
        let app = crate::models::application::OsnovaApplication::new(
            "com.test.app",
            "Test App",
            "1.0.0",
            "https://icon.url",
            "Test application",
            vec![],
        )?;
        service.sql_storage.upsert_application(&app)?;

        // Set config for user1
        let mut settings1 = std::collections::HashMap::new();
        settings1.insert("theme".to_string(), serde_json::json!("dark"));
        service.set_app_config("com.test.app", "user-1", settings1)?;

        // Set config for user2
        let mut settings2 = std::collections::HashMap::new();
        settings2.insert("theme".to_string(), serde_json::json!("light"));
        service.set_app_config("com.test.app", "user-2", settings2)?;

        // Verify configs are separate
        let config1 = service.get_app_config("com.test.app", "user-1")?;
        let config2 = service.get_app_config("com.test.app", "user-2")?;

        assert_eq!(config1.get_setting("theme"), Some(&serde_json::json!("dark")));
        assert_eq!(config2.get_setting("theme"), Some(&serde_json::json!("light")));

        Ok(())
    }

    #[test]
    fn test_get_app_cache_not_exists() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // App cache not implemented yet, should always return None
        let cache = service.get_app_cache("com.test.app", "user-123")?;
        assert!(cache.is_none());

        Ok(())
    }

    #[test]
    fn test_clear_app_cache() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // App cache not implemented yet, should be a no-op
        service.clear_app_cache("com.test.app", "user-123")?;

        Ok(())
    }

    #[test]
    fn test_system_config_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Set configs in first service instance
        {
            let service = ConfigService::new(temp_dir.path())?;
            service.set_launcher_manifest("xor://test-manifest")?;
            service.set_server("https://test-server.com")?;
        }

        // Verify configs persist in new service instance
        {
            let service = ConfigService::new(temp_dir.path())?;
            assert_eq!(
                service.get_launcher_manifest()?,
                Some("xor://test-manifest".to_string())
            );
            assert_eq!(
                service.get_server()?,
                Some("https://test-server.com".to_string())
            );
        }

        Ok(())
    }
}
