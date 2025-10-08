# Phase 3 Implementation Tasks

**Status**: Phase 2 Complete (100%) | Phase 3 Ready to Start
**Generated**: 2025-10-07
**Estimated Duration**: 4-6 weeks with multi-agent workflow

## Overview

Phase 3 focuses on implementing the core Osnova functionality: Autonomi Network integration, component packaging system, server mode, client-server pairing, and production deployment. This phase bridges the gap between the UI prototype (Phase 2) and a fully functional distributed application platform.

## Principles

- **Test-Driven Development (TDD)**: Write failing tests before implementation
- **Contract-First**: OpenRPC contracts define all inter-component communication
- **Incremental Integration**: Build and test components in isolation, then integrate
- **Documentation**: All public APIs documented with examples
- **Coverage Target**: ≥85% test coverage across all modules

## Task Categories

1. **Autonomi Network Integration** (Tasks 064-072) ✅ COMPLETE
2. **Cross-Platform Path Management** (Tasks 073-087) - See CROSS_PLATFORM_PATHS.md
3. **Component Packaging System** (Tasks 088-097) - **UPDATED FOR SOURCE DISTRIBUTION**
4. **Identity & Key Management** (Tasks 098-105) - Renumbered
5. **Server Mode & Client-Server Pairing** (Tasks 106-115) - Renumbered
6. **OpenRPC Infrastructure** (Tasks 116-125) - Renumbered
7. **App Management Backend** (Tasks 126-133) - Renumbered
8. **Production Deployment** (Tasks 134-138) - Renumbered
9. **Performance & Optimization** (Tasks 139-143) - Renumbered

**Note**: Tasks 073-087 were inserted for cross-platform path management. Original tasks 073+ have been renumbered accordingly. See `CROSS_PLATFORM_PATHS.md` for detailed cross-platform path implementation tasks.

**IMPORTANT ARCHITECTURAL CHANGE**: Component Packaging System (Tasks 088-097) updated on 2025-10-08 to use **source distribution for backend components** instead of distributing pre-compiled dynamic libraries. Backend components are now distributed as Rust source code tarballs and compiled locally on the user's machine using a Rust toolchain downloaded on first backend component install. This avoids macOS/Windows security warnings for unsigned binaries. See `docs/05-components/packaging.md` for complete architecture.

---

## 1. Autonomi Network Integration (Tasks 064-072)

### Task 064: Integrate Autonomi SDK v0.6.1
**Priority**: Critical | **Estimated**: 2-3 days | **Parallelizable**: No

**Description**: Integrate the Autonomi Rust SDK into osnova_lib for network operations.

**Acceptance Criteria**:
- [ ] Add `autonomi = "0.6.1"` to Cargo.toml
- [ ] Create `core/osnova_lib/src/network/autonomi_client.rs`
- [ ] Implement connection management (connect, disconnect, health check)
- [ ] Write unit tests for connection lifecycle
- [ ] Document API with examples
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml`
- `core/osnova_lib/src/network/mod.rs`
- `core/osnova_lib/src/network/autonomi_client.rs`
- `core/osnova_lib/tests/network/autonomi_client_test.rs`

**Dependencies**: None

---

### Task 065: Implement Autonomi Data Upload
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement uploading data to Autonomi Network with content addressing.

**Acceptance Criteria**:
- [ ] `upload_data(data: &[u8]) -> Result<Address>`
- [ ] Handle chunking for large files (>1MB)
- [ ] Generate and return content address (ant:// URI)
- [ ] Write contract tests for upload operations
- [ ] Test error cases (network failure, quota exceeded)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/network/upload.rs`
- `core/osnova_lib/tests/network/upload_test.rs`
- `docs/06-protocols/autonomi-operations.md`

**Dependencies**: Task 064

---

### Task 066: Implement Autonomi Data Retrieval
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement downloading data from Autonomi Network by address.

**Acceptance Criteria**:
- [ ] `download_data(address: &Address) -> Result<Vec<u8>>`
- [ ] Handle large file downloads with progress tracking
- [ ] Verify content integrity (hash check)
- [ ] Write contract tests for download operations
- [ ] Test error cases (not found, corrupted data, timeout)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/network/download.rs`
- `core/osnova_lib/tests/network/download_test.rs`

**Dependencies**: Task 064

---

### Task 067: Implement Local Component Cache
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes (parallel with 065-066)

**Description**: Implement local cache for downloaded components with TTL and eviction.

**Acceptance Criteria**:
- [ ] Create `CacheManager` with LRU eviction policy
- [ ] Store components by content hash
- [ ] Implement cache directory structure per platform
- [ ] Add cache size limits (configurable, default 500MB)
- [ ] Write unit tests for cache operations
- [ ] Test cache eviction and expiration
- [ ] Coverage ≥85%

**Platform-Specific Cache Locations**:
- Linux: `~/.cache/osnova/components/`
- macOS: `~/Library/Caches/Osnova/components/`
- Windows: `%LOCALAPPDATA%\Osnova\Cache\components\`
- Android: `getExternalCacheDir()/components/`
- iOS: `Library/Caches/components/`

**Files to Create/Modify**:
- `core/osnova_lib/src/cache/mod.rs`
- `core/osnova_lib/src/cache/manager.rs`
- `core/osnova_lib/tests/cache/manager_test.rs`

**Dependencies**: None

---

### Task 068: Implement Manifest Schema Validation
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement JSON schema validation for application manifests.

**Acceptance Criteria**:
- [ ] Define `ManifestSchema` struct matching docs/06-protocols/manifest-schema.md
- [ ] Implement JSON parsing and validation
- [ ] Support both `ant://` URIs (prod) and local paths (dev)
- [ ] Validate required fields (id, name, version, components)
- [ ] Write contract tests for valid/invalid manifests
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/manifest/schema.rs`
- `core/osnova_lib/src/manifest/validator.rs`
- `core/osnova_lib/tests/manifest/validator_test.rs`
- `docs/06-protocols/manifest-schema.md` (update with examples)

**Dependencies**: None

---

### Task 069: Implement Manifest Resolution
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement fetching and resolving application manifests from network or local paths.

**Acceptance Criteria**:
- [ ] `resolve_manifest(uri: &str) -> Result<Manifest>`
- [ ] Support `ant://` URIs (fetch from Autonomi)
- [ ] Support `file://` paths (local development)
- [ ] Support `https://` URLs (fallback for testing)
- [ ] Cache resolved manifests
- [ ] Write integration tests for all URI types
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/manifest/resolver.rs`
- `core/osnova_lib/tests/manifest/resolver_test.rs`

**Dependencies**: Tasks 064, 068

---

### Task 070: Implement Component Download Workflow
**Priority**: High | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Implement full workflow for downloading, caching, and verifying components.

**Acceptance Criteria**:
- [ ] `download_component(component_ref: &ComponentRef) -> Result<PathBuf>`
- [ ] Check cache before downloading
- [ ] Verify component integrity (hash/signature)
- [ ] Handle frontend (ZLIB tarball) and backend (binary) components
- [ ] Extract tarballs to temporary directory
- [ ] Write integration tests for download workflow
- [ ] Test error recovery (partial downloads, corrupted data)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/components/downloader.rs`
- `core/osnova_lib/tests/components/downloader_test.rs`

