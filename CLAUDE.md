# Osnova Development Guide for Claude Code

## Project Overview

Osnova is a cross-platform distributed application framework built with Tauri 2.0, Svelte, and Rust. It provides a browser-like experience for distributed applications, enabling users to run modular frontend and backend components in both stand-alone and client-server modes.

**Tech Stack:**
- **Frontend**: Svelte, TypeScript, HTML, CSS
- **Backend**: Rust (stable)
- **Framework**: Tauri 2.0
- **Storage**: Autonomi network (primary), SQLite (local), encrypted files
- **Communication**: OpenRPC (JSON-RPC 2.0)
- **Identity**: saorsa-core (4-word identity address + 12-word seed phrase)
- **Encryption**: saorsa-seal (encryption-at-rest), cocoon

**Target Platforms**: Windows, macOS, Linux, Android, iOS

## Development Workflow: Spec-Driven Development

This project follows a **specification-driven development** approach. Before generating any code, specifications must be clean, complete, and unambiguous. The workflow is:

1. **Specification First**: All features begin with detailed specifications in `docs/`
2. **Planning**: Create implementation plans from specifications
3. **Test-Driven Development**: Write tests before implementation
4. **Implementation**: Generate code to pass tests
5. **Validation**: Verify against specifications

### Multi-Agent Development

Osnova uses a multi-agent development system to automate implementation:

- **Orchestrator Agent**: Coordinates all agents, manages tasks, and handles integration
- **Backend Core Agent**: Implements Rust services and data models
- **Rust Testing Agent**: Validates backend code with tests and quality checks
- **Frontend Agent**: Implements Svelte UI components (Phase 2)
- **E2E Testing Agent**: Validates user flows with Playwright MCP (Phase 2)
- **Integration Agent**: Packages components and manages builds (Phase 3)

Agents work in parallel across separate git worktrees, communicating via `.agents/` directories. See [AGENTS.md](./AGENTS.md) for detailed documentation.

### Specification Structure

All specifications are organized in the `docs/` directory in a book-like format:

```
docs/
├── README.md                          # Documentation index
├── 01-introduction/                   # Project overview
├── 02-architecture/                   # System architecture
├── 03-core-services/                  # Built-in services
├── 04-core-screens/                   # Built-in GUI modules
├── 05-components/                     # Component system
├── 06-protocols/                      # Communication protocols
├── 07-security/                       # Security & identity
├── 08-networking/                     # Networking & pairing
├── 09-ui-ux/                         # User interface design
├── 10-development/                    # Testing & implementation
└── 11-apps/                          # Application specifications
```

## Code Quality Requirements

### DRY Principle (Don't Repeat Yourself)

- **No code duplication**: Extract shared logic into reusable functions/modules
- **Single source of truth**: Each concept should have exactly one representation
- **Maximum 3 lines**: If you're copying more than 3 lines, refactor into a function
- **Composition over repetition**: Build complex behaviors from simple, reusable pieces

### Documentation Requirements

Every function, struct, and module MUST have:

1. **Docstrings**: Clear, concise documentation explaining:
   - **Purpose**: What does this do?
   - **Parameters**: What inputs does it accept?
   - **Returns**: What does it return?
   - **Errors**: What errors can occur?
   - **Examples**: How do you use it?

2. **Tests**: Each function MUST have corresponding tests that prove correctness:
   - Unit tests for individual functions
   - Integration tests for component interactions
   - Contract tests for OpenRPC interfaces
   - E2E tests for user workflows

### Test-Driven Development (TDD)

All code MUST follow TDD:

1. **Write the test first**: Define expected behavior
2. **Run the test**: Verify it fails (red)
3. **Write minimal code**: Make the test pass (green)
4. **Refactor**: Clean up while keeping tests passing
5. **Repeat**: Continue for next feature

**Coverage target**: ≥ 85%

**Testing frameworks**:
- Rust: `cargo test`
- TypeScript: Vitest
- E2E: Playwright

## Architecture Overview

### Component Model

Osnova uses a component-based architecture:

