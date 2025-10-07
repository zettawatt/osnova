//! # Component Downloader
//!
//! Download, cache, and verify application components.
//!
//! Handles:
//! - Checking cache before downloading
//! - Downloading from network or local files
//! - Hash verification
//! - Extracting frontend tarballs
//! - Managing backend binaries

use crate::cache::CacheManager;
use crate::error::{OsnovaError, Result};
use crate::manifest::ComponentSchema;
use crate::network::{download_data, AutonomiClient};
use blake3::Hasher;
use flate2::read::GzDecoder;
use std::path::PathBuf;
use tar::Archive;

/// Component downloader with caching and verification
///
/// Manages the full workflow of downloading, caching, and verifying components.
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::components::ComponentDownloader;
/// use osnova_lib::cache::CacheManager;
///
/// let cache = CacheManager::new("/tmp/cache", 500 * 1024 * 1024)?;
/// let client = AutonomiClient::connect_alpha().await?;
///
/// let downloader = ComponentDownloader::new(cache, Some(client));
/// let path = downloader.download(&component).await?;
/// println!("Component at: {}", path.display());
/// ```
pub struct ComponentDownloader {
    /// Cache manager
    cache: CacheManager,
    /// Optional Autonomi client
    client: Option<AutonomiClient>,
}

impl ComponentDownloader {
    /// Create a new component downloader
    ///
    /// # Arguments
    ///
    /// * `cache` - Cache manager for storing components
    /// * `client` - Optional Autonomi client (required for ant:// URIs)
    pub fn new(cache: CacheManager, client: Option<AutonomiClient>) -> Self {
        Self { cache, client }
    }

    /// Download and prepare a component
    ///
    /// Checks cache first, then downloads if needed. Verifies integrity
    /// and extracts frontend tarballs.
    ///
    /// # Arguments
    ///
    /// * `component` - Component schema with download information
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - Path to the prepared component
    /// * `Err(OsnovaError)` - Download or verification failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let path = downloader.download(&component).await?;
    /// ```
    pub async fn download(&self, component: &ComponentSchema) -> Result<PathBuf> {
        // Check cache first
        let cache_key = Self::cache_key(component);
        if let Some(cached_data) = self.cache.get(&cache_key).await? {
            // Verify hash if provided
            if let Some(expected_hash) = &component.hash {
                Self::verify_hash(&cached_data, expected_hash)?;
            }

            // Return cached component path
            return self.prepare_component(component, &cached_data).await;
        }

        // Download from source
        let data = self.fetch_component(component).await?;

        // Verify hash if provided
        if let Some(expected_hash) = &component.hash {
            Self::verify_hash(&data, expected_hash)?;
        }

        // Store in cache
        self.cache.store(&cache_key, &data).await?;

        // Prepare component (extract if needed)
        self.prepare_component(component, &data).await
    }

    /// Fetch component from source
    async fn fetch_component(&self, component: &ComponentSchema) -> Result<Vec<u8>> {
        let uri = &component.id;

        if uri.starts_with("ant://") {
            let client = self.client.as_ref().ok_or_else(|| {
                OsnovaError::Network("Autonomi client required for ant:// URIs".to_string())
            })?;
            download_data(client, uri).await
        } else if uri.starts_with("file://") {
            let path = uri.strip_prefix("file://").unwrap_or(uri);
            tokio::fs::read(path)
                .await
                .map_err(|e| OsnovaError::Storage(format!("Failed to read component: {}", e)))
        } else if uri.starts_with("https://") || uri.starts_with("http://") {
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
                .map_err(|e| OsnovaError::Network(format!("Failed to read response: {}", e)))?;

            Ok(bytes.to_vec())
        } else {
            Err(OsnovaError::Other(format!(
                "Unsupported component URI scheme: {}",
                uri
            )))
        }
    }

    /// Prepare component for use (extract if needed)
    async fn prepare_component(
        &self,
        component: &ComponentSchema,
        data: &[u8],
    ) -> Result<PathBuf> {
        if component.kind == "frontend" {
            // Frontend components are ZLIB tarballs - extract them
            self.extract_tarball(component, data).await
        } else {
            // Backend components are binaries - write directly
            self.write_binary(component, data).await
        }
    }