**Dependencies**: Tasks 066, 067, 069

---

### Task 071: Write OpenRPC Contract for Autonomi Operations
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Define OpenRPC contract for Autonomi Network operations exposed to components.

**Acceptance Criteria**:
- [ ] Define `autonomi.upload` method
- [ ] Define `autonomi.download` method
- [ ] Define `autonomi.getStatus` method
- [ ] Include JSON schema for all parameters and responses
- [ ] Write contract tests (must fail initially)
- [ ] Document rate limits and quotas

**Files to Create/Modify**:
- `docs/06-protocols/openrpc-contracts.md` (update)
- `core/osnova_lib/tests/contracts/autonomi_contract_test.rs`

**Dependencies**: None (can run in parallel)

---

### Task 072: Integration Test - End-to-End Component Fetch
**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Write integration test for complete flow: manifest → download → cache → verify.

**Acceptance Criteria**:
- [ ] Create test manifest with `ant://` component references
- [ ] Mock Autonomi Network responses
- [ ] Test full download workflow
- [ ] Verify cached components are used on subsequent requests
- [ ] Test error scenarios (network failure, invalid manifest)
- [ ] Test runs in <5 seconds

**Files to Create/Modify**:
- `core/osnova_lib/tests/integration/component_fetch_test.rs`
- `core/osnova_lib/tests/fixtures/test_manifest.json`

**Dependencies**: Tasks 064-070

---

## 2. Component Packaging System (Tasks 088-097)

### Task 088: Implement Frontend Component Packager
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement native Rust packager for Svelte apps as ZLIB-compressed tarballs.

**Acceptance Criteria**:
- [ ] Create `core/osnova_lib/src/packaging/frontend.rs`
- [ ] Build Svelte app via `std::process::Command` (npm run build)
- [ ] Create tarball of build output using `tar` crate
- [ ] Compress with ZLIB (level 9) using `flate2` crate
- [ ] Calculate SHA-256 hash using `sha2` crate
- [ ] Generate `PackageManifest` with metadata
- [ ] Write unit tests for packaging operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/packaging/mod.rs`
- `core/osnova_lib/src/packaging/frontend.rs`
- `core/osnova_lib/src/packaging/manifest.rs`
- `core/osnova_lib/tests/packaging/frontend_test.rs`
- `docs/05-components/packaging.md` (see detailed spec)

**Dependencies**: None

---

### Task 089: Implement Backend Component Packager (Source Distribution)
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement native Rust packager for backend components as source code tarballs.

**Acceptance Criteria**:
- [ ] Create `core/osnova_lib/src/packaging/backend.rs`
- [ ] Validate Cargo.toml structure (require `crate-type = ["cdylib"]`)
- [ ] Create tarball of source code (Cargo.toml, src/, Cargo.lock)
- [ ] Compress with ZLIB (level 9) using `flate2` crate
- [ ] Calculate SHA-256 hash using `sha2` crate
- [ ] Generate `PackageManifest` with metadata
- [ ] Write unit tests for packaging operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/packaging/backend.rs`
- `core/osnova_lib/tests/packaging/backend_test.rs`
- `components/backend/example-service/` (example component)

**Dependencies**: None

---

### Task 090: Implement Component Unpacker
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement unpacking ZLIB-compressed component tarballs (both frontend and backend source).

**Acceptance Criteria**:
- [ ] Create `core/osnova_lib/src/packaging/unpacker.rs`
- [ ] `unpack_component(tarball: &Path, dest: &Path, expected_hash: &str) -> Result<()>`
- [ ] Verify SHA-256 hash before unpacking
- [ ] Decompress ZLIB tarball using `flate2` crate
- [ ] Extract to destination directory using `tar` crate
- [ ] Sanitize paths to prevent path traversal attacks
- [ ] Write unit tests for unpacking
- [ ] Test error cases (corrupted tarball, hash mismatch, path traversal)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/packaging/unpacker.rs`
- `core/osnova_lib/tests/packaging/unpacker_test.rs`

**Dependencies**: None

---

### Task 091: Implement Rust Toolchain Downloader
**Priority**: Critical | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Download and install Rust toolchain on first backend component install (download on first run approach).

**Acceptance Criteria**:
- [ ] Create `core/osnova_lib/src/packaging/toolchain.rs`
- [ ] Download rustup-init from official Rust servers (https://static.rust-lang.org/rustup/dist/)
- [ ] Support platforms: Linux (x86_64/aarch64), macOS (x86_64/aarch64), Windows (x86_64)
- [ ] Verify SHA-256 hash against official checksums
- [ ] Install minimal toolchain (rustc + cargo only, no docs) to `{data_dir}/rust-toolchain/`
- [ ] Show progress dialog to user during download and installation
- [ ] Write unit tests for download and verification
- [ ] Test error cases (network failure, hash mismatch)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/packaging/toolchain.rs`
- `core/osnova_lib/tests/packaging/toolchain_test.rs`
- `docs/05-components/packaging.md` (see Rust Toolchain Management section)

**Dependencies**: None

---

### Task 092: Implement Local Backend Component Compilation
**Priority**: Critical | **Estimated**: 2.5 days | **Parallelizable**: No

**Description**: Compile backend component source code locally using downloaded Rust toolchain.

