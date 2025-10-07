# Identity Management

Osnova's identity system is based on saorsa-core and provides secure, decentralized identity management with strong encryption.

## Identity Artifacts

Users manage two key identity artifacts:

### 1. Four-Word Identity Address
- **Purpose**: Addressing and lookup
- **Format**: Four human-readable words (e.g., "river-spark-honest-lion")
- **Source**: saorsa-core identity system
- **Usage**: Public identifier, pairing, messaging
- **Importable**: Yes, via saorsa-core import flow
- **Shareable**: Yes, safe to share publicly

### 2. Twelve-Word Seed Phrase
- **Purpose**: Master key derivation
- **Format**: 12-word BIP-39 mnemonic
- **Source**: Generated during installation or imported
- **Usage**: Derives master key for all cryptographic operations
- **Importable**: Yes, must be kept secret
- **Shareable**: NO - Never share, store securely

## Identity Creation Flow

### New Identity
1. User launches Osnova for first time
2. Onboarding wizard appears
3. User enters display name
4. System generates 12-word seed phrase
5. System derives master key from seed phrase
6. saorsa-core creates 4-word identity address
7. Identity and keys stored encrypted
8. User shown seed phrase with backup instructions

### Import Existing Identity
1. User launches Osnova for first time
2. Onboarding wizard appears
3. User chooses "Import" option
4. User enters 4-word identity address (saorsa-core import)
5. User enters 12-word seed phrase (in 12 input boxes)
6. System derives master key from seed phrase
7. saorsa-core imports identity
8. Identity and keys stored encrypted
9. Previous settings/data restored if available

## Master Key Derivation

The master key is the root of all cryptographic operations:

```
12-word seed phrase
    ↓
BIP-39 to seed (512 bits)
    ↓
Master Key (256 bits)
    ↓
Per-service key derivation (HKDF-SHA256)
```

### Derivation Process
1. Convert 12-word mnemonic to 512-bit seed (BIP-39)
2. Derive 256-bit master key using HKDF
3. Store master key in secure platform keystore
4. Never export or log master key in plaintext

## Per-Service Key Derivation

Each service and component derives unique keys:

```
Master Key + Component ID + Index
    ↓
HKDF-SHA256
    ↓
Component-specific key
```

### Key Derivation Parameters
- **Algorithm**: HKDF-SHA256
- **Salt**: Component ID (e.g., "com.osnova.wallet")
- **Info**: Index (for wallet: BIP-44 account index)
- **Output**: 256-bit key

### Examples
- **Wallet keys**: BIP-44 derivation path `m/44'/60'/0'/0/{index}`
- **Storage encryption**: Component-specific keys via HKDF
- **Component keys**: Per-component isolation via component ID

## Key Storage

All keys are stored encrypted in platform-specific secure storage:

### Platforms
- **Windows**: Windows Credential Manager
- **macOS**: Keychain
- **Linux**: Secret Service API (GNOME Keyring, KWallet)
- **Android**: Android Keystore
- **iOS**: iOS Keychain

### Storage Strategy
- Master key stored in platform keystore
- Never exported in plaintext
- Accessed only when needed for derivation
- Encrypted at rest with platform security
- Device-specific encryption

## Identity Import and Restore

### Via 4-Word Address
1. User provides 4-word identity address
2. System calls saorsa-core import
3. saorsa-core fetches identity from DHT
4. Device added to multi-device presence
5. User must also provide 12-word seed for full restore

### Via 12-Word Seed Phrase
1. User provides 12-word seed phrase
2. System derives master key
3. All component keys re-derivable
4. Encryption keys restored
5. Access to encrypted data restored

### Full Restore Process
Both artifacts required for complete restore:
- 4-word address: Identity and presence on network
- 12-word seed: Access to encrypted data

## Encryption and Key Management

### Data Encryption
Osnova uses multiple encryption libraries:
- **cocoon**: Local file encryption (config, cache)
- **saorsa-seal**: Threshold encryption for seed phrase backup
- **saorsa-pqc**: Post-quantum crypto (Phase 2+)

