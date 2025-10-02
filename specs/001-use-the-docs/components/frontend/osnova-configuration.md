# osnova-configuration component

It is written in Svelte as a static web app.
It is a single page application.

## Layout

### Desktop Layout
- Left sidebar navigation with sections:
  - General
  - Identity & Security
  - Server & Pairing
  - Applications
  - Components
  - Theme & Appearance
  - Advanced
- Main content area displaying settings for selected section
- Top header showing current section title
- Apply/Save/Cancel buttons at bottom of forms where applicable

### Mobile Layout
- Top navigation bar with back button and section title
- Scrollable list of configuration sections on main screen
- Tap section to navigate to full-screen detail view
- Settings within sections displayed in a scrollable form
- Floating action button or top-right "Save" button for changes

## Functionality

### General Settings
- **Display Name**: Edit user display name associated with identity
- **Launcher Configuration**: 
  - View and edit launcherManifestUri (ant:// address or dev path)
  - Set which App Launcher to use (allows swapping launchers)
  - OpenRPC calls: `config.getLauncherManifest`, `config.setLauncherManifest`

### Identity & Security
- **Identity Status**: Display current identity state via `identity.status`
  - Show 4-word identity address (for addressing/lookup)
  - Display whether identity is initialized
- **Seed Phrase Backup**: 
  - Access backup/restore flow for 12-word seed phrase
  - Display seed phrase securely (with warnings about keeping it private)
  - Copy seed phrase to clipboard with confirmation
  - Import existing seed phrase (12 input boxes)
- **Key Management**: View derived keys (without exposing private material)
- **Device Keys**: List paired devices and their public keys; revoke device access

### Server & Pairing
- **Server Address**: 
  - Input field for server address configuration
  - Save via `config.setServer` OpenRPC method
  - Validation with error feedback
- **Pairing**: 
  - QR code scanner for pairing (mobile)
  - Manual 4-word identity address entry option
  - Initiate pairing via `pairing.start` 
  - Display pairing status (attempts, elapsed time)
  - Show "Server not found" errors with retry option
  - List currently paired servers
- **Mode Selection**: Toggle between Stand-alone and Client-Server modes
- **Connection Status**: Display current server connection health and latency

### Applications
- **View Modes**: Toggle between "By App" and "By Component" views
- **By App View**:
  - List all installed applications
  - Display frontend and backend components used by each app
  - Show component versions
  - Per-app settings:
    - View/Export/Reset/Delete configuration via `config.getAppConfig`, `config.setAppConfig`
    - Clear cache with confirmation via `config.clearAppCache`
    - Force specific component versions (dropdown with compatible versions)
- **By Component View**:
  - List all cached components (frontend and backend)
  - Show which apps use each component
  - Version information and compatibility
  - Component status for backends via `component.status`
  - Clear component cache via `component.clearCache`
- **Installation**: Install new apps from manifest URI via `apps.install`
- **Uninstall**: Remove apps with confirmation via `apps.uninstall`

### Components
- **Component Cache**: 
  - List cached components with sizes and versions
  - Clear individual or all cached components
  - View cache statistics (total size, number of items)
- **Backend Components**:
  - List running backend components
  - Display status (ok/degraded/error) via `component.status`
  - Force restart if needed (Client-Server mode requires admin privileges)
- **Storage Management**: Configure local data and cache directories

### Theme & Appearance
- **Theme Mode**: 
  - Radio buttons for Light/Dark/System
  - Save via `ui.setTheme`, load via `ui.getTheme`
  - Preview theme changes in real-time
- **Mobile Bottom Menu** (mobile only):
  - Configure 5-icon bottom navigation menu
  - Drag-and-drop or select which apps appear in menu
  - Save via `nav.setBottomMenu`

### Advanced
- **Logging**: 
  - View log file locations per platform
  - Configure log level (INFO/DEBUG/WARN/ERROR)
  - View recent logs in-app (read-only)
  - Open log directory in file manager
- **Storage Locations**:
  - Display paths for:
    - Component cache directory
    - App data directory (per user/version)
    - Configuration storage location
- **Server Mode** (when running as server):
  - View server status via `status.get`
  - Display uptime, version, component statuses
  - Expose read-only status information
- **Developer Options** (optional, for debugging):
  - Enable/disable compression for local dev
  - Use local component paths instead of Autonomi URIs
  - View/edit raw manifest JSON

### User Interactions
- **Form Validation**: Real-time validation with error messages
- **Confirmations**: Destructive actions (delete, reset, clear cache) require confirmation dialogs
- **Loading States**: Show spinners during async operations (save, fetch, API calls)
- **Error Handling**: Display clear error messages from OpenRPC calls with retry options
- **Success Feedback**: Toast/snackbar notifications for successful saves
- **Help Text**: Contextual help icons/tooltips explaining each setting

### Data Persistence
All configuration changes are persisted via OpenRPC calls to the osnova-core backend component, which manages the encrypted user-scoped data store. Changes are scoped per-identity and preserved across devices when synced via the storage network.