**Acceptance Criteria**:
- [ ] `compile_backend_component(source_dir: &Path, component_id: &str) -> Result<PathBuf>`
- [ ] Call `cargo build --release --lib` via `std::process::Command`
- [ ] Use `CARGO_HOME` and `RUSTUP_HOME` environment variables pointing to Osnova's toolchain
- [ ] Locate compiled dynamic library (.so/.dylib/.dll) in `target/release/`
- [ ] Cache compiled binary in `{data_dir}/component-cache/{component-id}/library`
- [ ] Show progress dialog to user during compilation
- [ ] Write integration tests for compilation workflow
- [ ] Test error cases (compilation failure, missing dependencies)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/packaging/toolchain.rs` (extend)
- `core/osnova_lib/tests/packaging/compilation_test.rs`

**Dependencies**: Task 091

---

### Task 093: Implement Backend Component ABI
**Priority**: Critical | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Define and implement plugin ABI for backend components (loaded from locally-compiled dynamic libraries).

**Acceptance Criteria**:
- [ ] Define ABI trait with methods:
  - `component_configure(config: Value) -> Result<()>`
  - `component_start() -> Result<()>`
  - `component_stop() -> Result<()>`
  - `component_status() -> Result<ComponentStatus>`
- [ ] Implement dynamic library loading (libloading crate)
- [ ] Create example backend component implementing ABI
- [ ] Write contract tests for ABI compliance
- [ ] Document ABI in `docs/05-components/component-abi.md`
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/components/abi.rs`
- `core/osnova_lib/tests/components/abi_test.rs`
- `components/backend/example-service/src/lib.rs`
- `docs/05-components/component-abi.md`

**Dependencies**: Task 092 (need compiled binaries to load)

---

### Task 094: Implement Component Loader
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement system to load and manage backend components at runtime.

**Acceptance Criteria**:
- [ ] `ComponentLoader` manages loaded components
- [ ] Load backend component from binary path
- [ ] Call `component_configure` with config JSON
- [ ] Call `component_start` to initialize
- [ ] Track component lifecycle (loading, running, stopped, failed)
- [ ] Implement `component_stop` on shutdown
- [ ] Write integration tests for component lifecycle
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/components/loader.rs`
- `core/osnova_lib/tests/components/loader_test.rs`

**Dependencies**: Task 093

---

### Task 095: Implement Component Registry
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement registry to track installed and available components.

**Acceptance Criteria**:
- [ ] `ComponentRegistry` stores component metadata
- [ ] Track installed components with versions
- [ ] Support component queries (by id, by type)
- [ ] Persist registry to disk (JSON format)
- [ ] Write unit tests for registry operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/components/registry.rs`
- `core/osnova_lib/tests/components/registry_test.rs`

**Dependencies**: Task 068

---

### Task 096: Update Apps Service - Component Integration
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Update existing Apps Service to use component system.

**Acceptance Criteria**:
- [ ] Modify `apps_list` to return components from registry
- [ ] Implement `apps_install(manifest_uri: &str)` command
- [ ] Implement `apps_uninstall(app_id: &str)` command
- [ ] Update tests to cover component lifecycle
- [ ] Update Tauri commands
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/apps.rs`
- `core/osnova_lib/tests/services/apps_test.rs`
- `app/src-tauri/src/lib.rs` (update commands)

**Dependencies**: Tasks 070, 092, 094, 095

---

### Task 097: Create Example Application Bundle
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Create complete example app with frontend and backend components (source distribution).

**Acceptance Criteria**:
- [ ] Create "Hello World" Svelte frontend component
- [ ] Create "Hello World" Rust backend service with OpenRPC endpoint (source code)
- [ ] Create application manifest referencing both components
- [ ] Package both components using packaging system (Tasks 088-089)
- [ ] Upload to Autonomi Network (or use mock URIs for testing)
- [ ] Document in `components/examples/hello-world/README.md`
- [ ] Test local compilation of backend component

**Files to Create/Modify**:
- `components/examples/hello-world/frontend/` (Svelte app)
- `components/examples/hello-world/backend/` (Rust service - source code)
- `components/examples/hello-world/manifest.json`
- `components/examples/hello-world/README.md`

**Dependencies**: Tasks 088, 089, 092, 093

---

**NOTE**: Removed former Tasks 080-082 (WebView Component Hosting, Integration Test) as they were renumbered above. The Component Packaging System now consists of Tasks 088-097.

---

## 3. Identity & Key Management (Tasks 098-105)

### Task 098: Integrate saorsa-core for Identity
**Priority**: Critical | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Integrate saorsa-core for 4-word identity address management.

**Acceptance Criteria**:
- [ ] Add `saorsa-core` as git dependency (main branch)
- [ ] Create `IdentityManager` wrapping saorsa-core
- [ ] Implement `create_identity() -> Result<(Address, Mnemonic)>`
- [ ] Implement `import_identity(phrase: &str) -> Result<Address>`
- [ ] Implement `get_current_identity() -> Result<Option<Address>>`
- [ ] Write unit tests for identity operations
- [ ] Document API with examples
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml`
- `core/osnova_lib/src/identity/mod.rs`
- `core/osnova_lib/src/identity/manager.rs`
- `core/osnova_lib/tests/identity/manager_test.rs`

**Dependencies**: None

---

### Task 099: Implement Secure Seed Phrase Storage
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement secure storage for 12-word seed phrase using platform keystore.

**Acceptance Criteria**:
- [ ] Use platform-specific secure storage:
  - Linux: Secret Service API (libsecret)
  - macOS: Keychain
  - Windows: Credential Manager
  - Android: Android Keystore
  - iOS: iOS Keychain
- [ ] Implement `store_seed_phrase(phrase: &str) -> Result<()>`
- [ ] Implement `retrieve_seed_phrase() -> Result<String>`
- [ ] Never log seed phrase in plaintext
- [ ] Write platform-specific tests
- [ ] Document security properties
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/identity/secure_storage.rs`
- `core/osnova_lib/tests/identity/secure_storage_test.rs`

**Dependencies**: None

---

### Task 100: Implement Master Key Derivation
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Derive master key from seed phrase for all cryptographic operations.

**Acceptance Criteria**:
- [ ] Use Blake3 for key derivation
- [ ] Implement `derive_master_key(seed: &str) -> Result<[u8; 32]>`
- [ ] Master key never leaves secure storage
- [ ] Support namespaced sub-key derivation for components
- [ ] Write unit tests for key derivation
- [ ] Document key hierarchy in docs/07-security/keys.md
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/identity/key_derivation.rs`
- `core/osnova_lib/tests/identity/key_derivation_test.rs`
- `docs/07-security/keys.md` (update)

