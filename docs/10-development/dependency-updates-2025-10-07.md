# Dependency Documentation Updates - October 7, 2025

This document summarizes the latest information about Osnova's key dependencies and identifies necessary documentation updates.

## Key Dependencies - Latest Information

### 1. Autonomi Network v0.6.1

**Official Docs**: https://github.com/maidsafe/autonomi

**Key Changes**:
- Crate name: `autonomi` (confirmed)
- Current stable version available on crates.io
- Main API: `Client`, `Wallet`, `Bytes`
- Supports both public and private data storage
- Directory upload/download functionality
- EVM integration for payments

**API Example**:
```rust
use autonomi::{Bytes, Client, Wallet};

let client = Client::init().await?;
let wallet = Wallet::new_from_private_key(Default::default(), key)?;

// Put and fetch data
let data_addr = client
    .data_put_public(Bytes::from("Hello, World"), (&wallet).into())
    .await?;
let data_fetched = client.data_get_public(&data_addr).await?;

// Directory operations
let dir_addr = client.dir_upload_public("files/to/upload".into(), &wallet).await?;
client.dir_download_public(dir_addr, "files/downloaded".into()).await?;
```

**EVM Networks Supported**:
- Arbitrum One (default/mainnet)
- Arbitrum Sepolia (testnet)
- Custom networks (local testnet)

**Payment Model**:
- EVM-based payments using payment tokens
- Wallet initialization required for write operations
- Read operations free

**Documentation Status**: ✅ Accurate in current specs

---

### 2. saorsa-core (P2P Library)

**Official Repo**: https://github.com/dirvine/p2p

**Key Features**:
- Adaptive machine learning for routing
- Quantum-resistant security (ML-KEM-768 Kyber, ML-DSA-65 Dilithium)
- Sub-millisecond network lookups
- Four-word human-memorable addresses (e.g., "river-spark-honest-lion")
- Multi-Armed Bandit Routing with LSTM networks
- Model Context Protocol for AI integration

**API Example**:
```rust
let node = P2PNode::builder()
    .with_address("forest-ocean-mountain-sky")  // 4 words
    .enable_machine_learning()
    .build()
    .await?;
```

**Documentation Status**: ✅ Accurate - 4-word addresses confirmed

---

### 3. Tauri 2.0

**Official Docs**: https://v2.tauri.app/

**Key Features**:
- Cross-platform: Linux, macOS, Windows, Android, iOS
- Frontend: Any web framework (HTML/CSS/JS)
- Backend: Rust (with Swift/Kotlin integration)
- WebView-based UI
- Extensive JavaScript API

**When to Use Tauri**:
- Single UI codebase for all platforms
- Reach maximum users across platforms
- Web developers wanting native apps
- Rust developers wanting nice UIs

**Documentation Status**: ✅ Accurate in current specs

---

### 4. Svelte 5

**Official Docs**: https://svelte.dev/docs/svelte/what-are-runes

**Key Innovation - Runes**:
Runes are new in Svelte 5 - symbols prefixed with `$` that control the Svelte compiler:

```javascript
let message = $state('hello');  // Reactive state
```

**Key Characteristics**:
- Part of Svelte language syntax (keywords)
- Not imported - built into language
- Not values - can't assign to variables
- Only valid in specific positions
- Replace Svelte 3/4 reactive patterns

**Legacy Mode vs Runes Mode**:
- Svelte 5 supports both modes
- Once a component uses runes, legacy features unavailable
- Migration path available

**Documentation Status**: ⚠️ **SHOULD UPDATE** - Current docs don't mention Svelte 5 runes, still reference Svelte 4.x patterns

---

### 5. Cocoon v0.4

**Official Docs**: https://docs.rs/cocoon/latest/cocoon/

**Two Main Types**:

#### MiniCocoon
- Lightweight encryption container
- Smaller header
- Sequential data encryption
- Simple tasks
- In-memory secure containers

#### Cocoon
- Robust encryption container
- Long-term data storage
- File encryption
- Password/key stores

**Features**:
- 256-bit cryptography
- Cipher options: Chacha20-Poly1305 or AES256-GCM
- PBKDF2-SHA256 for key derivation
- Automatic key zeroization
- Methods: `wrap/unwrap`, `dump/parse`, `encrypt/decrypt`

**API Example**:
```rust
let mut cocoon = MiniCocoon::from_key(
    b"0123456789abcdef0123456789abcdef",
    &[0; 32]
);
let wrapped = cocoon.wrap(b"my secret data")?;
let unwrapped = cocoon.unwrap(&wrapped)?;
```

**Documentation Status**: ✅ Accurate - cocoon is used alongside saorsa-seal

---

### 6. saorsa-pqc

**Official Repo**: https://github.com/dirvine/saorsa-pqc

**Purpose**: Post-quantum cryptography library implementing NIST FIPS 203, 204, and 205 standards

**Supported Algorithms**:
- **ML-KEM** (Key Encapsulation): 512, 768, 1024 variants
- **ML-DSA** (Digital Signatures): 44, 65, 87 variants
- **SLH-DSA** (Stateless Hash-Based Signatures)

**Cryptographic Primitives**:
- Hash Functions: BLAKE3, SHA3-256/512
- Key Derivation: HKDF-SHA3
- Message Authentication: HMAC-SHA3
- Authenticated Encryption: AES-256-GCM, ChaCha20-Poly1305
- Hybrid Public Key Encryption (HPKE)

