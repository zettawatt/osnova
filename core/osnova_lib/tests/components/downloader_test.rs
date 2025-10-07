//! Tests for component download workflow

use osnova_lib::components::{download_component, ComponentDownloader};
use osnova_lib::manifest::ComponentSchema;
use osnova_lib::network::AutonomiClient;
use osnova_lib::cache::CacheManager;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_download_from_cache() {
    // Test that cached components are retrieved without downloading
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path(), 10 * 1024 * 1024).unwrap();

    let component = ComponentSchema {
        id: "cached-component".to_string(),
        name: "Cached Component".to_string(),
        kind: "frontend".to_string(),
        platform: Some("desktop".to_string()),
        target: None,
        version: "1.0.0".to_string(),
        hash: Some("abc123".to_string()),
        config: None,
    };

    let data = b"cached component data";
    cache.store(&component.id, data).await.unwrap();

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.exists());
}

#[tokio::test]
async fn test_download_from_network() {
    // Test downloading from network when not cached
    match AutonomiClient::connect().await {
        Ok(client) => {
            let temp_dir = TempDir::new().unwrap();
            let cache = CacheManager::new(temp_dir.path(), 10 * 1024 * 1024).unwrap();

            let component = ComponentSchema {
                id: "ant://test123".to_string(),
                name: "Network Component".to_string(),
                kind: "backend".to_string(),
                platform: None,
                target: Some("x86_64-unknown-linux-gnu".to_string()),
                version: "1.0.0".to_string(),
                hash: None,
                config: None,
            };

            let downloader = ComponentDownloader::new(cache, Some(client));
            let result = downloader.download(&component).await;

            // Will likely fail without real data, but tests the code path
            if result.is_err() {
                assert!(result.unwrap_err().to_string().contains("Network") ||
                        result.unwrap_err().to_string().contains("Failed"));
            }
        }
        Err(_) => {
            // Skip if can't connect
        }
    }
}

#[tokio::test]
async fn test_download_local_file() {
    // Test downloading from local file path
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().join("cache"), 10 * 1024 * 1024).unwrap();

    let component_data = b"local component data";
    let component_path = temp_dir.path().join("component.tar.gz");
    fs::write(&component_path, component_data).unwrap();

    let component = ComponentSchema {
        id: format!("file://{}", component_path.display()),
        name: "Local Component".to_string(),
        kind: "frontend".to_string(),
        platform: Some("desktop".to_string()),
        target: None,
        version: "1.0.0".to_string(),
        hash: None,
        config: None,
    };

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_hash_verification() {
    // Test that hash verification works
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().join("cache"), 10 * 1024 * 1024).unwrap();

    let component_data = b"test data for hashing";
    let component_path = temp_dir.path().join("component.bin");
    fs::write(&component_path, component_data).unwrap();

    // Calculate actual hash
    use blake3::Hasher;
    let mut hasher = Hasher::new();
    hasher.update(component_data);
    let hash = hasher.finalize();
    let hash_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash.as_bytes());

    let component = ComponentSchema {
        id: format!("file://{}", component_path.display()),
        name: "Verified Component".to_string(),
        kind: "backend".to_string(),
        platform: None,
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        version: "1.0.0".to_string(),
        hash: Some(hash_b64),
        config: None,
    };

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_hash_verification_fails() {
    // Test that wrong hash causes verification failure
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().join("cache"), 10 * 1024 * 1024).unwrap();

    let component_data = b"test data";
    let component_path = temp_dir.path().join("component.bin");
    fs::write(&component_path, component_data).unwrap();

    let component = ComponentSchema {
        id: format!("file://{}", component_path.display()),
        name: "Invalid Hash Component".to_string(),
        kind: "backend".to_string(),
        platform: None,
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        version: "1.0.0".to_string(),
        hash: Some("invalid_hash_value".to_string()),
        config: None,
    };

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("hash") ||
            result.unwrap_err().to_string().contains("integrity"));
}

#[tokio::test]
async fn test_extract_tarball() {
    // Test extracting frontend tarball
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().join("cache"), 10 * 1024 * 1024).unwrap();

    // Create a simple tarball
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use tar::Builder;

    let tarball_path = temp_dir.path().join("frontend.tar.gz");
    let file = fs::File::create(&tarball_path).unwrap();
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);

    // Add a test file to tarball
    let mut header = tar::Header::new_gnu();
    header.set_path("index.html").unwrap();
    header.set_size(12);
    header.set_cksum();
    tar.append(&header, "Hello World!".as_bytes()).unwrap();
    tar.finish().unwrap();

    let component = ComponentSchema {
        id: format!("file://{}", tarball_path.display()),
        name: "Tarball Component".to_string(),
        kind: "frontend".to_string(),
        platform: Some("desktop".to_string()),
        target: None,
        version: "1.0.0".to_string(),
        hash: None,
        config: None,
    };

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_ok());
    let extracted_path = result.unwrap();
    assert!(extracted_path.exists());
    assert!(extracted_path.join("index.html").exists());
}

#[tokio::test]
async fn test_backend_binary() {
    // Test handling backend binary (no extraction)
    let temp_dir = TempDir::new().unwrap();
    let cache = CacheManager::new(temp_dir.path().join("cache"), 10 * 1024 * 1024).unwrap();

    let binary_data = b"\x7fELF"; // ELF magic bytes
    let binary_path = temp_dir.path().join("backend.bin");
    fs::write(&binary_path, binary_data).unwrap();

    let component = ComponentSchema {
        id: format!("file://{}", binary_path.display()),
        name: "Backend Binary".to_string(),
        kind: "backend".to_string(),
        platform: None,
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        version: "1.0.0".to_string(),
        hash: None,
        config: None,
    };

    let downloader = ComponentDownloader::new(cache, None);
    let result = downloader.download(&component).await;

    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.exists());
}
