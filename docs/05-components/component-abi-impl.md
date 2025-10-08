# Component ABI Implementation

**Last Updated**: 2025-10-08
**Status**: Complete specification for MVP

## Overview

This document provides the concrete Rust implementation of the Component Application Binary Interface (ABI) for Osnova backend components. Backend components are distributed as source code and compiled locally, then loaded as dynamic libraries (.so, .dylib, .dll).

## Rust Trait Definition

### Core ABI Trait

```rust
// core/osnova_lib/src/components/abi.rs

use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Component status enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentStatus {
    Ok = 0,
    Degraded = 1,
    Error = 2,
    Stopped = 3,
}

/// Result type for component operations
pub type ComponentResult<T> = Result<T, ComponentError>;

/// Component error type
#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Initialization failed: {0}")]
    InitError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Component not initialized")]
    NotInitialized,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

/// Main component ABI trait
pub trait ComponentABI: Send + Sync {
    /// Configure the component with provided settings
    fn component_configure(&mut self, config: Value) -> ComponentResult<()>;

    /// Start the component (initialize resources, spawn tasks)
    fn component_start(&mut self) -> ComponentResult<()>;

    /// Stop the component (cleanup resources, stop tasks)
    fn component_stop(&mut self) -> ComponentResult<()>;

    /// Get current component status
    fn component_status(&self) -> ComponentResult<ComponentStatus>;

    /// Get component metadata
    fn component_info(&self) -> ComponentResult<ComponentInfo>;

    /// Handle OpenRPC method calls
    fn handle_rpc(&mut self, method: &str, params: Value) -> ComponentResult<Value>;
}

/// Component metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub rpc_methods: Vec<String>,
}
```

## FFI Boundary

### C-Compatible FFI Functions

Backend components must export these C-compatible functions:

```rust
// Component must implement these extern "C" functions

/// Create a new component instance
#[no_mangle]
pub extern "C" fn component_create() -> *mut c_void {
    let component = Box::new(MyComponent::new());
    Box::into_raw(component) as *mut c_void
}

/// Destroy a component instance
#[no_mangle]
pub extern "C" fn component_destroy(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut MyComponent);
        }
    }
}

/// Configure the component
#[no_mangle]
pub extern "C" fn component_configure(
    ptr: *mut c_void,
    config_json: *const c_char
) -> i32 {
    if ptr.is_null() || config_json.is_null() {
        return -1;
    }

    unsafe {
        let component = &mut *(ptr as *mut MyComponent);
        let config_str = match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        let config: Value = match serde_json::from_str(config_str) {
            Ok(v) => v,
            Err(_) => return -1,
        };

        match component.component_configure(config) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

/// Start the component
#[no_mangle]
pub extern "C" fn component_start(ptr: *mut c_void) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    unsafe {
        let component = &mut *(ptr as *mut MyComponent);
        match component.component_start() {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

/// Stop the component
#[no_mangle]
pub extern "C" fn component_stop(ptr: *mut c_void) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    unsafe {
        let component = &mut *(ptr as *mut MyComponent);
        match component.component_stop() {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

/// Get component status
#[no_mangle]
pub extern "C" fn component_status(ptr: *mut c_void) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    unsafe {
        let component = &*(ptr as *const MyComponent);
        match component.component_status() {
            Ok(status) => status as i32,
            Err(_) => -1,
        }
    }
}

/// Get component info (returns JSON string, caller must free)
#[no_mangle]
pub extern "C" fn component_info(ptr: *mut c_void) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let component = &*(ptr as *const MyComponent);
        match component.component_info() {
            Ok(info) => {
                match serde_json::to_string(&info) {
                    Ok(json) => {
                        match CString::new(json) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => std::ptr::null_mut(),
                        }
                    }
                    Err(_) => std::ptr::null_mut(),
                }
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Handle RPC call (returns JSON string, caller must free)
#[no_mangle]
pub extern "C" fn component_handle_rpc(
    ptr: *mut c_void,
    method: *const c_char,
    params_json: *const c_char
) -> *mut c_char {
    if ptr.is_null() || method.is_null() || params_json.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let component = &mut *(ptr as *mut MyComponent);

        let method_str = match CStr::from_ptr(method).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        let params_str = match CStr::from_ptr(params_json).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        let params: Value = match serde_json::from_str(params_str) {
            Ok(v) => v,
            Err(_) => return std::ptr::null_mut(),
        };

        match component.handle_rpc(method_str, params) {
            Ok(result) => {
                match serde_json::to_string(&result) {
                    Ok(json) => {
                        match CString::new(json) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => std::ptr::null_mut(),
                        }
                    }
                    Err(_) => std::ptr::null_mut(),
                }
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Free a string returned by the component
#[no_mangle]
pub extern "C" fn component_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
```

