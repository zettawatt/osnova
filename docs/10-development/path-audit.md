# Cross-Platform Path Audit Report

**Date**: 2025-10-07
**Status**: Complete
**Priority**: High - Affects cross-platform compatibility

## Executive Summary

Audit of all file path usage in Osnova to identify platform-specific assumptions and hardcoded paths that will fail on Windows/macOS/Android/iOS. This audit covers the Rust core library and Tauri backend.

## Current Path Usage

### ✅ Already Using `dirs` Crate

**File**: `app/src-tauri/src/lib.rs` (lines 296-300)
```rust
let storage_path = std::env::var("OSNOVA_STORAGE_PATH").unwrap_or_else(|_| {
    let mut path = dirs::data_local_dir().expect("Failed to get local data dir");
    path.push("osnova");
    path.to_str().unwrap().to_string()
});
```

**Status**: ✅ Good - Uses `dirs::data_local_dir()` correctly
**Issue**: Converts to String instead of keeping as PathBuf (minor)

### ❌ Platform-Specific Hardcoded Paths

**File**: `core/osnova_lib/src/cache/mod.rs` (lines 68-98)

**Current Implementation**:
```rust
pub fn get_platform_cache_dir() -> Result<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| OsnovaError::Storage("HOME environment variable not set".to_string()))?;
        Ok(PathBuf::from(home).join(".cache/osnova/components"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| OsnovaError::Storage("HOME environment variable not set".to_string()))?;
        Ok(PathBuf::from(home).join("Library/Caches/Osnova/components"))
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
            OsnovaError::Storage("LOCALAPPDATA environment variable not set".to_string())
        })?;
        Ok(PathBuf::from(local_app_data).join("Osnova\\Cache\\components"))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        // Fallback for other platforms (Android, iOS, etc.)
        let current_dir = std::env::current_dir()
            .map_err(|e| OsnovaError::Storage(format!("Failed to get current directory: {}", e)))?;
        Ok(current_dir.join("cache/components"))
    }
}
```

**Issues**:
- ❌ Uses `std::env::var("HOME")` - Linux/macOS only
- ❌ Uses `std::env::var("LOCALAPPDATA")` - Windows only
- ❌ Hardcodes path separators (`\\` on Windows)
- ❌ Manual platform detection instead of using `dirs` crate
- ❌ Fallback uses `current_dir` which is incorrect for mobile

**Should Use**: `dirs::cache_dir()` instead

### ✅ Services Using Storage Path Parameter

All services accept a `storage_path: P` parameter and use it correctly:

- ✅ `IdentityService::new(storage_path)` (identity.rs:57)
- ✅ `KeyService::new(storage_path, cocoon_key)` (keys.rs:88)
- ✅ `ConfigService::new(storage_path)` (config.rs:84)
- ✅ `AppsService::new(storage_path)` (apps.rs:56)
- ✅ `LauncherService::new(storage_path, user_id)` (launcher.rs:92)
- ✅ `UIService::new(storage_path, user_id)` (ui.rs:110)
- ✅ `NavigationService::new(storage_path, user_id)` (navigation.rs:110)

**Status**: ✅ Good - Services are designed to accept platform-specific paths

### ❌ Hardcoded Examples in Documentation

**File**: `core/osnova_lib/src/cache/manager.rs` (lines 18, 86)
```rust
// Example documentation:
let cache_dir = "/home/user/.cache/osnova/components";  // Line 18
let cache = CacheManager::new("/home/user/.cache/osnova", 500 * 1024 * 1024)?;  // Line 86
```

**Issue**: Examples use Linux-specific hardcoded paths
**Fix**: Update examples to use `dirs::cache_dir()` or platform utilities

## Files Requiring Updates

### Critical Priority

1. **`core/osnova_lib/src/cache/mod.rs`** (lines 68-98)
   - Replace `get_platform_cache_dir()` with `dirs::cache_dir()` based implementation
   - Remove manual platform detection
   - Remove environment variable usage

2. **`app/src-tauri/src/lib.rs`** (lines 296-300)
   - Keep PathBuf instead of converting to String
   - Use centralized platform utilities (once created)

### Medium Priority

3. **`core/osnova_lib/src/cache/manager.rs`** (documentation only)
   - Update example code in docstrings (lines 18, 86)
   - Use platform-agnostic examples

4. **`core/osnova_lib/src/cache/mod.rs`** (documentation only)
   - Update module-level documentation examples (lines 21-40)

## Search Results Summary

### Hardcoded Path Patterns Found

