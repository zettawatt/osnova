# Chapter 3: Core Services

This chapter documents Osnova's five built-in core services that provide essential functionality for all applications.

## Overview

Core services are integrated directly into the Osnova shell as in-process Rust modules. They provide the foundation for identity management, storage, payments, networking, and development tooling.

All core services are **required for MVP** functionality.

## Services Covered

### [Overview](./overview.md)
Comprehensive overview of all core services, their interactions, and architectural patterns.

**Key topics**:
- Service architecture and communication
- Service interactions and data flow
- Authentication and authorization
- Error handling and performance
- Security considerations
- Integration guidelines

### [osnova-core](./osnova-core.md)
Central coordination service providing application management, configuration, identity, key management, storage, and UI operations.

**Key features**:
- Application lifecycle (list, launch, install, uninstall)
- Configuration management (per-app settings, cache)
- Identity operations (create, import, status)
- Key derivation with cocoon-based encryption
- Storage operations (read, write, delete)
- Component management
- UI configuration (theme, bottom menu)
- Server status endpoint

**OpenRPC Methods**: apps.*, config.*, launcher.*, identity.*, pairing.*, keys.*, storage.*, component.*, ui.*, nav.*, status.*

### [osnova-saorsa](./osnova-saorsa.md)
Decentralized identity, presence, messaging, and storage service based on saorsa-core.

**Key features**:
- Four-word identity addresses (e.g., "river-spark-honest-lion")
- Post-quantum signatures (ML-DSA-65)
- Multi-device presence management
- P2P messaging (direct and group)
- Virtual disks (private and public/website)
- DHT storage with trust weighting
- Real-time media (audio/video calls)
- Group management

**OpenRPC Methods**: saorsa.identity.*, saorsa.device.*, saorsa.presence.*, saorsa.group.*, saorsa.dht.*, saorsa.messaging.*, saorsa.storage.*, saorsa.disk.*, saorsa.website.*, saorsa.call.*, saorsa.transport.*

### [osnova-wallet](./osnova-wallet.md)
Ethereum-compatible wallet service for cryptocurrency payments.

**Key features**:
- Ethereum wallet management (create, import, export, list)
- Multi-network support (Ethereum, Arbitrum)
- Token support (ETH, ERC-20 including AUTONOMI)
- BIP-44/BIP-32 key derivation
- Component authorization system
- Payment approval workflow
- Balance tracking
- Transaction history
- Spending limits

**OpenRPC Methods**: wallet.create, wallet.list, wallet.import, wallet.export, wallet.getBalance, wallet.getBalances, wallet.requestPayment, wallet.estimateGas, wallet.linkComponent, wallet.unlinkComponent, wallet.listLinkedComponents, wallet.getTransactions

### [osnova-autonomi](./osnova-autonomi.md)
Integration with the Autonomi distributed storage network.

**Key features**:
- Immutable data (chunks, up to 4MB)
- Mutable references (pointers)
- Mutable storage (scratchpads, up to 4MB)
- File collections (public and private archives)
- Graph structures (GraphEntry)
- User vaults (encrypted storage)
- CRDT registers
- Payment integration

**OpenRPC Methods**: autonomi.client.*, autonomi.chunk.*, autonomi.pointer.*, autonomi.scratchpad.*, autonomi.archive.*, autonomi.register.*, autonomi.graph.*, autonomi.vault.*

### [osnova-bundler](./osnova-bundler.md)
Developer tooling for building, packaging, and deploying Osnova applications.

**Key features**:
- Backend compilation (multi-target: Linux, macOS, Windows, Android, iOS)
- Frontend packaging (ZLIB-compressed tarballs)
- Manifest generation and validation
- Autonomi network uploads
- Complete build and deploy workflows
- Project initialization
- Payment integration for uploads

**OpenRPC Methods**: bundler.backend.*, bundler.frontend.*, bundler.manifest.*, bundler.workflow.*, bundler.project.*

## Service Interactions

### Common Integration Patterns

**Identity Creation**:
```
User → osnova-core → osnova-saorsa (4-word address)
     → Master key (from 12-word seed)
     → osnova-core (encrypted key storage)
```

