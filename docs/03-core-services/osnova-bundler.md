# osnova-bundler (built‑in service)

Architecture Update (2025-10-03): The bundler is integrated into the Osnova shell as an in‑process Rust service. It continues to build/package/deploy Osnova apps and, when needed, can expose an RPC surface in stand-alone and server modes. Uploads use the built‑in autonomi and wallet services.

**MVP Status**: This service is **REQUIRED for MVP** as it provides the tooling to build and deploy Osnova applications.

## Overview

The osnova-bundler component provides developer tooling for building, packaging, and deploying Osnova components and applications. It handles:
- Backend component compilation to multiple targets (Linux, macOS, Windows, Android, iOS)
- Frontend component packaging (ZLIB-compressed tarballs)
- Manifest generation and validation
- Autonomi network uploads with payment integration

### OpenRPC methods

The osnova-bundler backend component provides the following OpenRPC methods:

## Backend Component Operations

##### `bundler.backend.compile`
Compile a backend component to specified target(s).

**Request**:
```json
{
  "method": "bundler.backend.compile",
  "params": {
    "sourcePath": "/path/to/backend/component",
    "targets": ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"],
    "release": true,
    "features": ["default"]
  }
}
```

**Response**:
```json
{
  "result": {
    "componentId": "com.example.mycomponent",
    "builds": [
      {
        "target": "x86_64-unknown-linux-gnu",
        "binaryPath": "/path/to/output/mycomponent-linux-x64",
        "size": 5242880,
        "hash": "sha256_hash_of_binary"
      },
      {
        "target": "aarch64-apple-darwin",
        "binaryPath": "/path/to/output/mycomponent-macos-arm64",
        "size": 4718592,
        "hash": "sha256_hash_of_binary"
      }
    ],
    "compiledAt": 1696214400
  }
}
```

**Notes**:
- Uses `cargo build --release --target <target>`
- Supported targets: Linux (x64, ARM), macOS (x64, ARM), Windows (x64), Android (ARM64), iOS (ARM64)
- Validates component ABI compliance

##### `bundler.backend.upload`
Upload compiled backend binaries to Autonomi network.

**Request**:
```json
{
  "method": "bundler.backend.upload",
  "params": {
    "componentId": "com.example.mycomponent",
    "builds": [
      {
        "target": "x86_64-unknown-linux-gnu",
        "binaryPath": "/path/to/output/mycomponent-linux-x64"
      }
    ],
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "componentId": "com.example.mycomponent",
    "uploads": [
      {
        "target": "x86_64-unknown-linux-gnu",
        "autonomiAddress": "base64_encoded_chunk_address",
        "size": 5242880,
        "cost": {
          "eth": "0.005",
          "autonomi": "50.0"
        },
        "transactionHash": "0x..."
      }
    ],
    "uploadedAt": 1696214400
  }
}
```

**Notes**: Uses osnova-autonomi to upload binaries as chunks

## Frontend Component Operations

##### `bundler.frontend.package`
Package a frontend component into a ZLIB-compressed tarball.

**Request**:
```json
{
  "method": "bundler.frontend.package",
  "params": {
    "sourcePath": "/path/to/frontend/component",
    "componentId": "com.example.myapp",
    "buildCommand": "npm run build",
    "distPath": "dist"
  }
}
```

**Response**:
```json
{
  "result": {
    "componentId": "com.example.myapp",
    "tarballPath": "/path/to/output/myapp.tar.zlib",
    "size": 1048576,
    "compressedSize": 262144,
    "compressionRatio": 0.25,
    "hash": "sha256_hash_of_tarball",
    "packagedAt": 1696214400
  }
}
```

**Notes**:
- Runs build command if specified
- Creates tarball from dist directory
- Compresses with ZLIB
- Validates index.html exists

##### `bundler.frontend.upload`
Upload frontend tarball to Autonomi network.

**Request**:
```json
{
  "method": "bundler.frontend.upload",
  "params": {
    "componentId": "com.example.myapp",
    "tarballPath": "/path/to/output/myapp.tar.zlib",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "componentId": "com.example.myapp",
    "autonomiAddress": "base64_encoded_chunk_address",
    "size": 262144,
    "cost": {
      "eth": "0.001",
      "autonomi": "10.0"
    },
    "transactionHash": "0x...",
    "uploadedAt": 1696214400
  }
}
```

