# Cross-Platform Path Management Tasks

**Status**: Ready to Start
**Priority**: High (affects all file operations)
**Generated**: 2025-10-07
**Estimated Duration**: 1-2 weeks

## Overview

Ensure all file system operations use the `dirs` crate for cross-platform compatibility. Currently, some code uses hardcoded paths or platform-specific assumptions that will fail on Windows/macOS/iOS/Android.

## Principles

- **Never hardcode paths** - Use `dirs` crate functions
- **Test on all platforms** - Linux, macOS, Windows, Android, iOS
- **Document platform differences** - Comment on platform-specific behavior
- **Maintain backward compatibility** - Migrate existing data if paths change

---

## Task 073: Audit Existing Path Usage

**Priority**: Critical | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Audit all existing code to identify hardcoded paths and platform-specific assumptions.

**Acceptance Criteria**:
- [ ] Search codebase for hardcoded paths (`/home/`, `C:\`, `~/`)
- [ ] Identify all `std::env::var("HOME")` usage
- [ ] Find all direct path construction (not using `PathBuf::join()`)
- [ ] Document current path usage in all services
- [ ] Create migration plan for existing user data
- [ ] List all files that need updating

**Search Patterns**:
```bash
# Find hardcoded paths
grep -r '/home/' core/osnova_lib/src/
grep -r '~/' core/osnova_lib/src/
grep -r 'C:\\' core/osnova_lib/src/
grep -r '.local/share' core/osnova_lib/src/

# Find environment variable usage
grep -r 'env::var.*HOME' core/osnova_lib/src/
grep -r 'env::var.*APPDATA' core/osnova_lib/src/

# Find direct path construction
grep -r 'Path::new' core/osnova_lib/src/
```

**Files to Create**:
- `docs/10-development/path-audit.md` - Complete audit report
- `.agents/queue/path-migration-plan.md` - Migration strategy

**Dependencies**: None

---

## Task 074: Add `dirs` Crate Dependency

**Priority**: Critical | **Estimated**: 0.5 days | **Parallelizable**: Yes

**Description**: Add `dirs` crate to all Rust workspaces that handle file operations.

**Acceptance Criteria**:
- [ ] Add `dirs = "5"` to workspace `Cargo.toml`
- [ ] Add to `osnova_lib/Cargo.toml` dependencies
- [ ] Add to `app/src-tauri/Cargo.toml` dependencies
- [ ] Run `cargo build` to verify compilation
- [ ] Update lockfiles
- [ ] Document version rationale

**Files to Modify**:
- `Cargo.toml` (workspace)
- `core/osnova_lib/Cargo.toml`
- `app/src-tauri/Cargo.toml`

**Dependencies**: None

---

## Task 075: Create Platform Path Utilities Module

**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Create centralized module for platform-specific path management.

**Acceptance Criteria**:
- [ ] Create `core/osnova_lib/src/platform/paths.rs`
- [ ] Implement `get_data_dir() -> Result<PathBuf>`
- [ ] Implement `get_cache_dir() -> Result<PathBuf>`
- [ ] Implement `get_config_dir() -> Result<PathBuf>`
- [ ] Implement `get_component_cache_dir() -> Result<PathBuf>`
- [ ] Add fallback for platforms without standard dirs
- [ ] Write comprehensive unit tests
- [ ] Document platform-specific behavior
- [ ] Coverage ≥85%

**Example Implementation**:
```rust
//! Platform-specific path utilities
//!
//! Provides cross-platform directory paths for data, cache, and config.

use std::path::PathBuf;
use crate::error::{OsnovaError, Result};

/// Get application data directory
///
/// Platform-specific locations:
/// - Linux: `~/.local/share/osnova/`
/// - macOS: `~/Library/Application Support/osnova/`
/// - Windows: `%LOCALAPPDATA%\osnova\`
/// - Android: `/data/data/com.osnova.app/files/`
/// - iOS: `<app_sandbox>/Library/Application Support/`
pub fn get_data_dir() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir()
        .ok_or_else(|| OsnovaError::Storage("Failed to get data directory".to_string()))?;
    path.push("osnova");
    Ok(path)
}

/// Get application cache directory
///
/// Platform-specific locations:
/// - Linux: `~/.cache/osnova/`
/// - macOS: `~/Library/Caches/osnova/`
/// - Windows: `%LOCALAPPDATA%\osnova\Cache\`
/// - Android: `/data/data/com.osnova.app/cache/`
/// - iOS: `<app_sandbox>/Library/Caches/`
pub fn get_cache_dir() -> Result<PathBuf> {
    let mut path = dirs::cache_dir()
        .ok_or_else(|| OsnovaError::Storage("Failed to get cache directory".to_string()))?;
    path.push("osnova");
    Ok(path)
}

