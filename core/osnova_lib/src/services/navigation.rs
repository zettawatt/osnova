use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::storage::FileStorage;

/// Bottom menu tab identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BottomMenuTab {
    /// Launcher tab (app grid)
    Launcher,
    /// Wallet tab
    Wallet,
    /// Configuration tab
    Config,
}

impl Default for BottomMenuTab {
    fn default() -> Self {
        Self::Launcher
    }
}

/// Bottom menu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomMenuConfig {
    /// Current active tab
    pub active_tab: BottomMenuTab,
    /// Last updated timestamp
    pub updated_at: u64,
}

impl BottomMenuConfig {
    /// Create a new bottom menu config with default tab
    pub fn new() -> Self {
        Self {
            active_tab: BottomMenuTab::default(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create bottom menu config with specific tab
    pub fn with_tab(active_tab: BottomMenuTab) -> Self {
        Self {
            active_tab,
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

impl Default for BottomMenuConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation management service
///
/// Provides OpenRPC methods:
/// - `navigation.getBottomMenu` - Get the current active tab
/// - `navigation.setBottomMenu` - Set the active tab (launcher/wallet/config)
///
/// Navigation state is persisted per-identity and restored on relaunch.
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::{NavigationService, BottomMenuTab};
///
/// # fn example() -> anyhow::Result<()> {
/// let service = NavigationService::new("/path/to/storage", "user-123")?;
///
/// // Get current tab
/// let tab = service.get_bottom_menu()?;
/// println!("Active tab: {:?}", tab);
///
/// // Switch to wallet tab
/// service.set_bottom_menu(BottomMenuTab::Wallet)?;
/// # Ok(())
/// # }
/// ```
pub struct NavigationService {
    file_storage: FileStorage,
    nav_path: PathBuf,
    encryption_key: [u8; 32],
}

impl NavigationService {
    /// Create a new navigation service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    /// * `user_id` - User identifier (for per-identity navigation)
    pub fn new<P: Into<PathBuf>>(storage_path: P, user_id: &str) -> Result<Self> {
        let storage_path = storage_path.into();
        let file_storage = FileStorage::new(&storage_path)?;
        let nav_path = PathBuf::from(format!("navigation/{}/bottom_menu.json", user_id));

        // Derive encryption key from user_id
        // TODO: In production, use user's master key
        let encryption_key = Self::derive_nav_key(user_id);

        Ok(Self {
            file_storage,
            nav_path,
            encryption_key,
        })
    }

    /// Get the current bottom menu tab (OpenRPC: navigation.getBottomMenu)
    ///
    /// Returns the currently active bottom menu tab.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::NavigationService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = NavigationService::new("/tmp/storage", "user-123")?;
    /// let tab = service.get_bottom_menu()?;
    /// println!("Active tab: {:?}", tab);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_bottom_menu(&self) -> Result<BottomMenuTab> {
        if !self.file_storage.exists(&self.nav_path) {
            return Ok(BottomMenuTab::default());
        }

        let encrypted_data = self
            .file_storage
            .read(&self.nav_path, &self.encryption_key)
            .context("Failed to read navigation config")?;

        let config: BottomMenuConfig = serde_json::from_slice(&encrypted_data)
            .context("Failed to deserialize navigation config")?;

        Ok(config.active_tab)
    }

    /// Set the bottom menu tab (OpenRPC: navigation.setBottomMenu)
    ///
    /// Updates the active bottom menu tab. Changes are saved within 1s of drop.
    ///
    /// # Arguments
    ///
    /// * `tab` - Tab to activate (Launcher, Wallet, or Config)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::{NavigationService, BottomMenuTab};
    /// # fn example() -> anyhow::Result<()> {
    /// let service = NavigationService::new("/tmp/storage", "user-123")?;
    /// service.set_bottom_menu(BottomMenuTab::Wallet)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_bottom_menu(&self, tab: BottomMenuTab) -> Result<()> {
        let config = BottomMenuConfig::with_tab(tab);

        let config_json =
            serde_json::to_vec(&config).context("Failed to serialize navigation config")?;

        self.file_storage
            .write(&self.nav_path, &config_json, &self.encryption_key)
            .context("Failed to write navigation config")?;

        Ok(())
    }

    /// Derive encryption key for navigation config
    fn derive_nav_key(user_id: &str) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-navigation-key-v1:");
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

    fn create_test_service() -> Result<(NavigationService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = NavigationService::new(temp_dir.path(), "user-123")?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_get_bottom_menu_default() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let tab = service.get_bottom_menu()?;
        assert_eq!(tab, BottomMenuTab::Launcher);

        Ok(())
    }

    #[test]
    fn test_set_and_get_bottom_menu() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.set_bottom_menu(BottomMenuTab::Wallet)?;

        let tab = service.get_bottom_menu()?;
        assert_eq!(tab, BottomMenuTab::Wallet);

        Ok(())
    }

    #[test]
    fn test_update_bottom_menu() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Set initial tab
        service.set_bottom_menu(BottomMenuTab::Launcher)?;

        // Update tab
        service.set_bottom_menu(BottomMenuTab::Config)?;

        let tab = service.get_bottom_menu()?;
        assert_eq!(tab, BottomMenuTab::Config);

        Ok(())
    }

    #[test]
    fn test_bottom_menu_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Set tab in first service instance
        {
            let service = NavigationService::new(temp_dir.path(), "user-123")?;
            service.set_bottom_menu(BottomMenuTab::Wallet)?;
        }

        // Verify persistence in new service instance
        {
            let service = NavigationService::new(temp_dir.path(), "user-123")?;
            let tab = service.get_bottom_menu()?;
            assert_eq!(tab, BottomMenuTab::Wallet);
        }

        Ok(())
    }

    #[test]
    fn test_per_user_isolation() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let service1 = NavigationService::new(temp_dir.path(), "user-1")?;
        let service2 = NavigationService::new(temp_dir.path(), "user-2")?;

        // Set different tabs for each user
        service1.set_bottom_menu(BottomMenuTab::Launcher)?;
        service2.set_bottom_menu(BottomMenuTab::Config)?;

        // Verify tabs are separate
        let tab1 = service1.get_bottom_menu()?;
        let tab2 = service2.get_bottom_menu()?;

        assert_eq!(tab1, BottomMenuTab::Launcher);
        assert_eq!(tab2, BottomMenuTab::Config);
        assert_ne!(tab1, tab2);

        Ok(())
    }

    #[test]
    fn test_all_tab_variants() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Test Launcher
        service.set_bottom_menu(BottomMenuTab::Launcher)?;
        assert_eq!(service.get_bottom_menu()?, BottomMenuTab::Launcher);

        // Test Wallet
        service.set_bottom_menu(BottomMenuTab::Wallet)?;
        assert_eq!(service.get_bottom_menu()?, BottomMenuTab::Wallet);

        // Test Config
        service.set_bottom_menu(BottomMenuTab::Config)?;
        assert_eq!(service.get_bottom_menu()?, BottomMenuTab::Config);

        Ok(())
    }
}
