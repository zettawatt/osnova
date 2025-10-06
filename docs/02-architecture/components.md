# Components

Osnova's component architecture is the foundation of its flexibility and extensibility. This document describes how components work, their types, and how they interact.

## Component Overview

Osnova works on a principle of dynamically loaded components. This is the general workflow:
- User selects an Osnova application they wish to load
- The Osnova application contains a manifest of components used by the application
- The components are loaded into Osnova and run

## Component Types

There are two basic types of components:

### Backend Components
Backend components contain the business logic, interacting with:
- The host system
- Other backend components
- Various distributed networks

**Characteristics**:
- Written in Rust
- Precompiled binaries matching the host architecture
- Expose OpenRPC APIs for communication
- Can be loaded as plugins in stand-alone mode
- Run on the server in client-server mode

### Frontend Components
Frontend components contain the graphical frontend interface that the user interacts with.

**Characteristics**:
- Written in TypeScript/JavaScript, HTML, and CSS
- Static web applications
- Rendered in Tauri's WebView
- Each component gets its own WebView instance
- Distributed as ZLIB-compressed tarballs

## Built-In vs App-Supplied Components

### Built-In Components (Core Services and Screens)

**Update (2025-10-03)**: The Osnova shell's core services and screens are built into the shell itself:

**Core Services** (backend):
- osnova-core: Central coordination
- osnova-saorsa: Identity and security
- osnova-wallet: Cryptocurrency operations
- osnova-autonomi: Distributed storage
- osnova-bundler: Component packaging

**Core Screens** (frontend):
- Launcher: Application discovery
- Configuration: Settings management
- Deployment: Application deployment

These built-in components:
- Use in-process Rust APIs (no OpenRPC overhead)
- Are always available
- Cannot be replaced or removed
- Provide the foundation for all applications

### App-Supplied Components

Applications loaded from manifests may include their own components:
- Custom frontend components for specific UI needs
- Custom backend components for specialized logic
- These communicate via OpenRPC in both stand-alone and server modes

## Component Communication

Components communicate using generic protocols outside of the Osnova application itself. This design ensures that if Osnova development stopped or merged with another product, original Osnova applications could still run without issues.

### Built-In Component Communication

Core services and core screens communicate via:
- Direct in-process Rust function calls
- Type-safe interfaces
- Zero serialization overhead

### App Component Communication

App-supplied components communicate via:
- **OpenRPC (JSON-RPC 2.0)** protocol
- Stand-alone mode: Local IPC transport
- Client-server mode: Encrypted network channel

### Communication Patterns

**Frontend → Backend**:
```
Frontend Component (WebView)
    ↓
OpenRPC Request
    ↓
Backend Component (Rust OpenRPC Server)
    ↓
Response
```

**Backend → Backend**:
```
Backend Component A
    ↓
OpenRPC Request
    ↓
Backend Component B
    ↓
Response
```

**No Frontend → Frontend**: Frontend components are isolated from each other with no direct inter-tab communication.

## Component Versioning

### Immutability Principle

Each component version is immutable and exists at a static location in perpetuity. This ensures:
- Applications can be run at any point in the future
- No breaking changes to deployed applications
- Predictable behavior over time

### Version References

Manifests reference specific component versions:
```json
{
  "id": "ant://[content-address]",
  "version": "0.1.0",
  "hash": "blake3:[hash]"
}
```

### Content Addressing

Components are stored on content-addressed networks:
- Primary: Autonomi network
- URI format: `ant://[content-address]`
- Content hash verifies integrity
- Permanent storage guarantees availability

## Component Lifecycle

### Backend Component Lifecycle

The backend component plugin ABI supports:

1. **component_configure**: Create configuration JSON from user's cache
2. **component_start**: Start the component OpenRPC server with config
3. **component_stop**: Stop the server and unregister
4. **component_status**: Report current component status

### Frontend Component Lifecycle

1. **Load**: Uncompress tarball from cache or download
2. **Initialize**: Load into WebView with configuration
3. **Run**: Execute application code
4. **Terminate**: Close tab/window, terminate WebView process

### Tab and Window Management

- Each frontend component gets its own WebView
- Tabs managed with browser-like API
- Isolation between tabs (no inter-tab communication)
- WebView terminated when tab/app closed

## Component Packaging

### Frontend Component Packaging

Frontend components are packaged as:
- ZLIB-compressed tarballs
- Contains TypeScript/JavaScript, HTML, CSS, and assets
- Single-file distribution
- Uncompressed on load

**Development mode**: Local directories (no compression) for easier debugging

### Backend Component Packaging

Backend components are:
- Precompiled Rust binaries
- Target-specific (x86_64, ARM, etc.)
- One Cargo project per component
- Platform-specific builds for each target

## Component Caching

### Local Cache

Downloaded components are cached locally:
- Avoids repeated network fetches
- Improves load times
- Enables offline operation
- User can manage cache via Configuration Manager

### Cache Location

Platform-specific cache directories:
- Windows: `%LOCALAPPDATA%\Osnova\cache`
- macOS: `~/Library/Caches/Osnova`
- Linux: `~/.cache/osnova`
- Mobile: Platform-appropriate cache location

## Component Sandboxing

### MVP Approach

For MVP, all components are assumed to be trusted. Future versions may add:
- Permission systems
- Resource limits
- Network restrictions
- File system isolation

### Data Isolation

Even with trusted components:
- User data isolated per client
- Encrypted storage (at rest)
- End-to-end encryption (client-server mode)
- Components access only their designated data

## Component Discovery and Loading

### Manifest-Based Loading

1. User selects application from Launcher
2. Launcher reads application manifest
3. Manifest specifies required components
4. Components loaded from:
   - Local cache (if available)
   - Distributed storage (if needed)
5. Components initialized with configuration
6. Application renders in tab/window

### Component Resolution

For each component reference in manifest:
1. Check local cache by content address
2. Verify hash if present
3. Download if not cached
4. Extract/prepare for execution
5. Load and initialize

## Component Configuration

### Configuration Hierarchy

1. **User configuration** (highest priority): User's settings from Configuration Manager
2. **Manifest configuration**: Defaults specified in application manifest
3. **Component defaults**: Hardcoded defaults in component

### Configuration Format

JSON objects passed to components:
```json
{
  "apiEndpoint": "...",
  "maxConnections": 5,
  "userPreferences": {
    "theme": "dark",
    "language": "en"
  }
}
```

## MPC Client Support

Each backend component provides an MPC (Multi-Party Computation) client:
- Agent-compatible client binding
- Direct API invocation for AI agents
- Enables automated testing and development
- Same authentication as regular clients
- Developer/QA context only (MVP)

### Use Cases

- Automated iteration and research
- Contract tests
- Integration test harnesses
- Benchmark runs

## Component Architecture Benefits

### For Users

- Consistent experience across applications
- Fast loading with caching
- Offline capability
- Security through isolation

### For Developers

- Modular development
- Independent versioning
- Reusable components
- Clear interfaces
- Easy distribution

### For the Ecosystem

- Component marketplace potential
- Mix-and-match flexibility
- Long-term stability
- Innovation through composition

## Future Enhancements

Potential future additions to the component system:
- Enhanced sandboxing and permissions
- Component marketplace
- Hot reloading for development
- Component analytics
- Cross-component orchestration by AI agents
- Additional packaging formats
