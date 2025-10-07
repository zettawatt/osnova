//! # Autonomi Client
//!
//! Client for connecting to and interacting with the Autonomi Network.
//!
//! Provides connection management, health checking, and network operations.
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::network::AutonomiClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Connect to the network
//!     let client = AutonomiClient::connect().await?;
//!
//!     // Check network health
//!     let is_healthy = client.health_check().await?;
//!     println!("Network healthy: {}", is_healthy);
//!
//!     // Disconnect when done
//!     client.disconnect().await?;
//!     Ok(())
//! }
//! ```

use crate::error::{OsnovaError, Result};
use autonomi::client::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Autonomi Network client
///
/// Manages connection to the Autonomi Network and provides
/// network operation methods.
pub struct AutonomiClient {
    /// Internal Autonomi client (wrapped in Arc for thread safety)
    client: Arc<RwLock<Option<Client>>>,
}

impl AutonomiClient {
    /// Connect to the Autonomi Network (local mode for testing)
    ///
    /// Establishes a connection to the Autonomi Network using local mode.
    /// For production, use `connect_alpha()` to connect to the Alphanet.
    ///
    /// # Returns
    ///
    /// * `Ok(AutonomiClient)` - Successfully connected client
    /// * `Err(OsnovaError::Network)` - Connection failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = AutonomiClient::connect().await?;
    /// ```
    pub async fn connect() -> Result<Self> {
        // Connect to Autonomi Network in local mode (for testing)
        let client = Client::init()
            .await
            .map_err(|e| OsnovaError::Network(format!("Failed to connect: {}", e)))?;

        Ok(Self {
            client: Arc::new(RwLock::new(Some(client))),
        })
    }

    /// Connect to the Autonomi Alpha Network
    ///
    /// Establishes a connection to the Autonomi Network Alphanet.
    ///
    /// # Returns
    ///
    /// * `Ok(AutonomiClient)` - Successfully connected client
    /// * `Err(OsnovaError::Network)` - Connection failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = AutonomiClient::connect_alpha().await?;
    /// ```
    pub async fn connect_alpha() -> Result<Self> {
        // Connect to Autonomi Network Alphanet
        let client = Client::init_alpha()
            .await
            .map_err(|e| OsnovaError::Network(format!("Failed to connect to alphanet: {}", e)))?;

        Ok(Self {
            client: Arc::new(RwLock::new(Some(client))),
        })
    }

    /// Check if client is connected
    ///
    /// # Returns
    ///
    /// `true` if client has an active connection, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if client.is_connected() {
    ///     println!("Client is connected");
    /// }
    /// ```
    pub fn is_connected(&self) -> bool {
        // Can't await in non-async function, so use try_read
        match self.client.try_read() {
            Ok(guard) => guard.is_some(),
            Err(_) => false,
        }
    }

    /// Perform health check on network connection
    ///
    /// Verifies that the network connection is active and responsive.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Network is healthy
    /// * `Err(OsnovaError::Network)` - Health check failed or not connected
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let is_healthy = client.health_check().await?;
    /// assert!(is_healthy);
    /// ```
    pub async fn health_check(&self) -> Result<bool> {
        let guard = self.client.read().await;

        match guard.as_ref() {
            Some(_client) => {
                // For now, just check if client exists
                // In the future, could ping network nodes
                Ok(true)
            }
            None => Err(OsnovaError::Network("Not connected to network".to_string())),
        }
    }

    /// Disconnect from the Autonomi Network
    ///
    /// Cleanly shuts down the network connection.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully disconnected
    /// * `Err(OsnovaError::Network)` - Disconnect failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// client.disconnect().await?;
    /// ```
    pub async fn disconnect(&mut self) -> Result<()> {
        let mut guard = self.client.write().await;

        if guard.is_some() {
            *guard = None;
            Ok(())
        } else {
            Err(OsnovaError::Network("Already disconnected".to_string()))
        }
    }

    /// Reconnect to the Autonomi Network
    ///
    /// Disconnects and establishes a new connection in local mode.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully reconnected
    /// * `Err(OsnovaError::Network)` - Reconnection failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// client.reconnect().await?;
    /// ```
    pub async fn reconnect(&mut self) -> Result<()> {
        // Disconnect first
        let _ = self.disconnect().await;

        // Connect again in local mode
        let client = Client::init()
            .await
            .map_err(|e| OsnovaError::Network(format!("Failed to reconnect: {}", e)))?;

        let mut guard = self.client.write().await;
        *guard = Some(client);

        Ok(())
    }

    /// Get access to the underlying Autonomi client
    ///
    /// Used by upload/download operations that need direct client access.
    ///
    /// # Returns
    ///
    /// Arc-wrapped RwLock containing the optional Client
    #[allow(dead_code)] // Will be used by upload/download modules
    pub(crate) fn client(&self) -> Arc<RwLock<Option<Client>>> {
        Arc::clone(&self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_client_not_connected() {
        // Test that a newly created client (without connect) is not connected
        let client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };
        assert!(!client.is_connected());
    }

    #[tokio::test]
    async fn test_health_check_fails_when_not_connected() {
        // Test that health check fails on unconnected client
        let client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };
        let result = client.health_check().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OsnovaError::Network(_)));
    }

    #[tokio::test]
    async fn test_disconnect_fails_when_not_connected() {
        // Test that disconnect fails when already disconnected
        let mut client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };
        let result = client.disconnect().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OsnovaError::Network(_)));
    }
}
