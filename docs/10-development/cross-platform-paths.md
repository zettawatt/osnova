# Cross-Platform Path Management

**Last Updated**: 2025-10-07

## Overview

Osnova must work correctly on all target platforms: Windows, macOS, Linux, Android, and iOS. Each platform has different conventions for where applications store data, cache, and configuration files. To ensure cross-platform compatibility, **all file system operations MUST use the `dirs` crate**.

## Requirements

### Dependency

```toml
[dependencies]
dirs = "5"
```

**Why `dirs`?**
- Provides platform-specific standard directories
- Handles Windows/macOS/Linux/Android/iOS differences automatically
- Maintained and widely used in Rust ecosystem
- No manual platform detection needed

## Platform-Specific Directory Locations

### Data Directory (`dirs::data_local_dir()`)

Used for: Application databases, persistent user data, identity storage

| Platform | Location |
|----------|----------|
| Linux | `~/.local/share/osnova/` |
| macOS | `~/Library/Application Support/osnova/` |
| Windows | `%LOCALAPPDATA%\osnova\` |
| Android | `/data/data/com.osnova.app/files/` |
| iOS | `<app_sandbox>/Library/Application Support/` |

**Example**:
```rust
use dirs;
use std::path::PathBuf;

fn get_data_dir() -> Result<PathBuf, String> {
    let mut path = dirs::data_local_dir()
        .ok_or("Failed to get data directory")?;
    path.push("osnova");
    Ok(path)
}
```

**Usage in code**:
```rust
let mut db_path = get_data_dir()?;
db_path.push("identity.db");
```

### Cache Directory (`dirs::cache_dir()`)

Used for: Downloaded components, temporary files, evictable data

| Platform | Location |
|----------|----------|
| Linux | `~/.cache/osnova/` |
| macOS | `~/Library/Caches/osnova/` |
| Windows | `%LOCALAPPDATA%\osnova\Cache\` |
| Android | `/data/data/com.osnova.app/cache/` |
| iOS | `<app_sandbox>/Library/Caches/` |

**Example**:
```rust
fn get_cache_dir() -> Result<PathBuf, String> {
    let mut path = dirs::cache_dir()
        .ok_or("Failed to get cache directory")?;
    path.push("osnova");
    Ok(path)
}
```

**Component cache subdirectory**:
```rust
fn get_component_cache_dir() -> Result<PathBuf, String> {
    let mut path = get_cache_dir()?;
    path.push("components");
    Ok(path)
}
```

### Config Directory (`dirs::config_dir()`)

Used for: User preferences, UI settings, non-sensitive configuration

| Platform | Location |
|----------|----------|
| Linux | `~/.config/osnova/` |
| macOS | `~/Library/Application Support/osnova/` |
| Windows | `%APPDATA%\osnova\` |
| Android | `/data/data/com.osnova.app/shared_prefs/` |
| iOS | `<app_sandbox>/Library/Preferences/` |

**Example**:
```rust
fn get_config_dir() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir()
        .ok_or("Failed to get config directory")?;
    path.push("osnova");
    Ok(path)
}
```

## Implementation Rules

### Rule 1: Never Hardcode Paths

**❌ Wrong**:
```rust
let path = "/home/user/.local/share/osnova/identity.db";
let path = "~/.local/share/osnova/identity.db";
let path = format!("{}/osnova/data", std::env::var("HOME").unwrap());
```

**✅ Correct**:
```rust
let mut path = dirs::data_local_dir()
    .ok_or("Failed to get data directory")?;
path.push("osnova");
path.push("identity.db");
```

### Rule 2: Always Use `PathBuf::join()`

**❌ Wrong**:
```rust
let path = format!("{}/osnova/cache", cache_dir);  // Hardcoded separator
```

**✅ Correct**:
```rust
let mut path = cache_dir;
path.push("osnova");
path.push("cache");
```

### Rule 3: Handle Fallback Cases

Some environments (containers, test environments) may not have standard directories. Always handle `None` gracefully:

```rust
fn get_data_dir() -> Result<PathBuf, String> {
    let base = dirs::data_local_dir()
        .ok_or("Failed to get data directory. Is this running in a container?")?;

    let mut path = base;
    path.push("osnova");
    Ok(path)
}
```

### Rule 4: Document Platform-Specific Behavior

Always add comments explaining platform differences:

```rust
/// Get the application data directory
///
/// Returns platform-specific locations:
/// - Linux: `~/.local/share/osnova/`
/// - macOS: `~/Library/Application Support/osnova/`
/// - Windows: `%LOCALAPPDATA%\osnova\`
/// - Android: `/data/data/com.osnova.app/files/`
/// - iOS: `<app_sandbox>/Library/Application Support/`
pub fn get_data_dir() -> Result<PathBuf> {
    // Implementation...
}
```

### Rule 5: Test on All Platforms

Every PR that touches file paths must be tested on:
- ✅ Linux (primary development platform)
- ✅ macOS (via CI or manual testing)
- ✅ Windows (via CI or manual testing)
- ⏳ Android (when Tauri support is stable)
- ⏳ iOS (when Tauri support is stable)

## Implementation Utilities

Create centralized path utilities in `core/osnova_lib/src/platform/paths.rs`:

```rust
//! Platform-specific path utilities
//!
//! Provides cross-platform directory paths for data, cache, and config.

