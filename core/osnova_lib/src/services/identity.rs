use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::identity::RootIdentity;
use crate::storage::FileStorage;

/// Identity status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityStatus {
    /// Whether an identity has been initialized
    pub initialized: bool,
    /// 4-word address if identity exists (None if not initialized)
    pub address: Option<String>,
}

/// Identity service for managing user identity
///
/// Provides OpenRPC methods:
/// - `identity.status` - Check if identity is initialized
/// - `identity.create` - Create new identity
/// - `identity.importWithPhrase` - Import existing identity
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::IdentityService;
/// use osnova_lib::platform::paths::get_data_dir;
///
/// # async fn example() -> anyhow::Result<()> {
/// let storage_path = get_data_dir()?;
/// let service = IdentityService::new(&storage_path)?;
///
/// // Check status
/// let status = service.status()?;
/// if !status.initialized {
///     // Create new identity
///     let (phrase, address) = service.create()?;
///     println!("Identity created: {}", address);
/// }
/// # Ok(())
/// # }
/// ```
pub struct IdentityService {
    storage: FileStorage,
    identity_path: PathBuf,
}

impl IdentityService {
    /// Create a new identity service
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Base path for identity storage
    ///
    /// # Errors
    ///
    /// Returns an error if storage cannot be initialized
    pub fn new<P: Into<PathBuf>>(storage_path: P) -> Result<Self> {
        let storage_path = storage_path.into();
        let storage = FileStorage::new(&storage_path)?;
        let identity_path = PathBuf::from("identity/root.enc");

        Ok(Self {
            storage,
            identity_path,
        })
    }

    /// Check identity status (OpenRPC: identity.status)
    ///
    /// Returns whether an identity has been initialized and its 4-word address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::IdentityService;
    /// # use osnova_lib::platform::paths::get_data_dir;
    /// # fn example() -> anyhow::Result<()> {
    /// let storage_path = get_data_dir()?;
    /// let service = IdentityService::new(&storage_path)?;
    /// let status = service.status()?;
    /// println!("Initialized: {}", status.initialized);
    /// # Ok(())
    /// # }
    /// ```
    pub fn status(&self) -> Result<IdentityStatus> {
        if !self.storage.exists(&self.identity_path) {
            return Ok(IdentityStatus {
                initialized: false,
                address: None,
            });
        }

        // Try to load identity to get address
        // Note: We use a placeholder key here - in production this would come from
        // platform keystore (DPAPI/Keychain/Secret Service)
        let platform_key = Self::get_platform_key()?;

        match self.load_identity(&platform_key) {
            Ok(identity) => Ok(IdentityStatus {
                initialized: true,
                address: Some(Self::derive_address(&identity)),
            }),
            Err(e) => {
                // For debugging: log the error
                eprintln!("Failed to load identity: {}", e);
                Ok(IdentityStatus {
                    initialized: false,
                    address: None,
                })
            }
        }
    }

