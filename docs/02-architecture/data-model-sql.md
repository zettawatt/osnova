# Data Model SQL Schema

**Last Updated**: 2025-10-08
**Database**: SQLite 3.x (embedded, cross-platform)
**Encryption**: Database file encrypted at rest using Cocoon (ChaCha20-Poly1305)

## Overview

This document provides the concrete SQL schema implementation for the Osnova data model. All persistent data is stored in a single SQLite database per user identity, located at `{data_dir}/osnova.db`.

## Database Location

```rust
use osnova_lib::platform::paths::get_data_dir;

fn get_database_path() -> Result<PathBuf> {
    let mut path = get_data_dir()?;
    path.push("osnova.db");
    Ok(path)
}
```

## Schema Version Management

```sql
-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);

-- Current version
INSERT INTO schema_version (version, description)
VALUES (1, 'Initial schema');
```

## Core Tables

### 1. Applications Table

Stores installed Osnova applications and their metadata.

```sql
CREATE TABLE IF NOT EXISTS applications (
    -- Primary key is content address of manifest
    id TEXT PRIMARY KEY,                -- Content address (ant:// URI or hash)
    name TEXT NOT NULL,                  -- Human-readable name
    version TEXT NOT NULL,               -- Semver string (e.g., "1.2.3")
    manifest_uri TEXT NOT NULL,          -- Original manifest URI
    icon_uri TEXT,                       -- URI to application icon
    description TEXT,                    -- Application description
    publisher TEXT,                      -- Publisher identifier
    installed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    -- Ensure unique name+version combination
    UNIQUE(name, version)
);

CREATE INDEX idx_applications_name ON applications(name);
CREATE INDEX idx_applications_installed ON applications(installed_at);
```

### 2. Components Table

Tracks components referenced by applications (frontend and backend).

```sql
CREATE TABLE IF NOT EXISTS components (
    id TEXT PRIMARY KEY,                -- Component content address
    app_id TEXT NOT NULL,                -- Foreign key to applications
    name TEXT NOT NULL,                  -- Component name
    kind TEXT NOT NULL CHECK(kind IN ('frontend', 'backend')),
    version TEXT NOT NULL,               -- Semver string
    platform TEXT,                       -- Platform (iOS, Android, desktop)
    target TEXT,                         -- Target triple for backend components
    hash TEXT NOT NULL,                  -- Content hash for verification
    cache_path TEXT,                     -- Local cache location
    compiled_path TEXT,                  -- Path to compiled binary (backend only)
    config TEXT,                         -- JSON configuration
    installed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP,

    FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE
);

CREATE INDEX idx_components_app ON components(app_id);
CREATE INDEX idx_components_kind ON components(kind);
CREATE INDEX idx_components_last_used ON components(last_used);
```

### 3. Application Configuration Table

Per-app, per-user configuration settings.

```sql
CREATE TABLE IF NOT EXISTS app_configurations (
    app_id TEXT NOT NULL,                -- Foreign key to applications
    key TEXT NOT NULL,                   -- Configuration key
    value TEXT,                          -- JSON-encoded value
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (app_id, key),
    FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE
);

CREATE INDEX idx_app_config_updated ON app_configurations(updated_at);
```

### 4. Application Cache Metadata Table

Tracks cache entries for applications.

```sql
CREATE TABLE IF NOT EXISTS app_cache (
    app_id TEXT NOT NULL,                -- Foreign key to applications
    cache_key TEXT NOT NULL,             -- Cache entry key
    size_bytes INTEGER,                  -- Size in bytes
    content_hash TEXT,                   -- Hash of cached content
    expires_at TIMESTAMP,                -- Optional expiration time
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    accessed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (app_id, cache_key),
    FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE
);

CREATE INDEX idx_app_cache_expires ON app_cache(expires_at);
CREATE INDEX idx_app_cache_accessed ON app_cache(accessed_at);
```

### 5. Identity Table

Stores the current identity information (one row only).

```sql
CREATE TABLE IF NOT EXISTS identity (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Ensures single row
    address TEXT NOT NULL,                  -- 4-word saorsa address
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Seed phrase stored in platform keystore, never in database
```

### 6. Device Keys Table

Manages device-specific keys derived from the root identity.

```sql
CREATE TABLE IF NOT EXISTS device_keys (
    device_id TEXT PRIMARY KEY,          -- Unique device identifier
    public_key TEXT NOT NULL,            -- Base64-encoded public key
    key_type TEXT NOT NULL,              -- Key algorithm (ed25519, secp256k1, etc.)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    revoked_at TIMESTAMP,                -- NULL if active

    CHECK (key_type IN ('ed25519', 'secp256k1', 'ml_dsa'))
);

CREATE INDEX idx_device_keys_created ON device_keys(created_at);
CREATE INDEX idx_device_keys_revoked ON device_keys(revoked_at);
```

