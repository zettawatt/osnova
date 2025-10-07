//! Key cocoon models for encrypted key storage
//!
//! This module provides the key cocoon structure and related types for storing
//! derived cryptographic keys in encrypted storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of cryptographic key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyType {
    /// Ed25519 signature key
    Ed25519,
    /// X25519 encryption key
    X25519,
    /// Secp256k1 key (for cryptocurrency wallets)
    Secp256k1,
}

/// A derived key entry stored in the cocoon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedKeyEntry {
    /// Public key (base64-encoded)
    pub public_key: String,
    /// Secret key (base64-encoded)
    pub secret_key: String,
    /// Component ID that owns this key
    pub component_id: String,
    /// Derivation index (scoped per component)
    pub index: u64,
    /// Unix timestamp when key was created
    pub created_at: u64,
    /// Type of key
    pub key_type: KeyType,
}

impl DerivedKeyEntry {
    /// Create a new derived key entry
    pub fn new(
        public_key: String,
        secret_key: String,
        component_id: String,
        index: u64,
        key_type: KeyType,
    ) -> Self {
        Self {
            public_key,
            secret_key,
            component_id,
            index,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key_type,
        }
    }

    /// Get the unique key identifier (component_id:index)
    pub fn key_id(&self) -> String {
        format!("{}:{}", self.component_id, self.index)
    }
}

/// Key cocoon structure for encrypted storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCocoon {
    /// Master key derived from seed phrase
    pub master_key: [u8; 32],
    /// Derived keys indexed by component_id:index
    pub derived_keys: HashMap<String, DerivedKeyEntry>,
    /// Metadata about the cocoon
    pub metadata: KeyMetadata,
}

/// Metadata about the key cocoon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Version of the cocoon format
    pub version: u32,
    /// Unix timestamp when cocoon was created
    pub created_at: u64,
    /// Unix timestamp of last modification
    pub updated_at: u64,
}

impl KeyCocoon {
    /// Create a new key cocoon with a master key
    pub fn new(master_key: [u8; 32]) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            master_key,
            derived_keys: HashMap::new(),
            metadata: KeyMetadata {
                version: 1,
                created_at: now,
                updated_at: now,
            },
        }
    }

    /// Add a derived key to the cocoon
    pub fn add_key(&mut self, entry: DerivedKeyEntry) {
        let key_id = entry.key_id();
        self.derived_keys.insert(key_id, entry);
        self.update_timestamp();
    }

    /// Get a key by component ID and index
    pub fn get_key(&self, component_id: &str, index: u64) -> Option<&DerivedKeyEntry> {
        let key_id = format!("{}:{}", component_id, index);
        self.derived_keys.get(&key_id)
    }

    /// Get a key by public key
    pub fn get_by_public_key(&self, public_key: &str) -> Option<&DerivedKeyEntry> {
        self.derived_keys
            .values()
            .find(|entry| entry.public_key == public_key)
    }

    /// List all keys for a component
    pub fn list_keys(&self, component_id: &str) -> Vec<&DerivedKeyEntry> {
        self.derived_keys
            .values()
            .filter(|entry| entry.component_id == component_id)
            .collect()
    }

    /// Get the highest index for a component
    pub fn highest_index(&self, component_id: &str) -> Option<u64> {
        self.list_keys(component_id)
            .iter()
            .map(|entry| entry.index)
            .max()
    }

    /// Update the timestamp
    fn update_timestamp(&mut self) {
        self.metadata.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_cocoon_creation() {
        let master_key = [0u8; 32];
        let cocoon = KeyCocoon::new(master_key);

        assert_eq!(cocoon.master_key, master_key);
        assert_eq!(cocoon.derived_keys.len(), 0);
        assert_eq!(cocoon.metadata.version, 1);
    }

    #[test]
    fn test_add_and_get_key() {
        let mut cocoon = KeyCocoon::new([0u8; 32]);
        let entry = DerivedKeyEntry::new(
            "pubkey1".to_string(),
            "seckey1".to_string(),
            "com.test.wallet".to_string(),
            0,
            KeyType::Ed25519,
        );

        cocoon.add_key(entry.clone());

        let retrieved = cocoon.get_key("com.test.wallet", 0).unwrap();
        assert_eq!(retrieved.public_key, "pubkey1");
        assert_eq!(retrieved.secret_key, "seckey1");
        assert_eq!(retrieved.index, 0);
    }

    #[test]
    fn test_get_by_public_key() {
        let mut cocoon = KeyCocoon::new([0u8; 32]);
        let entry = DerivedKeyEntry::new(
            "pubkey1".to_string(),
            "seckey1".to_string(),
            "com.test.wallet".to_string(),
            0,
            KeyType::Ed25519,
        );

        cocoon.add_key(entry);

        let retrieved = cocoon.get_by_public_key("pubkey1").unwrap();
        assert_eq!(retrieved.secret_key, "seckey1");
    }

    #[test]
    fn test_list_keys() {
        let mut cocoon = KeyCocoon::new([0u8; 32]);

        cocoon.add_key(DerivedKeyEntry::new(
            "pubkey1".to_string(),
            "seckey1".to_string(),
            "com.test.wallet".to_string(),
            0,
            KeyType::Ed25519,
        ));

        cocoon.add_key(DerivedKeyEntry::new(
            "pubkey2".to_string(),
            "seckey2".to_string(),
            "com.test.wallet".to_string(),
            1,
            KeyType::Ed25519,
        ));

        cocoon.add_key(DerivedKeyEntry::new(
            "pubkey3".to_string(),
            "seckey3".to_string(),
            "com.other.app".to_string(),
            0,
            KeyType::Ed25519,
        ));

        let wallet_keys = cocoon.list_keys("com.test.wallet");
        assert_eq!(wallet_keys.len(), 2);

        let other_keys = cocoon.list_keys("com.other.app");
        assert_eq!(other_keys.len(), 1);
    }

    #[test]
    fn test_highest_index() {
        let mut cocoon = KeyCocoon::new([0u8; 32]);

        cocoon.add_key(DerivedKeyEntry::new(
            "pubkey1".to_string(),
            "seckey1".to_string(),
            "com.test.wallet".to_string(),
            0,
            KeyType::Ed25519,
        ));

        cocoon.add_key(DerivedKeyEntry::new(
            "pubkey2".to_string(),
            "seckey2".to_string(),
            "com.test.wallet".to_string(),
            5,
            KeyType::Ed25519,
        ));

        assert_eq!(cocoon.highest_index("com.test.wallet"), Some(5));
        assert_eq!(cocoon.highest_index("com.other.app"), None);
    }
}
