//! # Manifest Schema
//!
//! Data structures for Osnova application manifests.
//!
//! Implements the schema defined in docs/06-protocols/manifest-schema.md

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application manifest schema
///
/// Defines metadata and components for an Osnova application.
///
/// # Example
///
/// ```rust,ignore
/// let manifest = ManifestSchema {
///     id: "ant://0123456789abcdef...".to_string(),
///     name: "My App".to_string(),
///     version: "1.0.0".to_string(),
///     icon_uri: "ant://icon123...".to_string(),
///     description: "My application".to_string(),
///     publisher: Some("ACME Corp".to_string()),
///     signature: None,
///     components: vec![...],
///     metadata: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManifestSchema {
    /// Manifest identifier (ant:// URI or local path for dev)
    pub id: String,

    /// Application name
    pub name: String,

    /// Semantic version (e.g., "1.0.0")
    pub version: String,

    /// Icon URI (ant:// or local path)
    #[serde(rename = "iconUri")]
    pub icon_uri: String,

    /// Application description
    pub description: String,

    /// Publisher identifier (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    /// Detached signature (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// List of components
    pub components: Vec<ComponentSchema>,

    /// Additional metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Component schema
///
/// Defines a single component within an application manifest.
///
/// # Example
///
/// ```rust,ignore
/// let component = ComponentSchema {
///     id: "ant://comp123...".to_string(),
///     name: "Frontend".to_string(),
///     kind: "frontend".to_string(),
///     platform: Some("desktop".to_string()),
///     target: None,
///     version: "1.0.0".to_string(),
///     hash: Some("abc123".to_string()),
///     config: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentSchema {
    /// Component identifier (ant:// URI or local path for dev)
    pub id: String,

    /// Component name
    pub name: String,

    /// Component kind ("frontend" or "backend")
    pub kind: String,

    /// Platform for frontend components ("iOS", "Android", or "desktop")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Target triple for backend components (e.g., "x86_64-unknown-linux-gnu")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Semantic version
    pub version: String,

    /// Content hash (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    /// Component configuration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, serde_json::Value>>,
}

impl ManifestSchema {
    /// Validate manifest against schema rules
    ///
    /// Checks:
    /// - Required fields are present
    /// - Version follows semver format
    /// - Component kinds are valid
    /// - Platform/target fields are appropriate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Manifest is valid
    /// * `Err(String)` - Validation error message
    pub fn validate(&self) -> Result<(), String> {
        // Validate version format (semver: x.y.z)
        if !Self::is_valid_semver(&self.version) {
            return Err(format!("Invalid version format: {}", self.version));
        }

        // Validate each component
        for (idx, component) in self.components.iter().enumerate() {
            if let Err(e) = component.validate() {
                return Err(format!("Component {}: {}", idx, e));
            }
        }

        Ok(())
    }

    /// Check if string is valid semver format (x.y.z)
    fn is_valid_semver(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }
}

impl ComponentSchema {
    /// Validate component against schema rules
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Component is valid
    /// * `Err(String)` - Validation error message
    pub fn validate(&self) -> Result<(), String> {
        // Validate kind
        if self.kind != "frontend" && self.kind != "backend" {
            return Err(format!(
                "Invalid component kind: '{}' (must be 'frontend' or 'backend')",
                self.kind
            ));
        }

        // Validate version format
        if !ManifestSchema::is_valid_semver(&self.version) {
            return Err(format!("Invalid version format: {}", self.version));
        }

        // Validate platform for frontend components
        if self.kind == "frontend" {
            if let Some(platform) = &self.platform {
                if platform != "iOS" && platform != "Android" && platform != "desktop" {
                    return Err(format!(
                        "Invalid platform: '{}' (must be 'iOS', 'Android', or 'desktop')",
                        platform
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_semver() {
        assert!(ManifestSchema::is_valid_semver("1.0.0"));
        assert!(ManifestSchema::is_valid_semver("0.1.0"));
        assert!(ManifestSchema::is_valid_semver("10.20.30"));
        assert!(!ManifestSchema::is_valid_semver("1.0"));
        assert!(!ManifestSchema::is_valid_semver("1.0.0.1"));
        assert!(!ManifestSchema::is_valid_semver("v1.0.0"));
        assert!(!ManifestSchema::is_valid_semver("1.0.a"));
    }

    #[test]
    fn test_component_validation() {
        let valid_frontend = ComponentSchema {
            id: "test".to_string(),
            name: "Test".to_string(),
            kind: "frontend".to_string(),
            platform: Some("desktop".to_string()),
            target: None,
            version: "1.0.0".to_string(),
            hash: None,
            config: None,
        };
        assert!(valid_frontend.validate().is_ok());

        let valid_backend = ComponentSchema {
            id: "test".to_string(),
            name: "Test".to_string(),
            kind: "backend".to_string(),
            platform: None,
            target: Some("x86_64-unknown-linux-gnu".to_string()),
            version: "1.0.0".to_string(),
            hash: None,
            config: None,
        };
        assert!(valid_backend.validate().is_ok());

        let invalid_kind = ComponentSchema {
            id: "test".to_string(),
            name: "Test".to_string(),
            kind: "middleware".to_string(),
            platform: None,
            target: None,
            version: "1.0.0".to_string(),
            hash: None,
            config: None,
        };
        assert!(invalid_kind.validate().is_err());
    }
}