- **Core Services** (built-in Rust modules):
  - `osnova-core`: Shell services (apps, config, storage, UI)
  - `osnova-saorsa`: Identity management via saorsa-core
  - `osnova-wallet`: Cryptocurrency wallet
  - `osnova-autonomi`: Autonomi network integration
  - `osnova-bundler`: Component packaging

- **Core Screens** (built-in GUI modules):
  - Launcher: App launcher with grid layout
  - Configuration: System settings
  - Deployment: App deployment

- **External Components** (optional, app-supplied):
  - Frontend: Static web apps (TypeScript/HTML/CSS), ZLIB compressed
  - Backend: Rust binaries with OpenRPC servers

### Operating Modes

1. **Stand-alone Mode** (default):
   - All components run locally on the device
   - Communication via local IPC transport
   - Full functionality without network

2. **Client-Server Mode**:
   - Backend components run on server
   - Frontend components run on client
   - Encrypted communication channel (saorsa-core)
   - Mobile devices offload heavy workloads
   - Server supports ≥5 concurrent clients

### Identity & Security

- **Identity**: 4-word identity address (saorsa-core) for addressing
- **Master Key**: Derived from 12-word seed phrase (BIP-39 compatible)
- **Key Derivation**: HKDF-SHA256 for component-specific keys
- **Encryption at Rest**: saorsa-seal for all user data
- **Client-Server**: End-to-end encrypted; server cannot decrypt user content
- **Device Pairing**: QR code or manual entry of 4-word address

### Communication Protocols

- **In-Process**: Direct Rust API calls for core services
- **OpenRPC**: JSON-RPC 2.0 for external components and server mode
- **Error Codes**: Standard JSON-RPC errors plus custom -32000..-32099 range
- **Authentication**: Via secure channel established during pairing

### Data Storage

