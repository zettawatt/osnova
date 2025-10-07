//! Application models for Osnova
//!
//! This module provides the OsnovaApplication type and related structures which manage:
//! - Application manifests (name, version, components)
//! - Component references (frontend/backend)
//! - Application metadata and configuration
//!
//! # Example
//!
//! ```rust,ignore
//! use osnova_lib::models::application::{OsnovaApplication, ComponentRef, ComponentKind};
//!
//! // Create application with components
//! let app = OsnovaApplication::new(
//!     "app-manifest-hash",
//!     "My App",
//!     "1.0.0",
//!     "icon-uri",
//!     "App description",
//!     vec![
//!         ComponentRef::new("comp-1", "Frontend", ComponentKind::Frontend, "1.0.0"),
//!         ComponentRef::new("comp-2", "Backend", ComponentKind::Backend, "1.0.0"),
//!     ],
//! ).unwrap();
//! ```

use crate::{OsnovaError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component kind (frontend or backend)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentKind {
    /// Frontend component (Svelte UI)
    Frontend,
    /// Backend component (Rust service)
    Backend,
}

/// Platform for frontend components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    /// iOS platform
    #[serde(rename = "iOS")]
    IOS,
    /// Android platform
    Android,
    /// Desktop (Windows, macOS, Linux)
    #[serde(rename = "desktop")]
    Desktop,
}

/// Component reference within an application
///
/// Each component is identified by its content address and has a specific kind
/// (frontend or backend) and version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRef {
    /// Content address of the component (Autonomi address or local path)
    id: String,

    /// Human-readable name of the component
    name: String,

    /// Component kind (frontend or backend)
    kind: ComponentKind,

    /// Component version (semver)
    version: String,

    /// Target for backend components (Rust target triple)
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,

    /// Platform for frontend components
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,

    /// Hash of the fetched artifact (BLAKE3)
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,

    /// Component configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<HashMap<String, serde_json::Value>>,
}

impl ComponentRef {
    /// Create a new component reference
    ///
    /// # Arguments
    ///
    /// * `id` - Content address or local path
    /// * `name` - Human-readable name
    /// * `kind` - Component kind (frontend or backend)
    /// * `version` - Semver version string
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::application::{ComponentRef, ComponentKind};
    ///
    /// let component = ComponentRef::new(
    ///     "component-hash-123",
    ///     "My Component",
    ///     ComponentKind::Frontend,
    ///     "1.0.0"
    /// ).expect("Failed to create component");
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: ComponentKind,
        version: impl Into<String>,
    ) -> Result<Self> {
        let version_str = version.into();
        Self::validate_version(&version_str)?;

        Ok(Self {
            id: id.into(),
            name: name.into(),
            kind,
            version: version_str,
            target: None,
            platform: None,
            hash: None,
            config: None,
        })
    }

    /// Set the target for a backend component
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Set the platform for a frontend component
    pub fn with_platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Set the hash for artifact verification
    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    /// Set the component configuration
    pub fn with_config(mut self, config: HashMap<String, serde_json::Value>) -> Self {
        self.config = Some(config);
        self
    }

    /// Get the component ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the component name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the component kind
    pub fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Get the component version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the target (for backend components)
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Get the platform (for frontend components)
    pub fn platform(&self) -> Option<Platform> {
        self.platform
    }

    /// Get the hash
    pub fn hash(&self) -> Option<&str> {
        self.hash.as_deref()
    }

    /// Get the configuration
    pub fn config(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.config.as_ref()
    }

    /// Validate semver version string
    fn validate_version(version: &str) -> Result<()> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(OsnovaError::Other(format!(
                "Invalid version format: {}. Expected semver (e.g., 1.0.0)",
                version
            )));
        }

        for part in parts {
            if part.parse::<u64>().is_err() {
                return Err(OsnovaError::Other(format!(
                    "Invalid version format: {}. Version parts must be numbers",
                    version
                )));
            }
        }

        Ok(())
    }
}

