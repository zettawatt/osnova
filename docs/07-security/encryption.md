# Encryption at Rest

Osnova implements comprehensive encryption-at-rest to protect user data on both stand-alone devices and servers.

## Encryption Architecture

All sensitive user data is encrypted at rest using saorsa-seal, which provides:
- **Strong encryption**: ChaCha20-Poly1305 AEAD
- **Key derivation**: From master key via HKDF
- **Per-user encryption**: Unique keys per user
- **Per-component isolation**: Separate keys per component

## What Gets Encrypted

### Stand-Alone Mode
- User configuration (per-app settings)
- Application cache
- Identity metadata
- Private keys (in platform keystore)
- Downloaded component metadata
- Transaction history
- User profile data

### Client-Server Mode
- All stand-alone data (on client)
- Server-side user data (encrypted with user keys)
- Per-client isolated data stores
- Backup and sync data

## Encryption Implementation

### saorsa-seal Integration

Osnova uses saorsa-seal for encryption operations:

```rust
// Encrypt data
let encrypted = saorsa_seal::encrypt(
    &data,
    &encryption_key,
    &nonce
)?;

// Decrypt data
let decrypted = saorsa_seal::decrypt(
    &encrypted,
    &encryption_key,
    &nonce
)?;
```

### Key Derivation

Each component gets unique encryption keys:

```
Master Key (from 12-word seed)
    ↓
HKDF-SHA256(master_key, component_id, "encryption")
    ↓
Component Encryption Key
```

### Nonce Management
- Random nonce generated per encryption operation
- Stored alongside ciphertext
- Never reused
- 192-bit (24 bytes) for XChaCha20-Poly1305

## Storage Layout

### Encrypted File Format
```
[Version: 1 byte]
[Nonce: 24 bytes]
[Ciphertext: variable]
[Auth Tag: 16 bytes]
```

### Directory Structure
```
~/.osnova/
  identity/
    identity.enc          # Encrypted identity metadata
  keys/
    [stored in platform keystore, not files]
  data/
    config/
      app-{id}.enc        # Encrypted per-app config
    cache/
      app-{id}/           # Encrypted cache data
      *.enc
  components/
    frontend/
      [cached, unencrypted - read-only]
    backend/
      [cached, unencrypted - read-only]
```

## Client-Server Mode Encryption

### End-to-End Encryption

User data encrypted on client before transmission:

```
Client:
  Data → Encrypt (user key) → Ciphertext
      ↓
  Network (encrypted channel via saorsa-core)
      ↓
Server:
  Ciphertext → Store encrypted
```

Server **cannot** decrypt user data because:
- User keys derived from 12-word seed phrase
- Seed phrase never leaves client device
- Master key never transmitted
- Server only stores ciphertext

### What Server Sees

**Encrypted** (server cannot access):
- User configuration data
- Application cache
- Private messages
- Transaction history
- Personal settings

**Plaintext** (server operational need):
- Routing metadata (for message delivery)
- Connection timestamps
- Storage quotas and usage
- Component version references

## Key Management

### Key Lifecycle

1. **Derivation**:
   - Master key from 12-word seed
   - Component keys via HKDF

2. **Storage**:
   - Master key in platform secure keystore
   - Component keys cached in memory (when in use)
   - Never stored in plaintext files

3. **Usage**:
   - Loaded on-demand
   - Cleared from memory after use
   - Re-derived when needed

4. **Rotation** (post-MVP):
   - Generate new key
   - Re-encrypt all data
   - Update key reference

### Platform-Specific Key Storage

#### Windows
- Windows Credential Manager via DPAPI
- User-scoped credentials
- Encrypted with user Windows password

#### macOS
- Keychain Services API
- Secured by user keychain password
- Optional Touch ID/Face ID unlock

#### Linux
- Secret Service API
- GNOME Keyring or KWallet
- Encrypted with user session

#### Android
- Android Keystore System
- Hardware-backed when available
- Biometric unlock support

#### iOS
- iOS Keychain Services
- Secure Enclave when available
- Touch ID/Face ID support

## Security Properties

### Confidentiality
- Data encrypted with strong algorithms (ChaCha20-Poly1305)
- Keys derived from user secret (12-word seed)
- Server cannot decrypt user data

### Integrity
- AEAD provides authentication
- Tampering detected via auth tag
- Version field prevents rollback

### Availability
- Users control their keys
- Data restorable with seed phrase
- No vendor lock-in

## Threat Model

### Protected Against
- ✅ Server compromise (encrypted user data)
- ✅ Stolen database (no plaintext data)
- ✅ Man-in-the-middle (encrypted channel + E2E)
- ✅ Malicious server operator (cannot decrypt)
- ✅ Lost device (encrypted local storage)

### Not Protected Against
- ❌ Compromised client device (keys accessible)
- ❌ Lost 12-word seed phrase (data unrecoverable)
- ❌ Keylogger on client (can capture seed phrase)
- ❌ Physical access with unlocked device

## Performance Considerations

### Encryption Overhead
- ChaCha20-Poly1305 is fast (software)
- Minimal impact on modern devices
- Cached decrypted data in memory when safe

### Optimization Strategies
- Lazy loading (decrypt on access)
- In-memory caching (when appropriate)
- Batch operations (reduce encrypt/decrypt calls)
- Stream processing for large files

## Best Practices

### For Users
- Keep 12-word seed phrase secure and backed up
- Use strong device passwords
- Enable device encryption
- Log out when device unattended

### For Developers
- Encrypt all sensitive data
- Never log plaintext sensitive data
- Clear sensitive data from memory
- Use constant-time comparisons for secrets
- Validate ciphertext before decryption

## Compliance and Standards

### Cryptographic Standards
- **Encryption**: ChaCha20-Poly1305 (RFC 8439)
- **Key Derivation**: HKDF-SHA256 (RFC 5869)
- **Seed Phrases**: BIP-39 (Bitcoin Improvement Proposal)

### Security Best Practices
- OWASP guidelines
- NIST recommendations
- Platform security guidelines (iOS, Android, etc.)

## Error Handling

### Decryption Failures
- Clear error messages (without exposing keys)
- Suggest re-entering seed phrase
- Offer data recovery options

### Key Access Failures
- Check platform keystore availability
- Provide fallback authentication
- Log technical details securely

## Monitoring and Auditing

### What to Log
- Encryption/decryption operations (count)
- Key derivation requests
- Authentication failures
- Performance metrics

### What NOT to Log
- Seed phrases
- Master keys
- Derived keys
- Plaintext sensitive data
- Decrypted user data

## Future Enhancements

Post-MVP improvements:
- Key rotation automation
- Hardware security module support
- Additional encryption algorithms
- Encrypted search
- Homomorphic encryption (research)
- Quantum-resistant algorithms
