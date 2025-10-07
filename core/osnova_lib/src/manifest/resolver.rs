//! # Manifest Resolver
//!
//! Fetch and resolve application manifests from various sources.
//!
//! Supports:
//! - ant:// URIs (Autonomi Network)
//! - file:// paths (local development)
//! - https:// URLs (fallback/testing)

use super::schema::ManifestSchema;
use super::validator::validate_manifest_bytes;
use crate::error::{OsnovaError, Result};
use crate::network::{AutonomiClient, download_data};

/// Resolve a manifest from a URI
///
/// Fetches manifest from various sources and validates it.
///
/// # Arguments
///
/// * `uri` - Manifest URI (ant://, file://, or https://)
/// * `client` - Optional Autonomi client (required for ant:// URIs)
///
/// # Returns
///
/// * `Ok(ManifestSchema)` - Successfully resolved and validated manifest
/// * `Err(OsnovaError)` - Resolution or validation failed
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::manifest::resolve_manifest;
/// use osnova_lib::network::AutonomiClient;
///
/// // Local file
/// let manifest = resolve_manifest("file:///path/to/manifest.json", None).await?;
///
/// // Autonomi network
/// let client = AutonomiClient::connect_alpha().await?;
/// let manifest = resolve_manifest("ant://...", Some(&client)).await?;
/// ```
pub async fn resolve_manifest(
    uri: &str,
    client: Option<&AutonomiClient>,
) -> Result<ManifestSchema> {
    // Determine source based on URI scheme
    let data = if uri.starts_with("ant://") {
        resolve_from_autonomi(uri, client).await?
    } else if uri.starts_with("file://") {
        resolve_from_file(uri).await?
    } else if uri.starts_with("https://") || uri.starts_with("http://") {
        resolve_from_http(uri).await?
    } else {
        return Err(OsnovaError::Other(format!(
            "Unsupported URI scheme: {} (must be ant://, file://, or https://)",
            uri
        )));
    };

    // Validate manifest
    validate_manifest_bytes(&data)
}

/// Resolve manifest from Autonomi Network
async fn resolve_from_autonomi(
    uri: &str,
    client: Option<&AutonomiClient>,
) -> Result<Vec<u8>> {
    let client = client.ok_or_else(|| {
        OsnovaError::Network("Autonomi client required for ant:// URIs".to_string())
    })?;

    download_data(client, uri).await
}

/// Resolve manifest from local file
async fn resolve_from_file(uri: &str) -> Result<Vec<u8>> {
    // Remove file:// prefix
    let path = uri.strip_prefix("file://").ok_or_else(|| {
        OsnovaError::Other(format!("Invalid file URI: {}", uri))
    })?;

    // Read file
    tokio::fs::read(path)
        .await
        .map_err(|e| OsnovaError::Storage(format!("Failed to read manifest file: {}", e)))
}

/// Resolve manifest from HTTP(S) URL
async fn resolve_from_http(uri: &str) -> Result<Vec<u8>> {
    // Use reqwest for HTTP fetching
    let response = reqwest::get(uri)
        .await
        .map_err(|e| OsnovaError::Network(format!("HTTP request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(OsnovaError::Network(format!(
            "HTTP error {}: {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown")
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| OsnovaError::Network(format!("Failed to read HTTP response: {}", e)))?;

    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unsupported_scheme() {
        let uri = "ftp://example.com/manifest.json";
        let result = resolve_manifest(uri, None).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported"));
    }

    #[tokio::test]
    async fn test_ant_uri_without_client() {
        let uri = "ant://test";
        let result = resolve_manifest(uri, None).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("client required"));
    }

    #[tokio::test]
    async fn test_invalid_file_uri() {
        let uri = "file://";
        let result = resolve_from_file(uri).await;

        assert!(result.is_err());
    }
}
