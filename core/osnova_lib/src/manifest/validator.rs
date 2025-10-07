//! # Manifest Validator
//!
//! Validation functions for Osnova application manifests.

use super::schema::ManifestSchema;
use crate::error::{OsnovaError, Result};

/// Validate a manifest from JSON string
///
/// Parses JSON and validates against the manifest schema.
///
/// # Arguments
///
/// * `json` - JSON string containing the manifest
///
/// # Returns
///
/// * `Ok(ManifestSchema)` - Valid manifest
/// * `Err(OsnovaError)` - Validation or parsing error
///
/// # Example
///
/// ```rust,ignore
/// use osnova_lib::manifest::validate_manifest;
///
/// let json = r#"{
///     "id": "ant://...",
///     "name": "My App",
///     "version": "1.0.0",
///     "iconUri": "ant://...",
///     "description": "...",
///     "components": []
/// }"#;
///
/// let manifest = validate_manifest(json)?;
/// println!("Validated: {}", manifest.name);
/// ```
pub fn validate_manifest(json: &str) -> Result<ManifestSchema> {
    // Parse JSON
    let manifest: ManifestSchema = serde_json::from_str(json)
        .map_err(|e| OsnovaError::Other(format!("Failed to parse manifest JSON: {}", e)))?;

    // Validate against schema rules
    manifest
        .validate()
        .map_err(|e| OsnovaError::Other(format!("Manifest validation failed: {}", e)))?;

    Ok(manifest)
}

/// Validate a manifest from bytes
///
/// Convenience function for validating manifests from downloaded data.
///
/// # Arguments
///
/// * `data` - Byte slice containing UTF-8 encoded JSON
///
/// # Returns
///
/// * `Ok(ManifestSchema)` - Valid manifest
/// * `Err(OsnovaError)` - Validation, parsing, or encoding error
///
/// # Example
///
/// ```rust,ignore
/// let data = download_data(&client, manifest_uri).await?;
/// let manifest = validate_manifest_bytes(&data)?;
/// ```
pub fn validate_manifest_bytes(data: &[u8]) -> Result<ManifestSchema> {
    let json = std::str::from_utf8(data)
        .map_err(|e| OsnovaError::Other(format!("Invalid UTF-8 in manifest: {}", e)))?;

    validate_manifest(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_manifest_success() {
        let json = r#"{
            "id": "ant://test",
            "name": "Test App",
            "version": "1.0.0",
            "iconUri": "ant://icon",
            "description": "Test application",
            "components": []
        }"#;

        let result = validate_manifest(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_manifest_invalid_json() {
        let json = "{ invalid json }";
        let result = validate_manifest(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_missing_field() {
        let json = r#"{
            "id": "ant://test",
            "version": "1.0.0"
        }"#;

        let result = validate_manifest(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_bytes_success() {
        let json = r#"{
            "id": "ant://test",
            "name": "Test App",
            "version": "1.0.0",
            "iconUri": "ant://icon",
            "description": "Test",
            "components": []
        }"#;

        let result = validate_manifest_bytes(json.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_manifest_bytes_invalid_utf8() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let result = validate_manifest_bytes(&invalid_utf8);
        assert!(result.is_err());
    }
}