- **Structured Data**: SQLite for queryable data (app configs, registry)
- **Blob Storage**: Encrypted files for keys, cache
- **Component Cache**: Downloaded components cached locally
- **Autonomi Network**: Immutable, content-addressed storage (ant:// URIs)
- **User Data**: Encrypted at rest, scoped per user/device

## Key Design Decisions

### Manifest Schema

Application manifests define component dependencies:

```json
{
  "id": "ant://...",
  "name": "App Name",
  "version": "1.0.0",
  "iconUri": "ant://...",
  "description": "App description",
  "publisher": "com.publisher",
  "components": [
    {
      "id": "ant://...",
      "name": "Component Name",
      "kind": "frontend|backend",
      "version": "1.0.0",
      "target": "x86_64-unknown-linux-gnu",
      "platform": "desktop|iOS|Android",
      "hash": "blake3:..."
    }
  ]
}
```

### Component ABI (Backend Components)

Backend components expose a consistent interface:

- `component_configure(config: JSON) -> Result<Config>`
- `component_start() -> Result<Handle>`
- `component_status() -> Result<Status>`
- `component_stop() -> Result<()>`

### Performance Targets

- **Launch Time**: p95 ≤ 2s from app launch to first meaningful render
- **Backend Latency**: Prompt fallback if p95 > 5s
- **Server Capacity**: Support ≥5 concurrent mobile clients
- **Availability**: Best-effort for MVP (no formal SLO)

## Implementation Checklist

When implementing a feature, ensure:

- [ ] Specification is complete and unambiguous
- [ ] Data model is defined (if applicable)
- [ ] OpenRPC contracts are specified (if applicable)
- [ ] Contract tests are written and failing
- [ ] Unit tests are written and failing
- [ ] Integration tests are written and failing
- [ ] Implementation makes tests pass
- [ ] Code coverage ≥ 85%
- [ ] All functions have docstrings with examples
- [ ] No code duplication (DRY principle)
- [ ] Code follows Rust/TypeScript style guidelines
- [ ] Manual testing follows quickstart.md scenarios
- [ ] Documentation is updated

## File Organization

### Repository Structure

```
osnova/
├── CLAUDE.md                          # This file
├── docs/                              # Specifications (see above)
├── app/
│   ├── desktop/                       # Tauri desktop app
│   │   ├── src/                       # Svelte frontend
│   │   ├── public/                    # Static assets
│   │   └── tauri/                     # Tauri configuration
│   └── mobile/                        # Tauri mobile targets
│       ├── ios/                       # iOS app
│       └── android/                   # Android app
├── core/
│   └── osnova_lib/                    # Core Rust library
│       ├── src/                       # Source code
│       └── tests/                     # Integration tests
├── components/
│   ├── frontend/                      # Frontend components
│   │   └── <component-name>/         # Individual frontend component
│   └── backend/                       # Backend components
│       └── <component-name>/          # Individual backend component
├── contracts/
│   └── openrpc/                       # OpenRPC specifications
└── tests/
    ├── contract/                      # Contract tests
    ├── integration/                   # Integration tests
    ├── unit/                          # Unit tests
    └── e2e/                          # End-to-end tests
```

### Naming Conventions

- **Rust**: snake_case for functions/variables, PascalCase for types/structs
- **TypeScript**: camelCase for functions/variables, PascalCase for classes/interfaces
- **Files**: kebab-case for filenames (e.g., `user-profile.ts`)
- **Components**: PascalCase for component names (e.g., `AppLauncher.svelte`)

## Common Tasks

### Running Tests

```bash
# Rust unit tests
cargo test

# Rust integration tests
cargo test --test '*'

# TypeScript tests
npm run test

# E2E tests
npm run test:e2e

# All tests with coverage
cargo test --coverage
npm run test:coverage
```

### Building Components

```bash
# Build frontend component (ZLIB tarball)
npm run build:component <component-name>

# Build backend component (Rust binary)
cargo build --release --package <component-name>

# Package all components
npm run package:all
```

### Running the App

```bash
# Development mode
npm run tauri dev

# Build release
npm run tauri build

# Run as server (headless)
./osnova --server
```

### Logging

- **Location**: File-based with rotation
- **Default Level**: INFO
- **Configuration**: Per-component/host logs
- **Security**: MUST redact secrets in all modes

## OpenRPC Methods Reference

### Core Service (osnova-core)

**Application Management:**
- `apps.list()` - List installed apps
- `apps.launch(appId)` - Launch app
- `apps.install(manifestUri)` - Install app
- `apps.uninstall(appId)` - Uninstall app

**Configuration:**
- `config.getLauncherManifest()` - Get launcher manifest address
- `config.setLauncherManifest(uri)` - Set launcher manifest
- `config.setServer(address)` - Configure server address
- `config.getAppConfig(appId)` - Get app configuration
- `config.setAppConfig(appId, config)` - Set app configuration
- `config.clearAppCache(appId)` - Clear app cache

**Launcher:**
- `launcher.getLayout()` - Get icon layout
- `launcher.setLayout(layout)` - Set icon layout

**Identity:**
- `identity.status()` - Check if identity initialized
- `identity.create()` - Create new identity
- `identity.importWithPhrase(address)` - Import identity
- `identity.getSeedBackup()` - Get seed backup guidance

**Pairing:**
- `pairing.start(address)` - Initiate pairing with server

**Key Management:**
- `keys.derive(componentId)` - Derive new key
- `keys.deriveAtIndex(componentId, index)` - Derive key at index
- `keys.getByPublicKey(publicKey)` - Get secret key
- `keys.listForComponent(componentId)` - List component keys

**Storage:**
- `storage.read(path)` - Read encrypted data
- `storage.write(path, data)` - Write encrypted data
- `storage.delete(path)` - Delete data

**Component Management:**
- `component.list()` - List cached components
- `component.status(componentId)` - Get component status
- `component.fetch(uri)` - Fetch component from network
- `component.clearCache()` - Clear component cache

**UI:**
- `ui.setTheme(mode)` - Set theme (light/dark/system)
- `ui.getTheme()` - Get current theme
- `nav.setBottomMenu(items)` - Configure mobile bottom menu
- `nav.switchTab(tabId)` - Switch active tab

**Server:**
- `status.get()` - Get server status (health, version, uptime)

## Common Patterns

### Error Handling

```rust
// Rust
use anyhow::{Context, Result};

pub fn load_manifest(path: &Path) -> Result<Manifest> {
    let content = fs::read_to_string(path)
        .context("Failed to read manifest file")?;

    serde_json::from_str(&content)
        .context("Failed to parse manifest JSON")
}
```

```typescript
// TypeScript
async function loadManifest(path: string): Promise<Manifest> {
  try {
    const content = await fs.readFile(path, 'utf-8');
    return JSON.parse(content);
  } catch (error) {
    throw new Error(`Failed to load manifest: ${error.message}`);
  }
}
```

### OpenRPC Client

```typescript
// TypeScript OpenRPC client
import { JsonRpcClient } from '@/lib/rpc';

const client = new JsonRpcClient('http://localhost:8080');

// Call method
const apps = await client.call('apps.list');

// Handle errors
try {
  await client.call('apps.launch', { appId: 'com.example.app' });
} catch (error) {
  if (error.code === -32001) {
    console.error('App not found');
  }
}
```

### Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_at_index() {
        let master_key = [0u8; 32];
        let component_id = "com.example.wallet";
        let index = 0;

        let key1 = derive_key_at_index(&master_key, component_id, index).unwrap();
        let key2 = derive_key_at_index(&master_key, component_id, index).unwrap();

        // Idempotent: same index returns same key
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_at_index_different_components() {
        let master_key = [0u8; 32];
        let index = 0;

        let key1 = derive_key_at_index(&master_key, "com.example.wallet", index).unwrap();
        let key2 = derive_key_at_index(&master_key, "com.example.storage", index).unwrap();

        // Different components have isolated keys
        assert_ne!(key1, key2);
    }
}
```

## Dependencies

### Rust Crates

```toml
[dependencies]
# Framework
tauri = "2.x"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"

