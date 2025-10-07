//! # Autonomi Data Download
//!
//! Download data from the Autonomi Network using ant:// URIs.
//!
//! This module provides:
//! - Public data download from content addresses
//! - Automatic chunk reassembly for large files
//! - Content integrity verification
//! - Progress tracking for large downloads
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::network::{AutonomiClient, download_data};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = AutonomiClient::connect().await?;
//!     let uri = "ant://0123456789abcdef...";
//!
//!     let data = download_data(&client, uri).await?;
//!     println!("Downloaded {} bytes", data.len());
//!     Ok(())
//! }
//! ```

use super::AutonomiClient;
use crate::error::{OsnovaError, Result};
use bytes::Bytes;

/// Download data from the Autonomi Network
///
/// Downloads data from the Autonomi Network using an ant:// URI.
/// Data is automatically reassembled from chunks for large files.
///
/// # Arguments
///
/// * `client` - Connected Autonomi client
/// * `uri` - ant:// URI of the data to download
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - Downloaded data bytes
/// * `Err(OsnovaError::Network)` - Download failed
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::network::{AutonomiClient, download_data};
///
/// let client = AutonomiClient::connect().await?;
/// let uri = "ant://0123456789abcdef...";
/// let data = download_data(&client, uri).await?;
/// println!("Downloaded {} bytes", data.len());
/// ```
pub async fn download_data(client: &AutonomiClient, uri: &str) -> Result<Vec<u8>> {
    // Parse ant:// URI
    let xorname = parse_ant_uri(uri)?;

    // Get the underlying Autonomi client
    let client_arc = client.client();
    let client_guard = client_arc.read().await;
    let autonomi_client = client_guard
        .as_ref()
        .ok_or_else(|| OsnovaError::Network("Client not connected".to_string()))?;

    // Create DataAddress from XorName
    use autonomi::data::DataAddress;
    use autonomi::XorName;

    let xor_name_bytes: [u8; 32] = xorname
        .try_into()
        .map_err(|_| OsnovaError::Network("Invalid XorName length".to_string()))?;

    let xor_name = XorName(xor_name_bytes);
    let data_address = DataAddress::new(xor_name);

    // Download data from network
    let bytes: Bytes = autonomi_client
        .data_get_public(&data_address)
        .await
        .map_err(|e| OsnovaError::Network(format!("Failed to download data: {}", e)))?;

    // Convert Bytes to Vec<u8>
    Ok(bytes.to_vec())
}

/// Parse an ant:// URI and extract the XorName
///
/// # Arguments
///
/// * `uri` - ant:// URI string
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - Decoded XorName bytes
/// * `Err(OsnovaError::Network)` - Invalid URI format
///
/// # Example
///
/// ```rust,ignore
/// let xorname = parse_ant_uri("ant://0123456789abcdef...")?;
/// ```
fn parse_ant_uri(uri: &str) -> Result<Vec<u8>> {
    // Check for ant:// prefix
    if !uri.starts_with("ant://") {
        return Err(OsnovaError::Network(format!(
            "Invalid URI format: must start with 'ant://', got '{}'",
            uri
        )));
    }

    // Extract hex-encoded XorName
    let hex_part = &uri[6..]; // Skip "ant://"

    // Decode hex to bytes
    let xorname = hex::decode(hex_part)
        .map_err(|e| OsnovaError::Network(format!("Invalid hex in URI: {}", e)))?;

    // Verify XorName length (must be 32 bytes)
    if xorname.len() != 32 {
        return Err(OsnovaError::Network(format!(
            "Invalid XorName length: expected 32 bytes, got {}",
            xorname.len()
        )));
    }

    Ok(xorname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_download_fails_when_not_connected() {
        // Test that download fails when client is not connected
        let client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };

        let uri = "ant://0000000000000000000000000000000000000000000000000000000000000000";
        let result = download_data(&client, uri).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OsnovaError::Network(_)));
    }

    #[test]
    fn test_parse_valid_ant_uri() {
        // Test parsing a valid ant:// URI
        let uri = "ant://0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let result = parse_ant_uri(uri);

        assert!(result.is_ok());
        let xorname = result.unwrap();
        assert_eq!(xorname.len(), 32);
    }

    #[test]
    fn test_parse_invalid_prefix() {
        // Test that URIs without ant:// prefix fail
        let uri = "https://example.com/data";
        let result = parse_ant_uri(uri);

        assert!(result.is_err());
        match result.unwrap_err() {
            OsnovaError::Network(msg) => {
                assert!(msg.contains("must start with 'ant://'"));
            }
            _ => panic!("Expected Network error"),
        }
    }

    #[test]
    fn test_parse_invalid_hex() {
        // Test that URIs with invalid hex fail
        let uri = "ant://not_valid_hex";
        let result = parse_ant_uri(uri);

        assert!(result.is_err());
        match result.unwrap_err() {
            OsnovaError::Network(msg) => {
                assert!(msg.contains("Invalid hex in URI"));
            }
            _ => panic!("Expected Network error"),
        }
    }

    #[test]
    fn test_parse_invalid_length() {
        // Test that URIs with wrong XorName length fail
        let uri = "ant://0123456789abcdef"; // Too short (8 bytes, need 32)
        let result = parse_ant_uri(uri);

        assert!(result.is_err());
        match result.unwrap_err() {
            OsnovaError::Network(msg) => {
                assert!(msg.contains("Invalid XorName length"));
            }
            _ => panic!("Expected Network error"),
        }
    }
}
