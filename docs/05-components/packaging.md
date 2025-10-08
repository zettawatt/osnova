# Component Packaging (Native Rust Implementation)

**Last Updated**: 2025-10-08

## Overview

Component packaging in Osnova is implemented as native Rust functionality within `osnova-core`, enabling the Osnova shell application to perform bundling natively across all platforms (Windows, macOS, Linux, Android, iOS) without relying on platform-specific shell scripts.

## Architecture

### Packaging Module Location

```
core/osnova_lib/src/packaging/
├── mod.rs              # Public API
├── frontend.rs         # Frontend component packager
├── backend.rs          # Backend component packager (source distribution)
├── unpacker.rs         # Component unpacker
├── manifest.rs         # Manifest generation
└── toolchain.rs        # Rust toolchain management and local compilation
```

## Frontend Component Packaging

### Purpose

Package Svelte/TypeScript frontend components as ZLIB-compressed tarballs suitable for distribution via Autonomi Network or local deployment.

### API

```rust
use osnova_lib::packaging::frontend::FrontendPackager;

/// Package a frontend component
pub async fn package_frontend(
    component_dir: &Path,
    output_dir: &Path,
) -> Result<PackageManifest> {
    let packager = FrontendPackager::new(component_dir)?;

    // Build the Svelte/Vite project
    packager.build().await?;

    // Create tarball from build output
    let tarball = packager.create_tarball().await?;

    // Compress with ZLIB (level 9)
    let compressed = packager.compress_zlib(tarball, 9).await?;

    // Calculate SHA-256 hash
    let hash = packager.calculate_hash(&compressed)?;

    // Generate manifest entry
    let manifest = PackageManifest {
        id: packager.component_id(),
        version: packager.version(),
        component_type: ComponentType::Frontend,
        format: PackageFormat::ZlibTarball,
        size: compressed.len(),
        hash: Hash::new(HashAlgorithm::Sha256, hash),
        path: compressed_path,
    };

    // Write to output directory
    packager.write_package(output_dir, &compressed, &manifest).await?;

    Ok(manifest)
}
```

### Implementation Details

**Build Step**:
- Invoke Node.js/npm programmatically via `std::process::Command`
- Run `npm install` if `node_modules/` missing
- Run `npm run build` to generate production bundle
- Validate build output exists (`dist/` or `build/` directory)

**Tarball Creation**:
- Use `tar-rs` crate for cross-platform tarball creation
- Include all files from build output directory
- Preserve file permissions and timestamps
- No compression at this stage (tar only)

**ZLIB Compression**:
- Use `flate2` crate with ZLIB backend
- Compression level: 9 (maximum compression)
- Streaming compression for large files to minimize memory usage

**Hash Calculation**:
- Use `sha2` crate for SHA-256 hashing
- Hash the final compressed tarball
- Return hex-encoded hash string