# Distributed storage
autonomi = { git = "https://github.com/maidsafe/autonomi", tag = "v0.6.1" }

# Identity & encryption
saorsa-core = { git = "https://github.com/dirvine/saorsa-core", branch = "main" }
cocoon = "0.4.3"

# Cryptography
blake3 = "1.5"
hkdf = "0.12"
sha2 = "0.10"

# Storage
rusqlite = "0.31"
flate2 = "1.0"  # ZLIB compression

# Async runtime
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
proptest = "1.0"
```

### TypeScript Dependencies

```json
{
  "dependencies": {
    "svelte": "^4.0.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-shell": "^2.0.0"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "playwright": "^1.40.0",
    "typescript": "^5.0.0"
  }
}
```

## References

- **Tauri Documentation**: https://tauri.app/
- **Svelte Documentation**: https://svelte.dev/
- **Autonomi Network**: https://github.com/maidsafe/autonomi
- **saorsa-core**: https://github.com/dirvine/saorsa-core
- **JSON-RPC 2.0 Specification**: https://www.jsonrpc.org/specification
- **OpenRPC Specification**: https://open-rpc.org/

## Getting Help

1. **Read the specifications**: Check `docs/` for detailed feature specifications
2. **Check existing code**: Look for similar patterns in the codebase
3. **Run tests**: Tests serve as executable documentation
4. **Review contracts**: OpenRPC contracts define all external interfaces

## Development Principles

1. **Specification First**: No code without complete specifications
2. **Test First**: No implementation without failing tests
3. **Document Everything**: Every function needs docstrings and examples
4. **DRY Always**: Never duplicate code; extract and reuse
5. **Security By Default**: Encrypt all user data; validate all inputs
6. **Simple & Composable**: Small functions; clear interfaces; easy to test
7. **Performance Matters**: Meet the performance targets (p95 ≤ 2s launch)
8. **User Privacy**: E2E encryption; no server access to user content

## Next Steps

To start working on a feature:

1. Read the specification in `docs/`
2. Review the data model and contracts
3. Write contract tests (OpenRPC interfaces)
4. Write unit tests (individual functions)
5. Write integration tests (component interactions)
6. Implement to make tests pass
7. Verify against specification
8. Document with examples
9. Submit for review

Remember: **Specifications → Tests → Implementation → Validation**

---

Last Updated: 2025-10-06
