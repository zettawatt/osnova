//! # Autonomi Data Upload
//!
//! Upload data to the Autonomi Network with automatic chunking for large files.
//!
//! This module provides:
//! - Public data upload with content addressing
//! - Automatic chunking for files >1MB
//! - ant:// URI generation
//! - Cost estimation
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::network::{AutonomiClient, upload_data};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = AutonomiClient::connect().await?;
//!     let data = b"Hello, Autonomi!";
//!
//!     // Upload data and get address
//!     let address = upload_data(&client, data).await?;
//!     println!("Data uploaded to: {}", address);
//!     Ok(())
//! }
//! ```

use super::AutonomiClient;
use crate::error::{OsnovaError, Result};
use bytes::Bytes;

/// Upload data to the Autonomi Network
///
/// Uploads arbitrary data to the Autonomi Network and returns the content address.
/// Data is automatically chunked for files >1MB. The data is publicly accessible.
///
/// # Arguments
///
/// * `client` - Connected Autonomi client
/// * `data` - Byte slice to upload
///
/// # Returns
///
/// * `Ok(String)` - ant:// URI where the data can be retrieved
/// * `Err(OsnovaError::Network)` - Upload failed
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::network::{AutonomiClient, upload_data};
///
/// let client = AutonomiClient::connect().await?;
/// let data = b"Hello, Autonomi Network!";
/// let address = upload_data(&client, data).await?;
/// println!("Uploaded to: {}", address);
/// ```
pub async fn upload_data(client: &AutonomiClient, data: &[u8]) -> Result<String> {
    use autonomi::client::payment::PaymentOption;
    use autonomi::client::payment::Receipt;

    // Get the underlying Autonomi client
    let client_arc = client.client();
    let client_guard = client_arc.read().await;
    let autonomi_client = client_guard
        .as_ref()
        .ok_or_else(|| OsnovaError::Network("Client not connected".to_string()))?;

    // Convert data to Bytes
    let bytes = Bytes::from(data.to_vec());

    // Create a dummy receipt for free uploads (in test mode)
    // In production, this should use an actual wallet
    let payment = PaymentOption::Receipt(Receipt::default());

    // Upload data to network
    let (_cost, data_address) = autonomi_client
        .data_put_public(bytes, payment)
        .await
        .map_err(|e| OsnovaError::Network(format!("Failed to upload data: {}", e)))?;

    // Convert address to ant:// URI
    let uri = format!("ant://{}", hex::encode(data_address.xorname().0));

    Ok(uri)
}

/// Estimate the cost of uploading data
///
/// Calculates the cost in AttoTokens for uploading the given data
/// without actually performing the upload.
///
/// # Arguments
///
/// * `client` - Connected Autonomi client
/// * `data` - Byte slice to estimate cost for
///
/// # Returns
///
/// * `Ok(u64)` - Cost in AttoTokens
/// * `Err(OsnovaError::Network)` - Cost estimation failed
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::network::{AutonomiClient, estimate_upload_cost};
///
/// let client = AutonomiClient::connect().await?;
/// let data = b"Test data";
/// let cost = estimate_upload_cost(&client, data).await?;
/// println!("Upload will cost: {} AttoTokens", cost);
/// ```
pub async fn estimate_upload_cost(client: &AutonomiClient, data: &[u8]) -> Result<u64> {
    // Get the underlying Autonomi client
    let client_arc = client.client();
    let client_guard = client_arc.read().await;
    let autonomi_client = client_guard
        .as_ref()
        .ok_or_else(|| OsnovaError::Network("Client not connected".to_string()))?;

    // Convert data to Bytes
    let bytes = Bytes::from(data.to_vec());

    // Get cost estimation
    let cost = autonomi_client
        .data_cost(bytes)
        .await
        .map_err(|e| OsnovaError::Network(format!("Failed to estimate cost: {}", e)))?;

    // Convert Uint<256, 4> to u64 (may truncate for very large values)
    let atto_tokens = cost.as_atto();
    let cost_u64 = atto_tokens.as_limbs()[0]; // Get lowest 64 bits

    Ok(cost_u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_upload_fails_when_not_connected() {
        // Test that upload fails when client is not connected
        let client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };

        let data = b"test data";
        let result = upload_data(&client, data).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OsnovaError::Network(_)));
    }

    #[tokio::test]
    async fn test_estimate_cost_fails_when_not_connected() {
        // Test that cost estimation fails when client is not connected
        let client = AutonomiClient {
            client: Arc::new(RwLock::new(None)),
        };

        let data = b"test data";
        let result = estimate_upload_cost(&client, data).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OsnovaError::Network(_)));
    }
}