**Key Features**:
- FIPS 140-3 compliant
- Memory safe
- Constant-time operations
- Extensive test vector validation
- High-performance implementations
- Trait-based API for type safety

**Use in Osnova**: Post-quantum cryptographic operations, future-proof encryption

**Documentation Status**: ⚠️ **NEEDS TO BE ADDED** - Not currently in dependency list

---

### 7. saorsa-fec

**Official Repo**: https://github.com/dirvine/saorsa-fec

**Purpose**: Forward Error Correction with advanced encryption and storage capabilities

**Key Features**:
- Three encryption modes: Convergent, ConvergentWithSecret, RandomKey
- AES-256-GCM authenticated encryption
- High-performance Reed-Solomon error correction
- Multiple storage backends (LocalStorage, MemoryStorage, MultiStorage)
- SIMD acceleration (AVX2, AVX, SSE4.1, NEON)

**Performance**:
- 1,000-7,500 MB/s with SIMD acceleration
- Scales with file size and CPU capabilities
- Async (Tokio) pipeline processing

**Security Features**:
- Zeroizes cryptographic keys in memory
- ConvergentWithSecret recommended for user-private data
- Different deduplication strategies

**Use in Osnova**: Distributed storage, error-resilient data transmission, component distribution

**Documentation Status**: ⚠️ **NEEDS TO BE ADDED** - Not currently in dependency list

---

### 8. saorsa-seal

**Official Repo**: https://github.com/dirvine/saorsa-seal

**Purpose**: Secure distributed data storage using threshold cryptography

**Key Features**:
- Threshold Sealing: Shamir's Secret Sharing with configurable thresholds
- Forward Error Correction: Reed-Solomon coding for recovery
- Post-Quantum Cryptography: ML-KEM-768 encryption
- Distributed Storage: DHT (Distributed Hash Table) abstraction
- Async-friendly API

**Capabilities**:
- Generate data shares across multiple recipients
- Set recovery thresholds (e.g., 3-of-5)
- Quantum-resistant encryption
- Recover data with minimum required shares

**API Example**:
```rust
// Seal data with 5 shares, 3 required for recovery
let shares = seal_data(&data, 5, 3, true).await?;

// Recover data with 3+ shares
let recovered = open_data(&shares[0..3]).await?;
```

**Use in Osnova**: User data encryption, distributed backup, threshold encryption

**Documentation Status**: ✅ **CONFIRMED EXISTS** - Currently referenced in docs, accurate

---

## Issues Found

### Issue #1: Missing Saorsa Dependencies ✅ RESOLVED

**Problem**: Documentation didn't include three critical saorsa libraries: saorsa-pqc, saorsa-fec, and saorsa-seal.

**Impact**: MEDIUM - Need to add these to dependency lists and update documentation

**Resolution**: All three libraries confirmed and documented above

**Required Actions**:
1. Add saorsa-pqc to CLAUDE.md dependency list
2. Add saorsa-fec to CLAUDE.md dependency list
3. Add saorsa-seal to CLAUDE.md dependency list
4. Update backend-core.md agent spec with saorsa library usage patterns
5. Document integration points in relevant specification files

---

### Issue #2: Svelte 5 Runes

**Problem**: Documentation doesn't mention Svelte 5's new runes system, which is a fundamental change in how reactive state works.

**Impact**: MEDIUM - Affects frontend implementation patterns but not architecture

**Required Updates**:
- Document runes-based reactive state patterns
- Update component examples to use `$state`, `$derived`, `$effect`
- Note migration from Svelte 4 → Svelte 5
- Update frontend agent specifications

---

## Recommended Actions

### Immediate (Before Phase 1)

1. **Fix 3-word addresses** (HIGH PRIORITY)
   - Update all 13 files referencing "four-word" addresses
   - Update example addresses throughout
   - Verify UI wireframes accommodate 3 words

2. **Clarify encryption library** (MEDIUM PRIORITY)
   - Decide: Use cocoon directly or create saorsa-seal wrapper?
   - Update all 11 files accordingly
   - Update CLAUDE.md and agent specs

3. **Document Svelte 5 runes** (MEDIUM PRIORITY)
   - Add runes patterns to frontend.md agent spec
   - Update component examples
   - Note Svelte 5 migration path

### Before Implementation

4. **Verify Autonomi API**
   - Current docs are accurate
   - Confirm v0.6.1 API stability
   - Test directory upload/download

5. **Verify Tauri 2.0**
   - Current docs are accurate
   - Test mobile support requirements
   - Verify IPC patterns

### Post-MVP

6. **Monitor dependency updates**
   - Set up automated checks for new versions
   - Document breaking changes
   - Plan migration strategies

---

## Summary

**✅ Accurate**:
- Autonomi v0.6.1
- Tauri 2.0
- saorsa-core (4-word addresses confirmed)
- Cocoon v0.4
- saorsa-seal (confirmed exists)

**⚠️ Needs Addition**:
- saorsa-pqc (post-quantum crypto)
- saorsa-fec (forward error correction)

**⚠️ Needs Documentation Update**:
- Svelte 5 runes (missing from frontend docs)

**Next Steps**:
1. Add saorsa-pqc, saorsa-fec, saorsa-seal to CLAUDE.md dependency list
2. Add Svelte 5 runes documentation to frontend.md
3. Update backend-core.md with saorsa library patterns
4. Verify all changes before Phase 1 launch