**Manifest Generation**:
- Extract metadata from `package.json`
- Include component ID, version, type, size, hash
- Write as JSON to `{component-name}-manifest.json`

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum PackagingError {
    #[error("Component directory not found: {0}")]
    ComponentNotFound(PathBuf),

    #[error("package.json not found or invalid")]
    InvalidPackageJson,

    #[error("Build failed: {0}")]
    BuildFailed(String),

    #[error("Build output not found")]
    BuildOutputMissing,

    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Dependencies

```toml
[dependencies]
tar = "0.4"           # Cross-platform tarball creation
flate2 = "1.0"        # ZLIB compression
sha2 = "0.10"         # SHA-256 hashing
serde_json = "1.0"    # package.json parsing
tokio = { version = "1", features = ["process", "fs"] }
```

## Backend Component Packaging

### Purpose

Package Rust backend components as ZLIB-compressed source code tarballs for distribution. Components are compiled locally on the user's machine to avoid OS security warnings for unsigned binaries.

### API

```rust
use osnova_lib::packaging::backend::BackendPackager;

/// Package a backend component (source distribution)
pub async fn package_backend(
    component_dir: &Path,
    output_dir: &Path,
) -> Result<PackageManifest> {
    let packager = BackendPackager::new(component_dir)?;

    // Validate Cargo.toml and component structure
    packager.validate_component_structure()?;

    // Create tarball from source code
    let tarball = packager.create_source_tarball().await?;

    // Compress with ZLIB (level 9)
    let compressed = packager.compress_zlib(tarball, 9).await?;

    // Calculate SHA-256 hash
    let hash = packager.calculate_hash(&compressed)?;

    // Generate manifest entry
    let manifest = PackageManifest {
        id: packager.component_id(),
        version: packager.version(),
        component_type: ComponentType::Backend,
        format: PackageFormat::ZlibTarball,
        size: compressed.len(),
        hash: Hash::new(HashAlgorithm::Sha256, hash),
        path: compressed_path,
    };

    // Write to output directory
    packager.write_package(output_dir, &compressed, &manifest).await?;

    Ok(manifest)
}
```

### Implementation Details

**Component Structure Validation**:
- Verify `Cargo.toml` exists and is valid
- Ensure `[lib]` section defines `crate-type = ["cdylib"]`
- Validate component metadata (name, version, description)
- Check for required dependencies (osnova-component-sdk)
- Verify src/lib.rs exists

**Source Tarball Creation**:
- Use `tar-rs` crate for cross-platform tarball creation
- Include: `Cargo.toml`, `Cargo.lock`, `src/`, `README.md` (if present)
- Exclude: `target/`, `.git/`, `node_modules/`, `.DS_Store`
- Preserve file permissions and timestamps
- No compression at this stage (tar only)

**ZLIB Compression**:
- Use `flate2` crate with ZLIB backend
- Compression level: 9 (maximum compression)
- Streaming compression for large codebases

**Hash Calculation**:
- Use `sha2` crate for SHA-256 hashing
- Hash the final compressed tarball
- Return hex-encoded hash string

**Manifest Generation**:
- Extract metadata from `Cargo.toml`
- Include component ID, version, type, size, hash
- Write as JSON to `{component-name}-manifest.json`

### Dependencies

```toml
[dependencies]
tar = "0.4"              # Cross-platform tarball creation
flate2 = "1.0"           # ZLIB compression
sha2 = "0.10"            # SHA-256 hashing
cargo_metadata = "0.18"  # Parse Cargo.toml
tokio = { version = "1", features = ["process", "fs"] }
```

## Component Unpacking

### Purpose

Unpack ZLIB-compressed component tarballs (both frontend and backend source) to a destination directory, verifying integrity via hash validation.

### API

```rust
use osnova_lib::packaging::unpacker::ComponentUnpacker;

/// Unpack a component (frontend or backend source)
pub async fn unpack_component(
    package_path: &Path,
    dest_dir: &Path,
    expected_hash: &str,
) -> Result<()> {
    let unpacker = ComponentUnpacker::new(package_path)?;

    // Verify hash before unpacking
    unpacker.verify_hash(expected_hash)?;

    // Decompress ZLIB
    let tarball = unpacker.decompress_zlib().await?;

    // Extract tarball to destination
    unpacker.extract_tarball(tarball, dest_dir).await?;

    // Verify extracted files match expectations
    unpacker.verify_extraction(dest_dir).await?;

    Ok(())
}
```

### Implementation Details

**Hash Verification**:
- Calculate SHA-256 of compressed package
- Compare with expected hash from manifest
- Abort if mismatch detected
- Prevents corrupted or tampered packages

**ZLIB Decompression**:
- Use `flate2` crate with ZLIB backend
- Stream decompression to handle large files
- Detect and report corruption errors

**Tarball Extraction**:
- Use `tar-rs` crate for extraction
- Preserve file permissions and timestamps
- Create destination directory if not exists
- Handle path traversal attacks (sanitize paths)

**Safety Checks**:
```rust
fn sanitize_path(path: &Path) -> Result<PathBuf> {
    // Prevent path traversal attacks
    let components: Vec<_> = path.components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .collect();

    if components.is_empty() {
        return Err(PackagingError::InvalidPath);
    }

    Ok(components.iter().collect())
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum UnpackError {
    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Invalid tarball: {0}")]
    InvalidTarball(String),

    #[error("Path traversal attempt detected")]
    PathTraversalDetected,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Rust Toolchain Management

### Purpose

Download and install Rust toolchain on first component compilation to enable local compilation of backend components without bundling compiler binaries.

### Architecture Decision

**Approach**: Download `rustup-init` from official Rust servers on first backend component install.

**Why This Approach**:
- ✅ Avoids macOS/Windows security warnings (no sidecar binaries in app bundle)
- ✅ `rustup-init` is already signed by Rust Foundation (Developer ID on macOS, Authenticode on Windows)
- ✅ Downloads from trusted rust-lang.org servers
- ✅ Smaller Osnova binary (~no extra MB vs ~150-200 MB for bundled toolchain)
- ✅ Always latest stable Rust version
- ✅ No code signing costs for Osnova developers

**Rejected Alternatives**:
- ❌ Bundle rustup-init: Could trigger "sidecar binary" warnings on macOS/Windows
- ❌ Distribute compiled binaries: Triggers OS security warnings for unsigned .dll/.dylib files
- ❌ WebAssembly: Cannot access P2P networks or unrestricted filesystem

### API

```rust
use osnova_lib::packaging::toolchain::RustToolchain;

/// Ensure Rust toolchain is installed (download on first run)
pub async fn ensure_rust_toolchain() -> Result<RustToolchain> {
    let toolchain_dir = get_data_dir()?.join("rust-toolchain");

    if toolchain_dir.exists() {
        return RustToolchain::from_existing(&toolchain_dir);
    }

    // Show progress dialog to user
    show_progress_dialog("Setting up Rust compiler (one-time, ~150 MB)...")?;

    // Download rustup-init from official Rust servers
    let rustup_init = download_rustup_init().await?;

    // Verify SHA-256 hash against official checksums
    verify_rustup_hash(&rustup_init).await?;

    // Run installation to toolchain_dir
    install_toolchain(&rustup_init, &toolchain_dir).await?;

    // Install minimal profile (rustc + cargo only, no docs)
    configure_minimal_toolchain(&toolchain_dir).await?;

    Ok(RustToolchain::new(toolchain_dir))
}

/// Compile a backend component locally
pub async fn compile_backend_component(
    source_dir: &Path,
    component_id: &str,
) -> Result<PathBuf> {
    let toolchain = ensure_rust_toolchain().await?;

    // Show progress to user
    show_progress_dialog(&format!("Compiling {}...", component_id))?;

    // Run cargo build
    let binary_path = toolchain.compile_library(source_dir).await?;

    // Cache compiled binary
    let cache_dir = get_component_cache_dir()?;
    let cached_binary = cache_dir.join(component_id).join("library");

    tokio::fs::copy(&binary_path, &cached_binary).await?;

    Ok(cached_binary)
}
```

### Implementation Details

**Download URLs**:
```rust
const RUSTUP_URLS: &[(&str, &str)] = &[
    ("linux-x86_64", "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"),
    ("linux-aarch64", "https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init"),
    ("macos-x86_64", "https://static.rust-lang.org/rustup/dist/x86_64-apple-darwin/rustup-init"),
    ("macos-aarch64", "https://static.rust-lang.org/rustup/dist/aarch64-apple-darwin/rustup-init"),
    ("windows-x86_64", "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"),
];

fn get_rustup_url() -> Result<&'static str> {
    let platform = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x86_64",
        ("linux", "aarch64") => "linux-aarch64",
        ("macos", "x86_64") => "macos-x86_64",
        ("macos", "aarch64") => "macos-aarch64",
        ("windows", "x86_64") => "windows-x86_64",
        _ => return Err("Unsupported platform".into()),
    };

    Ok(RUSTUP_URLS.iter()
        .find(|(p, _)| *p == platform)
        .map(|(_, url)| *url)
        .unwrap())
}
```

**Hash Verification**:
```rust
/// Download and parse official SHA-256 checksums from Rust
async fn download_official_checksums() -> Result<HashMap<String, String>> {
    let url = "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init.sha256";
    // Download checksums for all platforms...
    // Parse and return as HashMap<platform, hash>
}

