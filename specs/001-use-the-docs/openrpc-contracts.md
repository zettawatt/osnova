# OpenRPC Contract Consolidation

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Generate consolidated contracts/openrpc.json from component specifications during build

Amendment (2025-10-03): Core services/screens are integrated into the Osnova shell. OpenRPC contracts now describe the external RPC surfaces exposed by these built-in services (in stand-alone and server modes) and by any app-supplied components. The in-process Rust APIs are the primary source of truth; OpenRPC mirrors them when exposed.


## Overview

The OpenRPC contracts define the JSON-RPC 2.0 API surface for external endpoints exposed by Osnova in stand-alone and server modes and any app-supplied components. Rather than manually maintaining a single large `contracts/openrpc.json` file, we generate it from the service specification documents during the build process.

## Contract Generation Strategy

### Source of Truth

**Service Specification Files** are the source of truth:
- `components/backend/osnova-core.md`
- `components/backend/osnova-wallet.md`
- `components/backend/osnova-saorsa.md`
- `components/backend/osnova-autonomi.md`
- `components/backend/osnova-bundler.md`

Each file contains complete OpenRPC method definitions with:
- Method names
- Parameters (name, type, required, description)
- Return types
- Error codes
- Examples

### Generation Process

```
Component Specs (Markdown)
    ↓
Extract OpenRPC Definitions
    ↓
Validate JSON Schema
    ↓
Merge into Single Document
    ↓
contracts/openrpc.json (Generated)
    ↓
Generate Rust Stubs
    ↓
Generate TypeScript Client
```

### Build Tool

Create a build tool: `tools/generate-contracts.rs`

```rust
// Pseudo-code for contract generation
fn main() {
    let specs = [
        "components/backend/osnova-core.md",
        "components/backend/osnova-wallet.md",
        "components/backend/osnova-saorsa.md",
        "components/backend/osnova-autonomi.md",
        "components/backend/osnova-bundler.md",
    ];

    let mut openrpc_doc = OpenRpcDocument::new();

    for spec_path in specs {
        let methods = extract_openrpc_methods(spec_path)?;
        openrpc_doc.add_methods(methods);
    }

    openrpc_doc.validate()?;
    openrpc_doc.write_to_file("contracts/openrpc.json")?;

    generate_rust_stubs(&openrpc_doc)?;
    generate_typescript_client(&openrpc_doc)?;
}
```

## Contract Structure

### OpenRPC Document Format

```json
{
  "openrpc": "1.3.2",
  "info": {
    "title": "Osnova API",
    "version": "0.1.0",
    "description": "Complete OpenRPC specification for all Osnova components"
  },
  "methods": [
    {
      "name": "keys.derive",
      "summary": "Derive a new key for a component",
      "params": [
        {
          "name": "componentId",
          "schema": { "type": "string" },
          "required": true,
          "description": "Component identifier"
        },
        {
          "name": "keyType",
          "schema": {
            "type": "string",
            "enum": ["ml_dsa", "ed25519", "secp256k1"]
          },
          "required": true,
          "description": "Type of key to derive"
        }
      ],
      "result": {
        "name": "key",
        "schema": {
          "type": "object",
          "properties": {
            "publicKey": { "type": "string" },
            "keyType": { "type": "string" }
          }
        }
      },
      "errors": [
        {
          "code": -32001,
          "message": "Component not found"
        }
      ],
      "examples": [
        {
          "name": "Derive ML-DSA key",
          "params": [
            { "name": "componentId", "value": "com.example.app" },
            { "name": "keyType", "value": "ml_dsa" }
          ],
          "result": {
            "name": "key",
            "value": {
              "publicKey": "base64-encoded-public-key",
              "keyType": "ml_dsa"
            }
          }
        }
      ]
    }
  ],
  "components": {
    "schemas": {
      "KeyType": {
        "type": "string",
        "enum": ["ml_dsa", "ed25519", "secp256k1"]
      },
      "NetworkType": {
        "type": "string",
        "enum": ["mainnet", "arbitrum"]
      }
    },
    "errors": {
      "ComponentNotFound": {
        "code": -32001,
        "message": "Component not found"
      },
      "InvalidParameters": {
        "code": -32602,
        "message": "Invalid method parameters"
      }
    }
  }
}
```