| Pattern | Files Found | Status |
|---------|-------------|--------|
| `/home/` | `cache/manager.rs` | ⚠️ Documentation only |
| `~/` | `cache/mod.rs` | ⚠️ Documentation only |
| `.local/share` | None | ✅ Not used |
| `env::var.*HOME` | `cache/mod.rs` | ❌ Critical - Replace with dirs |
| `C:\\` | None | ✅ Not used |
| `Path::new()` | None | ✅ Not used for hardcoded paths |

### Environment Variable Usage

| Variable | File | Line | Status |
|----------|------|------|--------|
| `HOME` | `cache/mod.rs` | 71, 78 | ❌ Replace with `dirs::cache_dir()` |
| `LOCALAPPDATA` | `cache/mod.rs` | 85 | ❌ Replace with `dirs::cache_dir()` |
| `OSNOVA_STORAGE_PATH` | `app/src-tauri/src/lib.rs` | 296 | ✅ Override, fallback uses dirs |

## Dependencies Required

### Add `dirs` Crate

**Already in `app/src-tauri/Cargo.toml`**: ✅ `dirs = "5.0.1"`
**Missing from `core/osnova_lib/Cargo.toml`**: ❌ Need to add

**Action Required**:
```toml
[dependencies]
dirs = "5"
```

## Recommended Implementation Order

1. ✅ **Task 074**: Add `dirs = "5"` to `core/osnova_lib/Cargo.toml`
2. ✅ **Task 075**: Create `core/osnova_lib/src/platform/paths.rs` module with:
   - `get_data_dir() -> Result<PathBuf>`
   - `get_cache_dir() -> Result<PathBuf>`
   - `get_config_dir() -> Result<PathBuf>`
   - `get_component_cache_dir() -> Result<PathBuf>`

3. ✅ **Task 079**: Replace `get_platform_cache_dir()` in `cache/mod.rs` with `platform::paths::get_component_cache_dir()`

4. ✅ **Task 080**: Update `app/src-tauri/src/lib.rs` to use `platform::paths::get_data_dir()`

5. ✅ **Tasks 076-078**: Update services (if needed - currently they accept paths correctly)

6. ✅ **Task 082**: Update documentation examples

## Migration Impact

### Existing Users

**Current Behavior**:
- Linux: Data in `~/.local/share/osnova/`
- Cache in `~/.cache/osnova/components/`

**After Migration**:
- Linux: Data in `~/.local/share/osnova/` (unchanged)
- Cache in `~/.cache/osnova/` (unchanged, but path construction method changes)

**Migration Required**: No - paths remain the same on Linux

**New Platform Support**:
- macOS: Data in `~/Library/Application Support/osnova/`
- macOS: Cache in `~/Library/Caches/osnova/`
- Windows: Data in `%LOCALAPPDATA%\osnova\`
- Windows: Cache in `%LOCALAPPDATA%\osnova\Cache\`

## Testing Requirements

### Platform Tests Needed

- ✅ Linux: Verify paths use `~/.local/share/osnova/` and `~/.cache/osnova/`
- ✅ macOS: Verify paths use `~/Library/Application Support/osnova/` and `~/Library/Caches/osnova/`
- ✅ Windows: Verify paths use `%LOCALAPPDATA%\osnova\`
- ⏳ Android: Defer until Tauri support is stable
- ⏳ iOS: Defer until Tauri support is stable

### Test Coverage Required

- Unit tests for path utilities (≥85% coverage)
- Integration tests for each service using platform paths
- Cross-platform path creation tests
- Fallback behavior tests (when dirs unavailable)

## Conclusion

**Summary**:
- ✅ Services are well-designed to accept platform-specific paths
- ❌ Cache module has platform-specific hardcoded logic that must be replaced
- ✅ Tauri app already uses `dirs` crate but should use centralized utilities
- ⚠️ Documentation examples use hardcoded Linux paths

**Impact**: Medium - Affects cache functionality and cross-platform compatibility

**Estimated Effort**: 10-12 days (as per CROSS_PLATFORM_PATHS.md task breakdown)

**Next Steps**:
1. Add `dirs` dependency to `core/osnova_lib`
2. Create centralized platform path utilities
3. Replace cache module's platform detection
4. Update all documentation examples
5. Test on macOS and Windows

## Related Documentation

- Implementation guide: `docs/10-development/cross-platform-paths.md`
- Task breakdown: `.agents/queue/CROSS_PLATFORM_PATHS.md`
- Architecture: `docs/02-architecture/data-model.md`