### 7. Component Keys Table

Tracks keys derived for components.

```sql
CREATE TABLE IF NOT EXISTS component_keys (
    component_id TEXT NOT NULL,          -- Component identifier
    key_index INTEGER NOT NULL,          -- Key derivation index
    public_key TEXT NOT NULL,            -- Base64-encoded public key
    key_type TEXT NOT NULL,              -- Key algorithm
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (component_id, key_index),
    FOREIGN KEY (component_id) REFERENCES components(id) ON DELETE CASCADE
);

CREATE INDEX idx_component_keys_created ON component_keys(created_at);
```

### 8. Pairing Sessions Table

Manages client-server pairing sessions.

```sql
CREATE TABLE IF NOT EXISTS pairing_sessions (
    session_id TEXT PRIMARY KEY,         -- Unique session identifier
    server_public_key TEXT,              -- Server's public key
    device_public_key TEXT,              -- Device's public key
    shared_secret_hash TEXT,             -- Hash of derived shared secret
    server_address TEXT,                 -- Server network address
    status TEXT NOT NULL CHECK(status IN ('pending', 'established', 'failed', 'expired')),
    established_at TIMESTAMP,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (device_public_key) REFERENCES device_keys(public_key)
);

CREATE INDEX idx_pairing_status ON pairing_sessions(status);
CREATE INDEX idx_pairing_expires ON pairing_sessions(expires_at);
```

### 9. Server Configuration Table

Stores server mode configuration.

```sql
CREATE TABLE IF NOT EXISTS server_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Ensures single row
    server_id TEXT,                         -- Server identifier
    hostname TEXT,                          -- Server hostname/IP
    port INTEGER DEFAULT 8080,              -- Server port
    max_clients INTEGER DEFAULT 5,          -- Max concurrent clients
    tls_cert_path TEXT,                     -- Path to TLS certificate
    tls_key_path TEXT,                      -- Path to TLS private key
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 10. Launcher Layout Table

Stores the launcher app icon layout per user.

```sql
CREATE TABLE IF NOT EXISTS launcher_layout (
    position INTEGER PRIMARY KEY,         -- Icon position (0-based)
    app_id TEXT NOT NULL,                 -- Foreign key to applications
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE
);
```

### 11. Storage Backend Table

Encrypted key-value storage for components and user data.

```sql
CREATE TABLE IF NOT EXISTS encrypted_storage (
    namespace TEXT NOT NULL,              -- Component ID or 'system'
    key TEXT NOT NULL,                    -- Storage key
    value BLOB NOT NULL,                  -- Encrypted value (Cocoon)
    nonce BLOB NOT NULL,                  -- Encryption nonce
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (namespace, key)
);

CREATE INDEX idx_storage_namespace ON encrypted_storage(namespace);
CREATE INDEX idx_storage_updated ON encrypted_storage(updated_at);
```

## Rust Implementation

### Database Connection

```rust
use rusqlite::{Connection, Result};
use cocoon::Cocoon;

pub struct Database {
    conn: Connection,
    cocoon: Cocoon,
}

impl Database {
    pub fn open(path: &Path, password: &[u8]) -> Result<Self> {
        // Open SQLite connection
        let conn = Connection::open(path)?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        // Set journal mode for better concurrency
        conn.execute("PRAGMA journal_mode = WAL", [])?;

        // Initialize Cocoon for encryption
        let cocoon = Cocoon::new(password);

        // Run migrations
        Self::migrate(&conn)?;

        Ok(Database { conn, cocoon })
    }

    fn migrate(conn: &Connection) -> Result<()> {
        // Check current version
        let version = Self::get_schema_version(conn)?;

        // Apply migrations based on version
        if version < 1 {
            Self::migrate_v1(conn)?;
        }

        Ok(())
    }
}
```

### Data Access Layer

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_uri: String,
    pub icon_uri: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub installed_at: DateTime<Utc>,
}

impl Database {
    pub fn insert_application(&self, app: &Application) -> Result<()> {
        let sql = "
            INSERT INTO applications (id, name, version, manifest_uri, icon_uri, description, publisher)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        self.conn.execute(sql, params![
            app.id,
            app.name,
            app.version,
            app.manifest_uri,
            app.icon_uri,
            app.description,
            app.publisher,
        ])?;

        Ok(())
    }

    pub fn get_application(&self, app_id: &str) -> Result<Option<Application>> {
        let sql = "SELECT * FROM applications WHERE id = ?1";

        let app = self.conn.query_row(sql, [app_id], |row| {
            Ok(Application {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                manifest_uri: row.get(3)?,
                icon_uri: row.get(4)?,
                description: row.get(5)?,
                publisher: row.get(6)?,
                installed_at: row.get(7)?,
            })
        }).optional()?;

        Ok(app)
    }

    pub fn list_applications(&self) -> Result<Vec<Application>> {
        let sql = "SELECT * FROM applications ORDER BY name, version DESC";

        let mut stmt = self.conn.prepare(sql)?;
        let apps = stmt.query_map([], |row| {
            Ok(Application {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                manifest_uri: row.get(3)?,
                icon_uri: row.get(4)?,
                description: row.get(5)?,
                publisher: row.get(6)?,
                installed_at: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(apps)
    }
}
```

