//! # Manifest Module
//!
//! Application manifest schema and validation.
//!
//! This module provides:
//! - Manifest schema definition
//! - JSON parsing and validation
//! - Support for ant:// URIs and local paths
//!
//! ## Example
//!
//! ```rust,ignore
//! use osnova_lib::manifest::validate_manifest;
//!
//! let json = r#"{
//!     "id": "ant://...",
//!     "name": "My Application",
//!     "version": "1.0.0",
//!     "iconUri": "ant://...",
//!     "description": "My app description",
//!     "components": [
//!         {
//!             "id": "ant://...",
//!             "name": "Frontend",
//!             "kind": "frontend",
//!             "platform": "desktop",
//!             "version": "1.0.0"
//!         }
//!     ]
//! }"#;
//!
//! let manifest = validate_manifest(json)?;
//! println!("App: {} v{}", manifest.name, manifest.version);
//! ```

pub mod schema;
pub mod validator;
pub mod resolver;

pub use schema::{ManifestSchema, ComponentSchema};
pub use validator::{validate_manifest, validate_manifest_bytes};
pub use resolver::resolve_manifest;
