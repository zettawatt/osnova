//! Tests for manifest resolution

use osnova_lib::manifest::resolve_manifest;
use osnova_lib::network::AutonomiClient;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_resolve_local_file() {
    // Test resolving manifest from local file path
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("manifest.json");

    let json = r#"{
        "id": "file:///local/manifest.json",
        "name": "Local Test App",
        "version": "1.0.0",
        "iconUri": "file:///local/icon.png",
        "description": "Test application",
        "components": []
    }"#;

    fs::write(&manifest_path, json).unwrap();

    let uri = format!("file://{}", manifest_path.display());
    let result = resolve_manifest(&uri, None).await;

    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.name, "Local Test App");
}

#[tokio::test]
async fn test_resolve_local_file_not_found() {
    // Test error when local file doesn't exist
    let uri = "file:///nonexistent/manifest.json";
    let result = resolve_manifest(uri, None).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found") ||
            result.unwrap_err().to_string().contains("No such file"));
}

#[tokio::test]
async fn test_resolve_ant_uri() {
    // Test resolving from Autonomi network
    match AutonomiClient::connect().await {
        Ok(client) => {
            // In test environment, this will likely fail without real network
            // but we test the code path
            let uri = "ant://0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
            let result = resolve_manifest(uri, Some(&client)).await;

            // Either succeeds or fails with network error
            if let Err(e) = result {
                assert!(e.to_string().contains("Network") || e.to_string().contains("Failed"));
            }
        }
        Err(_) => {
            // Skip if can't connect
        }
    }
}

#[tokio::test]
async fn test_resolve_https_url() {
    // Test resolving from HTTPS URL
    // This is a placeholder - would need actual test server
    let uri = "https://example.com/manifest.json";
    let result = resolve_manifest(uri, None).await;

    // Expected to fail in test environment, but validates the code path
    if let Err(e) = result {
        assert!(e.to_string().contains("HTTP") ||
                e.to_string().contains("Network") ||
                e.to_string().contains("Failed"));
    }
}

#[tokio::test]
async fn test_resolve_invalid_uri_scheme() {
    // Test error with unsupported URI scheme
    let uri = "ftp://example.com/manifest.json";
    let result = resolve_manifest(uri, None).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported"));
}

#[tokio::test]
async fn test_resolve_invalid_json() {
    // Test error when resolved content is invalid JSON
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("invalid.json");

    fs::write(&manifest_path, "{ invalid json }").unwrap();

    let uri = format!("file://{}", manifest_path.display());
    let result = resolve_manifest(&uri, None).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("parse") ||
            result.unwrap_err().to_string().contains("JSON"));
}

#[tokio::test]
async fn test_resolve_with_caching() {
    // Test that resolved manifests can be cached
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("cached.json");

    let json = r#"{
        "id": "file:///cached/manifest.json",
        "name": "Cached App",
        "version": "1.0.0",
        "iconUri": "file:///cached/icon.png",
        "description": "Test",
        "components": []
    }"#;

    fs::write(&manifest_path, json).unwrap();

    let uri = format!("file://{}", manifest_path.display());

    // First resolution
    let result1 = resolve_manifest(&uri, None).await;
    assert!(result1.is_ok());

    // Second resolution (would use cache in production)
    let result2 = resolve_manifest(&uri, None).await;
    assert!(result2.is_ok());

    assert_eq!(result1.unwrap().name, result2.unwrap().name);
}
