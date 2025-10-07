//! Pairing session models for Osnova
//!
//! This module provides the PairingSession type which manages:
//! - Client-server pairing sessions
//! - QR code-based pairing with short-lived codes
//! - Session lifecycle (pending -> established | failed)
//! - Public key exchange for mutual authentication
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::models::pairing::{PairingSession, PairingStatus};
//!
//! // Create new pairing session
//! let session = PairingSession::new(
//!     "session-123",
//!     &server_public_key,
//!     &device_public_key,
//! ).unwrap();
//!
//! // Mark as established
//! let mut session = session;
//! session.mark_established();
//! assert_eq!(session.status(), PairingStatus::Established);
//! ```

use crate::{OsnovaError, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Pairing session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PairingStatus {
    /// Pairing initiated but not yet completed
    Pending,
    /// Pairing successfully established
    Established,
    /// Pairing failed
    Failed,
}

/// Pairing session for client-server authentication
///
/// Represents a pairing session between a client device and a server.
/// Sessions progress through states: pending -> established | failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairingSession {
    /// Unique session identifier
    session_id: String,

    /// Server's Ed25519 public key (32 bytes)
    server_public_key: Vec<u8>,

    /// Device's Ed25519 public key (32 bytes)
    device_public_key: Vec<u8>,

    /// Unix timestamp when session was established (None if pending/failed)
    established_at: Option<u64>,

    /// Unix timestamp when session expires (None if no expiry)
    expires_at: Option<u64>,

    /// Session status
    status: PairingStatus,
}

impl PairingSession {
    /// Create a new pairing session
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique session identifier
    /// * `server_public_key` - Server's Ed25519 public key (must be 32 bytes)
    /// * `device_public_key` - Device's Ed25519 public key (must be 32 bytes)
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::pairing::{PairingSession, PairingStatus};
    ///
    /// let server_key = vec![1u8; 32];
    /// let device_key = vec![2u8; 32];
    /// let session = PairingSession::new("session-123", &server_key, &device_key)
    ///     .expect("Failed to create session");
    ///
    /// assert_eq!(session.session_id(), "session-123");
    /// assert_eq!(session.status(), PairingStatus::Pending);
    /// ```
    pub fn new(
        session_id: impl Into<String>,
        server_public_key: &[u8],
        device_public_key: &[u8],
    ) -> Result<Self> {
        if server_public_key.len() != 32 {
            return Err(OsnovaError::Identity(format!(
                "Server public key must be 32 bytes, got {}",
                server_public_key.len()
            )));
        }

        if device_public_key.len() != 32 {
            return Err(OsnovaError::Identity(format!(
                "Device public key must be 32 bytes, got {}",
                device_public_key.len()
            )));
        }

        Ok(Self {
            session_id: session_id.into(),
            server_public_key: server_public_key.to_vec(),
            device_public_key: device_public_key.to_vec(),
            established_at: None,
            expires_at: None,
            status: PairingStatus::Pending,
        })
    }

    /// Create a pairing session with expiry time
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique session identifier
    /// * `server_public_key` - Server's Ed25519 public key (must be 32 bytes)
    /// * `device_public_key` - Device's Ed25519 public key (must be 32 bytes)
    /// * `expires_at` - Unix timestamp when session expires
    pub fn with_expiry(
        session_id: impl Into<String>,
        server_public_key: &[u8],
        device_public_key: &[u8],
        expires_at: u64,
    ) -> Result<Self> {
        let mut session = Self::new(session_id, server_public_key, device_public_key)?;
        session.expires_at = Some(expires_at);
        Ok(session)
    }

    /// Get the session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the server public key
    pub fn server_public_key(&self) -> &[u8] {
        &self.server_public_key
    }

    /// Get the device public key
    pub fn device_public_key(&self) -> &[u8] {
        &self.device_public_key
    }

    /// Get the established timestamp
    pub fn established_at(&self) -> Option<u64> {
        self.established_at
    }

