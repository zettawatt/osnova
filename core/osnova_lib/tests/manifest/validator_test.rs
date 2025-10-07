//! Tests for manifest validation

use osnova_lib::manifest::{ManifestSchema, ComponentSchema, validate_manifest};

#[test]
fn test_valid_manifest() {
    // Test a complete valid manifest
    let json = r#"{
        "id": "ant://0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "name": "Test Application",
        "version": "1.0.0",
        "iconUri": "ant://icon123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "description": "A test application",
        "publisher": "Test Publisher",
        "components": [
            {
                "id": "ant://comp123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "name": "Frontend Component",
                "kind": "frontend",
                "platform": "desktop",
                "version": "1.0.0",
                "hash": "abc123def456"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.name, "Test Application");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.components.len(), 1);
}

#[test]
fn test_missing_required_field() {
    // Test manifest missing required field 'name'
    let json = r#"{
        "id": "ant://0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "version": "1.0.0",
        "iconUri": "ant://icon123",
        "description": "Test",
        "components": []
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("name"));
}

#[test]
fn test_invalid_version_format() {
    // Test invalid semver format
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": []
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("version"));
}

#[test]
fn test_valid_backend_component() {
    // Test backend component with target
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": [
            {
                "id": "ant://backend",
                "name": "Backend Service",
                "kind": "backend",
                "target": "x86_64-unknown-linux-gnu",
                "version": "1.0.0",
                "hash": "abc123"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    let component = &manifest.components[0];
    assert_eq!(component.kind, "backend");
    assert_eq!(component.target.as_ref().unwrap(), "x86_64-unknown-linux-gnu");
}

#[test]
fn test_valid_frontend_component() {
    // Test frontend component with platform
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": [
            {
                "id": "ant://frontend",
                "name": "UI Component",
                "kind": "frontend",
                "platform": "iOS",
                "version": "1.0.0",
                "hash": "def456"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    let component = &manifest.components[0];
    assert_eq!(component.kind, "frontend");
    assert_eq!(component.platform.as_ref().unwrap(), "iOS");
}

#[test]
fn test_invalid_component_kind() {
    // Test invalid component kind
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": [
            {
                "id": "ant://comp",
                "name": "Invalid Component",
                "kind": "middleware",
                "version": "1.0.0"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("kind"));
}

#[test]
fn test_invalid_platform() {
    // Test invalid platform value
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": [
            {
                "id": "ant://comp",
                "name": "Component",
                "kind": "frontend",
                "platform": "Windows",
                "version": "1.0.0"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("platform"));
}

#[test]
fn test_local_dev_path() {
    // Test local filesystem path for development
    let json = r#"{
        "id": "/home/dev/my-app/manifest.json",
        "name": "Dev App",
        "version": "1.0.0",
        "iconUri": "/home/dev/my-app/icon.png",
        "description": "Development app",
        "components": [
            {
                "id": "/home/dev/my-app/frontend.tar.gz",
                "name": "Frontend",
                "kind": "frontend",
                "platform": "desktop",
                "version": "1.0.0"
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());
}

#[test]
fn test_optional_metadata() {
    // Test optional metadata fields
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "signature": "sig123abc",
        "components": [],
        "metadata": {
            "category": "productivity",
            "tags": ["office", "documents"]
        }
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert!(manifest.signature.is_some());
    assert!(manifest.metadata.is_some());
}

#[test]
fn test_component_with_config() {
    // Test component with config object
    let json = r#"{
        "id": "ant://test",
        "name": "Test App",
        "version": "1.0.0",
        "iconUri": "ant://icon",
        "description": "Test",
        "components": [
            {
                "id": "ant://comp",
                "name": "Configurable Component",
                "kind": "backend",
                "target": "x86_64-unknown-linux-gnu",
                "version": "1.0.0",
                "config": {
                    "port": 8080,
                    "workers": 4,
                    "enable_ssl": true
                }
            }
        ]
    }"#;

    let result = validate_manifest(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert!(manifest.components[0].config.is_some());
}
