# osnova-core backend component

This component is used to interact with the osnova shell application, in particular reading and writing data to the cache and data directories
It is also used to interact with the osnova server in the case of the client-server model.

### Data Storage

Use SQLite for structured/queryable data (app configs, registry); encrypted files for blobs (keys, cache)
Cached data can be managed by the user from the configuration app.
Like in Android, cached data and files can be deleted as necessary.
There is no automated cache cleanup mechanism.

### OpenRPC methods

The osnova-core backend component provides the following OpenRPC methods for interacting with the osnova shell application:

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
- `keys.deriveAtIndex` - Derive or retrieve a key at a specific index (idempotent) FIXME: the index here should take into account the component ID for component key derivation and/or wallet derivation index information in the case of a wallet component.
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

#### UI Operations
- `ui.setTheme` - Set theme mode (light/dark/system)
- `ui.getTheme` - Get current theme mode
- `nav.setBottomMenu` - Configure bottom 5-icon menu for mobile
- `nav.switchTab` - Switch active app tab (mobile)

#### Server Operations
- `status.get` - Get server/host status (read-only): status, version, uptime, component statuses

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
