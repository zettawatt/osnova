//! Encryption utilities for Osnova
//!
//! This module provides encryption-at-rest capabilities using cocoon for local file encryption.
//! Cocoon provides ChaCha20-Poly1305 or AES-256-GCM encryption with PBKDF2-SHA256 key derivation.
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::crypto::encryption::CocoonEncryption;
//!
//! // Create encryption instance with key
//! let key = [0u8; 32];
//! let encryption = CocoonEncryption::new(&key);
//!
//! // Encrypt data
//! let plaintext = b"sensitive data";
//! let ciphertext = encryption.encrypt(plaintext)?;
//!
//! // Decrypt data
//! let decrypted = encryption.decrypt(&ciphertext)?;
//! assert_eq!(decrypted, plaintext);
//! ```

use crate::{OsnovaError, Result};
use cocoon::{Error as CocoonError, MiniCocoon};

/// Encryption wrapper using cocoon for file encryption
///
/// Provides simple encrypt/decrypt operations for configuration files,
/// cache data, and other local storage.
pub struct CocoonEncryption {
    key: [u8; 32],
}

impl CocoonEncryption {
    /// Create a new encryption instance with a key
    ///
    /// # Arguments
    ///
    /// * `key` - 256-bit encryption key (derived from master key)
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::crypto::encryption::CocoonEncryption;
    ///
    /// let key = [0u8; 32];
    /// let encryption = CocoonEncryption::new(&key);
    /// ```
    pub fn new(key: &[u8; 32]) -> Self {
        Self { key: *key }
    }

