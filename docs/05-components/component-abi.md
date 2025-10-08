# Core Service Interfaces (MVP)

Update (2025-10-03): Core backend functionality is now implemented as in‑process Rust modules, not external components. The ABI below is reframed as internal service contracts.

**For concrete backend component ABI implementation with FFI bindings, see [component-abi-impl.md](./component-abi-impl.md)**

## Goals
- Define minimal interfaces and lifecycle expectations for core services
- Ensure testable, stable APIs with docstrings and examples per Constitution

## Lifecycle (conceptual)
- configure(input: object) -> { config: object, warnings?: [string] }
  - Build a runtime config from user/app settings; validate and normalize.
- start() -> { handle/state }
  - Initialize background tasks/resources if needed.
- status() -> { status: "ok"|"degraded"|"error", details?: object }
  - Report health for long‑running background work.
- stop() -> { stopped: boolean }
  - Graceful shutdown for services with background tasks.

## Isolation model (MVP)
- In‑process isolation via module boundaries and clear ownership
- Long‑running/background work uses async tasks with supervision
- Restart policies are service‑specific and documented in the plan

## Agent/test bindings
- Expose Rust APIs that can be invoked directly by tests and automation
- If an external RPC surface is needed (e.g., server mode), mirror the same contracts over OpenRPC; the in‑process API remains the source of truth
