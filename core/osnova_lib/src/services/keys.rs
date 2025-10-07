use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::crypto::key_derivation;
use crate::models::key_cocoon::{DerivedKeyEntry, KeyCocoon, KeyType};
use crate::storage::FileStorage;

/// Response for key derivation methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationResponse {
    /// Base64-encoded public key
    pub public_key: String,
    /// Derivation index
    pub index: u64,
    /// Unix timestamp when key was created
    pub created: u64,
}

/// Response for getByPublicKey method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeyResponse {
    /// Base64-encoded secret key
    pub secret_key: String,
    /// Component ID that owns this key
    pub component_id: String,
    /// Derivation index
    pub index: u64,
}

/// Key info for listForComponent method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Base64-encoded public key
    pub public_key: String,
    /// Derivation index
    pub index: u64,
    /// Key type
    pub key_type: KeyType,
    /// Unix timestamp when key was created
    pub created: u64,
}

/// Key management service for deriving and managing component keys
///
/// Provides OpenRPC methods:
/// - `keys.derive` - Derive a new key at the next available index
/// - `keys.deriveAtIndex` - Derive or retrieve a key at a specific index
/// - `keys.getByPublicKey` - Retrieve secret key by public key
/// - `keys.listForComponent` - List all keys for a component
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::KeyService;
/// use osnova_lib::models::key_cocoon::KeyType;
///
/// # fn example() -> anyhow::Result<()> {
/// let service = KeyService::new("/path/to/storage", &[0u8; 32])?;
///
/// // Derive a new key
/// let response = service.derive("com.osnova.wallet", KeyType::Ed25519)?;
/// println!("Derived key at index {}: {}", response.index, response.public_key);
///
/// // List all keys for component
/// let keys = service.list_for_component("com.osnova.wallet")?;
/// println!("Total keys: {}", keys.len());
/// # Ok(())
/// # }
/// ```
pub struct KeyService {
    storage: FileStorage,
    cocoon_path: PathBuf,
    cocoon_key: [u8; 32],
}

impl KeyService {
    /// Create a new key service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for storage
    /// * `cocoon_key` - Encryption key for the key cocoon
    ///
    /// # Errors
    ///
    /// Returns an error if storage cannot be initialized
    pub fn new<P: Into<PathBuf>>(storage_path: P, cocoon_key: &[u8; 32]) -> Result<Self> {
        let storage_path = storage_path.into();
        let storage = FileStorage::new(&storage_path)?;
        let cocoon_path = PathBuf::from("identity/keys.cocoon");

        Ok(Self {
            storage,
            cocoon_path,
            cocoon_key: *cocoon_key,
        })
    }

    /// Initialize cocoon with master key if it doesn't exist
    ///
    /// # Arguments
    ///
    /// * `master_key` - 256-bit master key from identity seed phrase
    pub fn initialize(&self, master_key: &[u8; 32]) -> Result<()> {
        if self.storage.exists(&self.cocoon_path) {
            return Ok(());
        }

        let cocoon = KeyCocoon::new(*master_key);
        self.save_cocoon(&cocoon)?;

        Ok(())
    }

    /// Derive a new key at the next available index (OpenRPC: keys.derive)
    ///
    /// # Arguments
    ///
    /// * `component_id` - Component requesting the key
    /// * `key_type` - Type of key to derive
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Cocoon is not initialized
    /// - Key derivation fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::KeyService;
    /// # use osnova_lib::models::key_cocoon::KeyType;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = KeyService::new("/tmp/storage", &[0u8; 32])?;
    /// let response = service.derive("com.osnova.wallet", KeyType::Ed25519)?;
    /// println!("Derived key at index {}", response.index);
    /// # Ok(())
    /// # }
    /// ```
    pub fn derive(&self, component_id: &str, key_type: KeyType) -> Result<KeyDerivationResponse> {
        let mut cocoon = self.load_cocoon()?;

        // Find next available index
        let next_index = cocoon.highest_index(component_id).map(|i| i + 1).unwrap_or(0);

        // Derive key at next index
        self.derive_at_index_internal(&mut cocoon, component_id, next_index, key_type)
    }

