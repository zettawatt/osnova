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

## Open Questions (to address during planning)
- Tech stack selection per platform (client, server, mobile)
- Observability signals (logs/metrics/traces) and minimal dashboards
- Data model normalization and storage format details; key derivation standard specifics
- External integration protocols and versioning (Autonomi and alternatives)

## References
- Feature spec: /home/system/osnova/specs/001-use-the-docs/spec.md
- Constitution: /home/system/osnova/.specify/memory/constitution.md

