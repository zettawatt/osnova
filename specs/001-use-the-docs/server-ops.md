# Server Operations and Status (MVP)

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
        "components": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "name": {"type": "string"},
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

