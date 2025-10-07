use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::storage::SqlStorage;

/// Application list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppListItem {
    /// Application ID
    pub id: String,
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Icon URI
    pub icon_uri: String,
    /// Manifest URI
    pub manifest_uri: String,
}

/// Application management service
///
/// Provides OpenRPC methods:
/// - `apps.list` - List all installed applications
/// - `apps.launch` - Launch an application by ID
/// - `apps.install` - Install a new application from manifest URI
/// - `apps.uninstall` - Remove an installed application
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::AppsService;
///
/// # fn example() -> anyhow::Result<()> {
/// let service = AppsService::new("/path/to/storage")?;
///
/// // List installed apps
/// let apps = service.list()?;
/// for app in apps {
///     println!("{}: {}", app.id, app.name);
/// }
/// # Ok(())
/// # }
/// ```
pub struct AppsService {
    sql_storage: SqlStorage,
}

impl AppsService {
    /// Create a new apps service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    pub fn new<P: Into<PathBuf>>(storage_path: P) -> Result<Self> {
        let storage_path = storage_path.into();
        let sql_storage = SqlStorage::new(storage_path.join("osnova.db"))?;

        Ok(Self { sql_storage })
    }

    /// List all installed applications (OpenRPC: apps.list)
    ///
    /// Returns a list of all installed applications with their metadata.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::AppsService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = AppsService::new("/tmp/storage")?;
    /// let apps = service.list()?;
    /// println!("Installed apps: {}", apps.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> Result<Vec<AppListItem>> {
        let apps = self.sql_storage.list_applications()?;

        Ok(apps
            .into_iter()
            .map(|app| AppListItem {
                id: app.id().to_string(),
                name: app.name().to_string(),
                version: app.version().to_string(),
                icon_uri: app.icon_uri().to_string(),
                manifest_uri: app.id().to_string(), // TODO: Store manifest URI separately
            })
            .collect())
    }

    /// Launch an application by ID (OpenRPC: apps.launch)
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application ID to launch
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::AppsService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = AppsService::new("/tmp/storage")?;
    /// service.launch("com.osnova.launcher")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn launch(&self, app_id: &str) -> Result<()> {
        // Verify app exists
        let _app = self
            .sql_storage
            .get_application(app_id)?
            .context(format!("Application {} not found", app_id))?;

        // TODO: Actually launch the application
        // For now, this is a stub that just verifies the app exists
        Ok(())
    }

    /// Install a new application from manifest URI (OpenRPC: apps.install)
    ///
    /// # Arguments
    ///
    /// * `manifest_uri` - URI to the application manifest (ant:// or local path)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::AppsService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = AppsService::new("/tmp/storage")?;
    /// service.install("ant://manifest-address")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install(&self, _manifest_uri: &str) -> Result<()> {
        // TODO: Implement manifest fetching and parsing
        // TODO: Download and cache components
        // TODO: Store application in database
        anyhow::bail!("Application installation not yet implemented")
    }

    /// Uninstall an application (OpenRPC: apps.uninstall)
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application ID to uninstall
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::AppsService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = AppsService::new("/tmp/storage")?;
    /// service.uninstall("com.example.app")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn uninstall(&self, app_id: &str) -> Result<()> {
        let deleted = self.sql_storage.delete_application(app_id)?;

        if !deleted {
            anyhow::bail!("Application {} not found", app_id);
        }

        // TODO: Clean up cached components
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::application::OsnovaApplication;
    use tempfile::TempDir;

    fn create_test_service() -> Result<(AppsService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = AppsService::new(temp_dir.path())?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_list_empty() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let apps = service.list()?;
        assert_eq!(apps.len(), 0);

        Ok(())
    }

    #[test]
    fn test_list_apps() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Add some test apps
        let app1 = OsnovaApplication::new(
            "com.test.app1",
            "Test App 1",
            "1.0.0",
            "https://icon1.url",
            "Test app 1",
            vec![],
        )?;
        let app2 = OsnovaApplication::new(
            "com.test.app2",
            "Test App 2",
            "2.0.0",
            "https://icon2.url",
            "Test app 2",
            vec![],
        )?;

        service.sql_storage.upsert_application(&app1)?;
        service.sql_storage.upsert_application(&app2)?;

        let apps = service.list()?;
        assert_eq!(apps.len(), 2);
        assert!(apps.iter().any(|a| a.id == "com.test.app1"));
        assert!(apps.iter().any(|a| a.id == "com.test.app2"));

        Ok(())
    }

    #[test]
    fn test_launch_existing_app() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let app = OsnovaApplication::new(
            "com.test.app",
            "Test App",
            "1.0.0",
            "https://icon.url",
            "Test app",
            vec![],
        )?;
        service.sql_storage.upsert_application(&app)?;

        // Should not error
        service.launch("com.test.app")?;

        Ok(())
    }

    #[test]
    fn test_launch_nonexistent_app() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let result = service.launch("com.nonexistent.app");
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_uninstall() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let app = OsnovaApplication::new(
            "com.test.app",
            "Test App",
            "1.0.0",
            "https://icon.url",
            "Test app",
            vec![],
        )?;
        service.sql_storage.upsert_application(&app)?;

        // Verify app exists
        let apps = service.list()?;
        assert_eq!(apps.len(), 1);

        // Uninstall
        service.uninstall("com.test.app")?;

        // Verify app is gone
        let apps = service.list()?;
        assert_eq!(apps.len(), 0);

        Ok(())
    }

    #[test]
    fn test_uninstall_nonexistent() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let result = service.uninstall("com.nonexistent.app");
        assert!(result.is_err());

        Ok(())
    }
}