    /// Derive or retrieve a key at a specific index (OpenRPC: keys.deriveAtIndex)
    ///
    /// This method is idempotent - calling it multiple times with the same parameters
    /// returns the same key.
    ///
    /// # Arguments
    ///
    /// * `component_id` - Component requesting the key
    /// * `index` - Specific derivation index
    /// * `key_type` - Type of key to derive
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Cocoon is not initialized
    /// - Key derivation fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::KeyService;
    /// # use osnova_lib::models::key_cocoon::KeyType;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = KeyService::new("/tmp/storage", &[0u8; 32])?;
    /// let response = service.derive_at_index("com.osnova.wallet", 5, KeyType::Ed25519)?;
    /// println!("Key at index 5: {}", response.public_key);
    /// # Ok(())
    /// # }
    /// ```
    pub fn derive_at_index(
        &self,
        component_id: &str,
        index: u64,
        key_type: KeyType,
    ) -> Result<KeyDerivationResponse> {
        let mut cocoon = self.load_cocoon()?;

        // Check if key already exists at this index
        if let Some(entry) = cocoon.get_key(component_id, index) {
            return Ok(KeyDerivationResponse {
                public_key: entry.public_key.clone(),
                index: entry.index,
                created: entry.created_at,
            });
        }

        // Derive new key at specified index
        self.derive_at_index_internal(&mut cocoon, component_id, index, key_type)
    }

    /// Retrieve secret key by public key (OpenRPC: keys.getByPublicKey)
    ///
    /// # Arguments
    ///
    /// * `public_key` - Base64-encoded public key
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Cocoon is not initialized
    /// - Public key not found
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::KeyService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = KeyService::new("/tmp/storage", &[0u8; 32])?;
    /// let response = service.get_by_public_key("base64-encoded-public-key")?;
    /// println!("Secret key: {}", response.secret_key);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_by_public_key(&self, public_key: &str) -> Result<SecretKeyResponse> {
        let cocoon = self.load_cocoon()?;

        let entry = cocoon
            .get_by_public_key(public_key)
            .context("Public key not found")?;