**Dependencies**: Task 099

---

### Task 101: Integrate saorsa-seal for Encryption
**Priority**: Critical | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Integrate saorsa-seal (ChaCha20-Poly1305) for encryption at rest.

**Acceptance Criteria**:
- [ ] Add `saorsa-seal` dependency
- [ ] Create `EncryptionManager` wrapping saorsa-seal
- [ ] Implement `encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>>`
- [ ] Implement `decrypt_data(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>>`
- [ ] Use master key material for encryption
- [ ] Write unit tests for encryption operations
- [ ] Test key rotation scenarios
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml`
- `core/osnova_lib/src/encryption/mod.rs`
- `core/osnova_lib/src/encryption/manager.rs`
- `core/osnova_lib/tests/encryption/manager_test.rs`

**Dependencies**: Task 100

---

### Task 102: Implement Encrypted Data Store
**Priority**: High | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Implement encrypted key-value store for user data persistence.

**Acceptance Criteria**:
- [ ] Create `EncryptedStore` using SQLite backend
- [ ] All data encrypted at rest with saorsa-seal
- [ ] Implement CRUD operations:
  - `set(key: &str, value: &[u8]) -> Result<()>`
  - `get(key: &str) -> Result<Option<Vec<u8>>>`
  - `delete(key: &str) -> Result<()>`
  - `list_keys() -> Result<Vec<String>>`
- [ ] Per-identity data isolation
- [ ] Write unit tests for store operations
- [ ] Test encryption/decryption correctness
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/storage/encrypted_store.rs`
- `core/osnova_lib/tests/storage/encrypted_store_test.rs`

**Dependencies**: Task 101

---

### Task 103: Update Identity Service - Complete Implementation
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Complete identity service implementation with full saorsa integration.

**Acceptance Criteria**:
- [ ] Replace mock implementation with real saorsa-core
- [ ] Implement seed phrase generation and validation
- [ ] Store identity address and seed phrase securely
- [ ] Update existing tests to use real implementation
- [ ] Add integration tests for complete flows
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/identity.rs` (update)
- `core/osnova_lib/tests/services/identity_test.rs` (update)

**Dependencies**: Tasks 098, 099, 100

---

### Task 104: Implement Identity Backup/Restore
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement secure backup and restore of identity data.

**Acceptance Criteria**:
- [ ] `export_identity() -> Result<EncryptedBackup>`
- [ ] `import_identity(backup: EncryptedBackup) -> Result<()>`
- [ ] Backup includes identity address and seed phrase
- [ ] Backup encrypted with user-provided password
- [ ] Export to file with .osnova-identity extension
- [ ] Write unit tests for backup/restore
- [ ] Test password verification
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/identity/backup.rs`
- `core/osnova_lib/tests/identity/backup_test.rs`

**Dependencies**: Task 103

---

### Task 105: Integration Test - Identity Lifecycle
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: No

**Description**: End-to-end test of identity creation, storage, and usage.

**Acceptance Criteria**:
- [ ] Test create identity flow (4-word address + 12-word seed)
- [ ] Test import identity flow
- [ ] Test key derivation for encryption
- [ ] Test encrypted data storage with identity key
- [ ] Test identity backup and restore
- [ ] Test completes in <5 seconds

**Files to Create/Modify**:
- `core/osnova_lib/tests/integration/identity_lifecycle_test.rs`

**Dependencies**: Tasks 098-104

---

## 4. Server Mode & Client-Server Pairing (Tasks 091-100)

### Task 091: Implement Server Mode Detection
**Priority**: High | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Detect if Osnova is running in server or standalone mode.

**Acceptance Criteria**:
- [ ] Parse `--server` command-line flag
- [ ] Create `ServerMode` enum (Standalone, Server)
- [ ] Expose `is_server_mode() -> bool`
- [ ] Write unit tests for mode detection
- [ ] Document server mode in docs/08-networking/server-ops.md
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `app/src-tauri/src/server_mode.rs`
- `app/src-tauri/tests/server_mode_test.rs`
- `docs/08-networking/server-ops.md` (update)

**Dependencies**: None

---

### Task 092: Implement Server Status Service
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement read-only status endpoint for server monitoring.

**Acceptance Criteria**:
- [ ] Create `ServerStatus` struct with fields:
  - `version: String`
  - `uptime: Duration`
  - `mode: ServerMode`
  - `components: Vec<ComponentStatus>`
- [ ] Implement `status.get` OpenRPC method
- [ ] Expose HTTP endpoint at `/status` (JSON)
- [ ] Write contract tests for status endpoint
- [ ] Test component status collection
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/server_status.rs`
- `core/osnova_lib/tests/services/server_status_test.rs`
- `docs/06-protocols/openrpc-contracts.md` (update)

**Dependencies**: Task 091

---

### Task 093: Implement File-Based Logging
**Priority**: Medium | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement structured logging with rotation for server mode.

**Acceptance Criteria**:
- [ ] Use `tracing` crate for structured logging
- [ ] Rotate logs at 10MB (keep 7 files)
- [ ] Log to platform-specific locations:
  - Linux: `/var/log/osnova/` or `~/.local/state/osnova/logs/`
  - macOS: `~/Library/Logs/Osnova/`
  - Windows: `%ProgramData%\Osnova\logs\`
- [ ] Include timestamp, level, module in logs
- [ ] Write unit tests for logging configuration
- [ ] Document log format and locations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/logging/mod.rs`
- `core/osnova_lib/src/logging/file_logger.rs`
- `core/osnova_lib/tests/logging/file_logger_test.rs`
- `docs/08-networking/server-ops.md` (update)

**Dependencies**: None

---

### Task 094: Implement OpenRPC Server for Server Mode
**Priority**: Critical | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Implement OpenRPC server that exposes backend services over network.

