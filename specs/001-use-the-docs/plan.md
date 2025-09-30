
# Implementation Plan: Initialize Osnova feature spec from docs/spec.md

**Branch**: `001-use-the-docs` | **Date**: 2025-09-30 | **Spec**: /home/system/osnova/specs/001-use-the-docs/spec.md
**Input**: /home/system/osnova/specs/001-use-the-docs/spec.md

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Osnova is a Tauri 2.x desktop/mobile application with a Svelte (TypeScript/HTML/CSS) UI and a Rust backend library. It loads application manifests and dynamically runs isolated frontend/backends as components (plugins). Frontends are static web apps (ZLIB-compressed tarballs) rendered in Tauri's WebView; backends are precompiled Rust binaries loaded via a Tauri plugin ABI (configure/start/stop/status). Components communicate over OpenRPC. Downloaded components are cached locally; production manifests reference Autonomi URIs (ant://). Modes: Stand-alone (all local) and Client-Server (remote backends over encrypted channels).

Clarified decisions: End-to-end encryption of user data in Client-Server mode; support >= 5 concurrent clients per server (MVP); p95 launch->first meaningful render <= 2s; prompt fallback if p95 backend latency > 5s; MVP best-effort availability (no formal SLO).

Arguments considered: leverage @docs/plan.md and templates to setup the implementation plan.

## Technical Context
**Language/Version**: Rust (stable), TypeScript (Svelte), Tauri 2.x
**Primary Dependencies**: Tauri 2.x, Svelte, OpenRPC, Zlib, Autonomi network
**Storage**: Encrypted user-scoped data store; local cache for downloaded components; content-addressed networks (primary: Autonomi) for component versions.
**Testing**: cargo test + Vitest + Playwright (TDD mandated by Constitution)
**Target Platform**: Windows, macOS, Linux, Android, iOS
**Project Type**: desktop+mobile app with backend library; componentized (frontend/backend)
**Performance Goals**: p95 launch <= 2s; responsive mobile client during remote backends
**Constraints**: Fallback if p95 backend latency > 5 s; E2E user-data encryption in Client-Server mode; plugin ABI (configure/start/stop/status); immutable component versions
**Scale/Scope**: MVP server supports >= 5 concurrent mobile clients; core apps: Launcher, Wallet & Fiat Bridge, Search, File Manager, Config Manager


## UI Baseline (from docs/spec.md)
- Desktop UX: theme toggle (light/dark) in top-right; auto-sync with OS theme
- Mobile UX: bottom 5-icon menu configurable to select an Osnova app tab
- Responsive Svelte UI for desktop and mobile contexts

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Test-First, Continuous Quality: TDD, CI gates, coverage ≥ 85% — Planned: YES (tests precede implementation; coverage target enforced in tasks).
- Documentation & Readability: API docs/examples, docstrings, lint/format — Planned: YES.
- Non-Duplication (OAOO/DRY): No copy-paste > 3 lines — Planned: YES.
- Segregated Roles (Author/Tester/Auditor): Distinct contexts — Planned: YES (documented in tasks and reviews).
- Simplicity & Extensibility: Small composable modules; stable APIs — Planned: YES.

Initial Constitution Check: PASS (no violations). Post-Design Check to be re-evaluated after Phase 1 artifacts.

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->
```
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Documentation-first. Current repo focuses on specifications and templates.
- Active feature docs: /home/system/osnova/specs/001-use-the-docs
- Templates & scripts: /home/system/osnova/.specify/templates, /home/system/osnova/.specify/scripts
- Source layout: To be established when implementation starts; default to Single project unless platform constraints require splitting into client/server/mobile.

Proposed initial source layout (from @docs/plan.md):
```
app/
  desktop/                  # Tauri 2.x app shell (Svelte frontend)
    src/                    # TypeScript/Svelte components/pages/services
    public/
    tauri/
  mobile/                   # Tauri mobile targets (iOS/Android)
    ios/
    android/
core/
  osnova_lib/               # Rust library: core business logic (public API)
components/
  frontend/                 # Static web apps (Svelte/TS), packaged as ZLIB tarballs
    <component-name>/
  backend/                  # Rust plugin binaries (one Cargo project per component)
    <component-name>/
contracts/
  openrpc/                  # OpenRPC service definitions for component APIs
  openapi/                  # Optional host control surface (REST) if needed
specs/
  001-use-the-docs/         # Feature documentation (this plan/spec/research/tasks)
```


## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action -> method
   - Use OpenRPC specification
   - Output OpenRPC schema to `/contracts/openrpc.json`

3. **Generate contract tests** from contracts:
   - Include UI scenarios: theme toggle behavior (desktop, auto-sync with OS); mobile bottom 5-icon menu navigation

   - One test file per method
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh auggie`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P]
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [ ] Complexity deviations documented

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*
