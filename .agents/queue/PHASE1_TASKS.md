# Phase 1: Data Models + osnova-core

## Overview
Phase 1 focuses on implementing the foundational data models and the osnova-core service with identity, key management, configuration, and storage operations.

## Execution Strategy
- **Agents**: Backend Core Agent + Rust Testing Agent
- **Worktree**: `/home/system/osnova_claude-backend/`
- **Estimated Tasks**: 28 tasks
- **Estimated Duration**: 3-5 days with parallel execution
- **Estimated Agent Invocations**: ~40-50

## Task List

### Group 1: Project Setup (Sequential)
**Dependencies**: None

#### Task 001: Initialize Rust Project Structure
- **Type**: backend-setup
- **Agent**: backend-core
- **Priority**: P0
- **Description**: Create Cargo workspace with osnova_lib crate
- **Deliverables**:
  - `Cargo.toml` workspace configuration
  - `core/osnova_lib/Cargo.toml` with dependencies
  - `core/osnova_lib/src/lib.rs` skeleton
  - README with project structure
- **Dependencies**: []
- **Context**: `CLAUDE.md`, `docs/02-architecture/overview.md`

#### Task 002: Add Project Dependencies
- **Type**: backend-setup
- **Agent**: backend-core
- **Priority**: P0
- **Description**: Add all required Rust dependencies to Cargo.toml
- **Deliverables**:
  - autonomi v0.6.1
  - saorsa-core (main branch)
  - cocoon v0.4.3
  - serde, tokio, anyhow, thiserror
  - blake3, hkdf, sha2
  - rusqlite, flate2
- **Dependencies**: [001]
- **Context**: `CLAUDE.md`, `docs/10-development/plan.md`

---

### Group 2: Data Models (Parallel After Group 1)
**Dependencies**: Task 002

#### Task 003: Implement RootIdentity Model
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P1
- **Description**: Implement RootIdentity struct with seed phrase handling
- **Deliverables**:
  - `src/models/identity.rs`
  - RootIdentity struct with fields
  - from_seed() method
  - Tests (≥85% coverage)
  - Documentation with examples
- **Dependencies**: [002]
- **Context**: `docs/02-architecture/data-model.md`, `docs/07-security/identity.md`

#### Task 004: Test RootIdentity Model
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P1
- **Description**: Validate RootIdentity implementation
- **Dependencies**: [003]
- **Feedback Target**: Task 003

#### Task 005: Implement DeviceKey Model
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P1
- **Description**: Implement DeviceKey struct for device management
- **Deliverables**:
  - `src/models/device.rs`
  - DeviceKey struct
  - Key generation and revocation logic
  - Tests
  - Documentation
- **Dependencies**: [002]
- **Context**: `docs/02-architecture/data-model.md`

#### Task 006: Test DeviceKey Model
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P1
- **Dependencies**: [005]
- **Feedback Target**: Task 005

#### Task 007: Implement OsnovaApplication Model
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P1
- **Description**: Implement OsnovaApplication and ComponentRef structs
- **Deliverables**:
  - `src/models/application.rs`
  - OsnovaApplication struct
  - ComponentRef struct
  - Validation logic
  - Tests
  - Documentation
- **Dependencies**: [002]
- **Context**: `docs/02-architecture/data-model.md`, `docs/06-protocols/manifest-schema.md`

#### Task 008: Test OsnovaApplication Model
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P1
- **Dependencies**: [007]
- **Feedback Target**: Task 007

#### Task 009: Implement AppConfiguration and AppCache Models
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P1
- **Description**: Implement app configuration and cache management models
- **Deliverables**:
  - `src/models/app_data.rs`
  - AppConfiguration struct
  - AppCache struct
  - Encryption/decryption logic
  - Tests
  - Documentation
- **Dependencies**: [002]
- **Context**: `docs/02-architecture/data-model.md`

#### Task 010: Test AppConfiguration and AppCache
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P1
- **Dependencies**: [009]
- **Feedback Target**: Task 009

#### Task 011: Implement PairingSession Model
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P1
- **Description**: Implement pairing session state management
- **Deliverables**:
  - `src/models/pairing.rs`
  - PairingSession struct
  - State transitions
  - Tests
  - Documentation
- **Dependencies**: [002]
- **Context**: `docs/02-architecture/data-model.md`, `docs/08-networking/pairing.md`

#### Task 012: Test PairingSession Model
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P1
- **Dependencies**: [011]
- **Feedback Target**: Task 011

---

### Group 3: Key Management (Sequential After Group 2)
**Dependencies**: Tasks 003-012 completed

#### Task 013: Implement Key Derivation
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement HKDF-based key derivation functions
- **Deliverables**:
  - `src/crypto/keys.rs`
  - derive_key_at_index() function
  - derive_master_key_from_seed() function
  - Component-isolated derivation
  - Tests (including cross-component isolation)
  - Documentation with examples
- **Dependencies**: [003, 004]
- **Context**: `docs/07-security/keys.md`, `docs/03-core-services/osnova-core.md`

#### Task 014: Test Key Derivation
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [013]
- **Feedback Target**: Task 013

#### Task 015: Implement Encryption-at-Rest
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement cocoon-based encryption for data at rest
- **Deliverables**:
  - `src/crypto/encryption.rs`
  - encrypt_data() function
  - decrypt_data() function
  - Cocoon integration
  - Tests
  - Documentation
