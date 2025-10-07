//! Key derivation functions for Osnova
//!
//! This module provides key derivation utilities for generating component-specific keys
//! from a master key using HKDF-SHA256.
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::crypto::key_derivation::{derive_symmetric_key, KeyType, generate_keypair};
//!
//! // Derive a symmetric key for a component
//! let master_key = [0u8; 32];
//! let symmetric_key = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)?;
//!
//! // Generate Ed25519 keypair from symmetric key
//! let keypair = generate_keypair(&symmetric_key, KeyType::Ed25519)?;
//! ```

use crate::{OsnovaError, Result};
use hkdf::Hkdf;
use sha2::Sha256;

/// Key type for cryptographic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// Ed25519 signing key
    Ed25519,
    /// X25519 encryption key
    X25519,
}

/// Key pair containing public and secret keys
#[derive(Debug, Clone)]
pub struct KeyPair {
    /// Public key bytes
    pub public_key: Vec<u8>,
    /// Secret key bytes
    pub secret_key: Vec<u8>,
    /// Key type
    pub key_type: KeyType,
}

/// Derive a 256-bit symmetric key for a component at a specific index
///
/// Uses HKDF-SHA256 with:
/// - IKM: master_key (256-bit)
/// - Salt: component_id
/// - Info: "osnova-v1-key-derivation" || index.to_le_bytes()
///
/// # Arguments
///
/// * `master_key` - 256-bit master key from RootIdentity
/// * `component_id` - Unique component identifier
/// * `index` - Key derivation index (0, 1, 2, ...)
///
/// # Example
///
/// ```
/// use osnova_lib::crypto::key_derivation::derive_symmetric_key;
///
/// let master_key = [0u8; 32];
/// let key = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
///     .expect("Failed to derive key");
/// assert_eq!(key.len(), 32);
/// ```
pub fn derive_symmetric_key(
    master_key: &[u8; 32],
    component_id: &str,
    index: u64,
) -> Result<[u8; 32]> {
    // Create info context string
    let mut info = Vec::from("osnova-v1-key-derivation");
    info.extend_from_slice(&index.to_le_bytes());

    // Use component_id as salt for HKDF
    let hk = Hkdf::<Sha256>::new(Some(component_id.as_bytes()), master_key);

    let mut derived_key = [0u8; 32];
    hk.expand(&info, &mut derived_key)
        .map_err(|e| OsnovaError::Crypto(format!("Key derivation failed: {}", e)))?;

    Ok(derived_key)
}

/// Generate a keypair from a symmetric key seed
///
/// # Arguments
///
/// * `seed` - 256-bit symmetric key to use as seed
/// * `key_type` - Type of keypair to generate (Ed25519 or X25519)
///
/// # Example
///
/// ```
/// use osnova_lib::crypto::key_derivation::{derive_symmetric_key, generate_keypair, KeyType};
///
/// let master_key = [0u8; 32];
/// let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0).unwrap();
/// let keypair = generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate keypair");
///
/// assert_eq!(keypair.public_key.len(), 32);
/// assert_eq!(keypair.secret_key.len(), 32);
/// ```
pub fn generate_keypair(seed: &[u8; 32], key_type: KeyType) -> Result<KeyPair> {
    match key_type {
        KeyType::Ed25519 => {
            // For Ed25519, we use the seed directly as the secret key
            // The public key is derived from the secret key
            let secret_key = *seed;

            // In a real implementation, we would use ed25519-dalek or similar
            // For now, we'll use a placeholder that derives the public key deterministically
            let public_key = derive_ed25519_public_key(&secret_key)?;

            Ok(KeyPair {
                public_key: public_key.to_vec(),
                secret_key: secret_key.to_vec(),
                key_type: KeyType::Ed25519,
            })
        }
        KeyType::X25519 => {
            // For X25519, we derive from the seed
            let secret_key = *seed;

            // Derive public key for X25519
            let public_key = derive_x25519_public_key(&secret_key)?;

            Ok(KeyPair {
                public_key: public_key.to_vec(),
                secret_key: secret_key.to_vec(),
                key_type: KeyType::X25519,
            })
        }
    }
}

/// Derive Ed25519 public key from secret key
///
/// This is a placeholder implementation. In production, use ed25519-dalek or similar.
fn derive_ed25519_public_key(secret_key: &[u8; 32]) -> Result<[u8; 32]> {
    // TODO: Replace with actual Ed25519 key derivation when adding ed25519-dalek
    // For now, use BLAKE3 hash as a deterministic placeholder
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"ed25519-public-key:");
    hasher.update(secret_key);
    Ok(*hasher.finalize().as_bytes())
}