    /// Extract frontend tarball
    async fn extract_tarball(
        &self,
        component: &ComponentSchema,
        data: &[u8],
    ) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir();
        let extract_dir = temp_dir.join(format!("osnova-{}-{}", component.name, component.version));

        // Create extraction directory
        tokio::fs::create_dir_all(&extract_dir)
            .await
            .map_err(|e| OsnovaError::Storage(format!("Failed to create extract dir: {}", e)))?;

        // Clone data for spawn_blocking (needs 'static lifetime)
        let data_owned = data.to_vec();

        // Use blocking task for tar extraction (not async-friendly)
        let extract_dir_clone = extract_dir.clone();
        tokio::task::spawn_blocking(move || {
            let decoder = GzDecoder::new(data_owned.as_slice());
            let mut archive = Archive::new(decoder);
            archive
                .unpack(&extract_dir_clone)
                .map_err(|e| OsnovaError::Storage(format!("Failed to extract tarball: {}", e)))
        })
        .await
        .map_err(|e| OsnovaError::Other(format!("Extraction task failed: {}", e)))??;

        Ok(extract_dir)
    }

    /// Write backend binary
    async fn write_binary(&self, component: &ComponentSchema, data: &[u8]) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir();
        let binary_path = temp_dir.join(format!("osnova-{}-{}", component.name, component.version));

        tokio::fs::write(&binary_path, data)
            .await
            .map_err(|e| OsnovaError::Storage(format!("Failed to write binary: {}", e)))?;

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = tokio::fs::metadata(&binary_path)
                .await
                .map_err(|e| OsnovaError::Storage(format!("Failed to get metadata: {}", e)))?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            tokio::fs::set_permissions(&binary_path, permissions)
                .await
                .map_err(|e| OsnovaError::Storage(format!("Failed to set permissions: {}", e)))?;
        }

        Ok(binary_path)
    }

    /// Verify component hash
    fn verify_hash(data: &[u8], expected_hash: &str) -> Result<()> {
        let mut hasher = Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize();

        let actual_hash_b64 =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash.as_bytes());

        if actual_hash_b64 != expected_hash {
            return Err(OsnovaError::Other(format!(
                "Hash verification failed: expected {}, got {}",
                expected_hash, actual_hash_b64
            )));
        }

        Ok(())
    }

    /// Generate cache key for component
    fn cache_key(component: &ComponentSchema) -> String {
        format!("{}-{}", component.id, component.version)
    }
}

/// Convenience function to download a component
///
/// # Arguments
///
/// * `component` - Component schema
/// * `cache` - Cache manager
/// * `client` - Optional Autonomi client
///
/// # Returns
///
/// * `Ok(PathBuf)` - Path to downloaded component
/// * `Err(OsnovaError)` - Download failed
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::components::download_component;
///
/// let path = download_component(&component, &cache, Some(&client)).await?;
/// ```
pub async fn download_component(
    component: &ComponentSchema,
    cache: &CacheManager,
    client: Option<&AutonomiClient>,
) -> Result<PathBuf> {
    let downloader = ComponentDownloader::new(
        cache.clone(),
        client.cloned(),
    );
    downloader.download(component).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key() {
        let component = ComponentSchema {
            id: "test-id".to_string(),
            name: "Test".to_string(),
            kind: "frontend".to_string(),
            platform: Some("desktop".to_string()),
            target: None,
            version: "1.0.0".to_string(),
            hash: None,
            config: None,
        };

        let key = ComponentDownloader::cache_key(&component);
        assert_eq!(key, "test-id-1.0.0");
    }

    #[test]
    fn test_verify_hash_success() {
        let data = b"test data";
        let mut hasher = Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let hash_b64 =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash.as_bytes());

        let result = ComponentDownloader::verify_hash(data, &hash_b64);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_hash_failure() {
        let data = b"test data";
        let wrong_hash = "wrong_hash_value";

        let result = ComponentDownloader::verify_hash(data, wrong_hash);
        assert!(result.is_err());
    }
}
