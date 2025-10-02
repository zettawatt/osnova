# Storage Backend Architecture

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Use SQLite for structured data, encrypted files for blobs

## Overview

The Osnova storage backend uses a hybrid approach combining SQLite for structured, queryable data and encrypted files for large binary objects (blobs). This provides the best balance of performance, simplicity, and security for the MVP.

## Storage Strategy

### SQLite for Structured Data

**Use Cases**:
- Component metadata (manifests, versions, dependencies)
- Application metadata (installed apps, configurations)
- User preferences and settings
- Component registry (running components, PIDs, endpoints)
- Transaction history (wallet transactions)
- Key metadata (public keys, derivation indices)
- Message metadata (sender, timestamp, channel)
- Cache indices and lookup tables

**Location**: `$DATA_ROOT/osnova.db`

**Schema Design**:
- Normalized tables for relational data
- JSON columns for flexible metadata
- Full-text search indices for searchable content
- Foreign key constraints for referential integrity

**Benefits**:
- Fast queries and joins
- ACID transactions
- Built-in full-text search
- No external dependencies
- Cross-platform support
- Excellent Rust support (rusqlite, sqlx)

### Encrypted Files for Blobs

**Use Cases**:
- Cocoon files (encrypted key storage)
- Cached component binaries
- Cached frontend tarballs
- Large message attachments
- Autonomi cached chunks
- User-uploaded files
- Backup archives

**Location**: `$DATA_ROOT/blobs/<hash>.enc`

**Encryption**:
- ChaCha20-Poly1305 for authenticated encryption
- Keys derived from master key via HKDF
- File naming: SHA-256 hash of plaintext content
- Metadata stored in SQLite, content in encrypted files

**Benefits**:
- Efficient for large files
- No database bloat
- Easy to backup/restore
- Content-addressable storage
- Encryption at rest

## Database Schema (MVP)

### Core Tables

#### components
```sql
CREATE TABLE components (
    component_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('frontend', 'backend')),
    description TEXT,
    author TEXT,
    license TEXT,
    manifest_json TEXT NOT NULL, -- Full manifest as JSON
    autonomi_address TEXT,
    installed_at INTEGER NOT NULL,
    last_used INTEGER,
    UNIQUE(component_id, version)
);

CREATE INDEX idx_components_type ON components(type);
CREATE INDEX idx_components_installed ON components(installed_at DESC);
```

#### applications
```sql
CREATE TABLE applications (
    application_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT,
    author TEXT,
    icon_blob_hash TEXT, -- Reference to encrypted blob
    manifest_json TEXT NOT NULL,
    autonomi_address TEXT,
    installed_at INTEGER NOT NULL,
    last_launched INTEGER,
    launch_count INTEGER DEFAULT 0,
    UNIQUE(application_id, version)
);

CREATE INDEX idx_applications_last_launched ON applications(last_launched DESC);
```

#### component_registry
```sql
CREATE TABLE component_registry (
    component_id TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    pid INTEGER NOT NULL,
    endpoint TEXT NOT NULL, -- OpenRPC endpoint URL
    status TEXT NOT NULL CHECK(status IN ('starting', 'running', 'stopping', 'stopped', 'crashed')),
    started_at INTEGER NOT NULL,
    last_heartbeat INTEGER,
    restart_count INTEGER DEFAULT 0,
    FOREIGN KEY(component_id) REFERENCES components(component_id)
);

CREATE INDEX idx_registry_status ON component_registry(status);
```

#### keys_metadata
```sql
CREATE TABLE keys_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    component_id TEXT NOT NULL,
    derivation_index INTEGER NOT NULL,
    public_key TEXT NOT NULL UNIQUE,
    key_type TEXT NOT NULL CHECK(key_type IN ('ml_dsa', 'ed25519', 'secp256k1')),
    purpose TEXT, -- 'identity', 'wallet', 'signing', etc.
    created_at INTEGER NOT NULL,
    last_used INTEGER,
    UNIQUE(component_id, derivation_index)
);

CREATE INDEX idx_keys_component ON keys_metadata(component_id);
CREATE INDEX idx_keys_public ON keys_metadata(public_key);
```

#### wallet_transactions
```sql
CREATE TABLE wallet_transactions (
    tx_hash TEXT PRIMARY KEY,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount TEXT NOT NULL, -- Store as string to avoid precision issues
    token_address TEXT, -- NULL for ETH
    network TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'confirmed', 'failed')),
    gas_used TEXT,
    gas_price_gwei TEXT,
    purpose TEXT,
    component_id TEXT, -- Component that requested the transaction
    timestamp INTEGER NOT NULL,
    confirmed_at INTEGER
);

CREATE INDEX idx_tx_from ON wallet_transactions(from_address, timestamp DESC);
CREATE INDEX idx_tx_status ON wallet_transactions(status);
```

#### autonomi_cache
```sql
CREATE TABLE autonomi_cache (
    autonomi_address TEXT PRIMARY KEY,
    content_hash TEXT NOT NULL, -- SHA-256 of content
    blob_hash TEXT NOT NULL, -- Reference to encrypted blob
    size INTEGER NOT NULL,
    data_type TEXT NOT NULL, -- 'chunk', 'pointer', 'scratchpad', etc.
    cached_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL,
    access_count INTEGER DEFAULT 1
);

CREATE INDEX idx_cache_accessed ON autonomi_cache(last_accessed);
CREATE INDEX idx_cache_content ON autonomi_cache(content_hash);
```