    /// Encrypt data
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    ///
    /// # Returns
    ///
    /// Encrypted data (ciphertext + authentication tag)
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::crypto::encryption::CocoonEncryption;
    ///
    /// let key = [0u8; 32];
    /// let encryption = CocoonEncryption::new(&key);
    ///
    /// let plaintext = b"sensitive configuration data";
    /// let ciphertext = encryption.encrypt(plaintext)
    ///     .expect("Failed to encrypt");
    ///
    /// assert_ne!(ciphertext, plaintext);
    /// assert!(ciphertext.len() > plaintext.len()); // Includes auth tag
    /// ```
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut cocoon = MiniCocoon::from_key(&self.key, &[0u8; 32]);
        cocoon.wrap(plaintext).map_err(Self::map_cocoon_error)
    }

    /// Decrypt data
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - Encrypted data to decrypt
    ///
    /// # Returns
    ///
    /// Decrypted data (plaintext)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Ciphertext is invalid or corrupted
    /// - Authentication tag verification fails
    /// - Wrong key is used
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::crypto::encryption::CocoonEncryption;
    ///
    /// let key = [0u8; 32];
    /// let encryption = CocoonEncryption::new(&key);
    ///
    /// let plaintext = b"sensitive data";
    /// let ciphertext = encryption.encrypt(plaintext).unwrap();
    /// let decrypted = encryption.decrypt(&ciphertext)
    ///     .expect("Failed to decrypt");
    ///
    /// assert_eq!(decrypted, plaintext);
    /// ```
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cocoon = MiniCocoon::from_key(&self.key, &[0u8; 32]);
        cocoon.unwrap(ciphertext).map_err(Self::map_cocoon_error)
    }

    /// Map cocoon errors to OsnovaError
    fn map_cocoon_error(err: CocoonError) -> OsnovaError {
        match err {
            CocoonError::Cryptography => {
                OsnovaError::Crypto("Encryption/decryption failed".to_string())
            }
            CocoonError::TooShort => {
                OsnovaError::Crypto("Ciphertext too short - invalid format".to_string())
            }
            CocoonError::TooLarge => OsnovaError::Crypto("Data too large to encrypt".to_string()),
            CocoonError::UnrecognizedFormat => {
                OsnovaError::Crypto("Unrecognized ciphertext format".to_string())
            }
            CocoonError::Io(io_err) => OsnovaError::Io(io_err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_key() -> [u8; 32] {
        [1u8; 32]
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let plaintext = b"Hello, Osnova!";
        let ciphertext = encryption.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = encryption.decrypt(&ciphertext).expect("Failed to decrypt");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_produces_different_output() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let plaintext = b"test data";
        let ciphertext = encryption.encrypt(plaintext).expect("Failed to encrypt");

        // Ciphertext should be different from plaintext
        assert_ne!(ciphertext, plaintext);

        // Ciphertext should be longer (includes auth tag and format)
        assert!(ciphertext.len() > plaintext.len());
    }

    #[test]
    fn test_encrypt_deterministic() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let plaintext = b"test data";
        let ciphertext1 = encryption.encrypt(plaintext).expect("Failed to encrypt");
        let ciphertext2 = encryption.encrypt(plaintext).expect("Failed to encrypt");

        // MiniCocoon with same key and nonce produces deterministic output
        assert_eq!(ciphertext1, ciphertext2);

        // Both should decrypt to the same plaintext
        let decrypted1 = encryption.decrypt(&ciphertext1).expect("Failed to decrypt");
        let decrypted2 = encryption.decrypt(&ciphertext2).expect("Failed to decrypt");

        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];

        let encryption1 = CocoonEncryption::new(&key1);
        let encryption2 = CocoonEncryption::new(&key2);

        let plaintext = b"secret message";
        let ciphertext = encryption1.encrypt(plaintext).expect("Failed to encrypt");

        // Decrypting with wrong key should fail
        let result = encryption2.decrypt(&ciphertext);
        assert!(result.is_err());

        match result {
            Err(OsnovaError::Crypto(_)) => (),
            _ => panic!("Expected Crypto error"),
        }
    }

    #[test]
    fn test_decrypt_invalid_ciphertext() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        // Try to decrypt invalid data
        let invalid_ciphertext = b"not a valid ciphertext";
        let result = encryption.decrypt(invalid_ciphertext);

        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_too_short() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        // Very short data
        let short_data = &[1, 2, 3];
        let result = encryption.decrypt(short_data);

        assert!(result.is_err());
        match result {
            Err(OsnovaError::Crypto(msg)) => {
                assert!(msg.contains("too short") || msg.contains("Unrecognized"));
            }
            _ => panic!("Expected Crypto error for short ciphertext"),
        }
    }

    #[test]
    fn test_encrypt_empty_data() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let plaintext = b"";
        let ciphertext = encryption.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = encryption.decrypt(&ciphertext).expect("Failed to decrypt");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_large_data() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        // Create 1MB of data
        let plaintext = vec![42u8; 1024 * 1024];
        let ciphertext = encryption.encrypt(&plaintext).expect("Failed to encrypt");
        let decrypted = encryption.decrypt(&ciphertext).expect("Failed to decrypt");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_different_keys_produce_different_ciphertexts() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];

        let encryption1 = CocoonEncryption::new(&key1);
        let encryption2 = CocoonEncryption::new(&key2);

        let plaintext = b"test data";
        let ciphertext1 = encryption1.encrypt(plaintext).expect("Failed to encrypt");
        let ciphertext2 = encryption2.encrypt(plaintext).expect("Failed to encrypt");

        // Different keys should produce different ciphertexts
        assert_ne!(ciphertext1, ciphertext2);
    }

    #[test]
    fn test_json_serialization_encryption() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Config {
            name: String,
            value: u32,
        }

        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let config = Config {
            name: "test".to_string(),
            value: 42,
        };

        // Serialize to JSON and encrypt
        let json = serde_json::to_vec(&config).expect("Failed to serialize");
        let ciphertext = encryption.encrypt(&json).expect("Failed to encrypt");

        // Decrypt and deserialize
        let decrypted = encryption.decrypt(&ciphertext).expect("Failed to decrypt");
        let recovered: Config = serde_json::from_slice(&decrypted).expect("Failed to deserialize");

        assert_eq!(config, recovered);
    }

    #[test]
    fn test_tampering_detection() {
        let key = sample_key();
        let encryption = CocoonEncryption::new(&key);

        let plaintext = b"important data";
        let mut ciphertext = encryption.encrypt(plaintext).expect("Failed to encrypt");

        // Tamper with the ciphertext
        let len = ciphertext.len();
        if let Some(byte) = ciphertext.get_mut(len - 5) {
            *byte ^= 0xFF;
        }

        // Decryption should fail due to authentication tag mismatch
        let result = encryption.decrypt(&ciphertext);
        assert!(result.is_err());
    }
}
