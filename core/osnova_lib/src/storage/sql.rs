use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

use crate::crypto::encryption::CocoonEncryption;
use crate::models::application::OsnovaApplication;
use crate::models::config_cache::AppConfiguration;
use crate::models::device_key::DeviceKey;
use crate::models::pairing::{PairingSession, PairingStatus};

/// SQLite-based storage backend for Osnova
///
/// Provides persistent storage for:
/// - Installed applications
/// - Device keys
/// - Pairing sessions
/// - App configurations (encrypted at rest)
/// - Encrypted blob storage
///
/// # Example
///
/// ```no_run
/// use osnova_lib::storage::SqlStorage;
///
/// # fn main() -> anyhow::Result<()> {
/// let storage = SqlStorage::new("osnova.db")?;
/// # Ok(())
/// # }
/// ```
pub struct SqlStorage {
    conn: Connection,
}

impl SqlStorage {
    /// Create or open SQLite database at the specified path
    ///
    /// Initializes the database schema if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database file cannot be created/opened
    /// - Schema initialization fails
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path).context("Failed to open database")?;
        let storage = Self { conn };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Create an in-memory database for testing
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to create in-memory database")?;
        let storage = Self { conn };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Initialize database schema
    fn initialize_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS applications (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TABLE IF NOT EXISTS device_keys (
                device_id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS pairing_sessions (
                session_id TEXT PRIMARY KEY,
                server_public_key BLOB NOT NULL,
                device_public_key BLOB NOT NULL,
                established_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL,
                status TEXT NOT NULL CHECK(status IN ('pending', 'established', 'failed'))
            );

            CREATE TABLE IF NOT EXISTS app_configurations (
                app_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                settings_encrypted BLOB NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                PRIMARY KEY (app_id, user_id),
                FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS encrypted_blobs (
                key TEXT PRIMARY KEY,
                value_encrypted BLOB NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE INDEX IF NOT EXISTS idx_pairing_sessions_status
                ON pairing_sessions(status);
            "#,
            )
            .context("Failed to initialize schema")?;

        Ok(())
    }

    // ========================================================================
    // Application Management
    // ========================================================================

    /// Insert or update an application
    pub fn upsert_application(&self, app: &OsnovaApplication) -> Result<()> {
        let app_json = serde_json::to_string(app).context("Failed to serialize application")?;

        self.conn
            .execute(
                "INSERT INTO applications (id, data)
             VALUES (?1, ?2)
             ON CONFLICT(id) DO UPDATE SET
                data = excluded.data",
                params![app.id(), &app_json],
            )
            .context("Failed to upsert application")?;

        Ok(())
    }

    /// Get an application by ID
    pub fn get_application(&self, app_id: &str) -> Result<Option<OsnovaApplication>> {
        let result = self
            .conn
            .query_row(
                "SELECT data FROM applications WHERE id = ?1",
                params![app_id],
                |row| {
                    let data: String = row.get(0)?;
                    let app: OsnovaApplication = serde_json::from_str(&data)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                    Ok(app)
                },
            )
            .optional()
            .context("Failed to query application")?;

        Ok(result)
    }

    /// List all installed applications
    pub fn list_applications(&self) -> Result<Vec<OsnovaApplication>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM applications")
            .context("Failed to prepare statement")?;

        let apps = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                let app: OsnovaApplication = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(app)
            })
            .context("Failed to query applications")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse applications")?;

        Ok(apps)
    }

    /// Delete an application by ID
    pub fn delete_application(&self, app_id: &str) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM applications WHERE id = ?1", params![app_id])
            .context("Failed to delete application")?;

        Ok(rows_affected > 0)
    }

    // ========================================================================
    // Device Key Management
    // ========================================================================

    /// Insert a device key
    pub fn insert_device_key(&self, key: &DeviceKey) -> Result<()> {
        let key_json = serde_json::to_string(key).context("Failed to serialize device key")?;

        self.conn
            .execute(
                "INSERT INTO device_keys (device_id, data) VALUES (?1, ?2)",
                params![key.device_id(), &key_json],
            )
            .context("Failed to insert device key")?;

        Ok(())
    }

    /// Get a device key by device ID
    pub fn get_device_key(&self, device_id: &str) -> Result<Option<DeviceKey>> {
        let result = self
            .conn
            .query_row(
                "SELECT data FROM device_keys WHERE device_id = ?1",
                params![device_id],
                |row| {
                    let data: String = row.get(0)?;
                    let key: DeviceKey = serde_json::from_str(&data)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                    Ok(key)
                },
            )
            .optional()
            .context("Failed to query device key")?;

        Ok(result)
    }

    /// List all non-revoked device keys
    pub fn list_active_device_keys(&self) -> Result<Vec<DeviceKey>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM device_keys")
            .context("Failed to prepare statement")?;

        let keys: Vec<DeviceKey> = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                let key: DeviceKey = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(key)
            })
            .context("Failed to query device keys")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse device keys")?;

        // Filter to only active (non-revoked) keys
        let active_keys: Vec<DeviceKey> = keys
            .into_iter()
            .filter(|k| k.revoked_at().is_none())
            .collect();

        Ok(active_keys)
    }

    /// Revoke a device key
    pub fn revoke_device_key(&self, device_id: &str, revoked_at: i64) -> Result<bool> {
        // Get the key
        let mut key = match self.get_device_key(device_id)? {
            Some(k) => k,
            None => return Ok(false),
        };

        // Already revoked?
        if key.is_revoked() {
            return Ok(false);
        }

        // Revoke and update
        key.revoke_at(revoked_at as u64);
        let key_json = serde_json::to_string(&key).context("Failed to serialize device key")?;

        let rows_affected = self
            .conn
            .execute(
                "UPDATE device_keys SET data = ?1 WHERE device_id = ?2",
                params![&key_json, device_id],
            )
            .context("Failed to revoke device key")?;

        Ok(rows_affected > 0)
    }

    // ========================================================================
    // Pairing Session Management
    // ========================================================================

    /// Insert or update a pairing session
    pub fn upsert_pairing_session(&self, session: &PairingSession) -> Result<()> {
        let status_str = match session.status() {
            PairingStatus::Pending => "pending",
            PairingStatus::Established => "established",
            PairingStatus::Failed => "failed",
        };

        self.conn
            .execute(
                "INSERT INTO pairing_sessions
             (session_id, server_public_key, device_public_key, established_at, expires_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(session_id) DO UPDATE SET
                status = excluded.status",
                params![
                    session.session_id(),
                    session.server_public_key(),
                    session.device_public_key(),
                    session.established_at().unwrap_or(0),
                    session.expires_at().unwrap_or(0),
                    status_str,
                ],
            )
            .context("Failed to upsert pairing session")?;

        Ok(())
    }

    /// Get a pairing session by ID
    pub fn get_pairing_session(&self, session_id: &str) -> Result<Option<PairingSession>> {
        let result = self
            .conn
            .query_row(
                "SELECT session_id, server_public_key, device_public_key, established_at, expires_at, status
                 FROM pairing_sessions WHERE session_id = ?1",
                params![session_id],
                |row| {
                    let session_id: String = row.get(0)?;
                    let server_key: Vec<u8> = row.get(1)?;
                    let device_key: Vec<u8> = row.get(2)?;
                    let status_str: String = row.get(5)?;

                    let mut session = PairingSession::new(&session_id, &server_key, &device_key)
                        .map_err(|_| rusqlite::Error::InvalidQuery)?;

                    // Set status based on string
                    match status_str.as_str() {
                        "established" => session.mark_established(),
                        "failed" => session.mark_failed(),
                        _ => {} // pending is default
                    }

                    Ok(session)
                },
            )
            .optional()
            .context("Failed to query pairing session")?;

        Ok(result)
    }

    /// List pairing sessions by status
    pub fn list_pairing_sessions_by_status(&self, status: &str) -> Result<Vec<PairingSession>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT session_id, server_public_key, device_public_key, established_at, expires_at, status
                 FROM pairing_sessions WHERE status = ?1",
            )
            .context("Failed to prepare statement")?;

        let sessions = stmt
            .query_map(params![status], |row| {
                let session_id: String = row.get(0)?;
                let server_key: Vec<u8> = row.get(1)?;
                let device_key: Vec<u8> = row.get(2)?;
                let status_str: String = row.get(5)?;

                let mut session = PairingSession::new(&session_id, &server_key, &device_key)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                // Set status based on string
                match status_str.as_str() {
                    "established" => session.mark_established(),
                    "failed" => session.mark_failed(),
                    _ => {} // pending is default
                }

                Ok(session)
            })
            .context("Failed to query pairing sessions")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse pairing sessions")?;

        Ok(sessions)
    }

    // ========================================================================
    // App Configuration (Encrypted)
    // ========================================================================

    /// Set app configuration (encrypted at rest)
    ///
    /// # Errors
    ///
    /// Returns an error if encryption or database write fails
    pub fn set_app_config(
        &self,
        app_id: &str,
        user_id: &str,
        config: &AppConfiguration,
        encryption_key: &[u8; 32],
    ) -> Result<()> {
        let config_json = serde_json::to_vec(config).context("Failed to serialize config")?;
        let encryption = CocoonEncryption::new(encryption_key);
        let encrypted = encryption
            .encrypt(&config_json)
            .context("Failed to encrypt config")?;

        self.conn
            .execute(
                "INSERT INTO app_configurations (app_id, user_id, settings_encrypted, updated_at)
             VALUES (?1, ?2, ?3, strftime('%s', 'now'))
             ON CONFLICT(app_id, user_id) DO UPDATE SET
                settings_encrypted = excluded.settings_encrypted,
                updated_at = excluded.updated_at",
                params![app_id, user_id, &encrypted],
            )
            .context("Failed to upsert app configuration")?;

        Ok(())
    }

    /// Get app configuration (decrypted)
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails or configuration doesn't exist
    pub fn get_app_config(
        &self,
        app_id: &str,
        user_id: &str,
        encryption_key: &[u8; 32],
    ) -> Result<Option<AppConfiguration>> {
        let encrypted: Option<Vec<u8>> = self
            .conn
            .query_row(
                "SELECT settings_encrypted FROM app_configurations
                 WHERE app_id = ?1 AND user_id = ?2",
                params![app_id, user_id],
                |row| row.get(0),
            )
            .optional()
            .context("Failed to query app configuration")?;

        match encrypted {
            Some(data) => {
                let encryption = CocoonEncryption::new(encryption_key);
                let decrypted = encryption
                    .decrypt(&data)
                    .context("Failed to decrypt config")?;
                let config: AppConfiguration =
                    serde_json::from_slice(&decrypted).context("Failed to deserialize config")?;
                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    /// Delete app configuration
    pub fn delete_app_config(&self, app_id: &str, user_id: &str) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute(
                "DELETE FROM app_configurations WHERE app_id = ?1 AND user_id = ?2",
                params![app_id, user_id],
            )
            .context("Failed to delete app configuration")?;

        Ok(rows_affected > 0)
    }

    // ========================================================================
    // Encrypted Blob Storage
    // ========================================================================

    /// Store an encrypted blob
    pub fn set_encrypted_blob(
        &self,
        key: &str,
        value: &[u8],
        encryption_key: &[u8; 32],
    ) -> Result<()> {
        let encryption = CocoonEncryption::new(encryption_key);
        let encrypted = encryption
            .encrypt(value)
            .context("Failed to encrypt blob")?;

        self.conn
            .execute(
                "INSERT INTO encrypted_blobs (key, value_encrypted, updated_at)
             VALUES (?1, ?2, strftime('%s', 'now'))
             ON CONFLICT(key) DO UPDATE SET
                value_encrypted = excluded.value_encrypted,
                updated_at = excluded.updated_at",
                params![key, &encrypted],
            )
            .context("Failed to upsert encrypted blob")?;

        Ok(())
    }

    /// Retrieve and decrypt a blob
    pub fn get_encrypted_blob(
        &self,
        key: &str,
        encryption_key: &[u8; 32],
    ) -> Result<Option<Vec<u8>>> {
        let encrypted: Option<Vec<u8>> = self
            .conn
            .query_row(
                "SELECT value_encrypted FROM encrypted_blobs WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .context("Failed to query encrypted blob")?;

        match encrypted {
            Some(data) => {
                let encryption = CocoonEncryption::new(encryption_key);
                let decrypted = encryption
                    .decrypt(&data)
                    .context("Failed to decrypt blob")?;
                Ok(Some(decrypted))
            }
            None => Ok(None),
        }
    }

    /// Delete an encrypted blob
    pub fn delete_encrypted_blob(&self, key: &str) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM encrypted_blobs WHERE key = ?1", params![key])
            .context("Failed to delete encrypted blob")?;

        Ok(rows_affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::application::{ComponentKind, ComponentRef};
    use crate::models::pairing::PairingStatus;

    fn create_test_app() -> OsnovaApplication {
        let component = ComponentRef::new(
            "component-001",
            "Test Component",
            ComponentKind::Frontend,
            "1.0.0",
        )
        .unwrap();

        OsnovaApplication::new(
            "app-001",
            "Test App",
            "1.0.0",
            "https://example.com/icon.png",
            "Test application description",
            vec![component],
        )
        .unwrap()
    }

    #[test]
    fn test_upsert_and_get_application() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;
        let app = create_test_app();

        // Insert
        storage.upsert_application(&app)?;
        let retrieved = storage.get_application(app.id())?;
        assert!(retrieved.is_some());
        let retrieved_app = retrieved.unwrap();
        assert_eq!(retrieved_app.name(), app.name());

        // Update
        let component = ComponentRef::new(
            "component-001",
            "Test Component",
            ComponentKind::Frontend,
            "1.0.0",
        )?;
        let updated_app = OsnovaApplication::new(
            app.id(),
            "Updated App",
            app.version(),
            app.icon_uri(),
            app.description(),
            vec![component],
        )?;
        storage.upsert_application(&updated_app)?;
        let retrieved = storage.get_application(updated_app.id())?;
        assert_eq!(retrieved.unwrap().name(), "Updated App");

        Ok(())
    }

    #[test]
    fn test_list_applications() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;

        let app1 = create_test_app();
        let component2 = ComponentRef::new(
            "component-002",
            "Component 2",
            ComponentKind::Backend,
            "1.0.0",
        )?;
        let app2 = OsnovaApplication::new(
            "app-002",
            "Second App",
            "1.0.0",
            "https://example.com/icon2.png",
            "Second app description",
            vec![component2],
        )?;

        storage.upsert_application(&app1)?;
        storage.upsert_application(&app2)?;

        let apps = storage.list_applications()?;
        assert_eq!(apps.len(), 2);

        Ok(())
    }

    #[test]
    fn test_delete_application() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;
        let app = create_test_app();

        storage.upsert_application(&app)?;
        assert!(storage.delete_application(app.id())?);
        assert!(storage.get_application(app.id())?.is_none());

        // Delete non-existent
        assert!(!storage.delete_application("nonexistent")?);

        Ok(())
    }

    #[test]
    fn test_device_key_operations() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;

        let public_key = [1u8; 32]; // DeviceKey requires 32-byte keys
        let key = DeviceKey::new("device-001", &public_key)?;

        // Insert
        storage.insert_device_key(&key)?;
        let retrieved = storage.get_device_key(key.device_id())?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.as_ref().unwrap().public_key(), key.public_key());

        // List active keys
        let active_keys = storage.list_active_device_keys()?;
        assert_eq!(active_keys.len(), 1);

        // Revoke
        assert!(storage.revoke_device_key(key.device_id(), 2000)?);
        let active_keys = storage.list_active_device_keys()?;
        assert_eq!(active_keys.len(), 0);

        // Verify revocation timestamp
        let retrieved = storage.get_device_key(key.device_id())?;
        assert_eq!(retrieved.unwrap().revoked_at(), Some(2000));

        Ok(())
    }

    #[test]
    fn test_pairing_session_operations() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;

        let server_key = [1u8; 32];
        let device_key = [2u8; 32];
        let mut session = PairingSession::new("session-001", &server_key, &device_key)?;

        // Insert
        storage.upsert_pairing_session(&session)?;
        let retrieved = storage.get_pairing_session(session.session_id())?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.as_ref().unwrap().status(), PairingStatus::Pending);

        // Update status
        session.mark_established();
        storage.upsert_pairing_session(&session)?;

        let retrieved = storage.get_pairing_session(session.session_id())?;
        assert_eq!(retrieved.unwrap().status(), PairingStatus::Established);

        // List by status
        let established = storage.list_pairing_sessions_by_status("established")?;
        assert_eq!(established.len(), 1);

        Ok(())
    }

    #[test]
    fn test_encrypted_config_operations() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;
        let encryption_key = [42u8; 32];

        // Create and insert app first (foreign key requirement)
        let app = create_test_app();
        storage.upsert_application(&app)?;

        let mut config = AppConfiguration::new(app.id(), "user-001");
        config.set_setting("theme", serde_json::json!("dark"));

        // Set config
        storage.set_app_config(config.app_id(), config.user_id(), &config, &encryption_key)?;

        // Get config
        let retrieved =
            storage.get_app_config(config.app_id(), config.user_id(), &encryption_key)?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.as_ref().unwrap().settings().len(), 1);

        // Delete config
        assert!(storage.delete_app_config(config.app_id(), config.user_id())?);
        assert!(storage
            .get_app_config(config.app_id(), config.user_id(), &encryption_key)?
            .is_none());

        Ok(())
    }

    #[test]
    fn test_encrypted_blob_operations() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;
        let encryption_key = [99u8; 32];
        let data = b"secret data";

        // Set blob
        storage.set_encrypted_blob("test-key", data, &encryption_key)?;

        // Get blob
        let retrieved = storage.get_encrypted_blob("test-key", &encryption_key)?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), data);

        // Delete blob
        assert!(storage.delete_encrypted_blob("test-key")?);
        assert!(storage
            .get_encrypted_blob("test-key", &encryption_key)?
            .is_none());

        Ok(())
    }

    #[test]
    fn test_wrong_encryption_key_fails() -> Result<()> {
        let storage = SqlStorage::new_in_memory()?;
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];

        storage.set_encrypted_blob("test", b"data", &key1)?;

        // Should fail with wrong key
        let result = storage.get_encrypted_blob("test", &key2);
        assert!(result.is_err());

        Ok(())
    }
}
