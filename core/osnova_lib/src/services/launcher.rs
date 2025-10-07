use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::storage::FileStorage;

/// Launcher layout (ordered list of app IDs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherLayout {
    /// Ordered list of application IDs
    pub app_ids: Vec<String>,
    /// Last updated timestamp
    pub updated_at: u64,
}

impl LauncherLayout {
    /// Create a new empty layout
    pub fn new() -> Self {
        Self {
            app_ids: Vec::new(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create layout with app IDs
    pub fn with_apps(app_ids: Vec<String>) -> Self {
        Self {
            app_ids,
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

impl Default for LauncherLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Launcher layout service
///
/// Provides OpenRPC methods:
/// - `launcher.getLayout` - Get the current icon order/placement
/// - `launcher.setLayout` - Set the icon order/placement
///
/// Layout is persisted per-identity and restored on relaunch.
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::LauncherService;
///
/// # fn example() -> anyhow::Result<()> {
/// let service = LauncherService::new("/path/to/storage", "user-123")?;
///
/// // Get current layout
/// let layout = service.get_layout()?;
/// println!("Apps: {:?}", layout.app_ids);
///
/// // Update layout
/// service.set_layout(vec!["app1".to_string(), "app2".to_string()])?;
/// # Ok(())
/// # }
/// ```
pub struct LauncherService {
    file_storage: FileStorage,
    layout_path: PathBuf,
    encryption_key: [u8; 32],
}

impl LauncherService {
    /// Create a new launcher service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    /// * `user_id` - User identifier (for per-identity layout)
    pub fn new<P: Into<PathBuf>>(storage_path: P, user_id: &str) -> Result<Self> {
        let storage_path = storage_path.into();
        let file_storage = FileStorage::new(&storage_path)?;
        let layout_path = PathBuf::from(format!("launcher/{}/layout.json", user_id));

        // Derive encryption key from user_id
        // TODO: In production, use user's master key
        let encryption_key = Self::derive_layout_key(user_id);

        Ok(Self {
            file_storage,
            layout_path,
            encryption_key,
        })
    }

    /// Get the current launcher layout (OpenRPC: launcher.getLayout)
    ///
    /// Returns the ordered list of application IDs representing the launcher icon layout.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::LauncherService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = LauncherService::new("/tmp/storage", "user-123")?;
    /// let layout = service.get_layout()?;
    /// println!("Layout has {} apps", layout.app_ids.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_layout(&self) -> Result<LauncherLayout> {
        if !self.file_storage.exists(&self.layout_path) {
            return Ok(LauncherLayout::new());
        }

        let encrypted_data = self
            .file_storage
            .read(&self.layout_path, &self.encryption_key)
            .context("Failed to read launcher layout")?;

        let layout: LauncherLayout = serde_json::from_slice(&encrypted_data)
            .context("Failed to deserialize launcher layout")?;

        Ok(layout)
    }

    /// Set the launcher layout (OpenRPC: launcher.setLayout)
    ///
    /// Updates the launcher icon order/placement. Changes are saved within 1s of drop.
    ///
    /// # Arguments
    ///
    /// * `app_ids` - Ordered list of application IDs
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::LauncherService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = LauncherService::new("/tmp/storage", "user-123")?;
    /// service.set_layout(vec![
    ///     "com.osnova.launcher".to_string(),
    ///     "com.osnova.wallet".to_string(),
    /// ])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_layout(&self, app_ids: Vec<String>) -> Result<()> {
        let layout = LauncherLayout::with_apps(app_ids);

        let layout_json =
            serde_json::to_vec(&layout).context("Failed to serialize launcher layout")?;

        self.file_storage
            .write(&self.layout_path, &layout_json, &self.encryption_key)
            .context("Failed to write launcher layout")?;

        Ok(())
    }

    /// Derive encryption key for launcher layout
    fn derive_layout_key(user_id: &str) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-launcher-layout-key-v1:");
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

    fn create_test_service() -> Result<(LauncherService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = LauncherService::new(temp_dir.path(), "user-123")?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_get_layout_empty() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let layout = service.get_layout()?;
        assert_eq!(layout.app_ids.len(), 0);

        Ok(())
    }

    #[test]
    fn test_set_and_get_layout() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let app_ids = vec![
            "com.osnova.launcher".to_string(),
            "com.osnova.wallet".to_string(),
            "com.osnova.config".to_string(),
        ];

        service.set_layout(app_ids.clone())?;

        let layout = service.get_layout()?;
        assert_eq!(layout.app_ids, app_ids);

        Ok(())
    }

    #[test]
    fn test_update_layout() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Set initial layout
        service.set_layout(vec!["app1".to_string(), "app2".to_string()])?;

        // Update layout
        let new_ids = vec!["app2".to_string(), "app1".to_string(), "app3".to_string()];
        service.set_layout(new_ids.clone())?;

        let layout = service.get_layout()?;
        assert_eq!(layout.app_ids, new_ids);

        Ok(())
    }

    #[test]
    fn test_layout_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let app_ids = vec!["app1".to_string(), "app2".to_string()];

        // Set layout in first service instance
        {
            let service = LauncherService::new(temp_dir.path(), "user-123")?;
            service.set_layout(app_ids.clone())?;
        }

        // Verify persistence in new service instance
        {
            let service = LauncherService::new(temp_dir.path(), "user-123")?;
            let layout = service.get_layout()?;
            assert_eq!(layout.app_ids, app_ids);
        }

        Ok(())
    }

    #[test]
    fn test_per_user_isolation() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let service1 = LauncherService::new(temp_dir.path(), "user-1")?;
        let service2 = LauncherService::new(temp_dir.path(), "user-2")?;

        // Set different layouts for each user
        service1.set_layout(vec!["app1".to_string()])?;
        service2.set_layout(vec!["app2".to_string(), "app3".to_string()])?;

        // Verify layouts are separate
        let layout1 = service1.get_layout()?;
        let layout2 = service2.get_layout()?;

        assert_eq!(layout1.app_ids.len(), 1);
        assert_eq!(layout2.app_ids.len(), 2);
        assert_ne!(layout1.app_ids, layout2.app_ids);

        Ok(())
    }
}