## Component Loader Implementation

### Dynamic Library Loading

```rust
// core/osnova_lib/src/components/loader.rs

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct ComponentLoader {
    libraries: HashMap<String, Library>,
    components: HashMap<String, ComponentHandle>,
}

pub struct ComponentHandle {
    library_id: String,
    ptr: *mut c_void,

    // Function pointers
    configure_fn: Symbol<'static, extern "C" fn(*mut c_void, *const c_char) -> i32>,
    start_fn: Symbol<'static, extern "C" fn(*mut c_void) -> i32>,
    stop_fn: Symbol<'static, extern "C" fn(*mut c_void) -> i32>,
    status_fn: Symbol<'static, extern "C" fn(*mut c_void) -> i32>,
    info_fn: Symbol<'static, extern "C" fn(*mut c_void) -> *mut c_char>,
    rpc_fn: Symbol<'static, extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_char>,
    free_fn: Symbol<'static, extern "C" fn(*mut c_char)>,
    destroy_fn: Symbol<'static, extern "C" fn(*mut c_void)>,
}

impl ComponentLoader {
    pub fn new() -> Self {
        ComponentLoader {
            libraries: HashMap::new(),
            components: HashMap::new(),
        }
    }

    /// Load a component from a dynamic library
    pub fn load_component(&mut self, component_id: &str, library_path: &Path) -> Result<()> {
        // Load the dynamic library
        let library = unsafe { Library::new(library_path)? };

        // Get function pointers
        let create_fn: Symbol<extern "C" fn() -> *mut c_void> =
            unsafe { library.get(b"component_create")? };

        let configure_fn = unsafe { library.get(b"component_configure")? };
        let start_fn = unsafe { library.get(b"component_start")? };
        let stop_fn = unsafe { library.get(b"component_stop")? };
        let status_fn = unsafe { library.get(b"component_status")? };
        let info_fn = unsafe { library.get(b"component_info")? };
        let rpc_fn = unsafe { library.get(b"component_handle_rpc")? };
        let free_fn = unsafe { library.get(b"component_free_string")? };
        let destroy_fn = unsafe { library.get(b"component_destroy")? };

        // Create component instance
        let ptr = create_fn();
        if ptr.is_null() {
            return Err("Failed to create component".into());
        }

        // Store library and component handle
        let handle = ComponentHandle {
            library_id: component_id.to_string(),
            ptr,
            configure_fn,
            start_fn,
            stop_fn,
            status_fn,
            info_fn,
            rpc_fn,
            free_fn,
            destroy_fn,
        };

        self.libraries.insert(component_id.to_string(), library);
        self.components.insert(component_id.to_string(), handle);

        Ok(())
    }

    /// Configure a loaded component
    pub fn configure_component(&self, component_id: &str, config: &Value) -> Result<()> {
        let handle = self.components.get(component_id)
            .ok_or("Component not loaded")?;

        let config_json = serde_json::to_string(config)?;
        let config_cstr = CString::new(config_json)?;

        let result = (handle.configure_fn)(handle.ptr, config_cstr.as_ptr());

        if result == 0 {
            Ok(())
        } else {
            Err("Configuration failed".into())
        }
    }

    /// Start a component
    pub fn start_component(&self, component_id: &str) -> Result<()> {
        let handle = self.components.get(component_id)
            .ok_or("Component not loaded")?;

        let result = (handle.start_fn)(handle.ptr);

        if result == 0 {
            Ok(())
        } else {
            Err("Start failed".into())
        }
    }

    /// Stop a component
    pub fn stop_component(&self, component_id: &str) -> Result<()> {
        let handle = self.components.get(component_id)
            .ok_or("Component not loaded")?;

        let result = (handle.stop_fn)(handle.ptr);

        if result == 0 {
            Ok(())
        } else {
            Err("Stop failed".into())
        }
    }

    /// Get component status
    pub fn get_component_status(&self, component_id: &str) -> Result<ComponentStatus> {
        let handle = self.components.get(component_id)
            .ok_or("Component not loaded")?;

        let status = (handle.status_fn)(handle.ptr);

        match status {
            0 => Ok(ComponentStatus::Ok),
            1 => Ok(ComponentStatus::Degraded),
            2 => Ok(ComponentStatus::Error),
            3 => Ok(ComponentStatus::Stopped),
            _ => Err("Invalid status".into()),
        }
    }

    /// Call RPC method on component
    pub fn call_rpc(&self, component_id: &str, method: &str, params: &Value) -> Result<Value> {
        let handle = self.components.get(component_id)
            .ok_or("Component not loaded")?;

        let method_cstr = CString::new(method)?;
        let params_json = serde_json::to_string(params)?;
        let params_cstr = CString::new(params_json)?;

        let result_ptr = (handle.rpc_fn)(
            handle.ptr,
            method_cstr.as_ptr(),
            params_cstr.as_ptr()
        );

        if result_ptr.is_null() {
            return Err("RPC call failed".into());
        }

        unsafe {
            let result_str = CStr::from_ptr(result_ptr).to_str()?;
            let result: Value = serde_json::from_str(result_str)?;

            // Free the returned string
            (handle.free_fn)(result_ptr);

            Ok(result)
        }
    }

    /// Unload a component
    pub fn unload_component(&mut self, component_id: &str) -> Result<()> {
        if let Some(handle) = self.components.remove(component_id) {
            // Stop component if running
            let _ = (handle.stop_fn)(handle.ptr);

            // Destroy component instance
            (handle.destroy_fn)(handle.ptr);
        }

        // Remove library
        self.libraries.remove(component_id);

        Ok(())
    }
}

impl Drop for ComponentLoader {
    fn drop(&mut self) {
        // Unload all components on drop
        let ids: Vec<String> = self.components.keys().cloned().collect();
        for id in ids {
            let _ = self.unload_component(&id);
        }
    }
}
```