## Method Naming Convention

### Namespace Structure

All methods follow the pattern: `<service>.<operation>`

**Core Service** (`osnova-core`):
- `keys.*` - Key management
- `config.*` - Configuration
- `storage.*` - Data storage
- `component.*` - Component lifecycle

**Wallet Service** (`osnova-wallet`):
- `wallet.*` - Wallet operations

**Saorsa Service** (`osnova-saorsa`):
- `saorsa.identity.*` - Identity management
- `saorsa.device.*` - Device management
- `saorsa.presence.*` - Presence
- `saorsa.group.*` - Group management
- `saorsa.dht.*` - DHT operations
- `saorsa.messaging.*` - Messaging
- `saorsa.storage.*` - Storage
- `saorsa.disk.*` - Virtual disks
- `saorsa.website.*` - Website hosting
- `saorsa.call.*` - Real-time media
- `saorsa.groupCall.*` - Group calls
- `saorsa.transport.*` - Transport

**Autonomi Service** (`osnova-autonomi`):
- `autonomi.client.*` - Client management
- `autonomi.chunk.*` - Chunk operations
- `autonomi.pointer.*` - Pointer operations
- `autonomi.scratchpad.*` - Scratchpad operations
- `autonomi.archive.*` - Archive operations
- `autonomi.register.*` - Register operations
- `autonomi.graph.*` - GraphEntry operations
- `autonomi.vault.*` - Vault operations

**Bundler Service** (`osnova-bundler`):
- `bundler.backend.*` - Backend compilation
- `bundler.frontend.*` - Frontend packaging
- `bundler.manifest.*` - Manifest operations
- `bundler.workflow.*` - Complete workflows
- `bundler.project.*` - Project management

## Code Generation

### Rust Server Stubs

Generate Rust trait definitions from OpenRPC:

```rust
// Generated code
#[async_trait]
pub trait KeysApi {
    async fn derive(
        &self,
        component_id: String,
        key_type: KeyType,
    ) -> Result<Key, Error>;

    async fn derive_at_index(
        &self,
        component_id: String,
        key_type: KeyType,
        index: u32,
    ) -> Result<Key, Error>;
}

// Implementation
pub struct KeysApiImpl {
    key_manager: Arc<KeyManager>,
}

#[async_trait]
impl KeysApi for KeysApiImpl {
    async fn derive(
        &self,
        component_id: String,
        key_type: KeyType,
    ) -> Result<Key, Error> {
        self.key_manager.derive(&component_id, key_type).await
    }
}
```

### TypeScript Client

Generate TypeScript client from OpenRPC:

```typescript
// Generated code
export class OsnovaClient {
  constructor(private endpoint: string) {}

  async keysDerve(
    componentId: string,
    keyType: 'ml_dsa' | 'ed25519' | 'secp256k1'
  ): Promise<Key> {
    return this.call('keys.derive', { componentId, keyType });
  }

  async walletGetBalance(
    address: string,
    network: 'mainnet' | 'arbitrum'
  ): Promise<Balance> {
    return this.call('wallet.getBalance', { address, network });
  }

  private async call(method: string, params: any): Promise<any> {
    const response = await fetch(this.endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        method,
        params,
        id: Math.random(),
      }),
    });
    const json = await response.json();
    if (json.error) throw new Error(json.error.message);
    return json.result;
  }
}
```

## Contract Testing

### Test Generation

Generate contract tests from OpenRPC examples:

```rust
#[cfg(test)]
mod contract_tests {
    use super::*;

    #[tokio::test]
    async fn test_keys_derive() {
        let api = KeysApiImpl::new();

        // From OpenRPC example
        let result = api.derive(
            "com.example.app".to_string(),
            KeyType::MlDsa,
        ).await.unwrap();

        assert_eq!(result.key_type, KeyType::MlDsa);
        assert!(!result.public_key.is_empty());
    }
}
```

