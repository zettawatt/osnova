# OpenRPC Conventions (MVP)

Note (2025-10-03): OpenRPC applies to external RPC surfaces in both stand-alone and server modes (for external app components). Core services and GUI screens are now built into the Osnova shell and communicate via in-process Rust APIs; when exposed externally, they mirror these contracts over OpenRPC.


## Error handling
- Use JSON-RPC standard errors where applicable:
  - -32600 Invalid Request
  - -32601 Method Not Found
  - -32602 Invalid Params
  - -32603 Internal Error
- Use server-defined codes in -32000..-32099 range with machine-readable `data.code`, e.g.:
  - -32000 ValidationError (`data.code = "validation_error"`)
  - -32001 NotFound (`data.code = "not_found"`)
  - -32002 Unavailable (`data.code = "unavailable"`)
  - -32003 Unauthorized (`data.code = "unauthorized"`)

Error shape (example):
```json
{
  "code": -32000,
  "message": "Validation failed",
  "data": { "code": "validation_error", "fields": {"address": "invalid"} }
}
```

## Auth
- Reuse the transport's authentication established via pairing (mutual keys/session).
- Methods that mutate state SHOULD require authenticated context; read-only methods MAY allow unauthenticated local calls subject to policy.

## Versioning and compatibility
- Methods MUST be stable for a manifest's pinned component versions.
- Backward-incompatible changes require a new method name or versioned namespace.
- Include a top-level `info.version` in OpenRPC documents; expose host version via `status.get`.