/// Derive X25519 public key from secret key
///
/// This is a placeholder implementation. In production, use x25519-dalek or similar.
fn derive_x25519_public_key(secret_key: &[u8; 32]) -> Result<[u8; 32]> {
    // TODO: Replace with actual X25519 key derivation when adding x25519-dalek
    // For now, use BLAKE3 hash as a deterministic placeholder
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"x25519-public-key:");
    hasher.update(secret_key);
    Ok(*hasher.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_master_key() -> [u8; 32] {
        [1u8; 32]
    }

    #[test]
    fn test_derive_symmetric_key() {
        let master_key = sample_master_key();
        let key = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        assert_eq!(key.len(), 32);
        assert_ne!(key, [0u8; 32]); // Should not be all zeros
    }

    #[test]
    fn test_derive_symmetric_key_deterministic() {
        let master_key = sample_master_key();

        let key1 = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");
        let key2 = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        // Same inputs should produce same key
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_symmetric_key_different_components() {
        let master_key = sample_master_key();

        let key1 = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");
        let key2 = derive_symmetric_key(&master_key, "com.osnova.storage", 0)
            .expect("Failed to derive key");

        // Different components should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_symmetric_key_different_indexes() {
        let master_key = sample_master_key();

        let key1 = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");
        let key2 = derive_symmetric_key(&master_key, "com.osnova.wallet", 1)
            .expect("Failed to derive key");

        // Different indexes should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_symmetric_key_different_master_keys() {
        let master_key1 = [1u8; 32];
        let master_key2 = [2u8; 32];

        let key1 = derive_symmetric_key(&master_key1, "com.osnova.wallet", 0)
            .expect("Failed to derive key");
        let key2 = derive_symmetric_key(&master_key2, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        // Different master keys should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_ed25519_keypair() {
        let master_key = sample_master_key();
        let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        let keypair =
            generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate keypair");

        assert_eq!(keypair.key_type, KeyType::Ed25519);
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.secret_key.len(), 32);
        assert_eq!(keypair.secret_key, seed.to_vec());
    }

    #[test]
    fn test_generate_x25519_keypair() {
        let master_key = sample_master_key();
        let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        let keypair = generate_keypair(&seed, KeyType::X25519).expect("Failed to generate keypair");

        assert_eq!(keypair.key_type, KeyType::X25519);
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.secret_key.len(), 32);
        assert_eq!(keypair.secret_key, seed.to_vec());
    }

    #[test]
    fn test_keypair_deterministic() {
        let master_key = sample_master_key();
        let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        let keypair1 =
            generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate keypair");
        let keypair2 =
            generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate keypair");

        // Same seed should produce same keypair
        assert_eq!(keypair1.public_key, keypair2.public_key);
        assert_eq!(keypair1.secret_key, keypair2.secret_key);
    }

    #[test]
    fn test_ed25519_vs_x25519_different() {
        let master_key = sample_master_key();
        let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        let ed25519_keypair =
            generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate Ed25519 keypair");
        let x25519_keypair =
            generate_keypair(&seed, KeyType::X25519).expect("Failed to generate X25519 keypair");

        // Different key types should have different public keys
        assert_ne!(ed25519_keypair.public_key, x25519_keypair.public_key);
    }

    #[test]
    fn test_public_key_derives_from_secret() {
        let master_key = sample_master_key();
        let seed = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");

        let keypair =
            generate_keypair(&seed, KeyType::Ed25519).expect("Failed to generate keypair");

        // Verify public key is deterministic from secret key
        let derived_public = derive_ed25519_public_key(&seed).expect("Failed to derive public key");
        assert_eq!(keypair.public_key, derived_public.to_vec());
    }

    #[test]
    fn test_different_indexes_different_keypairs() {
        let master_key = sample_master_key();

        let seed0 = derive_symmetric_key(&master_key, "com.osnova.wallet", 0)
            .expect("Failed to derive key");
        let seed1 = derive_symmetric_key(&master_key, "com.osnova.wallet", 1)
            .expect("Failed to derive key");

        let keypair0 =
            generate_keypair(&seed0, KeyType::Ed25519).expect("Failed to generate keypair");
        let keypair1 =
            generate_keypair(&seed1, KeyType::Ed25519).expect("Failed to generate keypair");

        // Different indexes should produce different keypairs
        assert_ne!(keypair0.public_key, keypair1.public_key);
        assert_ne!(keypair0.secret_key, keypair1.secret_key);
    }

    // Property-based tests
    #[cfg(test)]
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_derivation_always_32_bytes(index in 0u64..1000u64) {
            let master_key = sample_master_key();
            let key = derive_symmetric_key(&master_key, "test.component", index).unwrap();
            assert_eq!(key.len(), 32);
        }

        #[test]
        fn test_same_inputs_same_output(index in 0u64..100u64) {
            let master_key = sample_master_key();
            let key1 = derive_symmetric_key(&master_key, "test", index).unwrap();
            let key2 = derive_symmetric_key(&master_key, "test", index).unwrap();
            assert_eq!(key1, key2);
        }
    }
}