### Contract Validation

Validate all requests/responses against OpenRPC schema:

```rust
pub struct ContractValidator {
    schema: OpenRpcDocument,
}

impl ContractValidator {
    pub fn validate_request(
        &self,
        method: &str,
        params: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        let method_def = self.schema.get_method(method)?;
        validate_params(params, &method_def.params)?;
        Ok(())
    }

    pub fn validate_response(
        &self,
        method: &str,
        result: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        let method_def = self.schema.get_method(method)?;
        validate_result(result, &method_def.result)?;
        Ok(())
    }
}
```

## MVP Implementation Plan

### Phase 1: Manual Contracts (Week 1)
For MVP, start with manually maintained `contracts/openrpc.json`:
- Include core methods needed for walking skeleton
- Add methods incrementally as components are implemented
- Keep in sync with component specs manually

### Phase 2: Contract Generation (Week 4)
Once core components are working:
- Build contract generation tool
- Extract OpenRPC from markdown specs
- Generate consolidated contracts/openrpc.json
- Validate against existing manual contracts

### Phase 3: Code Generation (Week 6)
After contracts are stable:
- Generate Rust server stubs
- Generate TypeScript client
- Integrate into build process
- Add contract tests

## Tooling

### Recommended Tools

- **OpenRPC Generator**: https://github.com/open-rpc/generator
- **JSON Schema Validator**: `jsonschema` crate
- **Markdown Parser**: `pulldown-cmark` crate
- **Code Generation**: `quote` and `syn` crates for Rust

### Build Integration

Add to `Cargo.toml`:

```toml
[[bin]]
name = "generate-contracts"
path = "tools/generate-contracts.rs"
```

Add to build script:

```bash
#!/bin/bash
# scripts/build.sh

# Generate contracts
cargo run --bin generate-contracts

# Generate Rust stubs
openrpc-generator generate \
  -l rust \
  -i contracts/openrpc.json \
  -o src/generated/

# Generate TypeScript client
openrpc-generator generate \
  -l typescript \
  -i contracts/openrpc.json \
  -o frontend/src/generated/
```

## Documentation

### Contract Documentation

Generate API documentation from OpenRPC:

```bash
openrpc-generator generate \
  -l docs \
  -i contracts/openrpc.json \
  -o docs/api/
```

Output: HTML documentation with:
- Method list
- Parameter descriptions
- Return types
- Error codes
- Examples
- Try-it-out interface

## Versioning

### Contract Versioning

Follow semantic versioning for API:
- **Major**: Breaking changes (remove methods, change signatures)
- **Minor**: Additive changes (new methods, optional parameters)
- **Patch**: Documentation, examples, non-breaking fixes

### Version in OpenRPC

```json
{
  "openrpc": "1.3.2",
  "info": {
    "version": "0.1.0"
  }
}
```

### Backward Compatibility

- Keep old methods when adding new versions
- Deprecate before removing
- Document migration path

## Error Handling

### Standard Error Codes

```json
{
  "components": {
    "errors": {
      "ParseError": { "code": -32700 },
      "InvalidRequest": { "code": -32600 },
      "MethodNotFound": { "code": -32601 },
      "InvalidParams": { "code": -32602 },
      "InternalError": { "code": -32603 },
      "ComponentNotFound": { "code": -32001 },
      "KeyNotFound": { "code": -32002 },
      "InsufficientFunds": { "code": -32003 },
      "NetworkError": { "code": -32004 }
    }
  }
}
```

## Summary

**For MVP**:
- ✅ Start with manual `contracts/openrpc.json`
- ✅ Include core methods from component specs
- ✅ Add contract validation in tests
- ✅ Keep in sync with component specs

**Post-MVP**:
- Generate contracts from markdown specs
- Generate Rust stubs and TypeScript client
- Automate contract testing
- Generate API documentation