Key properties:
- Keys derived from master key per component
- Each user has unique encryption keys
- Server cannot decrypt user data (client-server mode)
- Keys rotatable via re-encryption (post-MVP)

### Key Lifecycle
1. **Creation**: Derive from master key
2. **Storage**: Store encrypted in platform keystore
3. **Usage**: Load only when needed
4. **Rotation**: Re-encrypt data with new keys (post-MVP)
5. **Revocation**: Remove from keystore, re-encrypt data

## Security Best Practices

### For Users
- **Backup seed phrase**: Write down 12 words, store securely offline
- **Never share seed phrase**: Not even with support
- **Verify 4-word address**: Double-check before sharing
- **Use strong device security**: Device PIN/password required
- **Regular backups**: Export and backup encrypted data

### For Developers
- **Never log secrets**: Redact all keys from logs
- **Use secure storage**: Always use platform keystore
- **Minimize exposure**: Load keys only when needed
- **Validate inputs**: Check seed phrase and address format
- **Clear memory**: Zero sensitive data after use

## Multi-Device Support

Osnova supports multiple devices per identity:

### Device Registration
1. Install Osnova on new device
2. Import 4-word identity address
3. Import 12-word seed phrase
4. Device added to presence system
5. Encrypted data accessible

### Device Management
- View all paired devices
- See last active timestamp
- Revoke device access
- Each device has unique encryption keys

## Recovery Scenarios

### Lost Device
- Install Osnova on new device
- Import identity using 4-word address and 12-word seed
- All data restored from network and local encrypted storage

### Forgotten 4-Word Address
- Cannot be recovered without backup
- Identity inaccessible on network
- Local encrypted data still accessible with 12-word seed
- Must create new identity

### Lost 12-Word Seed Phrase
- Cannot be recovered
- Encrypted data permanently inaccessible
- Can still use identity for non-encrypted operations
- Recommend creating new identity

## Privacy Considerations

### What's Public
- 4-word identity address
- Device endpoints (when online)
- Presence information
- Public virtual disk contents

### What's Private
- 12-word seed phrase
- Master key and derived keys
- Private encrypted data
- Transaction history (stored locally)
- User configuration

## Integration with Core Services

### osnova-core
- Stores encrypted identity metadata
- Manages key derivation
- Handles device registration

### osnova-saorsa
- Provides 4-word identity addresses
- Manages multi-device presence
- Handles network identity operations

### osnova-wallet
- Derives wallet keys from master key
- Uses BIP-44/BIP-32 standards
- Manages Ethereum addresses

## Error Handling

### Invalid Seed Phrase
- Clear error message
- Suggest checking word order and spelling
- Provide retry option

### Import Failure
- Show specific error (network, invalid address, etc.)
- Provide troubleshooting steps
- Never expose sensitive data in errors

### Key Derivation Failure
- Graceful degradation
- Clear user-facing error
- Log technical details (without secrets)

## Seed Phrase Backup (Future Enhancement)

### Threshold Backup with saorsa-seal

Post-MVP feature for distributed seed phrase backup:

**Purpose**: Protect seed phrase across multiple devices using threshold encryption

**How it works**:
1. User's seed phrase split into N shares (e.g., 5)
2. Only M shares needed to recover (e.g., 3)
3. Shares distributed across user's devices
4. Post-quantum encryption (ML-KEM-768)
5. Automatic recovery if device lost

**Example scenario**:
- User has 5 devices (phone, laptop, tablet, desktop, smart TV)
- Seed phrase split into 5 shares, need any 3 to recover
- Loses phone → still can recover with remaining 4 devices
- Loses phone + tablet → still can recover with 3 devices

**Benefits**:
- No single point of failure
- No manual backup writing required
- Quantum-resistant encryption
- Automatic across trusted devices

**Implementation**: Uses saorsa-seal library with Shamir's Secret Sharing

## Future Enhancements

Post-MVP improvements:
- **Threshold seed backup**: saorsa-seal-based distributed backup (described above)
- Hardware wallet integration
- Multi-signature accounts
- Key rotation automation
- Social recovery: Share backup with trusted contacts
- Biometric authentication
- Advanced device management
