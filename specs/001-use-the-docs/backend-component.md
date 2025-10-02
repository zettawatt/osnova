# Backend Components

This document describes the structure and lifecycle of backend components.

## Lifecycle

Backend components are constructed from Rust and have the standard rust project data structure and semantics.
Most are independent projects that are not part of the osnova shell itself, but follow the proper semantics to run under the osnova framework.
Core backend components may be developed within the osnova shell project itself under the 'components/backend' directory tree.
In this sub-hierarchy, they are constructed the same as any independent backend component project.

### Development

During development it does not make sense to make releases of backend components as these will be under constant flux.
In the osnova app schema a directory path may be provided that points to the osnova backend component project.
When ready to execute, the backend component developer will compile the backend component into a compatible binary for the target running the osnova shell application.
Osnova shell, when launching an osnova app that points to this backend component, will look to the proper target releease directory and run the backend component binary within the plugin loader framework.

### Release

A backend component is released when a set of backend component binaries is written to the Autonomi network.
There may be multiple component binaries for each version based on the desired target implementations.
Each binary will be uploaded to the Autonomi network as a public file and the cooresponding Autonomi address location will be recorded in the component manifest.

### Production

After release, a production binary of that version may not be used again.
Versions on particular targets are immutable.
Osnova apps pointing to a binary of a backend component will always pull from the backend component manifest address ensuring that the data has not been tampered with and was signed/uploaded by the project maintainer as only the maintainer has the keys to upload content to the manifest address.

## Backend component manifest schema

Each version contains a manifest that has the following skeleton schema that is loaded as a public file to the Autonomi network:

```json
{
  "title": "Osnova Backend Component Manifest",
  "type": "object",
  "required": ["id", "name", "version", "description"],
  "properties": {
    "id": {"type": "string", "description": "Autonomi content address of the manifest itself or a path on the local filesystem for development purposes"},
    "name": {"type": "string"},
    "version": {"type": "string", "pattern": "^\d+\.\d+\.\d+$", "description": "Semver; exact pinned version"},
    "description": {"type": "string"},
    "publisher": {"type": "string", "description": "Publisher identifier"},
    "signature": {"type": "string", "description": "Detached signature over canonical manifest"},
    "targets": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "target"],
        "properties": {
          "id": {"type": "string", "description": "Autonomi address of the backend component binary"},
          "target": {"type": "string", "description": "Target for compiled backend components following Rust's official target triple format (e.g., x86_64-unknown-linux-gnu)." },
          "hash": {"type": "string", "description": "Hash (e.g., blake3 base64) of the fetched artifact"},
          "config": {"type": "object", "additionalProperties": true},
        }
      }
    },
    "metadata": {"type": "object", "additionalProperties": true}
  }
}
```

Notes:
- The 'targets' field is required for production releases. For development the target is assumed to be the system running the development. This field is required for cross-platform compilation strategies

## Storage on the Autonomi Network

Compiled backend component binaries are always uploaded as public files, each target is its own file.
Each version's binary collateral is listed in a manifest file as described above.
The address for the component is a pointer that points to a graph entry.
Each time a new version is added, a new manifest file is created along with a graph entry.
The graph entry contains links to all of the previous version manifest files as well as the latest entry.
The pointer is updated to point to the latest graph entry.
In this way, we build up an immutable list of collateral for each component specified by version.

## Backend Component Installation

To install a new osnova app, it will need to download the necessary backend components. To read a backend component, it follows these steps:
- read the component address id from the osnova app manifest
- download the pointer at that address to get the graph entry address
- download the graph entry
- walk through all of the entries and download the version manifest files
- check if there is a newer compatible version in the manifest than is cached on the local machine or osnova server
  - if so, download this version compatible with the target architecture and store it in the backend component cache, shutdown the older compatible version if it is running, and execute the newly downloaded version
  - if there is no newer version than is already cached, move on to the next component
  
After an osnova app is installed, the backend component is stored in the component cache directory.

## Backend Component Loading