/// Get application config directory
///
/// Platform-specific locations:
/// - Linux: `~/.config/osnova/`
/// - macOS: `~/Library/Application Support/osnova/`
/// - Windows: `%APPDATA%\osnova\`
pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| OsnovaError::Storage("Failed to get config directory".to_string()))?;
    path.push("osnova");
    Ok(path)
}

/// Get component cache directory
///
/// Used for storing downloaded application components.
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

    #[test]
    fn test_get_cache_dir() {
        let path = get_cache_dir().unwrap();
        assert!(path.ends_with("osnova"));
    }

    #[test]
    fn test_get_component_cache_dir() {
        let path = get_component_cache_dir().unwrap();
        assert!(path.ends_with("components"));
    }
}
```

**Files to Create**:
- `core/osnova_lib/src/platform/mod.rs`
- `core/osnova_lib/src/platform/paths.rs`
- `core/osnova_lib/tests/platform/paths_test.rs`

**Dependencies**: Task 074

---

## Task 076: Update IdentityService Path Usage

**Priority**: High | **Estimated**: 0.5 days | **Parallelizable**: Yes (after Task 075)

**Description**: Update IdentityService to use platform-specific paths.

**Acceptance Criteria**:
- [ ] Replace hardcoded storage path with `get_data_dir()`
- [ ] Update `IdentityService::new()` to use platform paths
- [ ] Update all file operations to use `PathBuf::join()`
- [ ] Update tests to use platform paths
- [ ] Verify on Linux (primary)
- [ ] Document path structure in comments

**Files to Modify**:
- `core/osnova_lib/src/services/identity.rs`
- `core/osnova_lib/tests/services/identity_test.rs`

**Before**:
```rust
pub fn new(storage_path: &str) -> Result<Self> {
    let db_path = format!("{}/identity.db", storage_path);
    // ...
}
```

**After**:
```rust
use crate::platform::paths::get_data_dir;

pub fn new() -> Result<Self> {
    let mut db_path = get_data_dir()?;
    db_path.push("identity.db");
    // ...
}
```

**Dependencies**: Task 075

---

## Task 077: Update KeyService Path Usage

**Priority**: High | **Estimated**: 0.5 days | **Parallelizable**: Yes (after Task 075)

**Description**: Update KeyService to use platform-specific paths.

**Acceptance Criteria**:
- [ ] Replace hardcoded storage path with `get_data_dir()`
- [ ] Update `KeyService::new()` to use platform paths
- [ ] Update encrypted key storage paths
- [ ] Update tests to use platform paths
- [ ] Verify on Linux (primary)

**Files to Modify**:
- `core/osnova_lib/src/services/keys.rs`
- `core/osnova_lib/tests/services/keys_test.rs`

**Dependencies**: Task 075

---

## Task 078: Update ConfigService Path Usage

**Priority**: High | **Estimated**: 0.5 days | **Parallelizable**: Yes (after Task 075)

**Description**: Update ConfigService to use platform-specific config directory.

**Acceptance Criteria**:
- [ ] Replace hardcoded path with `get_config_dir()`
- [ ] Update all config file paths
- [ ] Update tests to use platform paths
- [ ] Verify on Linux (primary)

**Files to Modify**:
- `core/osnova_lib/src/services/config.rs`
- `core/osnova_lib/tests/services/config_test.rs`

**Dependencies**: Task 075

---

## Task 079: Update CacheManager Path Usage

**Priority**: Critical | **Estimated**: 0.5 days | **Parallelizable**: Yes (after Task 075)

**Description**: Update CacheManager to use `get_component_cache_dir()`.

**Acceptance Criteria**:
- [ ] Update `CacheManager::new()` to use `get_component_cache_dir()`
- [ ] Remove hardcoded cache directory logic
- [ ] Update all tests
- [ ] Verify cache eviction still works
- [ ] Test on Linux (primary)

**Files to Modify**:
- `core/osnova_lib/src/cache/manager.rs`
- `core/osnova_lib/src/cache/mod.rs` (remove `get_platform_cache_dir()`)
- `core/osnova_lib/tests/cache/manager_test.rs`

**Dependencies**: Task 075

---

## Task 080: Update Tauri Backend Path Usage

**Priority**: High | **Estimated**: 0.5 days | **Parallelizable**: Yes (after Task 075)

**Description**: Update Tauri app to use platform-specific paths.

**Acceptance Criteria**:
- [ ] Replace `OSNOVA_STORAGE_PATH` env var logic with `get_data_dir()`
- [ ] Update `AppState::new()` to use platform paths
- [ ] Remove hardcoded fallback paths
- [ ] Update error messages
- [ ] Test on Linux (primary)

**Files to Modify**:
- `app/src-tauri/src/lib.rs`

**Before**:
```rust
let storage_path = std::env::var("OSNOVA_STORAGE_PATH").unwrap_or_else(|_| {
    let mut path = dirs::data_local_dir().expect("Failed to get local data dir");
    path.push("osnova");
    path.to_str().unwrap().to_string()
});
```

**After**:
```rust
use osnova_lib::platform::paths::get_data_dir;