#### user_preferences
```sql
CREATE TABLE user_preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL, -- JSON-encoded value
    updated_at INTEGER NOT NULL
);
```

## File System Layout

```
$DATA_ROOT/
├── osnova.db                          # SQLite database
├── blobs/                             # Encrypted blob storage
│   ├── <sha256-hash-1>.enc           # Encrypted blob
│   ├── <sha256-hash-2>.enc
│   └── ...
├── identity/                          # Per-user identity data
│   └── <user-id>/
│       └── keys.cocoon               # Encrypted key storage
├── components/                        # Component data directories
│   └── <component-id>/
│       └── v<major>.<minor>/         # Version-specific data
│           ├── config.json
│           └── data/
└── logs/                              # Application logs
    ├── osnova.log
    └── components/
        └── <component-id>.log
```

## Data Access Patterns

### Read-Heavy Operations
- Component metadata queries
- Application list display
- Transaction history
- Cache lookups

**Optimization**: Use SQLite indices, prepared statements, connection pooling

### Write-Heavy Operations
- Log entries
- Cache updates
- Heartbeat updates

**Optimization**: Batch writes, WAL mode, async writes for non-critical data

### Large Data Operations
- Component binary caching
- File uploads/downloads
- Backup/restore

**Optimization**: Stream directly to/from encrypted files, avoid loading into memory

## Encryption Strategy

### Master Key Derivation
```
master_key = HKDF-SHA256(seed_phrase)
db_encryption_key = HKDF-SHA256(master_key, salt="osnova-db-encryption", info="v1")
blob_encryption_key = HKDF-SHA256(master_key, salt="osnova-blob-encryption", info="v1")
```

### Database Encryption
**MVP Decision**: Database is NOT encrypted at rest for MVP
- Rationale: SQLite encryption adds complexity (SQLCipher or custom)
- Mitigation: Sensitive data (keys, secrets) stored in cocoon files, not database
- Post-MVP: Consider SQLCipher for full database encryption

### Blob Encryption
All blobs are encrypted with ChaCha20-Poly1305:
```rust
// Encrypt blob
let nonce = generate_random_nonce();
let ciphertext = chacha20poly1305_encrypt(blob_encryption_key, nonce, plaintext);
let encrypted_blob = nonce || ciphertext;

// Store with content-addressed filename
let content_hash = sha256(plaintext);
let blob_path = format!("$DATA_ROOT/blobs/{}.enc", hex::encode(content_hash));
```

## Backup and Restore

### Backup Strategy
1. **Database**: Copy `osnova.db` (SQLite supports online backup)
2. **Blobs**: Copy entire `blobs/` directory
3. **Identity**: Copy `identity/` directory (includes cocoon files)
4. **Component Data**: Copy `components/` directory

### Restore Strategy
1. Verify backup integrity (checksums)
2. Stop all running components
3. Restore files to `$DATA_ROOT`
4. Restart application

## Performance Targets (MVP)

- **Query Latency**: p95 < 10ms for metadata queries
- **Blob Read**: p95 < 100ms for blobs < 10MB
- **Blob Write**: p95 < 200ms for blobs < 10MB
- **Database Size**: < 100MB for typical usage (1000 components, 10000 transactions)
- **Blob Storage**: Depends on cached content, recommend 1-10GB limit

## Migration Strategy

### Schema Versioning
```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL,
    description TEXT
);

INSERT INTO schema_version (version, applied_at, description)
VALUES (1, strftime('%s', 'now'), 'Initial schema');
```

### Migration Process
1. Check current schema version
2. Apply migrations sequentially
3. Update schema_version table
4. Verify integrity

## Rust Implementation

### Recommended Crates
- **Database**: `sqlx` (async, compile-time checked queries) or `rusqlite` (sync, simpler)
- **Encryption**: `chacha20poly1305` from RustCrypto
- **Hashing**: `sha2` from RustCrypto
- **Serialization**: `serde_json` for JSON columns

### Example Code
```rust
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

pub struct StorageBackend {
    db: SqlitePool,
    blob_dir: PathBuf,
    blob_cipher: ChaCha20Poly1305,
}

impl StorageBackend {
    pub async fn new(data_root: &Path, blob_key: &[u8; 32]) -> Result<Self> {
        let db_path = data_root.join("osnova.db");
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}", db_path.display()))
            .await?;
        
        let blob_dir = data_root.join("blobs");
        fs::create_dir_all(&blob_dir)?;
        
        let key = Key::from_slice(blob_key);
        let blob_cipher = ChaCha20Poly1305::new(key);
        
        Ok(Self { db, blob_dir, blob_cipher })
    }
    
    pub async fn store_blob(&self, content: &[u8]) -> Result<String> {
        let content_hash = sha256(content);
        let blob_hash = hex::encode(&content_hash);
        let blob_path = self.blob_dir.join(format!("{}.enc", blob_hash));
        
        // Encrypt
        let nonce = Nonce::from_slice(&generate_random_nonce());
        let ciphertext = self.blob_cipher.encrypt(nonce, content)?;
        
        // Write
        let mut file = File::create(&blob_path)?;
        file.write_all(nonce)?;
        file.write_all(&ciphertext)?;
        
        Ok(blob_hash)
    }
}
```

## Post-MVP Enhancements

- Full database encryption with SQLCipher
- Compression before encryption (zstd)
- Blob deduplication
- Automatic cache eviction (LRU)
- Database sharding for large datasets
- Replication for backup
- Incremental backup support
- Database vacuum scheduling

