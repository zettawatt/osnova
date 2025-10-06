# Cocoon Unlock Mechanism

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Cache derived master key in memory after first seed phrase entry

## Overview

The cocoon unlock mechanism determines how users authenticate to access their encrypted key storage. For MVP, we use a memory-caching approach that balances security and user experience.

## MVP Unlock Flow

### First Launch (Onboarding)

1. **User enters 12-word seed phrase** (or generates new one)
2. **Derive master key**: `master_key = HKDF-SHA256(seed_phrase)`
3. **Create cocoon**: Encrypt empty keystore with a user delivered password
4. **Cache master key in memory**: Store in secure memory structure
5. **Proceed to application**: User can now use all features

### Subsequent Launches

1. **Application starts**
2. **Prompt for password**: Display unlock dialog
3. **User enters password**
4. **Verify correctness**: Attempt to decrypt cocoon with password
   - Success: Cache master key in memory, proceed
   - Failure: Show error, allow retry
5. **Cache master key in memory**: Store for session duration
6. **Proceed to application**

### During Session

- **Master key remains in memory**: No re-authentication required
- **All key operations use cached key**: Transparent to user
- **No disk persistence**: Key never written to disk in plaintext

### Application Exit

- **Zeroize master key**: Overwrite memory with zeros
- **Clear all derived keys**: Zeroize component keys
- **Exit cleanly**: No key material persists

## Security Considerations

### Memory Protection

```rust
use zeroize::Zeroize;

pub struct MasterKeyCache {
    key: [u8; 32],
}

impl MasterKeyCache {
    pub fn new(seed_phrase: &str) -> Result<Self> {
        let key = derive_master_key(seed_phrase)?;
        Ok(Self { key })
    }
    
    pub fn get_key(&self) -> &[u8; 32] {
        &self.key
    }
}

impl Drop for MasterKeyCache {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}
```

### Memory Locking (Optional Enhancement)

For enhanced security, lock memory pages to prevent swapping:

```rust
#[cfg(unix)]
use libc::{mlock, munlock};

pub fn lock_memory(ptr: *const u8, len: usize) -> Result<()> {
    #[cfg(unix)]
    unsafe {
        if mlock(ptr as *const libc::c_void, len) != 0 {
            return Err(Error::MemoryLockFailed);
        }
    }
    Ok(())
}
```

**MVP Decision**: Memory locking is optional for MVP, can be added post-MVP

### Attack Vectors and Mitigations

#### 1. Memory Dump Attack
**Risk**: Attacker with system access dumps process memory
**Mitigation**: 
- Zeroize on exit
- Memory locking (post-MVP)
- OS-level protections (ASLR, DEP)

#### 2. Cold Boot Attack
**Risk**: Attacker with physical access reads RAM after power-off
**Mitigation**: 
- Zeroize on exit
- Encrypted swap (OS-level)
- Not practical for most threat models

#### 3. Malware/Keylogger
**Risk**: Malware captures seed phrase during entry
**Mitigation**:
- OS-level security (antivirus, sandboxing)
- Hardware security module (post-MVP)
- Not solvable at application level

#### 4. Shoulder Surfing
**Risk**: Attacker observes seed phrase entry
**Mitigation**:
- Masked input (show dots instead of words)
- Privacy screen protectors
- User awareness

## User Experience

### Unlock Dialog

```
┌─────────────────────────────────────────────┐
│  Unlock Osnova                              │
├─────────────────────────────────────────────┤
│                                             │
│  Enter your password:                       │
│                                             │
│  ┌─────────────────────────────────────┐    │
│  │ ●●●●●●●●●●●●●●●●●●                  │    │
│  └─────────────────────────────────────┘    │
│                                             │
│  [ ] Show password                          │
│                                             │
│  [Cancel]              [Unlock]             │
│                                             │
└─────────────────────────────────────────────┘
```

### Features

- **Masked by default**: Show dots instead of words
- **Show/hide toggle**: Allow user to reveal if needed
- **Word validation**: Validate against BIP-39 wordlist as user types
- **Auto-complete**: Suggest words from BIP-39 wordlist
- **Error handling**: Clear error messages for invalid phrases
- **Retry limit**: Lock after 5 failed attempts (post-MVP)

## Alternative Unlock Methods (Post-MVP)

### 1. Biometric Authentication

**Platforms**: iOS (Face ID, Touch ID), Android (Fingerprint, Face Unlock), macOS (Touch ID)

**Flow**:
1. User enrolls biometric during onboarding
2. Encrypt seed phrase with platform keystore
3. On unlock, use biometric to decrypt seed phrase
4. Derive master key and proceed

**Benefits**: Faster, more convenient
**Risks**: Platform keystore security varies

### 2. PIN/Password (default, described above)

**Flow**:
1. User sets PIN/password during onboarding
2. Derive encryption key from PIN: `pin_key = Argon2(pin, salt)`
3. Encrypt seed phrase with pin_key
4. On unlock, decrypt seed phrase with PIN
5. Derive master key and proceed

**Benefits**: Easy for users to use
**Risks**: Weaker security if PIN is weak

### 3. Hardware Security Key