    /// Get the expiry timestamp
    pub fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }

    /// Get the session status
    pub fn status(&self) -> PairingStatus {
        self.status
    }

    /// Check if the session is pending
    pub fn is_pending(&self) -> bool {
        self.status == PairingStatus::Pending
    }

    /// Check if the session is established
    pub fn is_established(&self) -> bool {
        self.status == PairingStatus::Established
    }

    /// Check if the session has failed
    pub fn is_failed(&self) -> bool {
        self.status == PairingStatus::Failed
    }

    /// Check if the session has expired
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::pairing::PairingSession;
    ///
    /// let server_key = vec![1u8; 32];
    /// let device_key = vec![2u8; 32];
    ///
    /// // Session that expired in the past
    /// let session = PairingSession::with_expiry("session-123", &server_key, &device_key, 1000)
    ///     .expect("Failed to create session");
    /// assert!(session.is_expired());
    ///
    /// // Session with no expiry
    /// let session = PairingSession::new("session-456", &server_key, &device_key)
    ///     .expect("Failed to create session");
    /// assert!(!session.is_expired());
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = Self::current_timestamp();
            now > expires_at
        } else {
            false
        }
    }

    /// Mark the session as established
    ///
    /// Updates the status to Established and sets the established_at timestamp.
    /// This is idempotent - marking an already-established session has no effect.
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::pairing::{PairingSession, PairingStatus};
    ///
    /// let server_key = vec![1u8; 32];
    /// let device_key = vec![2u8; 32];
    /// let mut session = PairingSession::new("session-123", &server_key, &device_key).unwrap();
    ///
    /// assert_eq!(session.status(), PairingStatus::Pending);
    ///
    /// session.mark_established();
    /// assert_eq!(session.status(), PairingStatus::Established);
    /// assert!(session.established_at().is_some());
    /// ```
    pub fn mark_established(&mut self) {
        if self.status == PairingStatus::Pending {
            self.status = PairingStatus::Established;
            self.established_at = Some(Self::current_timestamp());
        }
    }

    /// Mark the session as established at a specific timestamp (for testing/imports)
    pub fn mark_established_at(&mut self, timestamp: u64) {
        if self.status == PairingStatus::Pending {
            self.status = PairingStatus::Established;
            self.established_at = Some(timestamp);
        }
    }

    /// Mark the session as failed
    ///
    /// Updates the status to Failed.
    /// This is idempotent - marking an already-failed session has no effect.
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::pairing::{PairingSession, PairingStatus};
    ///
    /// let server_key = vec![1u8; 32];
    /// let device_key = vec![2u8; 32];
    /// let mut session = PairingSession::new("session-123", &server_key, &device_key).unwrap();
    ///
    /// session.mark_failed();
    /// assert_eq!(session.status(), PairingStatus::Failed);
    /// ```
    pub fn mark_failed(&mut self) {
        if self.status == PairingStatus::Pending {
            self.status = PairingStatus::Failed;
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

    fn sample_server_key() -> Vec<u8> {
        vec![1u8; 32]
    }

    fn sample_device_key() -> Vec<u8> {
        vec![2u8; 32]
    }

    #[test]
    fn test_pairing_status_serialization() {
        assert_eq!(
            serde_json::to_string(&PairingStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&PairingStatus::Established).unwrap(),
            "\"established\""
        );
        assert_eq!(
            serde_json::to_string(&PairingStatus::Failed).unwrap(),
            "\"failed\""
        );
    }

    #[test]
    fn test_new_pairing_session() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        assert_eq!(session.session_id(), "session-123");
        assert_eq!(session.server_public_key(), &server_key[..]);
        assert_eq!(session.device_public_key(), &device_key[..]);
        assert_eq!(session.status(), PairingStatus::Pending);
        assert_eq!(session.established_at(), None);
        assert_eq!(session.expires_at(), None);
        assert!(session.is_pending());
        assert!(!session.is_established());
        assert!(!session.is_failed());
    }

    #[test]
    fn test_new_with_invalid_server_key_length() {
        let invalid_key = vec![1u8; 16];
        let device_key = sample_device_key();

        let result = PairingSession::new("session-123", &invalid_key, &device_key);

        assert!(result.is_err());
        match result {
            Err(OsnovaError::Identity(msg)) => {
                assert!(msg.contains("Server public key must be 32 bytes"));
            }
            _ => panic!("Expected Identity error"),
        }
    }

    #[test]
    fn test_new_with_invalid_device_key_length() {
        let server_key = sample_server_key();
        let invalid_key = vec![2u8; 16];

        let result = PairingSession::new("session-123", &server_key, &invalid_key);

        assert!(result.is_err());
        match result {
            Err(OsnovaError::Identity(msg)) => {
                assert!(msg.contains("Device public key must be 32 bytes"));
            }
            _ => panic!("Expected Identity error"),
        }
    }

    #[test]
    fn test_with_expiry() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session = PairingSession::with_expiry("session-123", &server_key, &device_key, 5000)
            .expect("Failed to create session");

        assert_eq!(session.expires_at(), Some(5000));
    }

    #[test]
    fn test_mark_established() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        assert_eq!(session.status(), PairingStatus::Pending);

        session.mark_established();

        assert_eq!(session.status(), PairingStatus::Established);
        assert!(session.established_at().is_some());
        assert!(session.is_established());
        assert!(!session.is_pending());
        assert!(!session.is_failed());
    }

    #[test]
    fn test_mark_established_idempotent() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        session.mark_established();
        let first_established_at = session.established_at().unwrap();

        // Marking again should not change the timestamp
        session.mark_established();
        assert_eq!(session.established_at().unwrap(), first_established_at);
    }

    #[test]
    fn test_mark_established_at() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        session.mark_established_at(5000);

        assert_eq!(session.status(), PairingStatus::Established);
        assert_eq!(session.established_at(), Some(5000));
    }

    #[test]
    fn test_mark_failed() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        session.mark_failed();

        assert_eq!(session.status(), PairingStatus::Failed);
        assert!(session.is_failed());
        assert!(!session.is_pending());
        assert!(!session.is_established());
    }

    #[test]
    fn test_mark_failed_idempotent() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        session.mark_failed();
        let status = session.status();

        session.mark_failed();
        assert_eq!(session.status(), status);
    }

    #[test]
    fn test_is_expired_with_expiry() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        // Expired session (timestamp in the past)
        let session = PairingSession::with_expiry("session-123", &server_key, &device_key, 1000)
            .expect("Failed to create session");
        assert!(session.is_expired());

        // Future expiry
        let far_future = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;
        let session =
            PairingSession::with_expiry("session-456", &server_key, &device_key, far_future)
                .expect("Failed to create session");
        assert!(!session.is_expired());
    }

    #[test]
    fn test_is_expired_without_expiry() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        assert!(!session.is_expired());
    }

    #[test]
    fn test_serialization() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        let json = serde_json::to_string(&session).expect("Failed to serialize");
        let deserialized: PairingSession =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(session, deserialized);
    }

    #[test]
    fn test_serialization_established() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let mut session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        session.mark_established_at(5000);

        let json = serde_json::to_string(&session).expect("Failed to serialize");
        let deserialized: PairingSession =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(session, deserialized);
        assert!(deserialized.is_established());
        assert_eq!(deserialized.established_at(), Some(5000));
    }

    #[test]
    fn test_clone() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();
        let session = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");

        let cloned = session.clone();
        assert_eq!(session, cloned);
    }

    #[test]
    fn test_equality() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session1 = PairingSession::with_expiry("session-123", &server_key, &device_key, 5000)
            .expect("Failed to create session");
        let session2 = PairingSession::with_expiry("session-123", &server_key, &device_key, 5000)
            .expect("Failed to create session");

        assert_eq!(session1, session2);
    }

    #[test]
    fn test_different_sessions_not_equal() {
        let server_key = sample_server_key();
        let device_key = sample_device_key();

        let session1 = PairingSession::new("session-123", &server_key, &device_key)
            .expect("Failed to create session");
        let session2 = PairingSession::new("session-456", &server_key, &device_key)
            .expect("Failed to create session");

        assert_ne!(session1, session2);
    }
}
