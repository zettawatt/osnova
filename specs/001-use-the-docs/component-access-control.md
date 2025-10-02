# Component Access Control

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Allow all components to call all other components for MVP; add capability-based permissions post-MVP

## Overview

Component access control determines which components can call which other components' OpenRPC methods. For MVP, we use a permissive model to reduce complexity, with plans for a capability-based system post-MVP.

## MVP Access Control Model

### Permissive Model

**Rule**: Any component can call any other component's OpenRPC methods

**Rationale**:
- Simplifies MVP implementation
- Reduces development time
- Allows rapid prototyping
- Sufficient for trusted components
- Can be tightened post-MVP

**Assumptions**:
- All components are trusted (installed by user)
- No malicious components in MVP
- User controls what gets installed
- Focus on functionality over security

### Access Flow

```
Component A (Frontend or Backend)
    ↓
Tauri Backend (OpenRPC Router)
    ↓
Component B (Backend)
    ↓
Response
```

**No access checks** in MVP - all calls are forwarded

## Implementation

### OpenRPC Router (MVP)

```rust
pub struct OpenRpcRouter {
    components: Arc<RwLock<HashMap<String, ComponentEndpoint>>>,
}

impl OpenRpcRouter {
    pub async fn route_call(
        &self,
        method: &str,
        params: serde_json::Value,
        caller: Option<String>, // Component ID of caller (unused in MVP)
    ) -> Result<serde_json::Value, Error> {
        // Parse method to extract component
        let (component_id, method_name) = parse_method(method)?;
        
        // MVP: No access control check
        // Post-MVP: Check if caller has permission to call this method
        
        // Get component endpoint
        let components = self.components.read().await;
        let endpoint = components.get(&component_id)
            .ok_or(Error::ComponentNotFound)?;
        
        // Forward call
        let response = self.forward_call(endpoint, method_name, params).await?;
        
        Ok(response)
    }
}
```

### Logging (MVP)

Even without enforcement, log all inter-component calls for debugging:

```rust
pub async fn route_call(
    &self,
    method: &str,
    params: serde_json::Value,
    caller: Option<String>,
) -> Result<serde_json::Value, Error> {
    // Log the call
    tracing::debug!(
        caller = ?caller,
        method = method,
        "Component call"
    );
    
    // Route without access check
    self.route_call_internal(method, params).await
}
```

## Post-MVP: Capability-Based Access Control

### Capability Model

**Capabilities** are permissions granted to components:

```rust
pub enum Capability {
    // Key management
    DeriveKey,
    GetKey,
    
    // Storage
    ReadStorage,
    WriteStorage,
    
    // Network
    NetworkAccess,
    
    // Wallet
    ReadBalance,
    RequestPayment,
    
    // Autonomi
    UploadData,
    DownloadData,
    
    // Saorsa
    SendMessage,
    ReceiveMessage,
    
    // System
    SpawnComponent,
    StopComponent,
}
```

### Capability Declaration

Components declare required capabilities in manifest:

```json
{
  "componentId": "com.example.app",
  "capabilities": {
    "required": [
      "network.access",
      "wallet.read_balance",
      "wallet.request_payment",
      "autonomi.upload",
      "autonomi.download"
    ],
    "optional": [
      "saorsa.send_message"
    ]
  }
}
```

### Capability Granting

User grants capabilities during installation:

```
┌─────────────────────────────────────────────┐
│  Install Component                          │
├─────────────────────────────────────────────┤
│                                             │
│  com.example.app v1.0.0                    │
│                                             │
│  This component requests:                   │
│                                             │
│  ✓ Network access                           │
│  ✓ Read wallet balance                      │
│  ✓ Request payments                         │
│  ✓ Upload to Autonomi                       │
│  ✓ Download from Autonomi                   │
│                                             │
│  [Deny]  [Grant]                            │
│                                             │
└─────────────────────────────────────────────┘
```

### Capability Enforcement

```rust
pub struct AccessControlManager {
    grants: Arc<RwLock<HashMap<String, HashSet<Capability>>>>,
}

impl AccessControlManager {
    pub async fn check_permission(
        &self,
        component_id: &str,
        capability: Capability,
    ) -> Result<(), Error> {
        let grants = self.grants.read().await;
        let component_caps = grants.get(component_id)
            .ok_or(Error::ComponentNotFound)?;
        
        if component_caps.contains(&capability) {
            Ok(())
        } else {
            Err(Error::PermissionDenied)
        }
    }
}

// In router
pub async fn route_call(
    &self,
    method: &str,
    params: serde_json::Value,
    caller: Option<String>,
) -> Result<serde_json::Value, Error> {
    // Determine required capability
    let capability = method_to_capability(method)?;
    
    // Check permission
    if let Some(caller_id) = caller {
        self.access_control.check_permission(&caller_id, capability).await?;
    }
    
    // Route call
    self.route_call_internal(method, params).await
}
```

### Capability Mapping

Map OpenRPC methods to capabilities:

