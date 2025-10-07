use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Server connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    /// Not connected to server (stand-alone mode)
    Disconnected,
    /// Connected to server
    Connected,
    /// Attempting to connect
    Connecting,
    /// Connection failed
    Failed,
}

impl Default for ServerStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// Server status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatusResponse {
    /// Current connection status
    pub status: ServerStatus,
    /// Server address if connected
    pub server_address: Option<String>,
    /// Connection timestamp (UNIX epoch seconds)
    pub connected_at: Option<u64>,
    /// Last error message if failed
    pub error: Option<String>,
}

impl ServerStatusResponse {
    /// Create a disconnected status response
    pub fn disconnected() -> Self {
        Self {
            status: ServerStatus::Disconnected,
            server_address: None,
            connected_at: None,
            error: None,
        }
    }

    /// Create a connected status response
    pub fn connected(server_address: String) -> Self {
        Self {
            status: ServerStatus::Connected,
            server_address: Some(server_address),
            connected_at: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            error: None,
        }
    }

    /// Create a connecting status response
    pub fn connecting(server_address: String) -> Self {
        Self {
            status: ServerStatus::Connecting,
            server_address: Some(server_address),
            connected_at: None,
            error: None,
        }
    }

    /// Create a failed status response
    pub fn failed(server_address: String, error: String) -> Self {
        Self {
            status: ServerStatus::Failed,
            server_address: Some(server_address),
            connected_at: None,
            error: Some(error),
        }
    }
}

/// Status management service
///
/// Provides OpenRPC methods:
/// - `status.getServer` - Get current server connection status
///
/// This service tracks the connection state between client and server.
/// In stand-alone mode, status is always Disconnected.
///
/// # Example
///
/// ```no_run
/// use osnova_lib::services::StatusService;
///
/// # fn example() -> anyhow::Result<()> {
/// let service = StatusService::new();
///
/// // Get current server status
/// let status = service.get_server()?;
/// println!("Server status: {:?}", status.status);
/// # Ok(())
/// # }
/// ```
pub struct StatusService {
    // In stand-alone mode, we always return disconnected
    // In future: track actual server connection state
    status: ServerStatus,
    server_address: Option<String>,
}

impl StatusService {
    /// Create a new status service
    ///
    /// Initially starts in disconnected state (stand-alone mode).
    pub fn new() -> Self {
        Self {
            status: ServerStatus::Disconnected,
            server_address: None,
        }
    }

    /// Get the current server connection status (OpenRPC: status.getServer)
    ///
    /// Returns the current connection state and server information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::services::StatusService;
    /// # fn example() -> anyhow::Result<()> {
    /// let service = StatusService::new();
    /// let status = service.get_server()?;
    ///
    /// match status.status {
    ///     osnova_lib::services::ServerStatus::Connected => {
    ///         println!("Connected to {}", status.server_address.unwrap());
    ///     }
    ///     osnova_lib::services::ServerStatus::Disconnected => {
    ///         println!("Running in stand-alone mode");
    ///     }
    ///     _ => {}
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_server(&self) -> Result<ServerStatusResponse> {
        Ok(match self.status {
            ServerStatus::Disconnected => ServerStatusResponse::disconnected(),
            ServerStatus::Connected => {
                ServerStatusResponse::connected(self.server_address.clone().unwrap())
            }
            ServerStatus::Connecting => {
                ServerStatusResponse::connecting(self.server_address.clone().unwrap())
            }
            ServerStatus::Failed => ServerStatusResponse::failed(
                self.server_address.clone().unwrap(),
                "Connection failed".to_string(),
            ),
        })
    }

    /// Set connection status (internal use)
    ///
    /// Updates the current connection state. Used by pairing and connection logic.
    ///
    /// # Arguments
    ///
    /// * `status` - New connection status
    /// * `server_address` - Server address (if applicable)
    pub fn set_status(&mut self, status: ServerStatus, server_address: Option<String>) {
        self.status = status;
        self.server_address = server_address;
    }

    /// Simulate connection to server (for testing)
    #[cfg(test)]
    pub fn connect(&mut self, server_address: String) {
        self.status = ServerStatus::Connected;
        self.server_address = Some(server_address);
    }

    /// Disconnect from server
    #[cfg(test)]
    pub fn disconnect(&mut self) {
        self.status = ServerStatus::Disconnected;
        self.server_address = None;
    }
}

impl Default for StatusService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_status_disconnected() -> Result<()> {
        let service = StatusService::new();
        let status = service.get_server()?;

        assert_eq!(status.status, ServerStatus::Disconnected);
        assert!(status.server_address.is_none());
        assert!(status.connected_at.is_none());
        assert!(status.error.is_none());

        Ok(())
    }

    #[test]
    fn test_connect_to_server() -> Result<()> {
        let mut service = StatusService::new();

        service.connect("192.168.1.100:8080".to_string());
        let status = service.get_server()?;

        assert_eq!(status.status, ServerStatus::Connected);
        assert_eq!(status.server_address.as_deref(), Some("192.168.1.100:8080"));
        assert!(status.connected_at.is_some());
        assert!(status.error.is_none());

        Ok(())
    }

    #[test]
    fn test_disconnect_from_server() -> Result<()> {
        let mut service = StatusService::new();

        // Connect first
        service.connect("192.168.1.100:8080".to_string());

        // Then disconnect
        service.disconnect();
        let status = service.get_server()?;

        assert_eq!(status.status, ServerStatus::Disconnected);
        assert!(status.server_address.is_none());
        assert!(status.connected_at.is_none());

        Ok(())
    }

    #[test]
    fn test_set_status_connecting() -> Result<()> {
        let mut service = StatusService::new();

        service.set_status(
            ServerStatus::Connecting,
            Some("192.168.1.100:8080".to_string()),
        );
        let status = service.get_server()?;

        assert_eq!(status.status, ServerStatus::Connecting);
        assert_eq!(status.server_address.as_deref(), Some("192.168.1.100:8080"));
        assert!(status.connected_at.is_none());

        Ok(())
    }

    #[test]
    fn test_set_status_failed() -> Result<()> {
        let mut service = StatusService::new();

        service.set_status(ServerStatus::Failed, Some("192.168.1.100:8080".to_string()));
        let status = service.get_server()?;

        assert_eq!(status.status, ServerStatus::Failed);
        assert_eq!(status.server_address.as_deref(), Some("192.168.1.100:8080"));
        assert!(status.error.is_some());

        Ok(())
    }

    #[test]
    fn test_status_response_builders() -> Result<()> {
        // Test disconnected
        let disconnected = ServerStatusResponse::disconnected();
        assert_eq!(disconnected.status, ServerStatus::Disconnected);
        assert!(disconnected.server_address.is_none());

        // Test connected
        let connected = ServerStatusResponse::connected("server:8080".to_string());
        assert_eq!(connected.status, ServerStatus::Connected);
        assert_eq!(connected.server_address.as_deref(), Some("server:8080"));
        assert!(connected.connected_at.is_some());

        // Test connecting
        let connecting = ServerStatusResponse::connecting("server:8080".to_string());
        assert_eq!(connecting.status, ServerStatus::Connecting);
        assert!(connecting.connected_at.is_none());

        // Test failed
        let failed = ServerStatusResponse::failed("server:8080".to_string(), "timeout".to_string());
        assert_eq!(failed.status, ServerStatus::Failed);
        assert_eq!(failed.error.as_deref(), Some("timeout"));

        Ok(())
    }
}