/// Verify downloaded rustup-init matches official hash
async fn verify_rustup_hash(rustup_path: &Path) -> Result<()> {
    let expected_hash = download_official_checksums().await?
        .get(current_platform())
        .ok_or("No checksum for platform")?;

    let actual_hash = calculate_sha256(rustup_path)?;

    if actual_hash != *expected_hash {
        return Err("Rustup hash verification failed!".into());
    }

    Ok(())
}
```

**Installation**:
```rust
/// Install Rust toolchain to Osnova's data directory
async fn install_toolchain(rustup_init: &Path, toolchain_dir: &Path) -> Result<()> {
    tokio::fs::create_dir_all(toolchain_dir).await?;

    // Run rustup-init with minimal profile
    let status = Command::new(rustup_init)
        .args(&[
            "-y",                                    // Non-interactive
            "--no-modify-path",                      // Don't modify PATH
            "--profile", "minimal",                  // rustc + cargo only
            "--default-toolchain", "stable",         // Latest stable
        ])
        .env("CARGO_HOME", toolchain_dir.join("cargo"))
        .env("RUSTUP_HOME", toolchain_dir.join("rustup"))
        .status()
        .await?;

    if !status.success() {
        return Err("Rustup installation failed".into());
    }

    Ok(())
}
```

**Local Compilation**:
```rust
impl RustToolchain {
    /// Compile a component source directory to dynamic library
    pub async fn compile_library(&self, source_dir: &Path) -> Result<PathBuf> {
        let cargo_path = self.toolchain_dir.join("cargo/bin/cargo");

        let output = Command::new(&cargo_path)
            .current_dir(source_dir)
            .args(&["build", "--release", "--lib"])
            .env("CARGO_HOME", self.toolchain_dir.join("cargo"))
            .env("RUSTUP_HOME", self.toolchain_dir.join("rustup"))
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Compilation failed: {}", stderr).into());
        }

