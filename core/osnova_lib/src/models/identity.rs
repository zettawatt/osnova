//! Identity models for Osnova
//!
//! This module provides the RootIdentity type which manages:
//! - 12-word BIP-39 seed phrases
//! - Derived master keys (256-bit)
//! - Device key management
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::models::identity::RootIdentity;
//! use bip39::Mnemonic;
//!
//! // Generate new identity
//! let identity = RootIdentity::generate()?;
//!
//! // Import from seed phrase
//! let mnemonic_words = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
//! let identity = RootIdentity::from_seed(mnemonic_words)?;
//!
//! // Get master key
//! let master_key = identity.master_key();
//! ```

use crate::OsnovaError;
use crate::Result;
use bip39::{Language, Mnemonic};
use blake3::Hasher;
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

/// Root identity for an Osnova user
///
/// Contains the 12-word seed phrase and derived master key.
/// The seed phrase should never be stored in plaintext - only in secure platform keystores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootIdentity {
    /// 12-word BIP-39 mnemonic (never store in plaintext in production)
    #[serde(skip_serializing)]
    seed_mnemonic: String,

    /// 256-bit master key derived from seed phrase
    #[serde(skip_serializing)]
    master_key: [u8; 32],
}

impl RootIdentity {
    /// Generate a new random identity with 12-word BIP-39 mnemonic
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::identity::RootIdentity;
    ///
    /// let identity = RootIdentity::generate().expect("Failed to generate identity");
    /// assert_eq!(identity.seed_phrase().split_whitespace().count(), 12);
    /// ```
    pub fn generate() -> Result<Self> {
        // Generate random 12-word mnemonic
        let mnemonic = Mnemonic::generate(12)
            .map_err(|e| OsnovaError::Identity(format!("Failed to generate mnemonic: {}", e)))?;

        Self::from_mnemonic(&mnemonic)
    }

    /// Create identity from 12-word seed phrase
    ///
    /// # Arguments
    ///
    /// * `seed_phrase` - Space-separated 12-word BIP-39 mnemonic
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::identity::RootIdentity;
    ///
    /// let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    /// let identity = RootIdentity::from_seed(seed).expect("Failed to import identity");
    /// ```
    pub fn from_seed(seed_phrase: &str) -> Result<Self> {
        // Parse and validate mnemonic
        let mnemonic = Mnemonic::parse_in(Language::English, seed_phrase)
            .map_err(|e| OsnovaError::Identity(format!("Invalid seed phrase: {}", e)))?;

        Self::from_mnemonic(&mnemonic)
    }

    /// Internal: Create identity from Mnemonic object
    fn from_mnemonic(mnemonic: &Mnemonic) -> Result<Self> {
        // Convert mnemonic to seed (512 bits)
        let seed = mnemonic.to_seed("");

        // Derive 256-bit master key using HKDF-SHA256
        let master_key = Self::derive_master_key(&seed)?;

        Ok(Self {
            seed_mnemonic: mnemonic.to_string(),
            master_key,
        })
    }

    /// Derive 256-bit master key from 512-bit BIP-39 seed
    ///
    /// Uses HKDF-SHA256 with:
    /// - IKM: 512-bit seed from BIP-39
    /// - Salt: "osnova-master-key-v1"
    /// - Info: empty
    /// - Output: 32 bytes (256 bits)
    fn derive_master_key(seed: &[u8]) -> Result<[u8; 32]> {
        let hk = Hkdf::<Sha256>::new(Some(b"osnova-master-key-v1"), seed);

        let mut master_key = [0u8; 32];
        hk.expand(&[], &mut master_key)
            .map_err(|e| OsnovaError::Crypto(format!("HKDF expansion failed: {}", e)))?;

        Ok(master_key)
    }

    /// Get the seed phrase
    ///
    /// **Warning**: Never log or expose this in production.
    /// Only use for backup/export flows with explicit user consent.
    pub fn seed_phrase(&self) -> &str {
        &self.seed_mnemonic
    }

    /// Get the master key
    ///
    /// **Warning**: Never log or expose this.
    /// Use this only for deriving component-specific keys.
    pub fn master_key(&self) -> &[u8; 32] {
        &self.master_key
    }

    /// Derive a component-specific key using HKDF
    ///
    /// # Arguments
    ///
    /// * `component_id` - Unique component identifier (e.g., "com.osnova.wallet")
    /// * `index` - Key derivation index (for BIP-44 wallet paths, etc.)
    /// * `purpose` - Purpose string (e.g., "encryption", "signing")
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::identity::RootIdentity;
    ///
    /// let identity = RootIdentity::generate().expect("Failed to generate");
    /// let wallet_key = identity.derive_component_key("com.osnova.wallet", 0, "signing")
    ///     .expect("Failed to derive key");
    /// assert_eq!(wallet_key.len(), 32);
    /// ```
    pub fn derive_component_key(
        &self,
        component_id: &str,
        index: u32,
        purpose: &str,
    ) -> Result<[u8; 32]> {
        // Create salt from component_id
        let salt_data = format!("{}-{}", component_id, purpose);

        // Create info from index
        let info = index.to_le_bytes();

        let hk = Hkdf::<Sha256>::new(Some(salt_data.as_bytes()), &self.master_key);

        let mut component_key = [0u8; 32];
        hk.expand(&info, &mut component_key)
            .map_err(|e| OsnovaError::Crypto(format!("Component key derivation failed: {}", e)))?;

        Ok(component_key)
    }

