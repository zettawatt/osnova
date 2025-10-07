//! # Osnova Library
//!
//! Core library for the Osnova distributed application framework.
//!
//! This library provides:
//! - Data models for identity, applications, and configuration
//! - Cryptographic key derivation and encryption
//! - Storage layer with SQLite and encrypted file storage
//! - Core services: identity, keys, configuration, and storage
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::services::IdentityService;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let identity_service = IdentityService::new()?;
//!     let status = identity_service.status().await?;
//!     println!("Identity initialized: {}", status.initialized);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Data models for Osnova entities
pub mod models {
    pub mod identity;
    pub mod device_key;
    pub mod application;
    pub mod config_cache;
    pub mod pairing;
}

/// Cryptographic operations (key derivation, encryption)
pub mod crypto {
    // Crypto operations will be implemented in Phase 1 tasks
}

/// Storage layer (SQLite, encrypted files)
pub mod storage {
    // Storage backends will be implemented in Phase 1 tasks
}

/// Core services (identity, keys, config, storage)
pub mod services {
    // Services will be implemented in Phase 1 tasks
}

/// Error types for Osnova operations
pub mod error {
    use thiserror::Error;

    /// Main error type for Osnova operations
    #[derive(Error, Debug)]
    pub enum OsnovaError {
        /// Database operation failed
        #[error("Database error: {0}")]
        Database(String),

        /// Cryptographic operation failed
        #[error("Cryptographic error: {0}")]
        Crypto(String),

        /// Storage operation failed
        #[error("Storage error: {0}")]
        Storage(String),

        /// Identity operation failed
        #[error("Identity error: {0}")]
        Identity(String),

        /// Serialization/deserialization failed
        #[error("Serialization error: {0}")]
        Serialization(#[from] serde_json::Error),

        /// I/O operation failed
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        /// Generic error
        #[error("{0}")]
        Other(String),
    }

    /// Result type alias for Osnova operations
    pub type Result<T> = std::result::Result<T, OsnovaError>;
}

// Re-export commonly used types
pub use error::{OsnovaError, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn test_library_loads() {
        // Basic smoke test to verify the library loads
        assert!(true);
    }
}
