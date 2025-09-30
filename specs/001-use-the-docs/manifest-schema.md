# Manifest Schema and Trust Model (MVP)

## Purpose
Define a minimal, testable schema for Osnova application manifests and a trust model suitable for MVP, while remaining implementation-agnostic in the spec.

## JSON Schema (skeleton)
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://osnova.dev/schemas/manifest.json",
  "title": "Osnova Application Manifest",
  "type": "object",
  "required": ["id", "name", "version", "components"],
  "properties": {
    "id": {"type": "string", "description": "Content address or manifest hash"},
    "name": {"type": "string"},
    "version": {"type": "string", "pattern": "^\n?\n?\n?", "description": "Semver; exact pinned version"},
    "description": {"type": "string"},
    "publisher": {"type": "string", "description": "Publisher identifier"},
    "signature": {"type": "string", "description": "Detached signature over canonical manifest"},
    "components": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "kind", "version"],
        "properties": {
          "id": {"type": "string", "description": "Content address of the component"},
          "kind": {"type": "string", "enum": ["frontend", "backend"]},
          "version": {"type": "string"},
          "integrity": {"type": "string", "description": "Hash (e.g., blake3 base64) of the fetched artifact"},
          "config": {"type": "object", "additionalProperties": true},
          "devRef": {"type": "string", "description": "Local dev reference (path/URL)", "nullable": true},
          "prodRef": {"type": "string", "description": "Content-addressed URI in production"},
          "mirrors": {"type": "array", "items": {"type": "string"}}
        }
      }
    },
    "metadata": {"type": "object", "additionalProperties": true}
  }
}
```

Notes:
- Keep the schema strict for required fields; allow forward-compatible metadata via additionalProperties: true at top-level and component level.
- Dev vs Prod: manifests used in development MAY specify `devRef` (non-content-addressed). Production MUST specify `prodRef` and SHOULD include `integrity`.

## Trust model (MVP)
- Pinned versions: Manifests pin exact component versions by content address and version.
- Signing: A detached signature over a canonical JSON form (JCS) signed by the publisher. Verification is an implementation detail in the plan.
- Verification: On fetch, verify integrity hash and optional signature before activation. If verification fails, abort launch with a user-visible error.
- Mirrors: Optional list of mirror URIs. Fetch MUST verify integrity regardless of source.

## Validation rules and errors
- If a required component is missing/unresolvable: show a clear error and cancel launch.
- If integrity/signature verification fails: show a clear error and cancel launch.
- If schema validation fails: surface validation messages for debugging; do not start components.

