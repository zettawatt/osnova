# Backend Component ABI and Isolation (MVP)

## Goals
Define a minimal lifecycle ABI and isolation expectations for backend components, plus an agent-compatible client binding.

## Lifecycle ABI (conceptual)
- component_configure(input: object) -> { config: object, warnings?: [string] }
  - Build a runtime config from user/app settings; validate and normalize.
- component_start(config: object) -> { endpoint: string, pid?: number }
  - Start the component server; return endpoint for client connections.
- component_status() -> { status: "ok"|"degraded"|"error", details?: object }
  - Report health for server mode and stand-alone.
- component_stop() -> { stopped: boolean }
  - Graceful shutdown; allow restart.

## Isolation model (MVP)
- Process boundary isolation per backend component instance.
- Multi-client handling: components MUST support multiple concurrent clients.
- Resource limits (CPU/mem) and restart policies are implementation details documented in the plan.

## Agent-compatible client (MPC client)
- Requirement: Each backend component MUST expose a client binding to its public API for direct automated invocation by AI agents/tools.
- Binding: Connects to the same endpoint and methods as the regular server (e.g., OpenRPC), with identical schemas and auth.
- Parity: All public methods must be invocable via the MPC client; tests MUST demonstrate parity (success/error cases).
- Auth: Reuse existing auth and permissions; no additional capabilities beyond the public API.