        Ok(SecretKeyResponse {
            secret_key: entry.secret_key.clone(),
            component_id: entry.component_id.clone(),
            index: entry.index,
        })
    }

    /// List all keys for a component (OpenRPC: keys.listForComponent)
    ///
    /// # Arguments
    ///
    /// * `component_id` - Component ID to list keys for
    ///
    /// # Errors
    ///
    /// Returns an error if cocoon is not initialized
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::KeyService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = KeyService::new("/tmp/storage", &[0u8; 32])?;
    /// let keys = service.list_for_component("com.osnova.wallet")?;
    /// for key in keys {
    ///     println!("Index {}: {}", key.index, key.public_key);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_for_component(&self, component_id: &str) -> Result<Vec<KeyInfo>> {
        let cocoon = self.load_cocoon()?;

        let keys = cocoon
            .list_keys(component_id)
            .into_iter()
            .map(|entry| KeyInfo {
                public_key: entry.public_key.clone(),
                index: entry.index,
                key_type: entry.key_type.clone(),
                created: entry.created_at,
            })
            .collect();

        Ok(keys)
    }

    // Private helper methods

    /// Internal method to derive a key at a specific index
    fn derive_at_index_internal(
        &self,
        cocoon: &mut KeyCocoon,
        component_id: &str,
        index: u64,
        key_type: KeyType,
    ) -> Result<KeyDerivationResponse> {
        // Derive the key using HKDF
        let derived_seed = key_derivation::derive_symmetric_key(
            &cocoon.master_key,
            component_id,
            index,
        )?;

        // Generate key pair based on key type
        let (public_key, secret_key) = match key_type {
            KeyType::Ed25519 => Self::generate_ed25519(&derived_seed)?,
            KeyType::X25519 => Self::generate_x25519(&derived_seed)?,
            KeyType::Secp256k1 => Self::generate_secp256k1(&derived_seed)?,
        };

        // Create entry
        let entry = DerivedKeyEntry::new(
            public_key.clone(),
            secret_key,
            component_id.to_string(),
            index,
            key_type,
        );

        let response = KeyDerivationResponse {
            public_key: entry.public_key.clone(),
            index: entry.index,
            created: entry.created_at,
        };

        // Save to cocoon
        cocoon.add_key(entry);
        self.save_cocoon(cocoon)?;

        Ok(response)
    }

    /// Generate Ed25519 key pair from seed
    fn generate_ed25519(seed: &[u8; 32]) -> Result<(String, String)> {
        use ed25519_dalek::{SigningKey, VerifyingKey};
        use base64::{Engine as _, engine::general_purpose};

        let signing_key = SigningKey::from_bytes(seed);
        let verifying_key: VerifyingKey = (&signing_key).into();

        let public_key = general_purpose::STANDARD.encode(verifying_key.as_bytes());
        let secret_key = general_purpose::STANDARD.encode(signing_key.to_bytes());

        Ok((public_key, secret_key))
    }

    /// Generate X25519 key pair from seed
    fn generate_x25519(seed: &[u8; 32]) -> Result<(String, String)> {
        use x25519_dalek::PublicKey;
        use base64::{Engine as _, engine::general_purpose};

        // X25519: public key derived from scalar multiplication with base point
        // The seed directly serves as the secret key
        let secret_bytes = *seed;
        let public = PublicKey::from(secret_bytes);

        let public_key = general_purpose::STANDARD.encode(public.as_bytes());
        let secret_key = general_purpose::STANDARD.encode(&secret_bytes);

        Ok((public_key, secret_key))
    }

    /// Generate Secp256k1 key pair from seed
    fn generate_secp256k1(_seed: &[u8; 32]) -> Result<(String, String)> {
        // TODO: Implement secp256k1 key generation
        // For now, return a placeholder error
        anyhow::bail!("Secp256k1 key generation not yet implemented")
    }

    /// Load cocoon from encrypted storage
    fn load_cocoon(&self) -> Result<KeyCocoon> {
        let encrypted_data = self
            .storage
            .read(&self.cocoon_path, &self.cocoon_key)
            .context("Failed to read key cocoon")?;

        let cocoon: KeyCocoon = serde_json::from_slice(&encrypted_data)
            .context("Failed to deserialize key cocoon")?;

        Ok(cocoon)
    }

    /// Save cocoon to encrypted storage
    fn save_cocoon(&self, cocoon: &KeyCocoon) -> Result<()> {
        let cocoon_json = serde_json::to_vec(cocoon)
            .context("Failed to serialize key cocoon")?;

        self.storage
            .write(&self.cocoon_path, &cocoon_json, &self.cocoon_key)
            .context("Failed to write key cocoon")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_service() -> Result<(KeyService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let cocoon_key = [0u8; 32];
        let service = KeyService::new(temp_dir.path(), &cocoon_key)?;

        // Initialize with a test master key
        let master_key = [1u8; 32];
        service.initialize(&master_key)?;

        Ok((service, temp_dir))
    }

    #[test]
    fn test_initialize() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cocoon_key = [0u8; 32];
        let service = KeyService::new(temp_dir.path(), &cocoon_key)?;

        let master_key = [1u8; 32];
        service.initialize(&master_key)?;

        // Verify cocoon was created
        let cocoon = service.load_cocoon()?;
        assert_eq!(cocoon.master_key, master_key);

        Ok(())
    }

    #[test]
    fn test_derive_first_key() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response = service.derive("com.test.wallet", KeyType::Ed25519)?;

        assert_eq!(response.index, 0);
        assert!(!response.public_key.is_empty());
        assert!(response.created > 0);

        Ok(())
    }

    #[test]
    fn test_derive_multiple_keys() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response1 = service.derive("com.test.wallet", KeyType::Ed25519)?;
        let response2 = service.derive("com.test.wallet", KeyType::Ed25519)?;
        let response3 = service.derive("com.test.wallet", KeyType::Ed25519)?;

        assert_eq!(response1.index, 0);
        assert_eq!(response2.index, 1);
        assert_eq!(response3.index, 2);

        // Each key should be unique
        assert_ne!(response1.public_key, response2.public_key);
        assert_ne!(response2.public_key, response3.public_key);

        Ok(())
    }

    #[test]
    fn test_derive_at_index() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response = service.derive_at_index("com.test.wallet", 5, KeyType::Ed25519)?;

        assert_eq!(response.index, 5);
        assert!(!response.public_key.is_empty());

        Ok(())
    }

    #[test]
    fn test_derive_at_index_idempotent() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response1 = service.derive_at_index("com.test.wallet", 5, KeyType::Ed25519)?;
        let response2 = service.derive_at_index("com.test.wallet", 5, KeyType::Ed25519)?;

        assert_eq!(response1.public_key, response2.public_key);
        assert_eq!(response1.index, response2.index);

        Ok(())
    }

    #[test]
    fn test_get_by_public_key() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let derive_response = service.derive("com.test.wallet", KeyType::Ed25519)?;
        let secret_response = service.get_by_public_key(&derive_response.public_key)?;

        assert_eq!(secret_response.component_id, "com.test.wallet");
        assert_eq!(secret_response.index, 0);
        assert!(!secret_response.secret_key.is_empty());

        Ok(())
    }

    #[test]
    fn test_get_by_public_key_not_found() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let result = service.get_by_public_key("nonexistent-key");
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_list_for_component() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.derive("com.test.wallet", KeyType::Ed25519)?;
        service.derive("com.test.wallet", KeyType::Ed25519)?;
        service.derive("com.other.app", KeyType::Ed25519)?;

        let wallet_keys = service.list_for_component("com.test.wallet")?;
        assert_eq!(wallet_keys.len(), 2);

        let other_keys = service.list_for_component("com.other.app")?;
        assert_eq!(other_keys.len(), 1);

        Ok(())
    }

    #[test]
    fn test_list_for_component_empty() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let keys = service.list_for_component("com.nonexistent.app")?;
        assert_eq!(keys.len(), 0);

        Ok(())
    }

    #[test]
    fn test_component_isolation() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response1 = service.derive_at_index("com.wallet.a", 0, KeyType::Ed25519)?;
        let response2 = service.derive_at_index("com.wallet.b", 0, KeyType::Ed25519)?;

        // Same index, different components should have different keys
        assert_ne!(response1.public_key, response2.public_key);

        Ok(())
    }

    #[test]
    fn test_deterministic_derivation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cocoon_key = [0u8; 32];
        let master_key = [1u8; 32];

        // First service instance
        let response1 = {
            let service = KeyService::new(temp_dir.path(), &cocoon_key)?;
            service.initialize(&master_key)?;
            service.derive_at_index("com.test.wallet", 3, KeyType::Ed25519)?
        };

        // Second service instance (simulates restart)
        let response2 = {
            let service = KeyService::new(temp_dir.path(), &cocoon_key)?;
            service.derive_at_index("com.test.wallet", 3, KeyType::Ed25519)?
        };

        // Same master key, component, and index should produce same key
        assert_eq!(response1.public_key, response2.public_key);

        Ok(())
    }

    #[test]
    fn test_x25519_key_generation() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let response = service.derive("com.test.encryption", KeyType::X25519)?;

        assert_eq!(response.index, 0);
        assert!(!response.public_key.is_empty());

        Ok(())
    }
}
