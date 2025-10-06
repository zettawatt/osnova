# Core Services Overview

Osnova's core services are built directly into the application shell, providing essential functionality that all applications can rely on. These services use in-process Rust APIs for maximum performance and are always available.

## Architecture

Core services are integrated as in-process Rust modules within the Osnova shell. They expose:
- **Internal APIs**: Direct Rust function calls for built-in components
- **OpenRPC APIs**: JSON-RPC 2.0 endpoints for external components (when needed)

This dual approach ensures optimal performance for built-in functionality while maintaining compatibility with external components.

## The Five Core Services

### 1. osnova-core
The central coordination service that provides:
- Application management (list, launch, install, uninstall)
- Configuration management (per-app settings, cache, launcher)
- Identity management (create, import, status)
- Key derivation and management
- Storage operations (read, write, delete)
- Component management
- UI configuration (theme, navigation)
- Server status operations

**Status**: Required for MVP
**Details**: [osnova-core.md](./osnova-core.md)

### 2. osnova-saorsa
Decentralized identity, presence, and communication service based on saorsa-core:
- Four-word identity addresses with post-quantum signatures
- Multi-device presence management
- P2P messaging (direct and group)
- Virtual disks (private and public/website storage)
- DHT storage with trust-weighted Kademlia
- Real-time media (audio/video calling)
- Group management with membership control

**Status**: Required for MVP
**Details**: [osnova-saorsa.md](./osnova-saorsa.md)

### 3. osnova-wallet
Ethereum-compatible wallet functionality for payments:
- Ethereum wallet management (create, import, export)
- Multi-network support (Ethereum mainnet, Arbitrum)
- Token support (ETH and ERC-20 tokens, specifically AUTONOMI)
- BIP-44/BIP-32 key derivation from master key
- Component-based payment authorization
- Balance tracking and transaction history
- User consent for all payments

**Status**: Required for MVP
**Details**: [osnova-wallet.md](./osnova-wallet.md)

### 4. osnova-autonomi
Integration with the Autonomi distributed storage network:
- Immutable data storage (chunks, up to 4MB)
- Mutable references (pointers, scratchpads)
- File collections (public and private archives)
- Graph structures (GraphEntry with edges)
- User vaults (encrypted storage)
- CRDT registers for conflict-free updates
- Payment integration with osnova-wallet

**Status**: Required for MVP
**Details**: [osnova-autonomi.md](./osnova-autonomi.md)

### 5. osnova-bundler
Developer tooling for building and deploying applications:
- Backend component compilation (multi-target)
- Frontend component packaging (ZLIB-compressed tarballs)
- Manifest generation and validation
- Autonomi network uploads
- Complete build and deploy workflows
- Project initialization and management

**Status**: Required for MVP
**Details**: [osnova-bundler.md](./osnova-bundler.md)

## Service Interactions

### Identity and Security Flow
```
User Creates Identity
    ↓
osnova-core → osnova-saorsa (4-word address)
    ↓
Master key derived from 12-word seed
    ↓
osnova-core stores encrypted keys
```

### Payment Flow
```
Component needs storage
    ↓
osnova-autonomi estimates cost
    ↓
osnova-wallet requests user approval
    ↓
User approves payment
    ↓
osnova-wallet signs transaction
    ↓
osnova-autonomi uploads data
```

### Application Deployment Flow
```
Developer builds app
    ↓
osnova-bundler compiles backend
    ↓
osnova-bundler packages frontend
    ↓
osnova-wallet authorizes payments
    ↓
osnova-autonomi uploads components
    ↓
osnova-bundler creates manifest
    ↓
App available at ant:// address
```

## Communication Patterns

### Internal (Built-in Components)
- Direct Rust function calls
- Type-safe interfaces
- Zero serialization overhead
- Maximum performance

### External (App Components)
- OpenRPC over local IPC (stand-alone mode)
- OpenRPC over encrypted channel (client-server mode)
- Schema-validated JSON-RPC 2.0
- Standard error codes

## Data Storage

Each service manages its own data:
- **osnova-core**: SQLite for structured data, encrypted files for blobs
- **osnova-saorsa**: DHT storage, encrypted virtual disks
- **osnova-wallet**: Encrypted key storage, transaction history
- **osnova-autonomi**: Secret keys, cache of downloaded data
- **osnova-bundler**: Build artifacts, manifest cache