        // Locate compiled library
        let target_dir = source_dir.join("target/release");
        let lib_name = self.find_library(&target_dir)?;

        Ok(target_dir.join(lib_name))
    }

    fn find_library(&self, target_dir: &Path) -> Result<String> {
        let extension = match std::env::consts::OS {
            "windows" => "dll",
            "macos" => "dylib",
            _ => "so",
        };

        // Find first file matching lib*.{extension}
        for entry in std::fs::read_dir(target_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with("lib") && name.ends_with(extension) {
                return Ok(name);
            }
        }

        Err("No library found in target directory".into())
    }
}
```

### User Experience

**First Backend Component Install**:
1. User clicks "Install" on a backend component
2. Osnova detects no Rust toolchain installed
3. Shows dialog: "Setting up Rust compiler (one-time, ~150 MB)..."
4. Downloads rustup-init (~1-2 MB)
5. Verifies hash against official Rust checksums
6. Runs installation (downloads ~150 MB of Rust toolchain)
7. Compiles component (~10-60 seconds depending on complexity)
8. Caches compiled binary
9. Component ready to use

**Subsequent Backend Component Installs**:
1. User clicks "Install" on another backend component
2. Osnova detects toolchain already installed
3. Shows dialog: "Compiling [component name]..."
4. Compiles component (~10-60 seconds)
5. Caches compiled binary
6. Component ready to use

**Progress Indicators**:
- Download progress: "Downloading Rust compiler: 45.2 MB / 150 MB (30%)"
- Installation progress: "Installing Rust compiler..."
- Compilation progress: "Compiling [component]: rustc detected 15 source files..."

### Caching Strategy

**Compiled Binaries**:
- Cache location: `{data_dir}/component-cache/{component-id}/library.{so|dylib|dll}`
- Cache key: Component ID + version + source hash
- Invalidate cache when: Component updated (version change) or source hash mismatch

**Rust Toolchain**:
- Location: `{data_dir}/rust-toolchain/`
- Update strategy: User-initiated (Settings → "Update Rust Compiler")
- Disk space: ~150-200 MB for minimal profile

### Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["stream"] }  # Download rustup-init
sha2 = "0.10"                                          # Hash verification
tokio = { version = "1", features = ["process", "fs"] }
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ToolchainError {
    #[error("Rust toolchain not available for this platform")]
    UnsupportedPlatform,

    #[error("Failed to download rustup-init: {0}")]
    DownloadFailed(String),

    #[error("Hash verification failed (possible tampering detected)")]
    HashMismatch,

    #[error("Rustup installation failed: {0}")]
    InstallationFailed(String),

    #[error("Component compilation failed: {0}")]
    CompilationFailed(String),

    #[error("No network connection (required for first-time setup)")]
    NetworkUnavailable,
}
```