```rust
fn method_to_capability(method: &str) -> Result<Capability, Error> {
    match method {
        "keys.derive" => Ok(Capability::DeriveKey),
        "keys.getByPublicKey" => Ok(Capability::GetKey),
        "storage.set" => Ok(Capability::WriteStorage),
        "storage.get" => Ok(Capability::ReadStorage),
        "wallet.getBalance" => Ok(Capability::ReadBalance),
        "wallet.requestPayment" => Ok(Capability::RequestPayment),
        "autonomi.chunk.upload" => Ok(Capability::UploadData),
        "autonomi.chunk.download" => Ok(Capability::DownloadData),
        "saorsa.messaging.send" => Ok(Capability::SendMessage),
        _ => Err(Error::UnknownMethod),
    }
}
```

## Security Considerations

### MVP Risks

**Risk**: Malicious component can access all APIs
**Mitigation**: 
- Only install trusted components
- Review component source before installation
- Sandboxing at OS level (future)

**Risk**: Component can steal keys
**Mitigation**:
- Keys never exposed directly
- Only derived keys provided
- Component-specific key isolation

**Risk**: Component can drain wallet
**Mitigation**:
- User approval required for all payments
- Payment dialog shows amount and recipient
- Transaction history logged

### Post-MVP Enhancements

1. **Principle of Least Privilege**: Grant minimum capabilities needed
2. **Runtime Revocation**: User can revoke capabilities after installation
3. **Capability Auditing**: Log all capability usage
4. **Sandboxing**: OS-level process isolation
5. **Code Signing**: Verify component authenticity
6. **Reputation System**: Track component behavior

## Frontend Access Control

### Frontend to Backend

**MVP**: Frontend components can call any backend component
**Post-MVP**: Same capability model applies

### Frontend Isolation

**MVP**: All frontend components run in same WebView context
**Post-MVP**: Separate WebView contexts per component (Tauri isolation)

```rust
// Post-MVP: Isolated WebViews
let window = tauri::WindowBuilder::new(
    app,
    "component-window",
    tauri::WindowUrl::App("component.html".into())
)
.isolation(true) // Enable isolation
.build()?;
```

## Component-to-Component Communication

### Direct Communication (Not Allowed)

Components cannot communicate directly:
- No shared memory
- No IPC between components
- All communication through Tauri backend

### Indirect Communication (Allowed)

Components can communicate via shared storage:
- Component A writes to storage
- Component B reads from storage
- Mediated by osnova-core

```rust
// Component A
storage.set("shared.data", value).await?;

// Component B
let value = storage.get("shared.data").await?;
```

**Post-MVP**: Add explicit messaging API with access control

## User Controls

### MVP

- Install/uninstall components
- View running components
- Stop components manually

### Post-MVP

- View component capabilities
- Grant/revoke capabilities
- View capability usage logs
- Set spending limits per component
- Whitelist/blacklist components

## Audit Logging

### MVP Logging

Log all inter-component calls:

```rust
tracing::info!(
    caller = ?caller,
    method = method,
    timestamp = ?Utc::now(),
    "Component call"
);
```

### Post-MVP Logging

Enhanced audit trail:

```rust
pub struct AuditLog {
    timestamp: DateTime<Utc>,
    caller: String,
    method: String,
    capability: Capability,
    granted: bool,
    params_hash: String, // Hash of params for privacy
    result: Result<(), Error>,
}
```

## Testing

### MVP Tests

Test that all calls are routed correctly:

```rust
#[tokio::test]
async fn test_component_call_routing() {
    let router = OpenRpcRouter::new();
    
    // Any component can call any method
    let result = router.route_call(
        "wallet.getBalance",
        json!({"address": "0x123"}),
        Some("com.example.app".to_string()),
    ).await;
    
    assert!(result.is_ok());
}
```

### Post-MVP Tests

Test capability enforcement:

```rust
#[tokio::test]
async fn test_capability_enforcement() {
    let router = OpenRpcRouter::new();
    
    // Component without capability should be denied
    let result = router.route_call(
        "wallet.requestPayment",
        json!({"amount": "1.0"}),
        Some("untrusted.component".to_string()),
    ).await;
    
    assert!(matches!(result, Err(Error::PermissionDenied)));
}
```

## Migration Path

### Phase 1: MVP (Permissive)
- No access control
- All calls allowed
- Logging only

### Phase 2: Capability Declaration
- Components declare capabilities in manifest
- User sees capabilities during install
- No enforcement yet

### Phase 3: Capability Enforcement
- Enforce capability checks
- Deny unauthorized calls
- User can grant/revoke

### Phase 4: Advanced Features
- Runtime capability revocation
- Spending limits
- Reputation system
- Sandboxing

## Configuration

### Developer Mode

Allow bypassing access control for development:

```json
{
  "accessControl": {
    "enabled": false,
    "logOnly": true
  }
}
```

**Warning**: Never disable in production

## Documentation

Document access control model in:
- Component development guide
- Security documentation
- User guide (post-MVP)

## Summary

**MVP Approach**:
✅ Permissive model - all components can call all methods
✅ Logging for debugging
✅ User approval for sensitive operations (payments)
✅ Component isolation via separate processes

**Post-MVP Enhancements**:
- Capability-based access control
- User-granted permissions
- Runtime revocation
- Audit logging
- Sandboxing

This approach allows rapid MVP development while providing a clear path to robust security post-MVP.