let storage_path = get_data_dir()
    .map_err(|e| format!("Failed to get data directory: {}", e))?;
```

**Dependencies**: Task 075

---

## Task 081: Add Platform Path Integration Tests

**Priority**: Medium | **Estimated**: 1 day | **Parallelizable**: Yes

**Description**: Add comprehensive integration tests for platform-specific paths.

**Acceptance Criteria**:
- [ ] Test path creation on Linux
- [ ] Test directory creation and permissions
- [ ] Test file operations in platform directories
- [ ] Test path migration from old locations
- [ ] Test fallback behavior when dirs unavailable
- [ ] Mock different platforms for testing
- [ ] Coverage ≥85%

**Files to Create**:
- `core/osnova_lib/tests/platform/paths_integration_test.rs`

**Dependencies**: Tasks 075-080

---

## Task 082: Document Platform-Specific Behavior

**Priority**: Medium | **Estimated**: 0.5 days | **Parallelizable**: Yes

**Description**: Update documentation with platform-specific path information.

**Acceptance Criteria**:
- [ ] Update `docs/02-architecture/data-model.md` with path info
- [ ] Create `docs/10-development/platform-paths.md`
- [ ] Document data migration strategy
- [ ] Add troubleshooting guide for path issues
- [ ] Include platform-specific examples

**Files to Create/Modify**:
- `docs/10-development/platform-paths.md` (new)
- `docs/02-architecture/data-model.md` (update)
- `README.md` (add platform requirements section)

**Dependencies**: Tasks 075-080

---

## Task 083: Create Data Migration Tool

**Priority**: Medium | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Create tool to migrate existing user data to new platform-specific paths.

**Acceptance Criteria**:
- [ ] Detect old data locations
- [ ] Copy data to new platform-specific paths
- [ ] Verify data integrity after migration
- [ ] Handle migration failures gracefully
- [ ] Log migration progress
- [ ] Test migration on all platforms
- [ ] Add `--migrate` CLI flag to Tauri app

**Files to Create**:
- `core/osnova_lib/src/migration/paths.rs`
- `core/osnova_lib/tests/migration/paths_test.rs`
- `docs/10-development/data-migration.md`

**Dependencies**: Tasks 075-080

---

## Task 084: Test on macOS

**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Test all platform path functionality on macOS.

**Acceptance Criteria**:
- [ ] Install Rust toolchain on macOS
- [ ] Build and run all tests on macOS
- [ ] Verify paths use `~/Library/Application Support/osnova/`
- [ ] Verify paths use `~/Library/Caches/osnova/`
- [ ] Test Tauri app on macOS
- [ ] Document macOS-specific issues
- [ ] All tests passing on macOS

**Dependencies**: Tasks 075-083

---

## Task 085: Test on Windows

**Priority**: High | **Estimated**: 1 day | **Parallelizable**: No

**Description**: Test all platform path functionality on Windows.

**Acceptance Criteria**:
- [ ] Install Rust toolchain on Windows
- [ ] Build and run all tests on Windows
- [ ] Verify paths use `%LOCALAPPDATA%\osnova\`
- [ ] Verify paths use `%APPDATA%\osnova\`
- [ ] Test Tauri app on Windows
- [ ] Handle Windows path separators correctly
- [ ] Document Windows-specific issues
- [ ] All tests passing on Windows

**Dependencies**: Tasks 075-083

---

## Task 086: Test on Android (Future)

**Priority**: Low | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Test platform paths on Android when Tauri Android support is added.

**Note**: Defer until Tauri 2.0 Android support is stable.

**Dependencies**: Tasks 075-083, Tauri Android support

---

## Task 087: Test on iOS (Future)

**Priority**: Low | **Estimated**: 2 days | **Parallelizable**: Yes

**Description**: Test platform paths on iOS when Tauri iOS support is added.

**Note**: Defer until Tauri 2.0 iOS support is stable.

**Dependencies**: Tasks 075-083, Tauri iOS support

---

## Summary

**Total Tasks**: 15 (12 immediate, 3 future)
**Estimated Duration**: 10-12 days
**Critical Path**: Tasks 073 → 074 → 075 → 076-080 → 081-083 → 084-085

**Immediate Priority**:
1. Task 073: Audit existing paths
2. Task 074: Add `dirs` dependency
3. Task 075: Create platform utilities
4. Tasks 076-080: Update all services (can run in parallel)
5. Tasks 081-083: Testing and documentation
6. Tasks 084-085: Platform verification

**Blocked/Future**:
- Tasks 086-087: Android/iOS testing (blocked on Tauri support)