## Example Backend Component

### Complete Example Implementation

```rust
// components/backend/example-service/src/lib.rs

use osnova_lib::components::abi::*;
use serde_json::Value;
use std::sync::Mutex;
use tokio::runtime::Runtime;

pub struct ExampleComponent {
    config: Option<Value>,
    status: ComponentStatus,
    runtime: Option<Runtime>,
}

impl ExampleComponent {
    pub fn new() -> Self {
        ExampleComponent {
            config: None,
            status: ComponentStatus::Stopped,
            runtime: None,
        }
    }
}

impl ComponentABI for ExampleComponent {
    fn component_configure(&mut self, config: Value) -> ComponentResult<()> {
        // Validate configuration
        if !config.is_object() {
            return Err(ComponentError::ConfigError(
                "Configuration must be an object".to_string()
            ));
        }

        self.config = Some(config);
        Ok(())
    }

    fn component_start(&mut self) -> ComponentResult<()> {
        if self.config.is_none() {
            return Err(ComponentError::NotInitialized);
        }

        // Create async runtime for background tasks
        let runtime = Runtime::new()
            .map_err(|e| ComponentError::InitError(e.to_string()))?;

        // Spawn background tasks if needed
        runtime.spawn(async {
            // Background work here
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                // Periodic tasks
            }
        });

        self.runtime = Some(runtime);
        self.status = ComponentStatus::Ok;

        Ok(())
    }

    fn component_stop(&mut self) -> ComponentResult<()> {
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }

        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    fn component_status(&self) -> ComponentResult<ComponentStatus> {
        Ok(self.status)
    }

    fn component_info(&self) -> ComponentResult<ComponentInfo> {
        Ok(ComponentInfo {
            id: "com.example.service".to_string(),
            name: "Example Service".to_string(),
            version: "1.0.0".to_string(),
            author: "Example Corp".to_string(),
            description: "Example backend component".to_string(),
            rpc_methods: vec![
                "example.echo".to_string(),
                "example.status".to_string(),
            ],
        })
    }

    fn handle_rpc(&mut self, method: &str, params: Value) -> ComponentResult<Value> {
        match method {
            "example.echo" => {
                // Echo back the params
                Ok(params)
            }
            "example.status" => {
                // Return component status
                Ok(json!({
                    "status": format!("{:?}", self.status),
                    "configured": self.config.is_some(),
                }))
            }
            _ => {
                Err(ComponentError::InvalidParameter(
                    format!("Unknown method: {}", method)
                ))
            }
        }
    }
}

// FFI exports
#[no_mangle]
pub extern "C" fn component_create() -> *mut c_void {
    let component = Box::new(ExampleComponent::new());
    Box::into_raw(component) as *mut c_void
}

// ... (implement all other FFI functions as shown above)
```