## Manifest Operations

##### `bundler.manifest.createComponent`
Create a component manifest.

**Request**:
```json
{
  "method": "bundler.manifest.createComponent",
  "params": {
    "componentId": "com.example.mycomponent",
    "name": "My Component",
    "version": "1.0.0",
    "type": "backend",
    "description": "A sample backend component",
    "author": "developer@example.com",
    "license": "MIT",
    "backends": [
      {
        "target": "x86_64-unknown-linux-gnu",
        "autonomiAddress": "base64_encoded_chunk_address",
        "hash": "sha256_hash"
      }
    ],
    "dependencies": ["com.osnova.core"]
  }
}
```

**Response**:
```json
{
  "result": {
    "manifest": {
      "componentId": "com.example.mycomponent",
      "name": "My Component",
      "version": "1.0.0",
      "type": "backend",
      "description": "A sample backend component",
      "author": "developer@example.com",
      "license": "MIT",
      "backends": [
        {
          "target": "x86_64-unknown-linux-gnu",
          "autonomiAddress": "base64_encoded_chunk_address",
          "hash": "sha256_hash"
        }
      ],
      "dependencies": ["com.osnova.core"],
      "createdAt": 1696214400
    },
    "manifestPath": "/path/to/manifest.json"
  }
}
```

##### `bundler.manifest.createApplication`
Create an application manifest.

**Request**:
```json
{
  "method": "bundler.manifest.createApplication",
  "params": {
    "applicationId": "com.example.myapp",
    "name": "My Application",
    "version": "1.0.0",
    "description": "A sample Osnova application",
    "author": "developer@example.com",
    "license": "MIT",
    "icon": "base64_encoded_icon_png",
    "components": [
      {
        "componentId": "com.example.frontend",
        "type": "frontend",
        "autonomiAddress": "base64_encoded_chunk_address"
      },
      {
        "componentId": "com.example.backend",
        "type": "backend",
        "manifestAddress": "base64_encoded_manifest_address"
      }
    ],
    "permissions": ["network", "storage"]
  }
}
```

**Response**:
```json
{
  "result": {
    "manifest": {
      "applicationId": "com.example.myapp",
      "name": "My Application",
      "version": "1.0.0",
      "description": "A sample Osnova application",
      "author": "developer@example.com",
      "license": "MIT",
      "icon": "base64_encoded_icon_png",
      "components": [
        {
          "componentId": "com.example.frontend",
          "type": "frontend",
          "autonomiAddress": "base64_encoded_chunk_address"
        },
        {
          "componentId": "com.example.backend",
          "type": "backend",
          "manifestAddress": "base64_encoded_manifest_address"
        }
      ],
      "permissions": ["network", "storage"],
      "createdAt": 1696214400
    },
    "manifestPath": "/path/to/app-manifest.json"
  }
}
```

##### `bundler.manifest.upload`
Upload a manifest to Autonomi network.

**Request**:
```json
{
  "method": "bundler.manifest.upload",
  "params": {
    "manifestPath": "/path/to/manifest.json",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "autonomiAddress": "base64_encoded_chunk_address",
    "size": 2048,
    "cost": {
      "eth": "0.0001",
      "autonomi": "1.0"
    },
    "transactionHash": "0x...",
    "uploadedAt": 1696214400
  }
}
```

##### `bundler.manifest.validate`
Validate a manifest file.

**Request**:
```json
{
  "method": "bundler.manifest.validate",
  "params": {
    "manifestPath": "/path/to/manifest.json",
    "type": "component"
  }
}
```

**Response**:
```json
{
  "result": {
    "valid": true,
    "errors": [],
    "warnings": [
      "Icon size exceeds recommended 512x512"
    ]
  }
}
```

**Notes**: `type` can be "component" or "application"

## Complete Build & Deploy Workflow

##### `bundler.workflow.buildAndDeploy`
Complete workflow to build and deploy a component or application.

**Request**:
```json
{
  "method": "bundler.workflow.buildAndDeploy",
  "params": {
    "projectPath": "/path/to/project",
    "type": "application",
    "targets": ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"],
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "publish": true
  }
}
```

