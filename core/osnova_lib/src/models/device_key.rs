//! Device key models for Osnova
//!
//! This module provides the DeviceKey type which manages:
//! - Device-specific Ed25519 public keys
//! - Key creation and revocation timestamps
//! - Device key lifecycle management
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::models::device_key::DeviceKey;
//!
//! // Create new device key
//! let device_key = DeviceKey::new("device-123", &public_key_bytes);
//!
//! // Check if revoked
//! if device_key.is_revoked() {
//!     println!("Device key has been revoked");
//! }
//!
//! // Revoke device key
//! device_key.revoke();
//! ```

use crate::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Device key for a specific device
///
/// Each device has a unique public key derived from the root identity.
/// Device keys can be revoked to disable access from compromised devices.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceKey {
    /// Unique device identifier
    device_id: String,

    /// Ed25519 public key (32 bytes)
    public_key: Vec<u8>,

    /// Unix timestamp when key was created
    created_at: u64,

    /// Unix timestamp when key was revoked (None if active)
    revoked_at: Option<u64>,
}

impl DeviceKey {
    /// Create a new device key
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique device identifier
    /// * `public_key` - Ed25519 public key bytes (must be 32 bytes)
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::device_key::DeviceKey;
    ///
    /// let public_key = vec![0u8; 32];
    /// let device_key = DeviceKey::new("device-123", &public_key)
    ///     .expect("Failed to create device key");
    /// assert_eq!(device_key.device_id(), "device-123");
    /// ```
    pub fn new(device_id: impl Into<String>, public_key: &[u8]) -> Result<Self> {
        if public_key.len() != 32 {
            return Err(crate::OsnovaError::Identity(format!(
                "Public key must be 32 bytes, got {}",
                public_key.len()
            )));
        }

        Ok(Self {
            device_id: device_id.into(),
            public_key: public_key.to_vec(),
            created_at: Self::current_timestamp(),
            revoked_at: None,
        })
    }

    /// Create a device key with a specific creation timestamp (for testing/imports)
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique device identifier
    /// * `public_key` - Ed25519 public key bytes (must be 32 bytes)
    /// * `created_at` - Unix timestamp when key was created
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::device_key::DeviceKey;
    ///
    /// let public_key = vec![0u8; 32];
    /// let device_key = DeviceKey::with_timestamp("device-123", &public_key, 1000)
    ///     .expect("Failed to create device key");
    /// assert_eq!(device_key.created_at(), 1000);
    /// ```
    pub fn with_timestamp(
        device_id: impl Into<String>,
        public_key: &[u8],
        created_at: u64,
    ) -> Result<Self> {
        if public_key.len() != 32 {
            return Err(crate::OsnovaError::Identity(format!(
                "Public key must be 32 bytes, got {}",
                public_key.len()
            )));
        }

        Ok(Self {
            device_id: device_id.into(),
            public_key: public_key.to_vec(),
            created_at,
            revoked_at: None,
        })
    }

    /// Get the device ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get the public key bytes
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get the revocation timestamp (None if not revoked)
    pub fn revoked_at(&self) -> Option<u64> {
        self.revoked_at
    }

    /// Check if the device key has been revoked
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::device_key::DeviceKey;
    ///
    /// let public_key = vec![0u8; 32];
    /// let mut device_key = DeviceKey::new("device-123", &public_key).unwrap();
    ///
    /// assert!(!device_key.is_revoked());
    /// device_key.revoke();
    /// assert!(device_key.is_revoked());
    /// ```
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    /// Revoke this device key
    ///
    /// Once revoked, the device key cannot be used for authentication.
    /// This operation is idempotent - revoking an already-revoked key has no effect.
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::device_key::DeviceKey;
    ///
    /// let public_key = vec![0u8; 32];
    /// let mut device_key = DeviceKey::new("device-123", &public_key).unwrap();
    ///
    /// device_key.revoke();
    /// assert!(device_key.is_revoked());
    /// ```
    pub fn revoke(&mut self) {
        if self.revoked_at.is_none() {
            self.revoked_at = Some(Self::current_timestamp());
        }
    }

    /// Revoke this device key at a specific timestamp (for testing/imports)
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Unix timestamp when key was revoked
    pub fn revoke_at(&mut self, timestamp: u64) {
        if self.revoked_at.is_none() {
            self.revoked_at = Some(timestamp);
        }
    }