### Component Cargo.toml

```toml
[package]
name = "example-service"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Required for dynamic library

[dependencies]
osnova-component-sdk = { path = "../../../core/osnova-component-sdk" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["rt", "time"] }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Strip symbols
```

## Component SDK

### SDK for Component Developers

```rust
// core/osnova-component-sdk/src/lib.rs

pub use osnova_lib::components::abi::{
    ComponentABI,
    ComponentStatus,
    ComponentResult,
    ComponentError,
    ComponentInfo,
};

/// Macro to generate FFI exports
#[macro_export]
macro_rules! export_component {
    ($component_type:ty) => {
        #[no_mangle]
        pub extern "C" fn component_create() -> *mut ::std::ffi::c_void {
            let component = Box::new(<$component_type>::new());
            Box::into_raw(component) as *mut ::std::ffi::c_void
        }

        // ... generate all FFI functions
    };
}
```

## Testing Component ABI

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_component_lifecycle() {
        let mut loader = ComponentLoader::new();

        // Load test component
        let lib_path = PathBuf::from("target/debug/libtest_component.so");
        loader.load_component("test.component", &lib_path).unwrap();

        // Configure
        let config = json!({
            "setting1": "value1",
            "setting2": 42
        });
        loader.configure_component("test.component", &config).unwrap();

        // Start
        loader.start_component("test.component").unwrap();

        // Check status
        let status = loader.get_component_status("test.component").unwrap();
        assert_eq!(status, ComponentStatus::Ok);

        // Call RPC
        let params = json!({ "message": "Hello" });
        let result = loader.call_rpc("test.component", "echo", &params).unwrap();
        assert_eq!(result, params);

        // Stop
        loader.stop_component("test.component").unwrap();

        // Unload
        loader.unload_component("test.component").unwrap();
    }
}
```

## Security Considerations

1. **Sandboxing**: Components run in the same process but should be isolated
2. **Resource Limits**: Implement CPU/memory limits for components
3. **Capability-Based Security**: Components only access allowed APIs
4. **Input Validation**: Validate all RPC inputs before processing
5. **No Direct File Access**: Components use storage API, not direct filesystem

## Performance Optimization

1. **Lazy Loading**: Load components only when needed
2. **Shared Libraries**: Common dependencies in shared libraries
3. **Memory Pool**: Pre-allocate memory for component communication
4. **Async Operations**: Non-blocking RPC calls where possible
5. **Component Caching**: Keep frequently used components loaded

## Platform-Specific Notes

### Linux (.so files)
```bash
# Compile with proper flags
RUSTFLAGS="-C link-arg=-Wl,-soname,libcomponent.so" cargo build --release
```

### macOS (.dylib files)
```bash
# Set install name
install_name_tool -id @rpath/libcomponent.dylib target/release/libcomponent.dylib
```

### Windows (.dll files)
```bash
# Export symbols properly
cargo build --release --target x86_64-pc-windows-msvc
```

## Next Steps

1. Implement component sandboxing with WASI
2. Add resource usage monitoring
3. Create component marketplace integration
4. Implement hot-reload for development
5. Add component versioning and dependencies