**Acceptance Criteria**:
- [ ] Create `OpenRpcServer` using HTTP transport
- [ ] Register all service methods dynamically
- [ ] Support JSON-RPC 2.0 protocol
- [ ] Bind to configurable address (default: 127.0.0.1:8080)
- [ ] Implement authentication middleware (TLS + client certificates)
- [ ] Write integration tests for RPC calls
- [ ] Test error responses (method not found, invalid params)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/server.rs`
- `core/osnova_lib/src/rpc/middleware.rs`
- `core/osnova_lib/tests/rpc/server_test.rs`

**Dependencies**: Task 092

---

### Task 095: Implement Client-Server Pairing
**Priority**: Critical | **Estimated**: 4 days | **Parallelizable**: No

**Description**: Implement secure pairing between client and server.

**Acceptance Criteria**:
- [ ] Server generates pairing token (QR code + manual entry)
- [ ] Client initiates pairing with token
- [ ] Implement Diffie-Hellman key exchange
- [ ] Generate shared secret for session encryption
- [ ] Store paired client credentials
- [ ] Implement `pairing.start`, `pairing.complete` methods
- [ ] Write contract tests for pairing flow
- [ ] Test error cases (invalid token, timeout)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/pairing/mod.rs`
- `core/osnova_lib/src/pairing/server.rs`
- `core/osnova_lib/src/pairing/client.rs`
- `core/osnova_lib/tests/pairing/pairing_test.rs`
- `docs/08-networking/pairing.md` (complete implementation)

**Dependencies**: Task 094

---

### Task 096: Implement End-to-End Encryption for Client-Server
**Priority**: Critical | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Implement E2E encryption for all client-server communication.

**Acceptance Criteria**:
- [ ] Use TLS 1.3 for transport encryption
- [ ] Implement application-level encryption with shared secret
- [ ] All user data encrypted before leaving client
- [ ] Server never decrypts user data blobs
- [ ] Write policy enforcement tests
- [ ] Document encryption architecture in docs/07-security/encryption.md
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/encryption.rs`
- `core/osnova_lib/tests/rpc/encryption_test.rs`
- `docs/07-security/encryption.md` (complete)

**Dependencies**: Task 095

---

### Task 097: Implement Server Configuration Service
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Implement configuration service to set server address on client.

**Acceptance Criteria**:
- [ ] Implement `config.setServer(address: &str)` method
- [ ] Validate server address format
- [ ] Test reachability before saving
- [ ] Persist server address to encrypted store
- [ ] Implement `config.getServer() -> Option<String>`
- [ ] Write contract tests for config operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/config.rs`
- `core/osnova_lib/tests/services/config_test.rs`

**Dependencies**: Task 087

---

### Task 098: Implement Client Switching (Standalone ↔ Server)
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Allow client to switch between standalone and server modes.

**Acceptance Criteria**:
- [ ] Detect server mode vs standalone mode
- [ ] Use local IPC transport in standalone mode
- [ ] Use HTTP/TLS transport in server mode
- [ ] Seamlessly switch transport based on config
- [ ] Write integration tests for mode switching
- [ ] Test data sync after switch
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/transport.rs`
- `core/osnova_lib/tests/rpc/transport_test.rs`

**Dependencies**: Tasks 094, 097

---

### Task 099: Implement Multi-Client Support on Server
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Support ≥5 concurrent clients on server (MVP requirement).

**Acceptance Criteria**:
- [ ] Handle concurrent RPC requests from multiple clients
- [ ] Implement connection pooling
- [ ] Per-client session management
- [ ] Enforce rate limits (100 req/sec per client)
- [ ] Write load tests with 5+ concurrent clients
- [ ] Measure and document performance
- [ ] Test completes in <30 seconds

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/connection_pool.rs`
- `core/osnova_lib/tests/rpc/load_test.rs`

**Dependencies**: Task 094

---

### Task 100: Integration Test - Client-Server Communication
**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: End-to-end test of client-server pairing and RPC communication.

**Acceptance Criteria**:
- [ ] Start server in server mode
- [ ] Pair client with server
- [ ] Make RPC calls from client to server
- [ ] Verify E2E encryption
- [ ] Test component operations (install, launch)
- [ ] Test multiple clients concurrently
- [ ] Test completes in <15 seconds

**Files to Create/Modify**:
- `core/osnova_lib/tests/integration/client_server_test.rs`

**Dependencies**: Tasks 091-099

---

## 5. OpenRPC Infrastructure (Tasks 101-110)

### Task 101: Generate OpenRPC Schema from Rust Code
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement tooling to auto-generate OpenRPC schemas from Rust service code.

**Acceptance Criteria**:
- [ ] Create `scripts/generate-openrpc-schema.sh`
- [ ] Use procedural macros to annotate services
- [ ] Generate `openrpc.json` from annotated code
- [ ] Include method descriptions, params, return types
- [ ] Validate generated schema against OpenRPC spec
- [ ] Document in `docs/06-protocols/openrpc-conventions.md`

**Files to Create/Modify**:
- `scripts/generate-openrpc-schema.sh`
- `core/osnova_lib/src/rpc/macros.rs`
- `docs/06-protocols/openrpc-conventions.md` (update)

**Dependencies**: None

---

### Task 102: Implement OpenRPC Client Generator
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Generate TypeScript client from OpenRPC schema for frontend use.

**Acceptance Criteria**:
- [ ] Create `scripts/generate-ts-client.sh`
- [ ] Parse `openrpc.json` schema
- [ ] Generate TypeScript client code with types
- [ ] Include JSDoc comments from schema
- [ ] Output to `app/src/lib/rpc/generated/`
- [ ] Write tests for generated client

**Files to Create/Modify**:
- `scripts/generate-ts-client.sh`
- `app/src/lib/rpc/generated/client.ts` (generated)

**Dependencies**: Task 101

---

### Task 103: Implement MCP Client for Backend Components
**Priority**: Medium | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Create MCP client binding for AI agent access to backend APIs.