use std::path::PathBuf;
use crate::error::{OsnovaError, Result};

/// Get application data directory
pub fn get_data_dir() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir()
        .ok_or_else(|| OsnovaError::Storage(
            "Failed to get data directory".to_string()
        ))?;
    path.push("osnova");
    Ok(path)
}

/// Get application cache directory
pub fn get_cache_dir() -> Result<PathBuf> {
    let mut path = dirs::cache_dir()
        .ok_or_else(|| OsnovaError::Storage(
            "Failed to get cache directory".to_string()
        ))?;
    path.push("osnova");
    Ok(path)
}

/// Get application config directory
pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| OsnovaError::Storage(
            "Failed to get config directory".to_string()
        ))?;
    path.push("osnova");
    Ok(path)
}

/// Get component cache directory
pub fn get_component_cache_dir() -> Result<PathBuf> {
    let mut path = get_cache_dir()?;
    path.push("components");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_dir() {
        let path = get_data_dir().unwrap();
        assert!(path.ends_with("osnova"));

        // Verify platform-specific base
        #[cfg(target_os = "linux")]
        assert!(path.to_str().unwrap().contains(".local/share"));

        #[cfg(target_os = "macos")]
        assert!(path.to_str().unwrap().contains("Library/Application Support"));

        #[cfg(target_os = "windows")]
        assert!(path.to_str().unwrap().contains("AppData\\Local"));
    }
}
```

## Migration Strategy

### For Existing Users

If users already have data in old hardcoded paths, create a migration tool:

```rust
/// Migrate data from old hardcoded paths to platform-specific locations
pub fn migrate_legacy_data() -> Result<()> {
    // Check for old Linux-specific path
    let old_path = PathBuf::from("/home").join(
        std::env::var("USER").unwrap_or_default()
    ).join(".local/share/osnova");

    if old_path.exists() {
        let new_path = get_data_dir()?;

        // Copy data to new location
        copy_recursive(&old_path, &new_path)?;

        // Verify integrity
        verify_data_integrity(&new_path)?;

        // Keep old path as backup (don't delete)
        log::info!("Migrated data from {} to {}",
                   old_path.display(), new_path.display());
    }

    Ok(())
}
```

### Testing Migration

```rust
#[test]
fn test_migration() {
    // Create temp directory with old structure
    let temp = TempDir::new().unwrap();
    let old_path = temp.path().join(".local/share/osnova");
    fs::create_dir_all(&old_path).unwrap();
    fs::write(old_path.join("test.db"), b"test data").unwrap();

    // Run migration
    migrate_legacy_data().unwrap();

    // Verify new location has data
    let new_path = get_data_dir().unwrap();
    assert!(new_path.join("test.db").exists());
}
```

## Common Pitfalls

### ❌ Don't Use Environment Variables

```rust
// Wrong - Linux-only
let home = std::env::var("HOME").unwrap();
let path = format!("{}/.local/share/osnova", home);

// Correct - Cross-platform
let mut path = dirs::data_local_dir().unwrap();
path.push("osnova");
```

### ❌ Don't Hardcode Path Separators

```rust
// Wrong - Unix-only
let path = base_path + "/osnova/data";

// Correct - Cross-platform
let mut path = base_path;
path.push("osnova");
path.push("data");
```

### ❌ Don't Assume Filesystem Case Sensitivity

```rust
// Risky - macOS filesystem is case-insensitive by default
let path1 = data_dir.join("MyApp");
let path2 = data_dir.join("myapp");  // Same on macOS!

// Better - Use consistent casing
let path = data_dir.join("osnova");  // Always lowercase
```

## Debugging Path Issues

### Logging Current Paths

```rust
fn log_platform_paths() {
    log::debug!("Data dir: {:?}", dirs::data_local_dir());
    log::debug!("Cache dir: {:?}", dirs::cache_dir());
    log::debug!("Config dir: {:?}", dirs::config_dir());
    log::debug!("Osnova data: {:?}", get_data_dir().ok());
    log::debug!("Osnova cache: {:?}", get_cache_dir().ok());
}
```

### Testing Path Creation

```rust
#[test]
fn test_directory_creation() {
    let data_dir = get_data_dir().unwrap();

    // Ensure directory exists
    fs::create_dir_all(&data_dir).unwrap();

    // Verify it's writable
    let test_file = data_dir.join("test.txt");
    fs::write(&test_file, b"test").unwrap();

    // Cleanup
    fs::remove_file(&test_file).unwrap();
}
```

## Related Documentation

- Implementation tasks: `.agents/queue/CROSS_PLATFORM_PATHS.md`
- Testing strategy: `docs/10-development/testing.md`
- Architecture: `docs/02-architecture/data-model.md`

## References

- `dirs` crate documentation: https://docs.rs/dirs/latest/dirs/
- Platform directory standards:
  - Linux: XDG Base Directory Specification
  - macOS: Apple File System Programming Guide
  - Windows: Known Folders API