Backend components are treated as binary plugins and executed as separate OS processes managed by the Tauri backend.
This approach ensures cross-platform compatibility (Windows, macOS, Linux, Android, iOS) without relying on Tauri's shell plugin.

### Process-Based Component Execution

Each backend component runs as an independent OS process:
- **Lifecycle**: Spawned via `std::process::Command` in Rust
- **Communication**: Components expose OpenRPC servers; Tauri backend acts as client
- **Isolation**: Each component has its own process space, preventing crashes from affecting the shell
- **Cross-Platform**: Uses standard Rust process APIs available on all platforms

### Tauri Commands for Component Management

The Tauri backend exposes three core commands to manage backend components:

#### `component_start`
**Tauri Command** (invoked from frontend or shell logic):
```rust
#[tauri::command]
async fn component_start(
    component_id: String,
    config: serde_json::Value
) -> Result<ComponentStartResult, String>
```

**Behavior**:
1. Resolve component binary path from cache: `$COMPONENT_CACHE/backend/<component_id>/<version>/<target-triple>`
2. Spawn the binary as a child process using `tokio::process::Command`
3. Pass configuration via stdin or as CLI args (JSON-encoded)
4. Component binary starts its OpenRPC server on a designated port (ephemeral or configured)
5. Component writes endpoint info to stdout: `{"endpoint": "http://127.0.0.1:9001/rpc"}`
6. Tauri backend parses endpoint and registers component in a `ComponentRegistry` (in-memory map)
7. Store process handle (`Child`) for lifecycle management
8. Return endpoint and process ID to caller

**Return**:
```json
{
  "endpoint": "http://127.0.0.1:9001/rpc",
  "pid": 12345,
  "status": "running"
}
```

#### `component_stop`
**Note**: Not a Tauri command. Components are stopped via OpenRPC call to their endpoint.

**Behavior**:
1. Frontend/shell sends OpenRPC `component.stop` request to the component's endpoint
2. Component performs graceful shutdown (flush data, close connections)
3. Component exits with status code 0
4. Tauri backend detects process exit via `Child.wait()` and removes from registry
5. If component doesn't exit within 5 seconds, Tauri backend sends SIGTERM (Unix) or TerminateProcess (Windows)

**Why not a Tauri command?**: Components are independent services. They should handle their own shutdown logic via their public API (OpenRPC). This keeps the architecture decoupled.

#### `component_status`
**Note**: Not a Tauri command. Status is queried via OpenRPC call to the component's endpoint.

**Behavior**:
1. Frontend/shell sends OpenRPC `component.status` request to the component's endpoint
2. Component returns health status: `{"status": "ok"|"degraded"|"error", "details": {...}}`
3. If OpenRPC call fails (timeout, connection refused), Tauri backend can check if process is still running via `Child.try_wait()`

### Component Binary Interface

Each backend component binary must:
1. **Accept configuration** via stdin or as a `--config <json>` CLI argument
2. **Start an OpenRPC server** on an ephemeral port (or port from config)
3. **Output endpoint** to stdout on startup: `{"endpoint": "http://127.0.0.1:<port>/rpc"}`
4. **Implement required OpenRPC methods**:
   - `component.status() -> {status, details}`
   - `component.stop() -> {stopped: true}`
5. **Exit gracefully** when `component.stop` is called

### Component Registry (Tauri Backend State)

The Tauri backend maintains a global component registry:
```rust
struct ComponentRegistry {
    components: HashMap<String, RunningComponent>,
}

struct RunningComponent {
    component_id: String,
    version: String,
    endpoint: String,
    process: tokio::process::Child,
    started_at: u64,
}
```

**Operations**:
- `register(component_id, endpoint, process)`: Add component after successful start
- `get(component_id) -> Option<RunningComponent>`: Retrieve running component
- `remove(component_id)`: Remove on stop or crash
- `list() -> Vec<RunningComponent>`: List all running components (for `status.get`)

### Crash Detection and Restart Policy

