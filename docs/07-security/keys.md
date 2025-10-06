# Key Derivation and Management

When the osnova shell application is installed and run for the first time by a user, a 12 word seed phrase is randomly generated or entered in during the onboarding process.
The 12 word seed phrase follows the BIP-39 mnemonic and uses the English standard wordlist.
This seed phrase is used to generate a 256bit master key that is used for all downstream derivation.
The key facilities are contained within the osnova-core backend component.

## Key Storage

Osnova stores the master key and all derived keys using the 'cocoon' rust crate for encrypted key storage.
Each user has a separate encrypted cocoon file containing their key material, ensuring complete isolation between users.
The cocoon is encrypted using a key derived from the user's 12-word seed phrase, providing strong protection without relying on potentially compromised platform keystores.

### Per-User Cocoon Architecture

Each user's key material is stored in an isolated, encrypted cocoon file:
- Location: `$DATA_ROOT/identity/<user-id>/keys.cocoon`
- Encryption: Cocoon encryption derived from the 12-word seed phrase
- Contents: Master key, derived keys (indexed by component ID + index), key metadata
- Access: Only via osnova-core backend component OpenRPC methods

**Important**: The 12-word seed phrase is required for ALL installation types:
- Stand-alone desktop: Required during onboarding
- Server: Required during server setup
- Client-Server mobile: Required on mobile device for E2E encryption (each client has its own seed phrase and cocoon)

This ensures that in Client-Server mode, user data remains end-to-end encrypted—the server cannot decrypt client data without the client's seed phrase.

### Cocoon Structure

The cocoon stores keys in a structured format:
```rust
struct KeyCocoon {
    master_key: [u8; 32],              // 256-bit master key from seed phrase
    derived_keys: HashMap<String, DerivedKeyEntry>,  // component_id:index -> key entry
    metadata: KeyMetadata,
}

struct DerivedKeyEntry {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
    component_id: String,
    index: u64,
    created_at: u64,
    key_type: KeyType,  // Ed25519, X25519, etc.
}
```

### Key Retrieval by Public Key

Components can retrieve their secret keys by providing the public key:
1. Component calls `keys.getByPublicKey(publicKey)` OpenRPC method
2. osnova-core opens the user's cocoon (unlocked by seed-derived key)
3. Search `derived_keys` for matching `public_key`
4. Return the corresponding `secret_key` if found
5. Error if not found or cocoon is locked

### Security Considerations

- **No Platform Keystore**: Avoids potential backdoors in OS-provided keystores
- **User-Controlled**: Seed phrase is the sole unlock mechanism
- **Isolated**: Each user's cocoon is separate; server admins cannot access client keys
- **Encrypted at Rest**: Cocoon encryption protects keys even if storage is compromised
- **Memory Safety**: Keys are zeroed from memory after use (using `zeroize` crate)

## Key Derivation

All backend and frontend components can request key derivation via the osnova-core backend component's OpenRPC methods.
Keys are deterministically derived using a combination of the master key, component ID, and an index, ensuring no key overlap between components.

### Derivation Strategy: Master Key + Component ID + Index

The derivation path ensures isolation between components:
```
derived_key = HKDF-SHA256(
    ikm: master_key,
    salt: component_id,
    info: "osnova-v1-key-derivation" || index.to_bytes()
)
```

**Parameters**:
- `master_key`: 256-bit key derived from the 12-word seed phrase
- `component_id`: Unique component identifier (e.g., "com.osnova.wallet" or the Autonomi content address)
- `index`: 64-bit unsigned integer (0, 1, 2, ...) to allow multiple keys per component
- `info`: Context string with version tag and index

**Key Type Generation**:
After deriving a 256-bit symmetric key, convert it to the requested key type:
- Ed25519: Use derived key as seed for Ed25519 key generation
- X25519: Derive from Ed25519 or directly from seed
- Secp256k1: Use derived key as seed (for cryptocurrency components)

### OpenRPC Methods for Key Management

#### `keys.derive`
Derive a new key for a component at the next available index.

**Request**:
```json
{
  "method": "keys.derive",
  "params": {
    "componentId": "com.osnova.wallet",
    "keyType": "Ed25519"
  }
}
```

**Response**:
```json
{
  "result": {
    "publicKey": "base64-encoded-public-key",
    "index": 0,
    "created": 1696214400
  }
}
```

**Behavior**:
1. Check the cocoon for the highest index for `componentId`
2. Derive a new key at `index = highest + 1` (or 0 if none exist)
3. Generate key pair from derived seed
4. Store in cocoon with metadata
5. Return public key and index

#### `keys.deriveAtIndex`
Derive or retrieve a key at a specific index (idempotent).

**Request**:
```json
{
  "method": "keys.deriveAtIndex",
  "params": {
    "componentId": "com.osnova.wallet",
    "index": 5,
    "keyType": "Ed25519"
  }
}
```

**Response**: Same as `keys.derive`

**Behavior**:
- If key at `(componentId, index)` already exists, return it
- If not, derive and store it
- Ensures deterministic re-derivation (same seed + component + index = same key)

#### `keys.getByPublicKey`
Retrieve the secret key corresponding to a public key.

**Request**:
```json
{
  "method": "keys.getByPublicKey",
  "params": {
    "publicKey": "base64-encoded-public-key"
  }
}
```

**Response**:
```json
{
  "result": {
    "secretKey": "base64-encoded-secret-key",
    "componentId": "com.osnova.wallet",
    "index": 0
  }
}
```

#### `keys.listForComponent`
List all derived keys for a specific component.

**Request**:
```json
{
  "method": "keys.listForComponent",
  "params": {
    "componentId": "com.osnova.wallet"
  }
}
```

**Response**:
```json
{
  "result": {
    "keys": [
      {
        "publicKey": "base64-encoded-public-key-0",
        "index": 0,
        "keyType": "Ed25519",
        "created": 1696214400
      },
      {
        "publicKey": "base64-encoded-public-key-1",
        "index": 1,
        "keyType": "Ed25519",
        "created": 1696214500
      }
    ]
  }
}
```

**Use Case**: Components can query which keys they've already created to determine the next index to use or to display a list of addresses to the user.

### Example Workflow: Wallet Component

1. Wallet component starts and calls `keys.listForComponent("com.osnova.wallet")`
2. If empty, calls `keys.derive("com.osnova.wallet", "Ed25519")` to create first key (index 0)
3. To create a new address, calls `keys.derive("com.osnova.wallet", "Ed25519")` → gets index 1
4. To sign a transaction, retrieves the secret key via `keys.getByPublicKey(publicKey)`
5. After signing, the secret key is zeroed from memory

### Security Properties

- **Deterministic**: Same seed + component + index always produces the same key
- **Isolated**: Each component's keys are independent (different component IDs)
- **Recoverable**: With the 12-word seed phrase, all keys can be re-derived
- **Auditable**: `keys.listForComponent` shows all keys without exposing secrets
- **No Overlap**: HKDF with component ID as salt ensures unique derivation domains
