use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::storage::FileStorage;

/// UI theme setting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// System-based theme (follows OS preference)
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

/// UI theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Current theme setting
    pub theme: Theme,
    /// Last updated timestamp
    pub updated_at: u64,
}

impl ThemeConfig {
    /// Create a new theme config with default theme
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create theme config with specific theme
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            theme,
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Update timestamp
    pub fn touch(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// UI management service
///
/// Provides OpenRPC methods:
/// - `ui.getTheme` - Get the current theme setting
/// - `ui.setTheme` - Set the theme (light/dark/system)
///
/// Theme preference is persisted per-identity and restored on relaunch.
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::{UIService, Theme};
///
/// # fn example() -> anyhow::Result<()> {
/// let service = UIService::new("/path/to/storage", "user-123")?;
///
/// // Get current theme
/// let theme = service.get_theme()?;
/// println!("Theme: {:?}", theme);
///
/// // Set dark theme
/// service.set_theme(Theme::Dark)?;
/// # Ok(())
/// # }
/// ```
pub struct UIService {
    file_storage: FileStorage,
    theme_path: PathBuf,
    encryption_key: [u8; 32],
}

impl UIService {
    /// Create a new UI service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    /// * `user_id` - User identifier (for per-identity theme)
    pub fn new<P: Into<PathBuf>>(storage_path: P, user_id: &str) -> Result<Self> {
        let storage_path = storage_path.into();
        let file_storage = FileStorage::new(&storage_path)?;
        let theme_path = PathBuf::from(format!("ui/{}/theme.json", user_id));

        // Derive encryption key from user_id
        // TODO: In production, use user's master key
        let encryption_key = Self::derive_theme_key(user_id);

        Ok(Self {
            file_storage,
            theme_path,
            encryption_key,
        })
    }

    /// Get the current theme setting (OpenRPC: ui.getTheme)
    ///
    /// Returns the user's theme preference (light/dark/system).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::UIService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = UIService::new("/tmp/storage", "user-123")?;
    /// let theme = service.get_theme()?;
    /// println!("Current theme: {:?}", theme);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_theme(&self) -> Result<Theme> {
        if !self.file_storage.exists(&self.theme_path) {
            return Ok(Theme::default());
        }

        let encrypted_data = self
            .file_storage
            .read(&self.theme_path, &self.encryption_key)
            .context("Failed to read theme config")?;

        let config: ThemeConfig = serde_json::from_slice(&encrypted_data)
            .context("Failed to deserialize theme config")?;

        Ok(config.theme)
    }

    /// Set the theme (OpenRPC: ui.setTheme)
    ///
    /// Updates the user's theme preference. Changes are saved within 1s of drop.
    ///
    /// # Arguments
    ///
    /// * `theme` - Theme to set (Light, Dark, or System)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::{UIService, Theme};
    /// # fn example() -> anyhow::Result<()> {
    /// let service = UIService::new("/tmp/storage", "user-123")?;
    /// service.set_theme(Theme::Dark)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_theme(&self, theme: Theme) -> Result<()> {
        let config = ThemeConfig::with_theme(theme);

        let config_json = serde_json::to_vec(&config)
            .context("Failed to serialize theme config")?;

        self.file_storage
            .write(&self.theme_path, &config_json, &self.encryption_key)
            .context("Failed to write theme config")?;

        Ok(())
    }

    /// Derive encryption key for theme config
    fn derive_theme_key(user_id: &str) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-ui-theme-key-v1:");
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

    fn create_test_service() -> Result<(UIService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = UIService::new(temp_dir.path(), "user-123")?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_get_theme_default() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let theme = service.get_theme()?;
        assert_eq!(theme, Theme::System);

        Ok(())
    }

    #[test]
    fn test_set_and_get_theme() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.set_theme(Theme::Dark)?;

        let theme = service.get_theme()?;
        assert_eq!(theme, Theme::Dark);

        Ok(())
    }

    #[test]
    fn test_update_theme() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Set initial theme
        service.set_theme(Theme::Light)?;

        // Update theme
        service.set_theme(Theme::Dark)?;

        let theme = service.get_theme()?;
        assert_eq!(theme, Theme::Dark);

        Ok(())
    }

    #[test]
    fn test_theme_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Set theme in first service instance
        {
            let service = UIService::new(temp_dir.path(), "user-123")?;
            service.set_theme(Theme::Dark)?;
        }

        // Verify persistence in new service instance
        {
            let service = UIService::new(temp_dir.path(), "user-123")?;
            let theme = service.get_theme()?;
            assert_eq!(theme, Theme::Dark);
        }

        Ok(())
    }

    #[test]
    fn test_per_user_isolation() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let service1 = UIService::new(temp_dir.path(), "user-1")?;
        let service2 = UIService::new(temp_dir.path(), "user-2")?;

        // Set different themes for each user
        service1.set_theme(Theme::Light)?;
        service2.set_theme(Theme::Dark)?;

        // Verify themes are separate
        let theme1 = service1.get_theme()?;
        let theme2 = service2.get_theme()?;

        assert_eq!(theme1, Theme::Light);
        assert_eq!(theme2, Theme::Dark);
        assert_ne!(theme1, theme2);

        Ok(())
    }

    #[test]
    fn test_all_theme_variants() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Test Light
        service.set_theme(Theme::Light)?;
        assert_eq!(service.get_theme()?, Theme::Light);

        // Test Dark
        service.set_theme(Theme::Dark)?;
        assert_eq!(service.get_theme()?, Theme::Dark);

        // Test System
        service.set_theme(Theme::System)?;
        assert_eq!(service.get_theme()?, Theme::System);

        Ok(())
    }
}