The Tauri backend monitors component processes:
1. Use `tokio::spawn` to await `Child.wait()` in the background
2. When a process exits unexpectedly (non-zero status or no `component.stop` call):
   - Log the crash with exit code and stderr
   - Check component's restart policy (from configuration)
   - If `restart_on_crash: true` (default), re-spawn the component via `component_start`
   - Display a toast notification in the Osnova shell: "Component <name> crashed and was restarted"
3. If `restart_on_crash: false`, display toast: "Component <name> stopped unexpectedly"

### Example Component Start Flow

**User action**: Launch an Osnova app that requires backend component `com.osnova.wallet` v1.2.0

1. **Shell**: Checks `ComponentRegistry.get("com.osnova.wallet")`
2. **If not running**: Calls `component_start("com.osnova.wallet", config)`
3. **Tauri backend**:
   ```rust
   let binary_path = resolve_component_path("com.osnova.wallet", "1.2.0");
   let mut child = Command::new(binary_path)
       .arg("--config").arg(serde_json::to_string(&config)?)
       .stdout(Stdio::piped())
       .spawn()?;
   
   // Read endpoint from stdout
   let endpoint = parse_endpoint_from_stdout(&mut child)?;
   
   // Register component
   registry.register("com.osnova.wallet", endpoint.clone(), child);
   
   Ok(ComponentStartResult { endpoint, pid: child.id(), status: "running" })
   ```
4. **Shell**: Stores endpoint and uses it for all OpenRPC calls to the wallet component
5. **Component**: Runs independently, serving OpenRPC requests

### Why Not FFI/Dynamic Linking?

While FFI (Foreign Function Interface) or dynamic linking (`.so`, `.dylib`, `.dll`) could allow in-process components, it has significant drawbacks:
- **Crash Risk**: Component crash can crash entire Tauri process
- **Platform Complexity**: Different dynamic library formats per OS; Android/iOS have restricted dynamic loading
- **ABI Stability**: Rust does not guarantee stable ABI; version mismatches cause undefined behavior
- **Security**: In-process code has full access to shell memory; harder to sandbox

**Process-based approach** is simpler, safer, and more portable.

### Cross-Platform Compatibility Notes

- **Windows**: Uses `CreateProcess` under the hood; `Child.kill()` maps to `TerminateProcess`
- **macOS/Linux**: Uses `fork`+`exec`; `Child.kill()` sends SIGKILL
- **Android/iOS**: Tauri on mobile uses the same Rust process APIs; components are bundled as native executables in the app package
- **Port Allocation**: On mobile, use localhost ephemeral ports. On desktop, allow configuration via component config.

## Backend Component Lifecycle

If a component crashes, its restart policy is defined by the setting in the configuration page.
By default, backend components will restart automatically and throw up a warning 'toast' in the osnova shell GUI describing which component failed and the message that occurred.
The component configuration may also set the restart policy to not restart the component on failure.
By default, backend components, once started, will halt once all users' apps utilizing them are closed within the osnova shell app.
The component configuration may also disable the auto-shutdown policy and keep the process running.

## Versioning and Usage in Osnova Shell

### Semantic Versioning (Cargo.toml Semantics)

Backend component versions follow Rust's Cargo.toml semantic versioning rules (SemVer 2.0.0) with identical compatibility semantics.
This proven system eliminates the need to reinvent version resolution logic.

#### Version Specification in Manifests

App manifests can specify component versions using Cargo-style version requirements:

**Exact Version**:
```json
{"version": "1.2.3"}
```
Only version 1.2.3 is acceptable.

**Caret Requirements** (default):
```json
{"version": "^1.2.3"}
```
Allows updates that do not modify the leftmost non-zero digit:
- `^1.2.3` → `>= 1.2.3, < 2.0.0` (allows 1.2.4, 1.3.0, 1.9.9, but not 2.0.0)
- `^0.2.3` → `>= 0.2.3, < 0.3.0` (0.x is special: minor version is breaking)
- `^0.0.3` → `>= 0.0.3, < 0.0.4` (0.0.x: each patch is breaking)

**Tilde Requirements**:
```json
{"version": "~1.2.3"}
```
Allows patch-level updates:
- `~1.2.3` → `>= 1.2.3, < 1.3.0`
- `~1.2` → `>= 1.2.0, < 1.3.0`