    /// Get current Unix timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch")
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_public_key() -> Vec<u8> {
        vec![1u8; 32]
    }

    #[test]
    fn test_new_device_key() {
        let public_key = sample_public_key();
        let device_key = DeviceKey::new("device-123", &public_key).expect("Failed to create");

        assert_eq!(device_key.device_id(), "device-123");
        assert_eq!(device_key.public_key(), &public_key[..]);
        assert!(device_key.created_at() > 0);
        assert!(!device_key.is_revoked());
        assert_eq!(device_key.revoked_at(), None);
    }

    #[test]
    fn test_new_with_invalid_key_length() {
        let invalid_key = vec![1u8; 16]; // Wrong length
        let result = DeviceKey::new("device-123", &invalid_key);

        assert!(result.is_err());
        match result {
            Err(crate::OsnovaError::Identity(msg)) => {
                assert!(msg.contains("must be 32 bytes"));
            }
            _ => panic!("Expected Identity error"),
        }
    }

    #[test]
    fn test_with_timestamp() {
        let public_key = sample_public_key();
        let device_key =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");

        assert_eq!(device_key.created_at(), 1000);
    }

    #[test]
    fn test_revoke() {
        let public_key = sample_public_key();
        let mut device_key = DeviceKey::new("device-123", &public_key).expect("Failed to create");

        assert!(!device_key.is_revoked());

        device_key.revoke();

        assert!(device_key.is_revoked());
        assert!(device_key.revoked_at().is_some());
        assert!(device_key.revoked_at().unwrap() > 0);
    }

    #[test]
    fn test_revoke_idempotent() {
        let public_key = sample_public_key();
        let mut device_key = DeviceKey::new("device-123", &public_key).expect("Failed to create");

        device_key.revoke();
        let first_revoked_at = device_key.revoked_at().unwrap();

        // Revoking again should not change the timestamp
        device_key.revoke();
        assert_eq!(device_key.revoked_at().unwrap(), first_revoked_at);
    }

    #[test]
    fn test_revoke_at() {
        let public_key = sample_public_key();
        let mut device_key = DeviceKey::new("device-123", &public_key).expect("Failed to create");

        device_key.revoke_at(5000);

        assert!(device_key.is_revoked());
        assert_eq!(device_key.revoked_at(), Some(5000));
    }

    #[test]
    fn test_serialization() {
        let public_key = sample_public_key();
        let device_key =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");

        let json = serde_json::to_string(&device_key).expect("Failed to serialize");
        let deserialized: DeviceKey = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(device_key, deserialized);
    }

    #[test]
    fn test_serialization_with_revoked() {
        let public_key = sample_public_key();
        let mut device_key =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");
        device_key.revoke_at(2000);

        let json = serde_json::to_string(&device_key).expect("Failed to serialize");
        let deserialized: DeviceKey = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(device_key, deserialized);
        assert!(deserialized.is_revoked());
        assert_eq!(deserialized.revoked_at(), Some(2000));
    }

    #[test]
    fn test_equality() {
        let public_key = sample_public_key();
        let device_key1 =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");
        let device_key2 =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");

        assert_eq!(device_key1, device_key2);
    }

    #[test]
    fn test_different_keys_not_equal() {
        let public_key1 = sample_public_key();
        let mut public_key2 = sample_public_key();
        public_key2[0] = 99;

        let device_key1 =
            DeviceKey::with_timestamp("device-123", &public_key1, 1000).expect("Failed to create");
        let device_key2 =
            DeviceKey::with_timestamp("device-123", &public_key2, 1000).expect("Failed to create");

        assert_ne!(device_key1, device_key2);
    }

    #[test]
    fn test_different_device_ids_not_equal() {
        let public_key = sample_public_key();
        let device_key1 =
            DeviceKey::with_timestamp("device-123", &public_key, 1000).expect("Failed to create");
        let device_key2 =
            DeviceKey::with_timestamp("device-456", &public_key, 1000).expect("Failed to create");

        assert_ne!(device_key1, device_key2);
    }

    #[test]
    fn test_clone() {
        let public_key = sample_public_key();
        let device_key = DeviceKey::new("device-123", &public_key).expect("Failed to create");
        let cloned = device_key.clone();

        assert_eq!(device_key, cloned);
    }
}
