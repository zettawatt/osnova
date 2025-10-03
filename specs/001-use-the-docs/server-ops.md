# Server Operations and Status (MVP)


Note (2025-10-03): In server mode, the status surface reports built-in services and, when hosted, app-supplied components. The schema below uses "services" to cover both.

## status.get (read-only)
```json
{
  "name": "status.get",
  "params": [],
  "result": {
    "name": "status",
    "schema": {
      "type": "object",
      "properties": {
        "status": {"type": "string", "enum": ["ok", "degraded", "error"]},
        "version": {"type": "string"},
        "uptime": {"type": "integer"},
        "services": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "name": {"type": "string"},
              "kind": {"type": "string", "enum": ["built_in_service", "app_component"]},
              "status": {"type": "string", "enum": ["ok", "degraded", "error"]}
            },
            "required": ["name", "status"]
          }
        }
      },
      "required": ["status", "version", "uptime"]
    }
  }
}
```

## Service management (baseline)
- Headless server mode suitable for system services (start/stop/restart).
- Expose the read-only status method via the chosen control interface.
- File-based logging with rotation (policy documented in the plan).