**Wildcard**:
```json
{"version": "1.2.*"}
```
Equivalent to `~1.2.0`.

**Comparison Operators**:
```json
{"version": ">= 1.2.3, < 2.0.0"}
```
Multiple requirements can be combined.

### Version Resolution Algorithm

When an app requests a backend component:

1. **Parse version requirement** from manifest (e.g., `^1.2.3`)
2. **Query local cache** for all versions of the component (e.g., 1.2.3, 1.2.5, 1.3.0, 2.0.0)
3. **Filter compatible versions** using SemVer matching:
   - `^1.2.3` matches: [1.2.3, 1.2.5, 1.3.0]
   - Does not match: [2.0.0]
4. **Select highest compatible version**: 1.3.0
5. **Check if already running**:
   - If `com.osnova.backend@1.3.0` is running → reuse it
   - If `com.osnova.backend@1.2.5` is running → **upgrade** to 1.3.0 (shutdown old, start new)
   - If `com.osnova.backend@2.0.0` is running → **incompatible**, start 1.3.0 in parallel (different major version)
6. **Start component** if not running or needs upgrade

### Component Sharing and Upgrade Policy

Different osnova apps may utilize the same backend components.
It is undesirable to start a backend component OpenRPC server more than one time when multiple frontend components and users can share one service.
To alleviate this issue, frontend components and other backend components specifying different versions in their manifests may substitute with the running version if it is newer but on the same semantic version as that specified.
If an older but compatible version of the backend component is currently running and a newer and compatible version is downloaded for use in a different application, the newer version will run and be used.
The latest cached compatible version of the backend component will be used unless a specific version is forced through the configuration app.

From the osnova configuration application, a user sees the list of all installed osnova apps.
Selecting an app will display the various frontend and backend components used and their versions.
At the top of the menu there is a toggle selector that will enable viewing by components instead of apps.
This view will show components used, not the apps that use them.
Having both views gives the user the option the best view for whatever operation they are wanting to undertake.
By default, the app will use the latest compatible component version.
However, if there are incompatibilties, a user can force a component to be an older version for the system.
When a different version is requested, the user can click a checkbox that says 'force version' and a drop down for the component will be displayed with all of the compatible versions for that component.
The user selects which component version they want to use.
For standalone installations of osnova, it will tell the user to restart osnova to use this newly specified version in a dialog pop up.
For server-client installations of osnova, if the client has admin privileges, they will be given the option to restart the osnova server remotely.
If the client does not have admin privileges, a popup will be displayed telling them that they need to contact a server administrator to restart the server.

## Local caching and data storage

### Data Directory Versioning (Cargo SemVer Semantics)

Backend components can store data on the local device or server running the component.
Data directories follow SemVer compatibility rules: **major.minor** versions define the data directory path, while **patch** versions share the same directory.

**Rationale**: Components with the same **major.minor** version are assumed to have compatible data formats. Patch updates (bug fixes) should not break data compatibility. Major or minor version bumps may introduce incompatible schema changes, requiring new directories.

### Directory Structure

By default, the configuration app specifies a local data directory.
All component data is stored in a sub-directory hierarchy by user, component, and **major.minor** version.
Components may also share installation-wide data (not user-specific) in a 'shared' sub-directory.

**Path Template**:
```
<DATA_DIR>/
  <user-id>/
    <component-id>/
      v<major>.<minor>/
        (user-specific component data)
  shared/
    <component-id>/
      v<major>.<minor>/
        (shared component data)
```

### Compatibility Examples

#### Example 1: Compatible Patch Upgrades
**Bob** runs an Osnova app with backend component `foo` version **0.1.2**:
- Data directory: `<DATA_DIR>/Bob/foo/v0.1/`

Bob updates the app, which upgrades `foo` to **0.1.5**:
- Data directory: `<DATA_DIR>/Bob/foo/v0.1/` (same directory)
- Rationale: **0.1.5** is compatible with **0.1.2** (same **major.minor**)

