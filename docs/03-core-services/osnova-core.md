# osnova-core (built‑in service)

This is an in‑process Rust module that provides core shell services (apps, config, identity, storage, UI, server status). It is no longer a separate backend component process.

## Encryption Libraries

osnova-core uses the following encryption libraries:

- **cocoon v0.4.3**: Local file encryption (ChaCha20-Poly1305 or AES-256-GCM)
  - Used for: Configuration files, app cache, local storage
  - Key derivation: PBKDF2-SHA256 from component-specific keys

- **Platform keystore integration**: Secure storage for master key
  - Windows: Credential Manager (DPAPI)
  - macOS: Keychain Services
  - Linux: Secret Service API (GNOME Keyring/KWallet)
  - Android/iOS: Platform keystores

### Data Storage

Use SQLite for structured/queryable data (app configs, registry); encrypted files for blobs (keys, cache)
Cached data can be managed by the user from the configuration app.
Like in Android, cached data and files can be deleted as necessary.
There is no automated cache cleanup mechanism.

### OpenRPC methods

When exposed externally (stand-alone or server mode), the osnova-core service provides the following OpenRPC methods for interacting with the Osnova shell application:

#### Application Management
- `apps.list` - List all installed applications with metadata (id, name, version, iconUri, manifestUri)
- `apps.launch` - Launch an application by its manifest id
- `apps.install` - Install a new application from a manifest URI
- `apps.uninstall` - Remove an installed application

#### Configuration Management
- `config.getLauncherManifest` - Get the configured launcher manifest address
- `config.setLauncherManifest` - Set the launcher manifest address to swap launchers
- `config.setServer` - Configure the server address for Client-Server mode
- `config.getAppConfig` - Get per-app configuration data for a user
- `config.setAppConfig` - Update per-app configuration data
- `config.getAppCache` - Get per-app cache metadata
- `config.clearAppCache` - Clear cache for a specific app

#### Launcher Layout Management
- `launcher.getLayout` - Get the current icon order/placement persisted per-identity
- `launcher.setLayout` - Set the icon order/placement (saved within 1s of drop)

#### Identity and Pairing
- `identity.status` - Report whether identity is initialized
- `identity.create` - Create a new identity via saorsa-core flow
- `identity.importWithPhrase` - Import existing identity using 4-word address
- `identity.getSeedBackup` - Retrieve backup guidance for 12-word seed phrase
- `pairing.start` - Initiate pairing with server using 4-word identity address (QR or manual)

#### Key Management (Cocoon-Based)
- `keys.derive` - Derive a new key for a component at the next available index
- `keys.deriveAtIndex` - Derive or retrieve a key at a specific index (idempotent). The index is scoped per component ID, ensuring isolation between components. For wallet components, this supports BIP-44/BIP-32 derivation paths where the index represents the account/address index within the wallet's derivation hierarchy. The derivation uses HKDF-SHA256 with the master key, component ID as salt, and index as part of the info parameter.
- `keys.getByPublicKey` - Retrieve the secret key corresponding to a public key
- `keys.listForComponent` - List all derived keys for a specific component with their indexes and public keys

#### Storage Operations
- `storage.read` - Read encrypted user data from local or server storage
- `storage.write` - Write encrypted user data to local or server storage
- `storage.delete` - Delete user data

#### Component Management
- `component.list` - List cached components (frontend and backend)
- `component.status` - Get status of a backend component (ok/degraded/error)
- `component.fetch` - Fetch and cache a component from Autonomi network
- `component.clearCache` - Clear the component cache

##### `component.fetch`
Fetch and cache a component from the Autonomi network.

**Request**:
```json
{
  "method": "component.fetch",
  "params": {
    "componentId": "ant://...",
    "manifestUri": "ant://...",
    "expectedHash": "base64_blake3_hash"
  }
}
```

**Response**:
```json
{
  "result": {
    "componentId": "ant://...",
    "path": "/tmp/osnova-component-v1.0.0",
    "kind": "frontend",
    "size": 1024000,
    "cached": true,
    "extracted": true
  }
}
```

**Parameters**:
- `componentId` (required): Component ID/URI (ant://, file://, or https://)
- `manifestUri` (optional): Manifest URI to resolve component metadata
- `expectedHash` (optional): Blake3 hash for integrity verification (base64)

**Behavior**:
1. Check local cache first (by component ID + version)
2. If cached and hash matches, return cached path immediately
3. If not cached, fetch from source (ant://, file://, or https://)
4. Verify hash if provided (Blake3)
5. Store in cache
6. Extract if frontend tarball (ZLIB compressed)
7. Make executable if backend binary (Unix)
8. Return path to prepared component

**Error Codes**:
- `-32001`: Component not found at URI
- `-32002`: Network unavailable
- `-32004`: Hash verification failed
- `-32005`: Invalid URI scheme (must be ant://, file://, or https://)
- `-32006`: Extraction failed (frontend)
- `-32007`: Cache full/eviction failed

##### `component.list`
List all cached components.

**Request**:
```json
{
  "method": "component.list",
  "params": {}
}
```

**Response**:
```json
{
  "result": {
    "components": [
      {
        "id": "ant://abc123",
        "version": "1.0.0",
        "kind": "frontend",
        "size": 1024000,
        "lastAccessed": 1633024800,
        "path": "/cache/osnova-app-v1.0.0"
      }
    ],
    "totalSize": 5242880,
    "cacheLimit": 524288000
  }
}
```

##### `component.clearCache`
Clear the component cache.

**Request**:
```json
{
  "method": "component.clearCache",
  "params": {
    "componentId": "ant://abc123"
  }
}
```

**Parameters**:
- `componentId` (optional): Clear specific component, or all if omitted

**Response**:
```json
{
  "result": {
    "cleared": true,
    "bytesFreed": 1024000
  }
}
```

#### UI Operations
- `ui.setTheme` - Set theme mode (light/dark/system)
- `ui.getTheme` - Get current theme mode
- `nav.setBottomMenu` - Configure bottom 5-icon menu for mobile
- `nav.switchTab` - Switch active app tab (mobile)

#### Server Operations
- `status.get` - Get server/host status (read-only): status, version, uptime, component statuses

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