**Payment for Storage**:
```
App Component → osnova-autonomi (estimate cost)
              → osnova-wallet (request payment)
              → User (approve/reject)
              → osnova-wallet (sign transaction)
              → osnova-autonomi (upload data)
```

**Application Deployment**:
```
Developer → osnova-bundler (build components)
          → osnova-wallet (authorize payments)
          → osnova-autonomi (upload to network)
          → osnova-bundler (create manifest)
          → ant:// address (installable app)
```

## Communication Architecture

### Built-In Components
Core services communicate with built-in screens via:
- Direct Rust function calls
- Type-safe interfaces
- Zero serialization overhead

### External Components
App-supplied components communicate via:
- OpenRPC (JSON-RPC 2.0)
- Stand-alone: Local IPC transport
- Client-Server: Encrypted network channel

## Error Code Ranges

Each service has a dedicated error code range:
- **osnova-core**: -30000 to -39999
- **osnova-saorsa**: -40000 to -40999
- **osnova-wallet**: -32000 to -32999
- **osnova-autonomi**: -50000 to -50999
- **osnova-bundler**: -60000 to -60999

## Security Model

All core services follow consistent security practices:

**Key Management**:
- Master key from 12-word seed phrase
- Per-service key derivation (HKDF-SHA256)
- Secure platform keystore storage
- Private keys never logged

**Data Protection**:
- Encryption at rest (saorsa-seal)
- End-to-end encryption (client-server mode)
- Per-user encryption keys
- Server cannot decrypt user content

**Payment Security**:
- User approval required
- Component authorization
- Spending limits
- Audit trail

## MVP Requirements

All five core services are required for MVP:
- **osnova-core**: Foundation
- **osnova-saorsa**: Identity and networking
- **osnova-wallet**: Payments
- **osnova-autonomi**: Storage
- **osnova-bundler**: Development tooling

## Quick Reference

### Key Concepts
- **Built-In Service**: Integrated into Osnova shell, always available
- **In-Process API**: Direct Rust function calls for maximum performance
- **OpenRPC API**: JSON-RPC 2.0 for external component communication
- **Master Key**: Derived from 12-word seed phrase, used for all key derivation
- **Component Authorization**: Permission system for wallet access

### Common Operations
- **Create Identity**: `identity.create` → 4-word address + 12-word seed
- **Request Payment**: `wallet.requestPayment` → user approval → transaction
- **Upload Data**: `autonomi.chunk.upload` → payment → chunk address
- **Deploy App**: `bundler.workflow.buildAndDeploy` → ant:// address

### Service Dependencies
- **osnova-wallet** depends on osnova-core (key derivation)
- **osnova-autonomi** depends on osnova-wallet (payments)
- **osnova-bundler** depends on osnova-wallet (upload payments) and osnova-autonomi (uploads)
- **osnova-saorsa** depends on osnova-core (key storage)

## Related Documentation

- **Chapter 2: Architecture** - Overall system architecture
- **Chapter 4: Core Screens** - Frontend interfaces for these services
- **Chapter 5: Components** - Building components that use these services
- **Chapter 6: Protocols** - OpenRPC specifications
- **Chapter 7: Security** - Detailed security model
- **Chapter 8: Networking** - Network architecture and pairing

## For Developers

### Using Core Services

External components access core services via OpenRPC:

```javascript
// Example: Upload to Autonomi
const result = await openrpc.call("autonomi.chunk.upload", {
  data: base64Data,
  walletAddress: userWallet
});
// User approves payment in wallet dialog
// Returns: { address, size, cost, transactionHash }
```

### Authorization

Components must be authorized for wallet access:

```javascript
// Link component to wallet
await openrpc.call("wallet.linkComponent", {
  componentId: "com.myapp.backend",
  componentName: "My App Backend",
  permissions: ["payment"],
  maxAmountPerTransaction: "1.0"
});
```

### Best Practices

1. **Handle errors gracefully**: Use structured error codes
2. **Request minimal permissions**: Only ask for what you need
3. **Provide clear purposes**: Help users understand payments
4. **Cache aggressively**: Minimize network operations
5. **Respect spending limits**: Don't exceed configured limits
6. **Never log secrets**: Use secure storage for keys