All sensitive data is encrypted at rest using keys derived from the master key.

## Authentication and Authorization

### Stand-Alone Mode
- Services run in same process as shell
- Direct function calls require no additional auth
- Per-user data isolation enforced by service layer

### Client-Server Mode
- Encrypted channel established via saorsa-core
- All OpenRPC calls authenticated
- Per-client data isolation
- Server cannot decrypt user content

### Component Authorization
- Components must be linked for wallet access
- Per-component spending limits
- Audit trail for all payments
- User approval for sensitive operations

## Error Handling

All services follow consistent error handling:
- Structured error codes by service (-30000 to -69999)
- Human-readable error messages
- Detailed error contexts for debugging
- Secrets never logged

### Error Code Ranges
- **osnova-core**: -30000 to -39999
- **osnova-saorsa**: -40000 to -40999
- **osnova-wallet**: -32000 to -32999
- **osnova-autonomi**: -50000 to -50999
- **osnova-bundler**: -60000 to -60999

## Performance Characteristics

### osnova-core
- Highly optimized for frequent calls
- SQLite queries < 10ms
- Configuration reads cached in memory

### osnova-saorsa
- DHT lookups typically < 500ms
- P2P messaging < 1s (local network)
- Virtual disk operations depend on network

### osnova-wallet
- Local operations < 100ms
- Network operations (balance, tx) 1-5s
- Payment approval requires user interaction

### osnova-autonomi
- Downloads free and fast (cached)
- Uploads require payment approval
- Large files may take several seconds

### osnova-bundler
- Compilation time varies by size
- Frontend packaging typically < 10s
- Uploads depend on file size

## Security Considerations

### Key Management
- Master key derived from 12-word seed
- Per-service key derivation (HKDF-SHA256)
- Keys stored in platform secure keystore
- Private keys never exported or logged

### Data Encryption
- Encryption at rest (saorsa-seal)
- End-to-end encryption in client-server mode
- Per-user encryption keys
- No server access to user content

### Payment Security
- User approval required for all payments
- Component authorization system
- Spending limits enforced
- Transaction audit trail

### Network Security
- Encrypted channels (saorsa-core)
- Post-quantum signatures (ML-DSA-65)
- NAT traversal and hole punching
- DHT trust weighting

## MVP Scope

All five core services are required for MVP functionality:
- **osnova-core**: Foundation for all operations
- **osnova-saorsa**: Identity and networking
- **osnova-wallet**: Payments for storage
- **osnova-autonomi**: Distributed storage
- **osnova-bundler**: Developer tooling

## Post-MVP Enhancements

Potential future additions:
- Additional storage backends
- Enhanced caching strategies
- Performance monitoring
- Advanced analytics
- Additional payment methods
- Multi-sig wallet support
- Hardware wallet integration
- Build optimization
- Automated testing integration

## Integration Guidelines

### For Application Developers

When building Osnova applications:
1. Use built-in services via OpenRPC (external components)
2. Request wallet authorization before payments
3. Store sensitive data via osnova-core encryption
4. Use saorsa identities for user addressing
5. Leverage Autonomi for immutable storage
6. Use bundler for deployment

### For Service Extensions

When extending core services:
1. Follow OpenRPC conventions
2. Use consistent error codes
3. Implement comprehensive logging
4. Never log secrets or private keys
5. Provide user-friendly error messages
6. Support both stand-alone and server modes

## Monitoring and Diagnostics

### Logging
- File-based logs with rotation
- Per-service log sections
- Default level: INFO
- Secrets always redacted

### Health Checks
- `status.get` for overall health
- Per-component status queries
- Resource usage monitoring
- Network connectivity checks

### Metrics
- Request counts and latencies
- Error rates by type
- Storage usage
- Network bandwidth
- Payment volumes

## Related Documentation

- **Chapter 4: Core Screens** - Frontend interfaces for core services
- **Chapter 5: Components** - How to build components that use core services
- **Chapter 6: Protocols** - OpenRPC specifications
- **Chapter 7: Security** - Detailed security model
- **Chapter 8: Networking** - Network architecture and pairing