**Acceptance Criteria**:
- [ ] Create MCP client that mirrors OpenRPC methods
- [ ] Support authentication (same as regular clients)
- [ ] Implement method invocation via MCP protocol
- [ ] Write example usage for AI agents
- [ ] Document in `docs/06-protocols/mcp-client.md`
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/mcp/client.rs`
- `core/osnova_lib/tests/mcp/client_test.rs`
- `docs/06-protocols/mcp-client.md`

**Dependencies**: Task 094

---

### Task 104: Write Contract Tests for All OpenRPC Methods
**Priority**: High | **Estimated**: 3 days | **Parallelizable**: Yes

**Description**: Complete contract test coverage for all defined OpenRPC methods.

**Acceptance Criteria**:
- [ ] Contract tests for `apps.*` methods
- [ ] Contract tests for `config.*` methods
- [ ] Contract tests for `pairing.*` methods
- [ ] Contract tests for `identity.*` methods
- [ ] Contract tests for `autonomi.*` methods
- [ ] Contract tests for `status.*` methods
- [ ] All tests must fail initially (TDD)
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/tests/contracts/apps_contract_test.rs`
- `core/osnova_lib/tests/contracts/config_contract_test.rs`
- `core/osnova_lib/tests/contracts/pairing_contract_test.rs`
- `core/osnova_lib/tests/contracts/identity_contract_test.rs`
- `core/osnova_lib/tests/contracts/autonomi_contract_test.rs`
- `core/osnova_lib/tests/contracts/status_contract_test.rs`

**Dependencies**: Task 101

---

### Task 105: Implement OpenRPC Discovery Endpoint
**Priority**: Low | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Implement `rpc.discover` method to return OpenRPC schema.

**Acceptance Criteria**:
- [ ] Implement `rpc.discover` method returning full schema
- [ ] Include all available methods with signatures
- [ ] Support schema versioning
- [ ] Write unit tests for discovery
- [ ] Document discovery endpoint

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/discovery.rs`
- `core/osnova_lib/tests/rpc/discovery_test.rs`

**Dependencies**: Task 101

---

### Task 106: Implement OpenRPC Method Documentation
**Priority**: Low | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Generate HTML documentation from OpenRPC schema.

**Acceptance Criteria**:
- [ ] Create `scripts/generate-rpc-docs.sh`
- [ ] Parse `openrpc.json` schema
- [ ] Generate HTML documentation with examples
- [ ] Include request/response examples
- [ ] Output to `docs/api/index.html`
- [ ] Host on GitHub Pages

**Files to Create/Modify**:
- `scripts/generate-rpc-docs.sh`
- `docs/api/index.html` (generated)

**Dependencies**: Task 101

---

### Task 107: Implement Request/Response Logging
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Implement structured logging for all RPC requests and responses.

**Acceptance Criteria**:
- [ ] Log all incoming RPC requests (method, params)
- [ ] Log all outgoing RPC responses (result, error)
- [ ] Redact sensitive data (passwords, keys)
- [ ] Include request ID for tracing
- [ ] Write unit tests for logging
- [ ] Document logging format

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/logging.rs`
- `core/osnova_lib/tests/rpc/logging_test.rs`

**Dependencies**: Task 093

---

### Task 108: Implement RPC Error Handling
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Standardize error responses across all RPC methods.

**Acceptance Criteria**:
- [ ] Define `RpcError` enum with error codes
- [ ] Map Rust errors to RPC error codes
- [ ] Include error messages and context
- [ ] Follow JSON-RPC 2.0 error format
- [ ] Write unit tests for error handling
- [ ] Document error codes

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/errors.rs`
- `core/osnova_lib/tests/rpc/errors_test.rs`
- `docs/06-protocols/error-codes.md`

**Dependencies**: None

---

### Task 109: Implement RPC Rate Limiting
**Priority**: Medium | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Implement rate limiting for RPC endpoints to prevent abuse.

**Acceptance Criteria**:
- [ ] Limit to 100 requests/second per client
- [ ] Return 429 Too Many Requests on limit exceeded
- [ ] Implement sliding window algorithm
- [ ] Per-client rate limits
- [ ] Write unit tests for rate limiting
- [ ] Document rate limits

**Files to Create/Modify**:
- `core/osnova_lib/src/rpc/rate_limiter.rs`
- `core/osnova_lib/tests/rpc/rate_limiter_test.rs`

**Dependencies**: Task 094

---

### Task 110: Integration Test - Full RPC Stack
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: No

**Description**: End-to-end test of complete RPC infrastructure.

**Acceptance Criteria**:
- [ ] Start RPC server
- [ ] Make client requests from TypeScript
- [ ] Test all available methods
- [ ] Test error handling
- [ ] Test rate limiting
- [ ] Test MCP client access
- [ ] Test completes in <10 seconds

**Files to Create/Modify**:
- `core/osnova_lib/tests/integration/rpc_stack_test.rs`

**Dependencies**: Tasks 101-109

---

## 6. App Management Backend (Tasks 111-118)

### Task 111: Implement Launcher Layout Persistence
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Persist launcher app layout per identity.

**Acceptance Criteria**:
- [ ] Implement `launcher.getLayout() -> Vec<String>` (app IDs in order)
- [ ] Implement `launcher.setLayout(layout: Vec<String>)`
- [ ] Store in encrypted store per identity
- [ ] Write contract tests for layout operations
- [ ] Test layout restoration on app restart
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/launcher.rs` (update)
- `core/osnova_lib/tests/services/launcher_test.rs` (update)

**Dependencies**: Task 087

---

### Task 112: Implement App Icon Management
**Priority**: Medium | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Download and cache app icons from Autonomi Network.

