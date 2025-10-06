# Chapter 2: Architecture

This chapter provides a comprehensive overview of Osnova's architecture, from high-level design principles to specific implementation details.

## Overview

Osnova's architecture is designed around modularity, security, and cross-platform compatibility. The system uses a component-based approach that allows for flexible application composition while maintaining strong isolation and security guarantees.

## Topics Covered

### [Architecture Overview](./overview.md)
Learn about the high-level architecture, system layers, and core architectural principles.

**Key topics**:
- Technology stack (Rust, Tauri, Svelte)
- System layers (Shell, Core Services, Core Screens, Applications)
- Data flow in different modes
- Communication architecture
- Storage architecture
- Concurrency model
- Testing and monitoring

### [Components](./components.md)
Deep dive into Osnova's component system, the foundation of its flexibility.

**Key topics**:
- Component types (frontend and backend)
- Built-in vs app-supplied components
- Component communication patterns
- Versioning and immutability
- Component lifecycle
- Packaging and distribution
- Caching and loading
- MPC client support

### [Operating Modes](./modes.md)
Understand the two primary operating modes and how to choose between them.

**Key topics**:
- Stand-alone mode (all local)
- Client-server mode (mobile + server)
- Architecture for each mode
- Pairing process
- Data security and isolation
- Performance characteristics
- Fallback behavior
- Multi-client scenarios
- Headless server mode

### [Data Model](./data-model.md)
Explore the entities, relationships, and data structures used throughout Osnova.

**Key topics**:
- Core entities (Application, Component, Identity, etc.)
- Relationships between entities
- Validation and constraints
- State transitions
- Storage considerations

### [Platform Targets](./platform-targets.md)
See the complete list of supported platforms and target requirements.

**Key topics**:
- Desktop platforms (Windows, macOS, Linux)
- Mobile platforms (Android, iOS)
- Version requirements
- Future platform expansion

## Key Architectural Decisions

### 1. Tauri 2.x Foundation
Osnova is built on Tauri 2.x, providing:
- Cross-platform support with a single codebase
- Native performance
- Small binary sizes
- Secure WebView integration

### 2. Rust Backend
The backend is written in Rust for:
- Memory safety without garbage collection
- High performance
- Excellent concurrency support
- Strong type system

### 3. Svelte Frontend
The UI uses Svelte for:
- Reactive, efficient rendering
- Small bundle sizes
- Developer productivity
- Clean, readable code

### 4. Component-Based Design
Applications are composed of modular components:
- Independent development and versioning
- Reusability across applications
- Clear interfaces and contracts
- Dynamic loading at runtime

### 5. Built-In Core Services
Core services are integrated into the shell:
- Maximum performance (in-process)
- Always available
- Foundation for all applications
- Consistent API surface

### 6. OpenRPC for External Components
App-supplied components use OpenRPC:
- Language-independent protocol
- Schema validation
- Future-proof design
- Tool generation support

### 7. Content-Addressed Storage
Components stored on Autonomi network:
- Immutable versions
- Permanent availability
- Integrity verification
- Decentralized distribution

### 8. End-to-End Encryption
User data is encrypted throughout:
- Encryption at rest locally
- E2E encryption in client-server mode
- Server cannot decrypt user content
- Per-device encryption keys

## Architecture Principles

### Security First
- Encryption at rest and in transit
- Data isolation per user
- Secure identity management
- Secrets never logged

### Performance Focused
- p95 launch time ≤ 2 seconds
- Responsive UI even with remote backends
- Efficient caching strategies
- Graceful degradation

### User Privacy
- End-to-end encryption
- Local-first by default
- Server cannot access user data
- User controls all data

### Developer Friendly
- Clear component interfaces
- Comprehensive testing
- Good documentation
- Extensible design

### Future Proof
- Immutable component versions
- Content-addressed storage
- Generic communication protocols
- Modular, replaceable parts

## System Boundaries

### What Osnova Provides
- Application shell and lifecycle management
- Component loading and execution
- Identity and security
- Storage integration
- Communication infrastructure
- Core services and screens

### What Applications Provide
- Business logic (backend components)
- User interface (frontend components)
- Application-specific configuration
- Custom workflows and features

### What Users Control
- Which applications to install
- Operating mode (stand-alone vs client-server)
- Configuration and settings
- Data retention and backup

## Architecture Evolution

### Current State (MVP)
- Core services built into shell
- Stand-alone and client-server modes
- Basic component system
- Essential security features
- Cross-platform support

### Near-Term Enhancements
- Enhanced component sandboxing
- Component marketplace
- Advanced caching strategies
- Performance optimizations
- Additional platform targets

### Long-Term Vision
- AI agent integration
- Cross-component orchestration
- Enhanced permission systems
- Additional storage backends
- Advanced analytics

## Related Documentation

- **Chapter 3: Core Services** - Details on built-in backend services
- **Chapter 4: Core Screens** - Details on built-in frontend screens
- **Chapter 5: Components** - Component development guide
- **Chapter 6: Protocols** - Communication protocols and schemas
- **Chapter 7: Security** - Security model and implementation

## Quick Reference

### Architecture Documents
- [Overview](./overview.md) - High-level architecture
- [Components](./components.md) - Component system
- [Modes](./modes.md) - Operating modes
- [Data Model](./data-model.md) - Entities and relationships
- [Platform Targets](./platform-targets.md) - Supported platforms

### Key Concepts
- **Component**: Modular unit of functionality (frontend or backend)
- **Manifest**: Describes an application and its components
- **Stand-Alone Mode**: All operations on local device
- **Client-Server Mode**: Frontend on client, backend on server
- **Core Services**: Built-in backend services (always available)
- **Core Screens**: Built-in frontend screens (always available)

### Communication Protocols
- **In-Process**: Rust APIs (for built-in components)
- **OpenRPC**: JSON-RPC 2.0 (for app components)
- **Local IPC**: Unix sockets or named pipes (stand-alone)
- **Network**: Encrypted channel via saorsa-core (client-server)
