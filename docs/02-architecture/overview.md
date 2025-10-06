# Architecture Overview

Osnova is built on a modular, component-based architecture designed for flexibility, extensibility, and long-term maintainability.

## High-Level Architecture

Osnova is a Tauri 2.x application combining:
- **Frontend**: TypeScript/Svelte UI running in WebView
- **Backend**: Rust library providing core business logic
- **Components**: Dynamically loadable frontend and backend modules

### Technology Stack

**Frontend**:
- TypeScript for type safety
- HTML and CSS for structure and styling
- Svelte framework for reactive UI
- Responsive design for desktop and mobile

**Backend**:
- Rust (stable) for performance and safety
- Tauri 2.x for cross-platform capabilities
- Core business logic packaged as a library

**Protocols**:
- OpenRPC (JSON-RPC 2.0) for component communication

**Storage**:
- Encrypted user-scoped data store
- Local cache for downloaded components
- Content-addressed networks (primarily Autonomi) for component distribution

## Core Architectural Principles

### 1. Component-Based Design

Applications are composed of modular components that can be:
- Developed independently
- Versioned immutably
- Reused across multiple applications
- Loaded dynamically at runtime

### 2. Separation of Concerns

Clear boundaries between:
- **UI Layer**: Frontend components handle presentation
- **Business Logic**: Backend components handle processing
- **Core Services**: Built-in services provide common functionality
- **Storage Layer**: Encrypted data persistence

### 3. Cross-Platform Compatibility

Single codebase targeting:
- Desktop: Windows, macOS, Linux
- Mobile: Android, iOS

Platform-specific adaptations where needed while maintaining core consistency.

### 4. Security by Design

- End-to-end encryption for user data
- Encryption at rest for local storage
- Secure identity management via saorsa-core
- Isolated data stores per user

### 5. Performance Focus

- p95 launch time ≤ 2 seconds
- Responsive mobile clients even with remote backends
- Efficient local caching
- Graceful degradation under poor network conditions

## System Layers

### 1. Shell Layer (Tauri Application)

The Tauri shell provides:
- Application lifecycle management
- Window and tab management
- Native OS integration
- WebView hosting for UI components

### 2. Core Services Layer

Built-in backend services:
- **osnova-core**: Central coordination and management
- **osnova-saorsa**: Identity and secure communications
- **osnova-wallet**: Cryptocurrency wallet functionality
- **osnova-autonomi**: Distributed storage integration
- **osnova-bundler**: Component packaging and distribution

### 3. Core Screens Layer

Built-in frontend interfaces:
- **Launcher**: Application discovery and launching
- **Configuration**: System and app settings management
- **Deployment**: Application deployment and management

### 4. Application Layer

User-installed applications composed of:
- Application manifest (metadata and dependencies)
- Frontend components (UI)
- Backend components (business logic)
- Configuration and cached data

## Data Flow

### Stand-Alone Mode
```
User Interaction
    ↓
Frontend Component (WebView)
    ↓
OpenRPC (Local IPC)
    ↓
Backend Component (Rust)
    ↓
Core Services
    ↓
Encrypted Local Storage
```

### Client-Server Mode
```
User Interaction (Mobile)
    ↓
Frontend Component (WebView)
    ↓
OpenRPC (Encrypted Channel)
    ↓
Server: Backend Component (Rust)
    ↓
Server: Core Services
    ↓
Server: Encrypted Storage (per-user)
```

## Communication Architecture

### Component Communication

**Built-in Services** (Core Services ↔ Core Screens):
- In-process Rust API calls
- Direct function invocation
- Type-safe interfaces

**External Components** (App-supplied):
- OpenRPC over local IPC (stand-alone mode)
- OpenRPC over encrypted network channel (client-server mode)
- Schema-validated JSON-RPC 2.0 messages

### Inter-Component Isolation

- Frontend components run in separate WebView instances
- No direct inter-tab communication
- Backend components communicate via OpenRPC
- Shared state managed through core services

## Storage Architecture

### Local Storage
- User configuration (per-app settings)
- Application cache (regenerable data)
- Downloaded components (for offline use)
- Identity and keys (in secure keystore)

### Distributed Storage
- Application manifests
- Component versions (immutable)
- App assets and resources
- User-uploaded content (optional)

All stored on content-addressed networks (primarily Autonomi).

## Concurrency Model

### Stand-Alone Mode
- Single-user environment
- All components run in local process space
- Efficient in-process communication

### Client-Server Mode
- Multi-user environment
- Server handles ≥5 concurrent clients (MVP)
- Per-user data isolation
- Independent execution contexts

## Update and Evolution Strategy

### Immutable Component Versions
- Each component version has a permanent address
- Applications reference specific versions
- No breaking changes to existing deployments

### Graceful Migration
- Multiple versions can coexist
- Applications update on their own schedule
- Backward compatibility for core APIs

### Plugin Architecture
Applications can extend functionality through:
- Custom backend components
- Custom frontend components
- Configuration and customization

## Testing Architecture

Following Test-Driven Development:
- **Contract Tests**: Validate OpenRPC schemas
- **Unit Tests**: Test individual modules
- **Integration Tests**: Test component interactions
- **End-to-End Tests**: Validate user scenarios

Target: ≥85% code coverage

## Monitoring and Diagnostics

### Logging
- File-based logging with rotation
- Per-component/host logs acceptable for MVP
- Default log level: INFO
- Secrets redacted in all modes

### Health Monitoring
- Server mode: read-only status endpoint
- Component health checks
- Performance metrics
- Error tracking

## Extensibility Points

The architecture supports future extensions:
- AI agent integration via MPC clients
- Additional storage backends
- Custom component types
- Enhanced security models
- Advanced caching strategies

## Architecture Updates

**Update (2025-10-03)**: Core services and core screens are now built directly into the Osnova shell rather than being separate components. They use in-process Rust APIs for maximum performance. The component architecture described here applies primarily to app-supplied components loaded from manifests.

This change simplifies the architecture while maintaining the flexibility of the component model for third-party applications.