**Acceptance Criteria**:
- [ ] Download icon from `manifest.iconUri` (ant:// address)
- [ ] Cache icons in local cache directory
- [ ] Support image formats: PNG, JPEG, SVG
- [ ] Generate fallback icon if unavailable
- [ ] Implement `apps.getIcon(app_id: &str) -> Result<PathBuf>`
- [ ] Write unit tests for icon operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/icons.rs`
- `core/osnova_lib/tests/services/icons_test.rs`

**Dependencies**: Task 066

---

### Task 113: Implement App Metadata Management
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Store and retrieve app metadata (name, version, description).

**Acceptance Criteria**:
- [ ] Parse metadata from manifest
- [ ] Store in encrypted store
- [ ] Implement `apps.getMetadata(app_id: &str) -> Result<AppMetadata>`
- [ ] Include fields: id, name, version, description, author, iconUri
- [ ] Write unit tests for metadata operations
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/metadata.rs`
- `core/osnova_lib/tests/services/metadata_test.rs`

**Dependencies**: Task 087

---

### Task 114: Implement Launcher Manifest Swapping
**Priority**: Low | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Allow users to swap the launcher app itself.

**Acceptance Criteria**:
- [ ] Implement `config.getLauncherManifest() -> String` (current launcher URI)
- [ ] Implement `config.setLauncherManifest(uri: &str)`
- [ ] Validate launcher manifest before switching
- [ ] Restart launcher component after switch
- [ ] Write contract tests for launcher switching
- [ ] Coverage ≥85%

**Files to Create/Modify**:
- `core/osnova_lib/src/services/config.rs` (update)
- `core/osnova_lib/tests/services/config_test.rs` (update)

**Dependencies**: Task 097

---

### Task 115: Update Frontend - Real App Installation
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Connect frontend install dialog to real backend install operation.

**Acceptance Criteria**:
- [ ] Update `AppInstallDialog` to call `apps_install` Tauri command
- [ ] Show download progress indicator
- [ ] Handle installation errors gracefully
- [ ] Refresh app list after successful install
- [ ] Add toast notifications for install status
- [ ] Write E2E test for install flow

**Files to Create/Modify**:
- `app/src/lib/components/AppInstallDialog.svelte` (update)
- `app/src/lib/stores/apps.ts` (update)

**Dependencies**: Task 079

---

### Task 116: Update Frontend - Real App Uninstallation
**Priority**: High | **Estimated**: 1.5 days | **Parallelizable**: No

**Description**: Connect frontend uninstall dialog to real backend uninstall operation.

**Acceptance Criteria**:
- [ ] Update `AppUninstallDialog` to call `apps_uninstall` Tauri command
- [ ] Show uninstall progress
- [ ] Cleanup cached components after uninstall
- [ ] Refresh app list after uninstall
- [ ] Add confirmation toast
- [ ] Write E2E test for uninstall flow

**Files to Create/Modify**:
- `app/src/lib/components/AppUninstallDialog.svelte` (update)
- `app/src/lib/stores/apps.ts` (update)

**Dependencies**: Task 079

---

### Task 117: Update Frontend - Real App Icons
**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Display real app icons downloaded from Autonomi Network.

**Acceptance Criteria**:
- [ ] Load icon from cache via `apps.getIcon`
- [ ] Display icon in `AppIcon` component
- [ ] Show fallback icon if download fails
- [ ] Add loading skeleton during icon fetch
- [ ] Write E2E test for icon display

**Files to Create/Modify**:
- `app/src/lib/components/AppIcon.svelte` (update)
- `app/src-tauri/src/lib.rs` (add `get_icon` command)

**Dependencies**: Task 112

---

### Task 118: Integration Test - App Management E2E
**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Full E2E test of app installation, display, and uninstallation.

**Acceptance Criteria**:
- [ ] Install app from manifest URI
- [ ] Verify app appears in launcher
- [ ] Verify icon displays correctly
- [ ] Reorder apps and verify layout persistence
- [ ] Uninstall app and verify removal
- [ ] Test completes in <10 seconds

**Files to Create/Modify**:
- `core/osnova_lib/tests/integration/app_management_test.rs`

**Dependencies**: Tasks 111-117

---

## 7. Production Deployment (Tasks 119-125)

### Task 119: Configure Production Build Pipeline
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Set up CI/CD pipeline for building production releases.

**Acceptance Criteria**:
- [ ] GitHub Actions workflow for releases
- [ ] Build for all platforms: Windows, macOS, Linux, Android, iOS
- [ ] Sign binaries with code signing certificates
- [ ] Generate installers (MSI, DMG, AppImage, APK, IPA)
- [ ] Run all tests before release
- [ ] Upload artifacts to GitHub Releases
- [ ] Document release process

**Files to Create/Modify**:
- `.github/workflows/release.yml`
- `scripts/build-release.sh`
- `docs/10-development/release-process.md`

**Dependencies**: None

---

### Task 120: Implement Auto-Update System
**Priority**: Medium | **Estimated**: 3 days | **Parallelizable**: No

**Description**: Implement auto-update functionality using Tauri updater.

**Acceptance Criteria**:
- [ ] Enable Tauri updater in config
- [ ] Check for updates on app start
- [ ] Download and verify updates
- [ ] Prompt user to restart for update
- [ ] Support delta updates
- [ ] Write integration tests for updater
- [ ] Document update process

**Files to Create/Modify**:
- `app/src-tauri/tauri.conf.json` (update)
- `app/src/lib/services/updater.ts`
- `app/src/lib/components/UpdateDialog.svelte`

**Dependencies**: None

---

### Task 121: Implement Crash Reporting
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement crash reporting for production debugging.

**Acceptance Criteria**:
- [ ] Integrate crash reporting library (sentry-rust)
- [ ] Capture panic backtraces
- [ ] Send crash reports to Sentry (opt-in)
- [ ] Redact sensitive data from reports
- [ ] Write unit tests for crash handling
- [ ] Document privacy policy

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml` (add sentry dependency)
- `core/osnova_lib/src/crash_reporting.rs`
- `docs/privacy-policy.md`

**Dependencies**: None

---

### Task 122: Implement Analytics (Opt-in)
**Priority**: Low | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Implement opt-in analytics for usage statistics.

**Acceptance Criteria**:
- [ ] Track app launches, feature usage
- [ ] Opt-in during onboarding
- [ ] Anonymous usage data (no PII)
- [ ] Send to analytics backend (PostHog or similar)
- [ ] Allow users to disable in settings
- [ ] Document data collection policy

**Files to Create/Modify**:
- `core/osnova_lib/src/analytics.rs`
- `app/src/lib/services/analytics.ts`
- `docs/privacy-policy.md` (update)

**Dependencies**: None

---

### Task 123: Create Installation Documentation
**Priority**: High | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Write comprehensive installation guide for all platforms.

**Acceptance Criteria**:
- [ ] Installation instructions for Windows
- [ ] Installation instructions for macOS
- [ ] Installation instructions for Linux (multiple distros)
- [ ] Installation instructions for Android
- [ ] Installation instructions for iOS (TestFlight)
- [ ] Include screenshots and troubleshooting

**Files to Create/Modify**:
- `docs/installation/windows.md`
- `docs/installation/macos.md`
- `docs/installation/linux.md`
- `docs/installation/android.md`
- `docs/installation/ios.md`
- `docs/installation/README.md`

**Dependencies**: None

---

### Task 124: Create User Guide
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Write user-facing documentation for end users.

**Acceptance Criteria**:
- [ ] Getting started guide
- [ ] Identity management guide
- [ ] Installing and managing apps
- [ ] Connecting to a server
- [ ] Troubleshooting common issues
- [ ] FAQ section
- [ ] Include screenshots and videos

**Files to Create/Modify**:
- `docs/user-guide/getting-started.md`
- `docs/user-guide/identity.md`
- `docs/user-guide/apps.md`
- `docs/user-guide/server-mode.md`
- `docs/user-guide/faq.md`

**Dependencies**: None

---

### Task 125: Create Developer Guide
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Write documentation for third-party developers building Osnova apps.

**Acceptance Criteria**:
- [ ] Component development guide
- [ ] Frontend component tutorial
- [ ] Backend component tutorial
- [ ] OpenRPC API reference
- [ ] Example apps with source code
- [ ] Publishing guide (manifests, Autonomi upload)

**Files to Create/Modify**:
- `docs/developer-guide/overview.md`
- `docs/developer-guide/frontend-components.md`
- `docs/developer-guide/backend-components.md`
- `docs/developer-guide/openrpc-api.md`
- `docs/developer-guide/publishing.md`

**Dependencies**: None

---

## 8. Performance & Optimization (Tasks 126-130)

### Task 126: Implement Performance Monitoring
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Add performance metrics for critical operations.

**Acceptance Criteria**:
- [ ] Track app launch time (target: p95 ≤ 2s)
- [ ] Track RPC call latency (target: p95 ≤ 100ms local, ≤ 5s remote)
- [ ] Track component download time
- [ ] Log performance metrics
- [ ] Expose metrics via status endpoint
- [ ] Write unit tests for metrics collection

**Files to Create/Modify**:
- `core/osnova_lib/src/metrics/mod.rs`
- `core/osnova_lib/src/metrics/collector.rs`
- `core/osnova_lib/tests/metrics/collector_test.rs`

**Dependencies**: Task 092

---

### Task 127: Implement Performance Tests
**Priority**: High | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Write automated performance tests for critical paths.

**Acceptance Criteria**:
- [ ] Benchmark app launch time
- [ ] Benchmark RPC call latency
- [ ] Benchmark component download/install
- [ ] Benchmark identity operations
- [ ] Benchmark encrypted storage operations
- [ ] All benchmarks must pass p95 targets
- [ ] Document benchmark results

**Files to Create/Modify**:
- `core/osnova_lib/benches/app_launch.rs`
- `core/osnova_lib/benches/rpc_latency.rs`
- `core/osnova_lib/benches/component_install.rs`
- `core/osnova_lib/benches/README.md`

**Dependencies**: Task 126

---

### Task 128: Optimize Binary Size
**Priority**: Medium | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Reduce application binary size for faster downloads.

**Acceptance Criteria**:
- [ ] Enable LTO (Link-Time Optimization)
- [ ] Strip debug symbols in release builds
- [ ] Use `opt-level = "z"` for size optimization
- [ ] Compress binary with UPX (optional)
- [ ] Measure and document binary sizes
- [ ] Target: <10MB desktop, <5MB mobile

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml` (update profile)
- `app/src-tauri/Cargo.toml` (update profile)
- `docs/10-development/optimization.md`

**Dependencies**: None

---

### Task 129: Implement Lazy Loading for Components
**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: No

**Description**: Load components on-demand rather than at startup.

**Acceptance Criteria**:
- [ ] Defer component loading until first use
- [ ] Show loading indicator during component load
- [ ] Cache loaded components in memory
- [ ] Unload unused components after timeout
- [ ] Write unit tests for lazy loading
- [ ] Measure startup time improvement

**Files to Create/Modify**:
- `core/osnova_lib/src/components/lazy_loader.rs`
- `core/osnova_lib/tests/components/lazy_loader_test.rs`

**Dependencies**: Task 077

---

### Task 130: Implement Memory Profiling
**Priority**: Low | **Estimated**: 1.5 days | **Parallelizable**: Yes

**Description**: Add memory profiling for detecting leaks and optimizing usage.

**Acceptance Criteria**:
- [ ] Integrate memory profiling (jemalloc)
- [ ] Track heap allocations
- [ ] Generate memory profiles
- [ ] Identify and fix memory leaks
- [ ] Document memory usage patterns
- [ ] Write tests for memory leak detection

**Files to Create/Modify**:
- `core/osnova_lib/Cargo.toml` (add jemalloc)
- `core/osnova_lib/src/profiling/memory.rs`
- `docs/10-development/profiling.md`

**Dependencies**: None

---

## Summary

**Total Tasks**: 67 (Tasks 064-130)
**Estimated Duration**: 4-6 weeks with parallel execution
**Critical Path**: Autonomi Integration → Component System → Identity → Server Mode → Production

### Phase Breakdown:
- **Week 1-2**: Autonomi Network Integration + Component Packaging
- **Week 3**: Identity & Key Management
- **Week 4**: Server Mode & Client-Server Pairing
- **Week 5**: OpenRPC Infrastructure + App Management Backend
- **Week 6**: Production Deployment + Performance Optimization

### Parallelization Opportunities:
- Tasks 067, 068, 071, 073, 074, 089, 091-093, 101-109, 112-114, 119-125, 126-130 can run in parallel
- Multi-agent workflow can execute independent tasks concurrently

### Dependencies:
- Most tasks depend on core infrastructure (Tasks 064-070)
- Server mode tasks depend on OpenRPC server (Task 094)
- Frontend updates depend on backend implementations

### Success Criteria:
- ✅ All 67 tasks complete
- ✅ ≥85% test coverage maintained
- ✅ All performance targets met (p95 ≤ 2s launch, ≤ 5s RPC)
- ✅ Production releases for all platforms
- ✅ Complete documentation (user + developer guides)

---

**Last Updated**: 2025-10-07
**Status**: Ready for implementation
**Next Step**: Begin Task 064 (Autonomi SDK Integration)