**Response**:
```json
{
  "result": {
    "applicationId": "com.example.myapp",
    "version": "1.0.0",
    "manifestAddress": "base64_encoded_manifest_address",
    "components": [
      {
        "componentId": "com.example.frontend",
        "autonomiAddress": "base64_encoded_frontend_address"
      },
      {
        "componentId": "com.example.backend",
        "manifestAddress": "base64_encoded_backend_manifest_address"
      }
    ],
    "totalCost": {
      "eth": "0.01",
      "autonomi": "100.0"
    },
    "deployedAt": 1696214400,
    "installUrl": "ant://base64_encoded_manifest_address"
  }
}
```

**Workflow Steps**:
1. Validate project structure
2. Compile backend components (if any)
3. Package frontend components (if any)
4. Upload binaries/tarballs to Autonomi
5. Generate component manifests
6. Upload component manifests
7. Generate application manifest
8. Upload application manifest
9. Return install URL

## Project Management

##### `bundler.project.init`
Initialize a new Osnova project.

**Request**:
```json
{
  "method": "bundler.project.init",
  "params": {
    "projectPath": "/path/to/new/project",
    "type": "application",
    "template": "svelte-rust",
    "name": "My New App",
    "applicationId": "com.example.mynewapp"
  }
}
```

**Response**:
```json
{
  "result": {
    "projectPath": "/path/to/new/project",
    "type": "application",
    "template": "svelte-rust",
    "filesCreated": [
      "osnova.json",
      "frontend/package.json",
      "backend/Cargo.toml",
      "README.md"
    ]
  }
}
```

**Notes**:
- Templates: "svelte-rust", "svelte-only", "rust-only"
- Creates project structure with osnova.json config

##### `bundler.project.validate`
Validate project structure and configuration.

**Request**:
```json
{
  "method": "bundler.project.validate",
  "params": {
    "projectPath": "/path/to/project"
  }
}
```

**Response**:
```json
{
  "result": {
    "valid": true,
    "errors": [],
    "warnings": [
      "Backend component missing tests"
    ],
    "projectType": "application",
    "components": [
      {
        "componentId": "com.example.frontend",
        "type": "frontend",
        "valid": true
      },
      {
        "componentId": "com.example.backend",
        "type": "backend",
        "valid": true
      }
    ]
  }
}
```

## Integration with Other Components

### Integration with osnova-autonomi

All upload operations use osnova-autonomi:
- `autonomi.chunk.upload` for binaries, tarballs, and manifests
- Handles payment integration automatically
- Returns Autonomi addresses for manifest references

### Integration with osnova-wallet

All paid operations request payment via wallet:
- Estimates total cost before starting
- Requests payment approval from user
- Provides detailed cost breakdown
- Includes purpose descriptions for each upload

### Integration with osnova-core

Uses osnova-core for:
- Component ID validation
- Dependency resolution
- Configuration storage

## Error Codes

- `-60000`: Compilation failed
- `-60001`: Packaging failed
- `-60002`: Upload failed
- `-60003`: Invalid manifest
- `-60004`: Invalid project structure
- `-60005`: Missing dependencies
- `-60006`: Target not supported
- `-60007`: Build command failed
- `-60008`: Payment required
- `-60009`: Insufficient balance

## Security Considerations

1. **Code Signing**: All binaries should be signed (post-MVP)
2. **Manifest Validation**: Strict validation of all manifests
3. **Dependency Verification**: Verify all dependencies exist
4. **Sandboxed Builds**: Build in isolated environment
5. **Hash Verification**: SHA-256 hashes for all artifacts

## MVP Implementation Notes

1. **Rust Compilation**: Use `cargo build` with cross-compilation support
2. **Frontend Builds**: Support npm/yarn/pnpm build commands
3. **ZLIB Compression**: Use flate2 crate for compression
4. **Manifest Schema**: JSON schema validation
5. **Progress Tracking**: Emit progress events for long operations
6. **Error Recovery**: Retry failed uploads with exponential backoff

## Post-MVP Enhancements

- Code signing and verification
- Incremental builds
- Build caching
- Multi-stage builds
- Docker-based builds for consistency
- Automated testing before deployment
- Rollback functionality
- Version management
- Dependency graph visualization
- Build analytics

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
