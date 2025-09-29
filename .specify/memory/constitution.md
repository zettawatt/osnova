<!--
Sync Impact Report
- Version change: n/a → 1.0.0
- Modified principles: none (initial adoption)
- Added sections:
  • Core Principles (5)
  • Additional Constraints
  • Development Workflow & Quality Gates
  • Governance
- Templates reviewed:
  • .specify/templates/plan-template.md — ✅ aligned (Constitution Check gates compatible)
  • .specify/templates/spec-template.md — ✅ aligned (testable requirements emphasis)
  • .specify/templates/tasks-template.md — ✅ aligned (duplication removal present)
- Follow-ups: none
-->

# osnova Constitution

## Core Principles

### Test-First, Continuous Quality (NON-NEGOTIABLE)
- TDD is mandatory: write a failing test before implementation; then Red–Green–Refactor.
- CI gates MUST pass in local and CI environments; no skipped tests in default run.
- Test coverage MUST be ≥ 85% across lines/branches; justify any targeted exceptions in plan.md.
- No production code without a corresponding test that proves behavior.

### Documentation and Readability
- Public APIs MUST include reference docs and at least one usage example.
- Modules, classes, and functions MUST have docstrings explaining intent and contracts.
- Formatting and linting MUST pass; code is written for humans first (clear names, small units).
- No dead code or unused dependencies; remove or justify explicitly in plan.md.

### Non‑Duplication: Once and Only Once (OAOO) and DRY
- Maintain a single source of truth for data, logic, and configuration.
- Copy‑paste blocks > 3 logical lines are prohibited; factor common logic into shared abstractions.
- Duplication gate MUST pass: identical or ≥ 85% similar code segments are refactored or
  explicitly justified in plan.md with a migration note.
- Generated artifacts are tracked separately from source and not duplicated across modules.

### Segregated Roles via Distinct AI/Person Contexts
- Author, Tester, and Auditor MUST be different people or AI contexts with independent memory.
- Tester context writes/extends tests and verifies gates; cannot share chat/session state with Author.
- Auditor context enforces cleanliness: naming, structure, comments, dead code, duplication.
- A release requires explicit sign‑off from Tester and Auditor roles.

### Simplicity and Extensibility
- Prefer small, composable modules with clear interfaces; avoid premature optimization (YAGNI).
- Public APIs are stable; breaking changes require MAJOR version bump and migration notes.
- Behavior and configuration are explicit and documented; implicit magic is avoided.

## Additional Constraints
- Single‑developer workflow: PRs and multi‑review processes are optional; self‑review is required.
- Trunk‑based or small feature branches; commit atomically with clear messages.
- Required quality gates per change: tests green, ≥ 85% coverage, docs updated,
  format/lint/static analysis clean, duplication gate passes.
- Tooling is agnostic; any stack is acceptable that can enforce these gates.

## Development Workflow, Review Process, Quality Gates
1) Authoring (Context A – Author)
   - Write minimal implementation guided by tests; keep modules small and readable.
   - Update public API docs and examples alongside code changes.
2) Testing (Context B – Tester)
   - Write/extend tests first; ensure they fail before implementation; verify coverage ≥ 85%.
   - Validate integration paths and edge cases; no flakiness.
3) Audit (Context C – Auditor)
   - Run formatter, linter, static analysis, and duplication checks.
   - Enforce OAOO/DRY; remove dead code; ensure naming and structure are clear.

Must‑pass gates (failure blocks release):
- Tests: all green; coverage ≥ 85%.
- Docs: public APIs and examples updated for any user‑visible change.
- Cleanliness: formatter/linter/static analysis pass; no dead code.
- Duplication: no prohibited duplicates; similar segments consolidated or justified.

## Governance
- Supremacy: This Constitution overrides other guidelines when conflicting.
- Amendment procedure:
  - Propose a change via a short design note including rationale, impact, and migration plan.
  - For a single‑developer project, self‑ratify with Auditor sign‑off recorded in this file.
  - Record version bump and dates upon adoption.
- Constitution versioning (semver):
  - MAJOR: Remove/redefine principles or incompatible governance changes.
  - MINOR: Add a new principle/section or materially expand guidance.
  - PATCH: Clarifications/wording fixes without changing requirements.
- Compliance review cadence: at each release and at least quarterly; Auditor leads review.
- Runtime guidance: plan/spec/tasks templates must remain aligned with these principles.

**Version**: 1.0.0 | **Ratified**: 2025-09-29 | **Last Amended**: 2025-09-29