#### Example 2: Incompatible Minor Upgrade
Bob's app upgrades `foo` to **0.2.0**:
- Data directory: `<DATA_DIR>/Bob/foo/v0.2/` (new directory)
- Rationale: **0.2.0** is incompatible with **0.1.x** per SemVer (for 0.x, minor is breaking)
- The old `v0.1/` directory remains on disk for rollback or migration

#### Example 3: Major Upgrade
**Alice** runs an Osnova app with backend component `bar` version **1.2.1**:
- Data directory: `<DATA_DIR>/Alice/bar/v1.2/`

Alice upgrades to **2.0.0**:
- Data directory: `<DATA_DIR>/Alice/bar/v2.0/` (new directory)
- Rationale: **2.0.0** is incompatible with **1.x** (major version change)

#### Example 4: Multiple Users and Components

Current state:
- **Bob**: runs `foo` v0.1.2
- **Alice**: runs `bar` v1.2.1
- **Shared data**: `foo` v0.1, `bar` v1.2

Directory structure:
```
<DATA_DIR>/
 |-> Bob/
   |-> foo/
       |-> v0.1/
           |-> (Bob's 'foo' v0.1.x data)
 |-> Alice/
   |-> bar/
       |-> v1.2/
           |-> (Alice's 'bar' v1.2.x data)
 |-> shared/
   |-> foo/
       |-> v0.1/
           |-> (shared 'foo' v0.1.x data)
   |-> bar/
       |-> v1.2/
           |-> (shared 'bar' v1.2.x data)
```

Alice upgrades `bar` to v1.5.2:
- Data directory: `<DATA_DIR>/Alice/bar/v1.5/` (new directory, minor bump)
- Shared data: `<DATA_DIR>/shared/bar/v1.5/` (new shared directory)

### Incompatible Component Versions Running Concurrently

If two apps require **incompatible** versions of the same component (different **major** or **major.minor**), the system will:
1. Run **both versions** in parallel (each as a separate process)
2. Each version uses its own data directory (e.g., `v1.2/` and `v2.0/`)
3. Each version has its own OpenRPC endpoint
4. Apps communicate with the version they requested

**Example**:
- App X requires `foo` **^1.2.0** → uses `foo` v1.3.0 (latest compatible)
- App Y requires `foo` **^2.0.0** → uses `foo` v2.1.0 (latest compatible)
- Both run simultaneously with separate data directories and endpoints

### Data Migration

When a component is upgraded to an incompatible version:
- **Automatic Migration**: If the component provides a migration method (e.g., `migrate.from(old_version)`), Osnova can call it to copy/transform data from the old directory to the new one
- **Manual Migration**: If no migration method exists, the component starts fresh with the new data directory. Users can export/import data manually via the component's UI
- **Rollback**: Old data directories are retained (configurable retention policy in Configuration app) to allow downgrade

### Directory Path Resolution Algorithm

When a component starts, it queries its data directory:
```rust
fn get_component_data_dir(
    user_id: &str,
    component_id: &str,
    version: &Version,  // semver::Version
    shared: bool
) -> PathBuf {
    let major_minor = format!("v{}.{}", version.major, version.minor);
    
    if shared {
        format!("<DATA_DIR>/shared/{}/{}/", component_id, major_minor).into()
    } else {
        format!("<DATA_DIR>/{}/{}/{}/", user_id, component_id, major_minor).into()
    }
}
```

**Given**: Component `com.osnova.wallet` version `1.5.3`, user `alice`
- User data: `<DATA_DIR>/alice/com.osnova.wallet/v1.5/`
- Shared data: `<DATA_DIR>/shared/com.osnova.wallet/v1.5/`

### Notes

- **Disk Space**: Multiple major.minor versions consume more disk space. The Configuration app shows storage usage and allows users to delete old version data directories.
- **0.x Versions**: For 0.x versions, each **minor** bump is treated as breaking (per SemVer), so `v0.1/` and `v0.2/` are separate directories.
- **Cleanup**: When a component version is uninstalled, its data directory can be deleted (with user confirmation) to reclaim space.

