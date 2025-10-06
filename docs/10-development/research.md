# Research — Initialize Osnova feature spec from docs/spec.md

This document consolidates Phase 0 research outcomes and clarifications captured on 2025-09-30.

## Decisions and Rationales

- Decision: End-to-end encryption (E2E) of user data in Client–Server mode
  - Rationale: Preserve confidentiality on user-controlled servers; align with privacy expectations; limits breach impact
  - Alternatives considered:
    - TLS-only with server-side decryption (rejected: higher trust in server, larger attack surface)
    - Hybrid E2E for data blobs + TLS for control (viable later; adds complexity for MVP)

- Decision: Server supports at least 5 concurrent mobile clients (MVP)
  - Rationale: Covers common household/family setups; provides baseline for scalability tests
  - Alternatives considered: 2 (too limiting), 10+ (defer until post-MVP scaling)

- Decision: Performance target p95 launch→first meaningful render ≤ 2s
  - Rationale: Perceived snappiness; competitive UX bar for local/remote apps
  - Alternatives considered: 3–5s (risk of sluggish feel)

- Decision: “Slow server” fallback threshold at p95 backend latency > 5s
  - Rationale: Minimizes user frustration while avoiding premature failover prompts
  - Alternatives considered: 1–2s (too aggressive), 8s (too lenient)

- Decision: Availability stance for MVP = Best-effort (no formal SLO)
  - Rationale: Focus on core functionality and architecture first; defer SLO commitment to post-MVP
  - Alternatives considered: 99–99.9% (adds ops overhead without clear baseline)

## Open Questions and Current Assumptions (for your confirmation)

1) Tech stack selection per platform (client, server, mobile)
- Clarification: All use the Tauri framework.
- Clarification: The server uses the same desktop frontend with an option to run headless by passing a `--server` argument on invocation.
- Assumptions: One codebase with Tauri targets for desktop and mobile; server mode toggled by CLI flag and config. Headless implies no WebView; backend plugins still loaded and OpenRPC servers exposed.
- Please confirm: Are there any non-Tauri surfaces (e.g., pure CLI utilities, system services) we should plan for, or is everything launched via the Tauri shell?

2) Observability signals (logs/metrics/traces) and minimal dashboards
- Assumptions: Structured logs (JSON) from host and components; metrics via a lightweight exporter (e.g., Prometheus textfile or in-process endpoint) when running in server mode; tracing via OpenTelemetry (no external collector required for MVP, local file/export acceptable).
- Minimal dashboards: A simple “Status” panel in the Config Manager showing component health (component_status), last errors, and key metrics (CPU/mem for component processes if available, request counts/latency p95 for OpenRPC methods).
- Please confirm: Target observability stack preference (none | file-based | OpenTelemetry | Prometheus) and whether server mode should expose a read-only status endpoint for headless operations.

3) Data model normalization and storage format; key derivation standard specifics
- Assumptions: Root identity uses a 12-word seed (BIP-39 mnemonic). Key derivation via SLIP-0010 (Ed25519) or BIP32/Ed25519 depending on crypto libs; per-device and per-account keys derived from root.
- Storage: Encrypted at rest using OS keyring where available + user secret; JSON or SQLite for metadata; content blobs chunked and encrypted, addressed by hash (cache directory per user).
- Normalization: App manifests, component refs, and per-user config normalized to avoid duplication; explicit version pins for components.
- Please confirm: Preferred curve (Ed25519 vs secp256k1) and preferred storage format for structured data (SQLite vs JSON files) for MVP.

4) External integration protocols and versioning (Autonomi and alternatives)
- Assumptions: Component distribution via content-addressed URIs (primary: `ant://` for Autonomi). Fallback/read-only support for Arweave/IPFS may be added post-MVP.
- Versioning: Semantic versioning (semver) for components; manifests pin exact versions; compatibility constraints documented in contracts.
- Protocols: OpenRPC (JSON-RPC 2.0) for component and host interactions. Transport over local IPC for stand-alone and mutually authenticated encrypted channel for client–server.
- Please confirm: Any mandatory alternative networks for MVP besides Autonomi, and whether we should support mirror URIs in manifests from day one.


## Resolutions (confirmed by product owner)

1) Tech stack and server mode
- All platforms use Tauri.
- Headless server mode: launch via CLI with `--server`, runs in background launching required backend plugins and exposing their OpenRPC servers.
- Operations: intended to run under systemd (or equivalent) as a service with start/stop/restart.
- No other CLI utilities or additional system services are planned.

2) Observability
- File-based logging (rotating), sufficient for MVP.
- In server mode, expose a read-only status endpoint that the host OS can query (health/version/uptime/basic component status).

3) Identity and encryption
- Use saorsa-core identity and APIs per https://github.com/dirvine/saorsa-core/blob/main/AGENTS_API.md.
- Encryption-at-rest handled by saorsa-seal (part of saorsa-core ecosystem).
- Replace prior seed-phrase approach; identity import/restore flows follow saorsa-core.

4) Hosting/distribution network
- Only the Autonomi network (and its Rust crate) will be used to store and fetch immutable components for MVP.
- The autonomi crate will use the latest version, at the time of this writing it is v0.6.1
- The autonomi crate repository is here: https://github.com/maidsafe/autonomi

## Remaining open questions
- Status endpoint shape/protocol in server mode (HTTP JSON on localhost vs OpenRPC method vs both); minimal fields proposed: {status, version, uptime, components:[{name, status}]}
  - Decision: use only OpenRPC method
- Logging rotation policy and log directory locations per platform
  - Decision: see log rotation policy elsewhere in the documentation 
- Exact Autonomi Rust crate(s) and versions to target; mirror configuration not required for MVP
  - Decision: use a local git submodule of the saorsa-core repository to get the latest updates. Use the latest published crate for autonomi.

## References
- Feature spec: /home/system/osnova/specs/001-use-the-docs/spec.md
- Constitution: /home/system/osnova/.specify/memory/constitution.md