/// Osnova application manifest
///
/// Represents a complete application with its metadata, components, and configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsnovaApplication {
    /// Content address of the manifest (or local path for dev)
    id: String,

    /// Application name
    name: String,

    /// Application version (semver)
    version: String,

    /// Icon URI (Autonomi address or local path)
    icon_uri: String,

    /// Application description
    description: String,

    /// Publisher identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,

    /// Detached signature over canonical manifest
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,

    /// Application components
    components: Vec<ComponentRef>,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl OsnovaApplication {
    /// Create a new Osnova application
    ///
    /// # Arguments
    ///
    /// * `id` - Content address or local path of the manifest
    /// * `name` - Application name
    /// * `version` - Application version (semver)
    /// * `icon_uri` - URI for the application icon
    /// * `description` - Application description
    /// * `components` - List of component references
    ///
    /// # Example
    ///
    /// ```
    /// use osnova_lib::models::application::{OsnovaApplication, ComponentRef, ComponentKind};
    ///
    /// let app = OsnovaApplication::new(
    ///     "app-manifest-hash",
    ///     "My App",
    ///     "1.0.0",
    ///     "icon-uri",
    ///     "A sample application",
    ///     vec![
    ///         ComponentRef::new("comp-1", "UI", ComponentKind::Frontend, "1.0.0").unwrap(),
    ///     ],
    /// ).expect("Failed to create application");
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        icon_uri: impl Into<String>,
        description: impl Into<String>,
        components: Vec<ComponentRef>,
    ) -> Result<Self> {
        let version_str = version.into();
        ComponentRef::validate_version(&version_str)?;

        Ok(Self {
            id: id.into(),
            name: name.into(),
            version: version_str,
            icon_uri: icon_uri.into(),
            description: description.into(),
            publisher: None,
            signature: None,
            components,
            metadata: None,
        })
    }

    /// Set the publisher identifier
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Set the signature
    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    /// Set the metadata
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Get the application ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the application name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the application version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the icon URI
    pub fn icon_uri(&self) -> &str {
        &self.icon_uri
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the publisher
    pub fn publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }

    /// Get the signature
    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }

    /// Get the components
    pub fn components(&self) -> &[ComponentRef] {
        &self.components
    }

    /// Get the metadata
    pub fn metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }

    /// Add a component to the application
    pub fn add_component(&mut self, component: ComponentRef) {
        self.components.push(component);
    }

    /// Get components by kind
    pub fn components_by_kind(&self, kind: ComponentKind) -> Vec<&ComponentRef> {
        self.components
            .iter()
            .filter(|c| c.kind() == kind)
            .collect()
    }

    /// Find a component by ID
    pub fn find_component(&self, id: &str) -> Option<&ComponentRef> {
        self.components.iter().find(|c| c.id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_kind_serialization() {
        let frontend = ComponentKind::Frontend;
        let backend = ComponentKind::Backend;

        assert_eq!(serde_json::to_string(&frontend).unwrap(), "\"frontend\"");
        assert_eq!(serde_json::to_string(&backend).unwrap(), "\"backend\"");
    }

    #[test]
    fn test_platform_serialization() {
        let ios = Platform::IOS;
        let android = Platform::Android;
        let desktop = Platform::Desktop;

        assert_eq!(serde_json::to_string(&ios).unwrap(), "\"iOS\"");
        assert_eq!(serde_json::to_string(&android).unwrap(), "\"Android\"");
        assert_eq!(serde_json::to_string(&desktop).unwrap(), "\"desktop\"");
    }

    #[test]
    fn test_component_ref_new() {
        let component =
            ComponentRef::new("comp-id", "My Component", ComponentKind::Frontend, "1.0.0")
                .expect("Failed to create component");

        assert_eq!(component.id(), "comp-id");
        assert_eq!(component.name(), "My Component");
        assert_eq!(component.kind(), ComponentKind::Frontend);
        assert_eq!(component.version(), "1.0.0");
        assert_eq!(component.target(), None);
        assert_eq!(component.platform(), None);
        assert_eq!(component.hash(), None);
        assert_eq!(component.config(), None);
    }

    #[test]
    fn test_component_ref_with_target() {
        let component = ComponentRef::new("comp-id", "Backend", ComponentKind::Backend, "1.0.0")
            .unwrap()
            .with_target("x86_64-unknown-linux-gnu");

        assert_eq!(component.target(), Some("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn test_component_ref_with_platform() {
        let component = ComponentRef::new("comp-id", "Frontend", ComponentKind::Frontend, "1.0.0")
            .unwrap()
            .with_platform(Platform::Desktop);

        assert_eq!(component.platform(), Some(Platform::Desktop));
    }

    #[test]
    fn test_component_ref_with_hash() {
        let component = ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0")
            .unwrap()
            .with_hash("blake3-hash-here");

        assert_eq!(component.hash(), Some("blake3-hash-here"));
    }

    #[test]
    fn test_component_ref_with_config() {
        let mut config = HashMap::new();
        config.insert("key".to_string(), serde_json::json!("value"));

        let component = ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0")
            .unwrap()
            .with_config(config.clone());

        assert_eq!(component.config(), Some(&config));
    }

    #[test]
    fn test_component_ref_invalid_version() {
        let result = ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0");
        assert!(result.is_err());

        let result = ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_osnova_application_new() {
        let component =
            ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0").unwrap();

        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "App description",
            vec![component],
        )
        .expect("Failed to create application");

        assert_eq!(app.id(), "app-id");
        assert_eq!(app.name(), "My App");
        assert_eq!(app.version(), "1.0.0");
        assert_eq!(app.icon_uri(), "icon-uri");
        assert_eq!(app.description(), "App description");
        assert_eq!(app.publisher(), None);
        assert_eq!(app.signature(), None);
        assert_eq!(app.components().len(), 1);
        assert_eq!(app.metadata(), None);
    }

    #[test]
    fn test_osnova_application_with_publisher() {
        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![],
        )
        .unwrap()
        .with_publisher("publisher-id");

        assert_eq!(app.publisher(), Some("publisher-id"));
    }

    #[test]
    fn test_osnova_application_with_signature() {
        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![],
        )
        .unwrap()
        .with_signature("signature-data");

        assert_eq!(app.signature(), Some("signature-data"));
    }

    #[test]
    fn test_osnova_application_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));

        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![],
        )
        .unwrap()
        .with_metadata(metadata.clone());

        assert_eq!(app.metadata(), Some(&metadata));
    }

    #[test]
    fn test_osnova_application_add_component() {
        let mut app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![],
        )
        .unwrap();

        assert_eq!(app.components().len(), 0);

        let component =
            ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0").unwrap();
        app.add_component(component);

        assert_eq!(app.components().len(), 1);
    }

    #[test]
    fn test_osnova_application_components_by_kind() {
        let frontend =
            ComponentRef::new("frontend-id", "Frontend", ComponentKind::Frontend, "1.0.0").unwrap();
        let backend =
            ComponentRef::new("backend-id", "Backend", ComponentKind::Backend, "1.0.0").unwrap();

        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![frontend, backend],
        )
        .unwrap();

        let frontends = app.components_by_kind(ComponentKind::Frontend);
        assert_eq!(frontends.len(), 1);
        assert_eq!(frontends[0].id(), "frontend-id");

        let backends = app.components_by_kind(ComponentKind::Backend);
        assert_eq!(backends.len(), 1);
        assert_eq!(backends[0].id(), "backend-id");
    }

    #[test]
    fn test_osnova_application_find_component() {
        let component =
            ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0").unwrap();

        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![component],
        )
        .unwrap();

        assert!(app.find_component("comp-id").is_some());
        assert!(app.find_component("non-existent").is_none());
    }

    #[test]
    fn test_osnova_application_serialization() {
        let component = ComponentRef::new("comp-id", "Component", ComponentKind::Frontend, "1.0.0")
            .unwrap()
            .with_platform(Platform::Desktop);

        let app = OsnovaApplication::new(
            "app-id",
            "My App",
            "1.0.0",
            "icon-uri",
            "Description",
            vec![component],
        )
        .unwrap()
        .with_publisher("publisher-id");

        let json = serde_json::to_string(&app).expect("Failed to serialize");
        let deserialized: OsnovaApplication =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(app, deserialized);
    }

    #[test]
    fn test_osnova_application_invalid_version() {
        let result =
            OsnovaApplication::new("app-id", "My App", "1.0", "icon-uri", "Description", vec![]);

        assert!(result.is_err());
    }
}