    /// Generate a deterministic identity fingerprint
    ///
    /// This is a BLAKE3 hash of the master key, useful for:
    /// - Identifying the same identity across devices
    /// - Verifying identity without exposing the seed phrase
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::identity::RootIdentity;
    ///
    /// let identity = RootIdentity::generate().expect("Failed to generate");
    /// let fingerprint = identity.fingerprint();
    /// assert_eq!(fingerprint.len(), 32);
    /// ```
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&self.master_key);
        *hasher.finalize().as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_identity() {
        let identity = RootIdentity::generate().expect("Failed to generate identity");

        // Seed phrase should be 12 words
        assert_eq!(identity.seed_phrase().split_whitespace().count(), 12);

        // Master key should be 32 bytes
        assert_eq!(identity.master_key().len(), 32);

        // Master key should not be all zeros
        assert_ne!(*identity.master_key(), [0u8; 32]);
    }

    #[test]
    fn test_from_seed_valid() {
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let identity = RootIdentity::from_seed(seed).expect("Failed to create from seed");

        assert_eq!(identity.seed_phrase(), seed);
        assert_eq!(identity.master_key().len(), 32);
    }

    #[test]
    fn test_from_seed_invalid() {
        let invalid_seed = "invalid word word word word word word word word word word word";
        let result = RootIdentity::from_seed(invalid_seed);

        assert!(result.is_err());
        match result {
            Err(OsnovaError::Identity(_)) => (),
            _ => panic!("Expected Identity error"),
        }
    }

    #[test]
    fn test_from_seed_wrong_length() {
        let short_seed = "abandon abandon abandon";
        let result = RootIdentity::from_seed(short_seed);

        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic_master_key() {
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let identity1 = RootIdentity::from_seed(seed).expect("Failed");
        let identity2 = RootIdentity::from_seed(seed).expect("Failed");

        // Same seed should produce same master key
        assert_eq!(identity1.master_key(), identity2.master_key());
    }

    #[test]
    fn test_different_seeds_different_keys() {
        let seed1 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let seed2 = "legal winner thank year wave sausage worth useful legal winner thank yellow";

        let identity1 = RootIdentity::from_seed(seed1).expect("Failed");
        let identity2 = RootIdentity::from_seed(seed2).expect("Failed");

        // Different seeds should produce different master keys
        assert_ne!(identity1.master_key(), identity2.master_key());
    }

    #[test]
    fn test_derive_component_key() {
        let identity = RootIdentity::generate().expect("Failed to generate");

        let wallet_key = identity
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed to derive key");

        assert_eq!(wallet_key.len(), 32);
        assert_ne!(wallet_key, [0u8; 32]);
    }

    #[test]
    fn test_component_key_isolation() {
        let identity = RootIdentity::generate().expect("Failed to generate");

        let wallet_key = identity
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed to derive wallet key");
        let storage_key = identity
            .derive_component_key("com.osnova.storage", 0, "encryption")
            .expect("Failed to derive storage key");

        // Different components should have different keys
        assert_ne!(wallet_key, storage_key);
    }

    #[test]
    fn test_component_key_index_isolation() {
        let identity = RootIdentity::generate().expect("Failed to generate");

        let key0 = identity
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed");
        let key1 = identity
            .derive_component_key("com.osnova.wallet", 1, "signing")
            .expect("Failed");

        // Different indexes should produce different keys
        assert_ne!(key0, key1);
    }

    #[test]
    fn test_component_key_purpose_isolation() {
        let identity = RootIdentity::generate().expect("Failed to generate");

        let signing_key = identity
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed");
        let encryption_key = identity
            .derive_component_key("com.osnova.wallet", 0, "encryption")
            .expect("Failed");

        // Different purposes should produce different keys
        assert_ne!(signing_key, encryption_key);
    }

    #[test]
    fn test_component_key_deterministic() {
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let identity1 = RootIdentity::from_seed(seed).expect("Failed");
        let identity2 = RootIdentity::from_seed(seed).expect("Failed");

        let key1 = identity1
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed");
        let key2 = identity2
            .derive_component_key("com.osnova.wallet", 0, "signing")
            .expect("Failed");

        // Same identity should produce same component keys
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_fingerprint() {
        let identity = RootIdentity::generate().expect("Failed to generate");
        let fingerprint = identity.fingerprint();

        assert_eq!(fingerprint.len(), 32);
        assert_ne!(fingerprint, [0u8; 32]);
    }

    #[test]
    fn test_fingerprint_deterministic() {
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let identity1 = RootIdentity::from_seed(seed).expect("Failed");
        let identity2 = RootIdentity::from_seed(seed).expect("Failed");

        // Same identity should have same fingerprint
        assert_eq!(identity1.fingerprint(), identity2.fingerprint());
    }

    #[test]
    fn test_fingerprint_unique() {
        let identity1 = RootIdentity::generate().expect("Failed");
        let identity2 = RootIdentity::generate().expect("Failed");

        // Different identities should have different fingerprints
        assert_ne!(identity1.fingerprint(), identity2.fingerprint());
    }

    // Property-based test for key derivation
    #[cfg(test)]
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_component_key_always_32_bytes(index in 0u32..1000u32) {
            let identity = RootIdentity::generate().unwrap();
            let key = identity.derive_component_key("test.component", index, "test").unwrap();
            assert_eq!(key.len(), 32);
        }

        #[test]
        fn test_different_indexes_different_keys(index1 in 0u32..100u32, index2 in 100u32..200u32) {
            let identity = RootIdentity::generate().unwrap();
            let key1 = identity.derive_component_key("test", index1, "test").unwrap();
            let key2 = identity.derive_component_key("test", index2, "test").unwrap();
            assert_ne!(key1, key2);
        }
    }
}