- **Dependencies**: [013, 014]
- **Context**: `docs/07-security/encryption.md`, `docs/07-security/cocoon-unlock.md`

#### Task 016: Test Encryption-at-Rest
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [015]
- **Feedback Target**: Task 015

---

### Group 4: Storage Layer (Sequential After Group 3)
**Dependencies**: Tasks 013-016 completed

#### Task 017: Implement SQLite Storage Backend
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement SQLite storage for structured data
- **Deliverables**:
  - `src/storage/sql.rs`
  - Database initialization
  - CRUD operations for apps, configs
  - Encrypted blob storage
  - Tests
  - Documentation
- **Dependencies**: [015, 016]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 018: Test SQLite Storage
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [017]
- **Feedback Target**: Task 017

#### Task 019: Implement File-based Storage
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement encrypted file storage for keys and cache
- **Deliverables**:
  - `src/storage/file.rs`
  - File read/write with encryption
  - Cache management
  - Tests
  - Documentation
- **Dependencies**: [015, 016]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 020: Test File-based Storage
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [019]
- **Feedback Target**: Task 019

---

### Group 5: Identity Service (Sequential After Group 4)
**Dependencies**: Tasks 017-020 completed

#### Task 021: Implement Identity Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement identity management service
- **Deliverables**:
  - `src/services/identity.rs`
  - IdentityService struct
  - status(), create(), importWithPhrase() methods
  - saorsa-core integration
  - Tests
  - Documentation
- **Dependencies**: [003, 004, 017, 018]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/07-security/identity.md`

#### Task 022: Test Identity Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [021]
- **Feedback Target**: Task 021

#### Task 023: Implement Key Management Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement key management service with OpenRPC interface
- **Deliverables**:
  - `src/services/keys.rs`
  - KeyService struct
  - derive(), deriveAtIndex(), getByPublicKey(), listForComponent()
  - Tests
  - Documentation
- **Dependencies**: [013, 014, 017, 018]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/07-security/keys.md`

#### Task 024: Test Key Management Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [023]
- **Feedback Target**: Task 023

---

### Group 6: Configuration and Storage Services (Parallel After Group 5)
**Dependencies**: Tasks 021-024 completed

#### Task 025: Implement Configuration Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement configuration management service
- **Deliverables**:
  - `src/services/config.rs`
  - ConfigService struct
  - getAppConfig(), setAppConfig(), clearAppCache()
  - setServer(), getLauncherManifest(), setLauncherManifest()
  - Tests
  - Documentation
- **Dependencies**: [017, 018, 019, 020]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 026: Test Configuration Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [025]
- **Feedback Target**: Task 025

#### Task 027: Implement Storage Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement storage operations service
- **Deliverables**:
  - `src/services/storage.rs`
  - StorageService struct
  - read(), write(), delete() methods
  - Encryption integration
  - Tests
  - Documentation
- **Dependencies**: [017, 018, 019, 020]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 028: Test Storage Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [027]
- **Feedback Target**: Task 027

---

## Dependency Graph

```
001 (Project Setup)
 ├─> 002 (Dependencies)
      ├─> 003 (RootIdentity) ─> 004 (Test)
      ├─> 005 (DeviceKey) ─> 006 (Test)
      ├─> 007 (OsnovaApp) ─> 008 (Test)
      ├─> 009 (AppData) ─> 010 (Test)
      └─> 011 (Pairing) ─> 012 (Test)
           └─> 013 (Key Derivation) ─> 014 (Test)
                └─> 015 (Encryption) ─> 016 (Test)
                     ├─> 017 (SQLite) ─> 018 (Test)
                     └─> 019 (File Storage) ─> 020 (Test)
                          ├─> 021 (Identity Service) ─> 022 (Test)
                          ├─> 023 (Key Service) ─> 024 (Test)
                          ├─> 025 (Config Service) ─> 026 (Test)
                          └─> 027 (Storage Service) ─> 028 (Test)
```

## Parallel Execution Opportunities

**Wave 1** (After 002):
- Tasks 003, 005, 007, 009, 011 (all data models in parallel)

**Wave 2** (After data model tests pass):
- Task 013 (key derivation)

**Wave 3** (After 016):
- Tasks 017, 019 (storage backends in parallel)

**Wave 4** (After 020):
- Tasks 021, 023 (services in parallel)

**Wave 5** (After 024):
- Tasks 025, 027 (config and storage services in parallel)

## Success Criteria

### Phase 1 Complete When:
- ✅ All 28 tasks completed
- ✅ All tests passing
- ✅ Overall coverage ≥85%
- ✅ No clippy warnings
- ✅ All public items documented
- ✅ Data models validated
- ✅ Core services functional
- ✅ Storage layer operational
- ✅ Identity and key management working

### Deliverables:
- Complete osnova-core library
- ~15-20 Rust source files
- ~120-150 tests
- Comprehensive documentation
- Ready for Phase 2 (frontend integration)

## Estimated Timeline

**With sequential execution**: ~8-10 days
**With parallel execution (multi-agent)**: ~3-5 days

**Agent invocation breakdown**:
- Backend Core Agent: ~20-25 invocations
- Rust Testing Agent: ~20-25 invocations
- **Total**: ~40-50 invocations

## Next Phase

After Phase 1 completion, proceed to **Phase 2**:
- Frontend implementation (Launcher, Configuration, Deployment screens)
- E2E testing with Playwright MCP
- Frontend-backend integration

---

**Status**: Ready for Orchestrator execution
**Generated**: 2025-10-06