## Package Manifest Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Component ID (e.g., "com.example.myapp")
    pub id: String,

    /// Semantic version (e.g., "1.2.3")
    pub version: String,

    /// Component type
    #[serde(rename = "type")]
    pub component_type: ComponentType,

    /// Package format (always ZlibTarball for both frontend and backend)
    pub format: PackageFormat,

    /// Package size in bytes
    pub size: usize,

    /// Content hash
    pub hash: Hash,

    /// Relative path to package file (or ant:// URI for network-hosted components)
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Frontend,
    Backend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PackageFormat {
    /// ZLIB-compressed tarball (frontend JS/HTML/CSS or backend Rust source)
    ZlibTarball,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hash {
    pub algorithm: HashAlgorithm,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
}
```

## Integration with Autonomi Network

### Upload Workflow

```rust
use osnova_lib::packaging::frontend::package_frontend;
use osnova_lib::network::upload::upload_data;

async fn publish_component(component_dir: &Path) -> Result<String> {
    // 1. Package component
    let manifest = package_frontend(component_dir, temp_dir).await?;

    // 2. Read packaged file
    let package_data = tokio::fs::read(&manifest.path).await?;

    // 3. Upload to Autonomi Network
    let address = upload_data(&package_data).await?;

    // 4. Update manifest with network address
    let mut manifest = manifest;
    manifest.path = format!("ant://{}", address);

    // 5. Upload manifest to network
    let manifest_json = serde_json::to_vec(&manifest)?;
    let manifest_address = upload_data(&manifest_json).await?;

    Ok(format!("ant://{}", manifest_address))
}
```

### Download Workflow

```rust
use osnova_lib::packaging::unpacker::unpack_component;
use osnova_lib::packaging::toolchain::compile_backend_component;
use osnova_lib::network::download::download_data;

async fn install_component(manifest_uri: &str) -> Result<()> {
    // 1. Download and parse manifest
    let manifest_data = download_data(manifest_uri).await?;
    let manifest: PackageManifest = serde_json::from_slice(&manifest_data)?;

    // 2. Download package
    let package_data = download_data(&manifest.path).await?;

    // 3. Write to temporary file
    let temp_path = temp_dir().join(&manifest.id);
    tokio::fs::write(&temp_path, &package_data).await?;

    // 4. Unpack with hash verification
    let dest_dir = match manifest.component_type {
        ComponentType::Frontend => get_data_dir()?.join("components").join(&manifest.id),
        ComponentType::Backend => temp_dir().join(&manifest.id).join("source"),
    };

    unpack_component(&temp_path, &dest_dir, &manifest.hash.value).await?;

    // 5. Compile backend components locally
    if manifest.component_type == ComponentType::Backend {
        compile_backend_component(&dest_dir, &manifest.id).await?;

        // Cleanup source after compilation
        tokio::fs::remove_dir_all(&dest_dir).await?;
    }

    // 6. Cleanup temporary tarball
    tokio::fs::remove_file(&temp_path).await?;

    Ok(())
}
```

## Testing Strategy

### Unit Tests

Test each packaging operation in isolation:
- Tarball creation and extraction
- ZLIB compression and decompression
- Hash calculation and verification
- Manifest generation and parsing

### Integration Tests

Test complete workflows:
- Package frontend component → verify manifest
- Package backend component → verify binary
- Unpack component → verify contents
- Round-trip: package → unpack → compare

### Cross-Platform Tests

Test on all target platforms:
- Linux: Primary development platform
- macOS: Intel and Apple Silicon
- Windows: MSVC toolchain
- Mobile: Android and iOS (when Tauri support stable)

### Test Fixtures

```
core/osnova_lib/tests/fixtures/
├── frontend-component/     # Example Svelte component
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
├── backend-component/      # Example Rust component
│   ├── Cargo.toml
│   └── src/lib.rs
└── packaged/              # Pre-packaged test files
    ├── example-1.0.0.tar.zz
    └── example-manifest.json
```

## Performance Considerations

### Memory Usage

- **Streaming**: Use streaming compression/decompression to avoid loading entire files in memory
- **Buffer Size**: 8KB buffers for optimal I/O performance
- **Async I/O**: Use `tokio::fs` for non-blocking file operations

### Build Parallelization

- Package multiple components concurrently
- Use Rayon for parallel compression when packaging multiple files
- Limit concurrency to avoid resource exhaustion

### Caching

- Cache build artifacts to speed up repeated packaging
- Skip npm install if `node_modules/` exists and `package-lock.json` unchanged
- Skip cargo build if source files unchanged (use cargo's built-in caching)

## Security Considerations

### Hash Verification

- **Always verify hashes** before unpacking
- Use cryptographically secure hash (SHA-256)
- Detect tampering and corruption

### Path Traversal Protection

- Sanitize all paths during extraction
- Reject paths with `..` components
- Validate all paths stay within destination directory

### Code Signing

Future enhancement: Sign packages with developer keys
- Use Ed25519 signatures
- Verify signatures before installation
- Maintain revocation list for compromised keys

## Future Enhancements

### Component Signing (Not in MVP)

```rust
pub struct SignedPackageManifest {
    pub manifest: PackageManifest,
    pub signature: Signature,
    pub public_key: PublicKey,
}

impl SignedPackageManifest {
    pub fn verify(&self) -> Result<()> {
        // Verify signature matches manifest
        // Check public key against trusted keys
        // Validate not revoked
    }
}
```

### Delta Updates (Not in MVP)

- Binary diff between versions
- Reduce download size for updates
- Use `bsdiff` algorithm

### Dependency Resolution (Not in MVP)

- Parse component dependencies
- Resolve dependency tree
- Download and install dependencies automatically

---

## Related Documentation

- Component ABI: `docs/05-components/component-abi.md`
- Autonomi Operations: `docs/06-protocols/autonomi-operations.md`
- Manifest Schema: `docs/06-protocols/manifest-schema.md`
- Testing Guide: `docs/10-development/testing.md`