**Flow**:
1. User enrolls hardware key (YubiKey, etc.)
2. Store encrypted seed phrase on device
3. On unlock, challenge hardware key
4. Decrypt seed phrase and proceed

**Benefits**: Strong security, phishing-resistant
**Risks**: Requires hardware, can be lost

## Session Management

### Session Duration

**MVP**: Session lasts until application exit
- No automatic lock after inactivity
- User must manually close application to lock

**Post-MVP**: Configurable auto-lock
- Lock after N minutes of inactivity
- Lock on screen lock
- Lock on sleep/hibernate

### Re-Authentication

**MVP**: Not required during session
- All operations use cached master key
- No re-prompting for sensitive operations

**Post-MVP**: Re-authenticate for sensitive operations
- Large payments (>$100)
- Key export
- Seed phrase display
- Component installation from untrusted sources

## Multi-User Support (Post-MVP)

### User Switching

1. **Lock current session**: Zeroize master key
2. **Prompt for different seed phrase**
3. **Load different cocoon file**: `$DATA_ROOT/identity/<user-id>/keys.cocoon`
4. **Cache new master key**
5. **Switch UI context**

### Concurrent Users

**Not supported in MVP**: Only one user session at a time
**Post-MVP**: Multiple user profiles with separate cocoons

## Implementation Example

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

pub struct UnlockManager {
    master_key: Arc<RwLock<Option<MasterKeyCache>>>,
}

impl UnlockManager {
    pub fn new() -> Self {
        Self {
            master_key: Arc::new(RwLock::new(None)),
        }
    }
    
    pub async fn unlock(&self, seed_phrase: &str) -> Result<()> {
        // Derive master key
        let master_key = derive_master_key(seed_phrase)?;
        
        // Verify by attempting to decrypt cocoon
        let cocoon_path = get_cocoon_path()?;
        let cocoon = Cocoon::new(&master_key);
        cocoon.decrypt_file(&cocoon_path)?;
        
        // Cache master key
        let cache = MasterKeyCache::new(seed_phrase)?;
        *self.master_key.write().await = Some(cache);
        
        Ok(())
    }
    
    pub async fn lock(&self) {
        // Zeroize and clear master key
        *self.master_key.write().await = None;
    }
    
    pub async fn is_unlocked(&self) -> bool {
        self.master_key.read().await.is_some()
    }
    
    pub async fn get_master_key(&self) -> Result<[u8; 32]> {
        let guard = self.master_key.read().await;
        match guard.as_ref() {
            Some(cache) => Ok(*cache.get_key()),
            None => Err(Error::NotUnlocked),
        }
    }
}
```

## Testing Considerations

### Unit Tests

- Test master key derivation
- Test cocoon encryption/decryption
- Test zeroization on drop
- Test invalid seed phrase handling

### Integration Tests

- Test full unlock flow
- Test lock/unlock cycle
- Test application restart
- Test concurrent access

### Security Tests

- Verify zeroization occurs
- Verify no plaintext keys on disk
- Verify memory is cleared on exit
- Verify failed unlock attempts are logged

## Configuration

### User Preferences

```json
{
  "unlock": {
    "maskSeedPhrase": true,
    "autoCompleteWords": true,
    "validateWords": true,
    "rememberLastUser": false
  }
}
```

### Developer Options

```json
{
  "unlock": {
    "devAutoUnlock": false,
    "devSeedPhrase": null
  }
}
```

**Warning**: Never enable devAutoUnlock in production builds

## Accessibility

- **Screen reader support**: Announce unlock status
- **Keyboard navigation**: Full keyboard support for unlock dialog
- **High contrast**: Ensure visibility in all themes
- **Font size**: Respect system font size settings

## Error Messages

- **Invalid password**: "Invalid password. Please check and try again."
- **Cocoon decrypt failed**: "Unable to unlock. Please verify your password."
- **Too many attempts**: "Too many failed attempts. Please try again in 5 minutes." (post-MVP)
- **Cocoon file missing**: "Key storage not found. Please restore from backup or create new identity."

## Logging and Monitoring

### Log Events

- Unlock attempt (success/failure)
- Lock event
- Session duration
- Failed attempt count

### Do NOT Log

- Seed phrase (never)
- Master key (never)
- Derived keys (never)
- Any key material (never)

## Compliance Considerations

- **GDPR**: Seed phrase is user data, must be deletable
- **Data Residency**: Keys stored locally, not transmitted
- **Right to Export**: User can export seed phrase (with re-authentication)
- **Right to Erasure**: User can delete cocoon file

## Recommendations

### For MVP
✅ Use memory caching approach as specified
✅ Implement zeroization on exit
✅ Masked input by default
✅ Word validation and auto-complete
✅ Clear error messages
   PIN/password option

### For Post-MVP
- Add auto-lock after inactivity
- Add biometric authentication option
- Add re-authentication for sensitive operations
- Add memory locking for enhanced security
- Add retry limits and lockout

## Security Audit Checklist

- [ ] Master key never written to disk in plaintext
- [ ] Master key zeroized on exit
- [ ] Cocoon file properly encrypted
- [ ] No key material in logs
- [ ] Memory cleared on lock
- [ ] Failed attempts logged
- [ ] Input validation prevents injection
- [ ] Error messages don't leak information