### Encrypted Storage Implementation

```rust
impl Database {
    pub fn store_encrypted(&self, namespace: &str, key: &str, value: &[u8]) -> Result<()> {
        // Encrypt value with Cocoon
        let encrypted = self.cocoon.encrypt(value)?;

        let sql = "
            INSERT OR REPLACE INTO encrypted_storage (namespace, key, value, nonce)
            VALUES (?1, ?2, ?3, ?4)
        ";

        self.conn.execute(sql, params![
            namespace,
            key,
            encrypted.data,
            encrypted.nonce,
        ])?;

        Ok(())
    }

    pub fn retrieve_encrypted(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>> {
        let sql = "SELECT value, nonce FROM encrypted_storage WHERE namespace = ?1 AND key = ?2";

        let result = self.conn.query_row(sql, params![namespace, key], |row| {
            let encrypted_data: Vec<u8> = row.get(0)?;
            let nonce: Vec<u8> = row.get(1)?;

            // Decrypt with Cocoon
            let decrypted = self.cocoon.decrypt(&encrypted_data, &nonce)?;
            Ok(decrypted)
        }).optional()?;

        Ok(result)
    }
}
```

## Database Maintenance

### Cleanup Expired Cache

```sql
-- Delete expired cache entries
DELETE FROM app_cache
WHERE expires_at IS NOT NULL
  AND expires_at < CURRENT_TIMESTAMP;
```

### Vacuum Database

```sql
-- Reclaim space after deletions
VACUUM;

-- Analyze for query optimization
ANALYZE;
```

### Export/Import for Backup

```rust
impl Database {
    pub fn export_backup(&self, path: &Path) -> Result<()> {
        let backup_conn = Connection::open(path)?;
        self.conn.backup(DatabaseName::Main, &backup_conn, DatabaseName::Main)?;
        Ok(())
    }

    pub fn import_backup(&self, path: &Path) -> Result<()> {
        let backup_conn = Connection::open(path)?;
        backup_conn.backup(DatabaseName::Main, &self.conn, DatabaseName::Main)?;
        Ok(())
    }
}
```

## Performance Considerations

1. **Write-Ahead Logging (WAL)**: Enabled for better concurrency
2. **Foreign Keys**: Enforced for referential integrity
3. **Indexes**: Created on frequently queried columns
4. **Prepared Statements**: Use parameter binding to prevent SQL injection
5. **Connection Pooling**: Single connection per process (SQLite limitation)
6. **Encryption Overhead**: ~5-10% performance impact from Cocoon encryption

## Migration Strategy

```rust
impl Database {
    fn migrate_v2(conn: &Connection) -> Result<()> {
        // Example migration to version 2
        conn.execute("
            ALTER TABLE applications
            ADD COLUMN auto_update BOOLEAN DEFAULT FALSE
        ", [])?;

        conn.execute("
            INSERT INTO schema_version (version, description)
            VALUES (2, 'Add auto-update support')
        ", [])?;

        Ok(())
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let password = b"test_password";

        let db = Database::open(&db_path, password).unwrap();

        // Verify schema creation
        let count: i32 = db.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get(0)
        ).unwrap();

        assert!(count > 0);
    }

    #[test]
    fn test_application_crud() {
        let db = create_test_database();

        let app = Application {
            id: "ant://test123".to_string(),
            name: "Test App".to_string(),
            version: "1.0.0".to_string(),
            manifest_uri: "ant://manifest".to_string(),
            icon_uri: Some("ant://icon".to_string()),
            description: Some("Test application".to_string()),
            publisher: Some("TestCorp".to_string()),
            installed_at: Utc::now(),
        };

        // Insert
        db.insert_application(&app).unwrap();

        // Read
        let loaded = db.get_application(&app.id).unwrap().unwrap();
        assert_eq!(loaded.name, app.name);

        // List
        let apps = db.list_applications().unwrap();
        assert_eq!(apps.len(), 1);
    }
}
```

## Security Notes

1. **Database Encryption**: Entire database file encrypted at rest using Cocoon
2. **Key Storage**: Master key never stored in database, only in platform keystore
3. **SQL Injection**: Use parameter binding for all queries
4. **Access Control**: Database file permissions set to 0600 (user read/write only)
5. **Sensitive Data**: Seed phrases and private keys never stored in database

## Next Steps

1. Implement database connection pooling for server mode
2. Add database backup scheduling
3. Implement automatic migration system
4. Add database integrity checks
5. Create performance benchmarks