    /// Create a new identity (OpenRPC: identity.create)
    ///
    /// Generates a new 12-word seed phrase and derives the identity.
    /// Returns the seed phrase (for backup) and the 4-word address.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Identity already exists
    /// - Identity cannot be generated
    /// - Identity cannot be saved
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::IdentityService;
    /// # use osnova_lib::platform::paths::get_data_dir;
    /// # async fn example() -> anyhow::Result<()> {
    /// let storage_path = get_data_dir()?;
    /// let service = IdentityService::new(&storage_path)?;
    /// let (seed_phrase, address) = service.create()?;
    /// println!("BACKUP THIS SEED PHRASE: {}", seed_phrase);
    /// println!("Your address: {}", address);
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self) -> Result<(String, String)> {
        // Check if identity already exists
        if self.storage.exists(&self.identity_path) {
            anyhow::bail!("Identity already exists. Use importWithPhrase to restore from backup.");
        }

        // Generate new identity
        let identity = RootIdentity::generate()?;
        let seed_phrase = identity.seed_phrase().to_string();
        let address = Self::derive_address(&identity);

        // Save identity
        let platform_key = Self::get_platform_key()?;
        self.save_identity(&identity, &platform_key)?;

        Ok((seed_phrase, address))
    }

    /// Import identity from seed phrase (OpenRPC: identity.importWithPhrase)
    ///
    /// Restores identity from a 12-word seed phrase backup.
    ///
    /// # Arguments
    ///
    /// * `seed_phrase` - 12-word BIP39 seed phrase
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Identity already exists
    /// - Seed phrase is invalid
    /// - Identity cannot be saved
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::IdentityService;
    /// # use osnova_lib::platform::paths::get_data_dir;
    /// # async fn example() -> anyhow::Result<()> {
    /// let storage_path = get_data_dir()?;
    /// let service = IdentityService::new(&storage_path)?;
    /// let seed = "witch collapse practice feed shame open despair creek road again ice least";
    /// let address = service.import_with_phrase(seed)?;
    /// println!("Identity restored: {}", address);
    /// # Ok(())
    /// # }
    /// ```
    pub fn import_with_phrase(&self, seed_phrase: &str) -> Result<String> {
        // Check if identity already exists
        if self.storage.exists(&self.identity_path) {
            anyhow::bail!("Identity already exists. Delete existing identity first.");
        }

        // Create identity from seed phrase
        let identity = RootIdentity::from_seed(seed_phrase)?;
        let address = Self::derive_address(&identity);

        // Save identity
        let platform_key = Self::get_platform_key()?;
        self.save_identity(&identity, &platform_key)?;

        Ok(address)
    }

    /// Get the root identity (if initialized)
    ///
    /// Returns the RootIdentity for internal use by other services.
    ///
    /// # Errors
    ///
    /// Returns an error if identity is not initialized or cannot be loaded
    pub fn get_identity(&self) -> Result<RootIdentity> {
        let platform_key = Self::get_platform_key()?;
        self.load_identity(&platform_key)
    }

    /// Delete the identity
    ///
    /// WARNING: This permanently deletes the identity. Ensure seed phrase is backed up.
    ///
    /// # Errors
    ///
    /// Returns an error if identity cannot be deleted
    pub fn delete_identity(&self) -> Result<()> {
        self.storage.delete(&self.identity_path)?;
        Ok(())
    }

    // Private helper methods

    /// Load identity from encrypted storage
    fn load_identity(&self, encryption_key: &[u8; 32]) -> Result<RootIdentity> {
        let encrypted_data = self
            .storage
            .read(&self.identity_path, encryption_key)
            .context("Failed to read identity from storage")?;

        // Deserialize the seed phrase
        let seed_phrase: String =
            serde_json::from_slice(&encrypted_data).context("Failed to deserialize seed phrase")?;

        // Reconstruct identity from seed phrase
        let identity = RootIdentity::from_seed(&seed_phrase)
            .context("Failed to reconstruct identity from seed phrase")?;

        Ok(identity)
    }

    /// Save identity to encrypted storage
    fn save_identity(&self, identity: &RootIdentity, encryption_key: &[u8; 32]) -> Result<()> {
        // Save only the seed phrase (it's enough to reconstruct everything)
        let seed_phrase = identity.seed_phrase();
        let seed_json =
            serde_json::to_vec(seed_phrase).context("Failed to serialize seed phrase")?;

        self.storage
            .write(&self.identity_path, &seed_json, encryption_key)
            .context("Failed to write identity to storage")?;

        Ok(())
    }

    /// Get platform-specific encryption key
    ///
    /// In production, this should integrate with:
    /// - Windows: DPAPI (Credential Manager)
    /// - macOS: Keychain Services
    /// - Linux: Secret Service API (GNOME Keyring/KWallet)
    /// - Android/iOS: Platform keystores
    ///
    /// For now, we use a deterministic key for development.
    fn get_platform_key() -> Result<[u8; 32]> {
        // TODO: Implement platform-specific keystore integration
        // For now, use a deterministic development key
        // In production, this should be stored in the platform keystore

        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-platform-key-v1");

        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_bytes());

        Ok(key)
    }

    /// Derive a 4-word address from identity
    ///
    /// TODO: This should use saorsa-core to generate the proper 4-word address.
    /// For now, we use a simple derivation from the fingerprint.
    fn derive_address(identity: &RootIdentity) -> String {
        use bip39::Mnemonic;

        // Use fingerprint to generate a 4-word mnemonic as the address
        let fingerprint = identity.fingerprint();

        // Take first 11 bits (4 words * 11 bits/word = 44 bits needed, we'll use 128 bits and take first 4 words)
        let entropy = &fingerprint[0..16];

        // Generate a mnemonic and take first 4 words
        let mnemonic = Mnemonic::from_entropy(entropy).expect("Valid entropy");

        let words: Vec<&str> = mnemonic.words().take(4).collect();
        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_service() -> Result<(IdentityService, TempDir)> {
        let temp_dir = TempDir::new()?;
        let service = IdentityService::new(temp_dir.path())?;
        Ok((service, temp_dir))
    }

    #[test]
    fn test_status_uninitialized() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let status = service.status()?;
        assert!(!status.initialized);
        assert!(status.address.is_none());

        Ok(())
    }

    #[test]
    fn test_create_identity() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let (seed_phrase, address) = service.create()?;

        // Verify seed phrase is 12 words
        assert_eq!(seed_phrase.split_whitespace().count(), 12);

        // Verify address is 4 words
        assert_eq!(address.split_whitespace().count(), 4);

        // Verify status shows initialized
        let status = service.status()?;
        assert!(status.initialized);
        assert_eq!(status.address.unwrap(), address);

        Ok(())
    }

    #[test]
    fn test_create_fails_if_exists() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.create()?;

        // Second create should fail
        let result = service.create();
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_import_with_phrase() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Use a valid BIP39 test vector
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        let address = service.import_with_phrase(seed)?;

        // Verify address is 4 words
        assert_eq!(address.split_whitespace().count(), 4);

        // Verify status shows initialized
        let status = service.status()?;
        assert!(status.initialized);
        assert_eq!(status.address.unwrap(), address);

        Ok(())
    }

    #[test]
    fn test_import_fails_if_exists() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.create()?;

        // Import should fail when identity exists
        let seed = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = service.import_with_phrase(seed);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_import_invalid_phrase() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Invalid seed phrase (not 12 words)
        let result = service.import_with_phrase("invalid seed phrase");
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_get_identity() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        let (_, address) = service.create()?;

        // Get identity
        let identity = service.get_identity()?;
        let retrieved_address = IdentityService::derive_address(&identity);
        assert_eq!(retrieved_address, address);

        Ok(())
    }

    #[test]
    fn test_get_identity_not_initialized() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        // Should fail when not initialized
        let result = service.get_identity();
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_delete_identity() -> Result<()> {
        let (service, _temp) = create_test_service()?;

        service.create()?;

        // Verify initialized
        let status = service.status()?;
        assert!(status.initialized);

        // Delete
        service.delete_identity()?;

        // Verify not initialized
        let status = service.status()?;
        assert!(!status.initialized);

        Ok(())
    }

    #[test]
    fn test_identity_persistence() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let (seed_phrase, address) = {
            let service = IdentityService::new(temp_dir.path())?;
            service.create()?
        };

        // Create new service instance (simulates app restart)
        let service = IdentityService::new(temp_dir.path())?;

        // Identity should still be initialized
        let status = service.status()?;
        assert!(status.initialized);
        assert_eq!(status.address.unwrap(), address);

        // Should be able to retrieve same identity
        let identity = service.get_identity()?;
        let retrieved_address = IdentityService::derive_address(&identity);
        assert_eq!(retrieved_address, address);
        assert_eq!(identity.seed_phrase(), seed_phrase);

        Ok(())
    }
}
