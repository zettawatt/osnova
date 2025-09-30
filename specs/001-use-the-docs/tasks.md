# Tasks - Initialize Osnova feature spec from docs/spec.md

Generated from Phase 1 design docs and clarifications. Follow TDD: write failing tests first.

## Ordering (high level)
1. Contract tests (API)
2. Data model scaffolding
3. Core flows (Launcher, Pairing/Config, Search, Files)
4. Encryption/E2E policy enforcement
5. Performance checks and fallback handling

## Tasks
1. Create OpenRPC contract tests for methods: apps.list, apps.launch
2. Create OpenRPC contract tests for methods: config.setServer
3. Create OpenRPC contract tests for methods: pairing.start
4. Create OpenRPC contract tests for methods: search.query
5. Create OpenRPC contract tests for methods: files.list
6. Create OpenRPC contract tests for method: status.get (server)
7. Create OpenRPC contract tests for UI methods: ui.setTheme, ui.getTheme, nav.setBottomMenu, nav.switchTab
8. Create OpenRPC contract tests for onboarding methods: identity.status, identity.importWithPhrase, identity.create
9. Model definitions: OsnovaApplication, ComponentRef
10. Model definitions: AppConfiguration, AppCache
11. Model definitions: Identity, UserProfile
10. Model definitions: PairingSession, ServerInstance, ClientDevice
11. Implement data persistence interface (encrypted store) - stubs only
12. Implement App Launcher flow against contracts (stub handlers)
13. Implement Pairing initiation flow (stub handling, key exchange placeholder)
14. Implement Config Manager server address update (validation + persistence)
15. Implement Search stub returning typed results (apps, media, images, pages)
16. Implement Files list stub
17. Implement UI methods: theme mode set/get; bottom menu configure; tab switch
18. Enforce E2E policy boundary: ensure user data blobs are never decrypted on server (tests)
19. Add performance guardrails: track launch timing; assert p95 target in tests is configurable
20. Add fallback behavior when p95 backend latency exceeds 5 seconds (prompt signal)
21. Lint/format/static analysis configuration and run gates
22. Implement server status.get handler (read-only), including uptime, version, component statuses
23. Configure file-based logging with rotation (size 10MB, keep 7 files); default locations per platform (Linux: /var/log/osnova or ~/.local/state/osnova/logs; macOS: ~/Library/Logs/Osnova; Windows: %ProgramData%/Osnova/logs)
24. Integrate saorsa-core identity and saorsa-seal encryption into data persistence (MVP stubs; latest crates; saorsa-core via git dependency)
25. Pin and fetch latest Autonomi Rust crate for component storage/fetch
26. Documentation updates: API examples and usage snippets
27. Auditor pass: duplication, naming, dead code removal

## Notes
- Parallelizable tasks: 1-5 (contract tests) and 6-9 (models) can run in parallel.
- Coverage target: >= 85% overall; justify exceptions in plan.md if needed.

