# Manifest Schema and Trust Model (MVP)

## Purpose
Define a minimal, testable schema for Osnova application manifests and a trust model suitable for MVP, while remaining implementation-agnostic in the spec.

## JSON Schema (skeleton)
```json
{
  "title": "Osnova Application Manifest",
  "type": "object",
  "required": ["id", "name", "version", "iconUri", "description", "components"],
  "properties": {
    "id": {"type": "string", "description": "Autonomi content address of the manifest itself or a path on the local filesystem for development purposes"},
    "name": {"type": "string"},
    "version": {"type": "string", "pattern": "^\d+\.\d+\.\d+$", "description": "Semver; exact pinned version"},
    "iconUri": {"type": "string", "description": "Autonomi address of the app icon, a 1024x1024 PNG"},
    "description": {"type": "string"},
    "publisher": {"type": "string", "description": "Publisher identifier"},
    "signature": {"type": "string", "description": "Detached signature over canonical manifest"},
    "components": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "name", "kind", "version"],
        "properties": {
          "id": {"type": "string", "description": "Autonomi address of the component or local path for development"},
          "name": {"type": "string", "description": "Human-readable name of the component"},
          "kind": {"type": "string", "enum": ["frontend", "backend"]},
          "target": {"type": "string", "description": "Target for compiled backend components following Rust's official target triple format (e.g., x86_64-unknown-linux-gnu). Backend components only." },
          "platform": {"type": "string", "enum": ["iOS", "Android", "desktop"], "description": "Specifies platform the frontend should operate under. Frontend components only"},
          "version": {"type": "string", "pattern": "^\d+\.\d+\.\d+$", "description": "Semver; exact pinned version"},
          "hash": {"type": "string", "description": "Hash (e.g., blake3 base64) of the fetched artifact"},
          "config": {"type": "object", "additionalProperties": true},
        }
      }
    },
    "metadata": {"type": "object", "additionalProperties": true}
  }
}
```

Notes:
- Keep the schema strict for required fields; allow forward-compatible metadata via additionalProperties: true at top-level and component level.
- Dev vs Prod: manifests used in development MAY specify `devRef` (non-content-addressed). Production MUST specify `prodRef` and SHOULD include `integrity`.
- Though the target and platform fields are optional, in practice all components will specify them. They are only optional to preserve forward-compatibility.
- The target field must match the host OS and architecture. If it does not, the component MUST NOT be loaded and a user-visible error MUST be shown.
- The platform field must match the host OS. If it does not, the component MUST NOT be loaded and a user-visible error MUST be shown.

## Trust model (post-MVP, out of scope for now)
- Pinned versions: Manifests pin exact component versions by content address and version.
- Signing: A detached signature over a canonical JSON form (JCS) signed by the publisher. For now this is an optional field and is outside the scope of MVP. Will come back to this later.
- Verification: On fetch, verify integrity hash and optional signature before activation. If verification fails, abort launch with a user-visible error.
- Mirrors: Optional list of mirror URIs. Fetch MUST verify integrity regardless of source.

## Validation rules and errors
- If a required component is missing/unresolvable: show a clear error and cancel launch.
- If integrity/signature verification fails: show a clear error and cancel launch.
- If schema validation fails: surface validation messages for debugging; do not start components.
- Version should follow semver standards, e.g. 1.0.0

## Rust Implementation

The manifest schema is implemented in `core/osnova_lib/src/manifest/`:

### Usage Example

```rust
use osnova_lib::manifest::{validate_manifest, ManifestSchema};

// Validate from JSON string
let json = r#"{
    "id": "ant://...",
    "name": "My Application",
    "version": "1.0.0",
    "iconUri": "ant://...",
    "description": "Description",
    "components": [
        {
            "id": "ant://...",
            "name": "Frontend",
            "kind": "frontend",
            "platform": "desktop",
            "version": "1.0.0"
        }
    ]
}"#;

let manifest: ManifestSchema = validate_manifest(json)?;
println!("App: {} v{}", manifest.name, manifest.version);
```

### Validation Rules

1. **Required Fields**: id, name, version, iconUri, description, components
2. **Version Format**: Must be valid semver (x.y.z where x, y, z are integers)
3. **Component Kind**: Must be "frontend" or "backend"
4. **Platform** (frontend only): Must be "iOS", "Android", or "desktop"
5. **Target** (backend only): Should match Rust target triple format

### Error Messages

- Missing required field: `"Failed to parse manifest JSON: missing field 'name'"`
- Invalid version: `"Manifest validation failed: Invalid version format: 1.0"`
- Invalid component kind: `"Component 0: Invalid component kind: 'middleware'"`
- Invalid platform: `"Component 0: Invalid platform: 'Windows'"`

## Storage on the Autonomi Network

Application manifests are always uploaded as public files, each version is its own file.
Each version's component collateral is listed in a manifest file as described above.
The address for the component is a pointer that points to a graph entry.
Each time a new version is added, a new manifest file is created along with a graph entry.
The graph entry contains links to all of the previous version manifest files as well as the latest entry.
The pointer is updated to point to the latest graph entry.
In this way, we build up an immutable list of collateral for each osnova app specified by version.

## Osnova App Installation

To install a new Osnova app, the shell loads the application manifest and fetches any app‑supplied assets or components referenced therein.
Core services and core screens are built into the shell and do not require fetching.
Once app assets are resolved, the icon and any additional metadata will be downloaded and added to the app configuration. The icon is then added to the app screen.

## Local caching and data storage

Osnova apps can store data on the local device.
By default, the configuration app will specify an app data directory.
All app data will be stored in a sub-directory by user and component version.
