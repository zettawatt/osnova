# Osnova Development Guide for Claude Code

## Project Overview

Osnova is a cross-platform distributed application framework built with Tauri 2.0, Svelte, and Rust. It provides a browser-like experience for distributed applications, enabling users to run modular frontend and backend components in both stand-alone and client-server modes.

**Tech Stack**: Tauri 2.0 • Svelte • TypeScript • Rust • SQLite • Autonomi Network • OpenRPC

**Target Platforms**: Windows • macOS • Linux • Android • iOS

**Key Features**: Component-based architecture • Encrypted data at rest • Identity management • Client-server pairing • Responsive UI

## Development Philosophy

### Specification-Driven Development

Before writing any code, specifications must be complete and unambiguous. The workflow is:

1. **Specification** → Complete specs in `docs/`
2. **Planning** → Implementation plans from specs
3. **Test-Driven Development** → Tests before implementation
4. **Implementation** → Code to pass tests
5. **Validation** → Verify against specifications

### Core Principles

**DRY (Don't Repeat Yourself)**
- No code duplication greater than 3 lines
- Extract shared logic into reusable functions
- Single source of truth for each concept

**Test-Driven Development (TDD)**
- Write tests BEFORE implementation
- Tests must fail initially (red)
- Implement minimal code to pass (green)
- Refactor while keeping tests passing
- Minimum coverage: **≥85%**

**Documentation Requirements**
- Every public function/struct MUST have docstrings
- Include purpose, parameters, returns, errors
- Provide usage examples
- Document non-obvious implementation details

### Multi-Agent Workflow

Osnova uses a multi-agent development system to automate implementation:

- **Orchestrator**: Coordinates agents, manages tasks, handles integration
- **Backend Core**: Implements Rust services and data models
- **Rust Testing**: Validates code quality and provides feedback
- **Frontend**: Implements Svelte UI components (Phase 2)
- **E2E Testing**: Validates user flows with Playwright MCP (Phase 2)
- **Integration**: Packages components and manages builds (Phase 3)

Agents work in parallel across separate git worktrees, communicating via `.agents/` directories.

**See [AGENTS.md](./AGENTS.md) for complete multi-agent workflow documentation.**

## Code Quality Standards

### Universal Principles

**Test-Driven Development (TDD)**
- Tests MUST be written before implementation
- Tests must fail initially (red), then pass (green), then refactor
- Minimum coverage: **≥85%** for all modules
- Test types: Unit, integration, contract, E2E

**Don't Repeat Yourself (DRY)**
- No code duplication greater than 3 lines
- Extract shared logic into reusable functions
- Single source of truth for each concept

**Documentation**
- Every public function/struct MUST have documentation
- Include: purpose, parameters, returns, errors, examples
- Document non-obvious implementation details
- Keep documentation up to date with code changes

**Error Handling**
- Handle all error cases gracefully
- Provide meaningful error messages
- No silent failures
- Use appropriate error types for the language

**Code Style**
- Follow language-specific formatters and linters
- Use meaningful variable and function names
- Keep functions small and focused
- Consistent naming conventions throughout

### Language-Specific Details

**Rust patterns and conventions**: See `.claude/agents/backend-core.md`
- Error handling with Result/Option
- Code style (rustfmt, clippy)
- Documentation format
- Testing patterns

**TypeScript/Svelte patterns**: See `.claude/agents/frontend.md`
- Error handling with try-catch
- Code style (ESLint, Prettier)
- Component documentation
- Testing patterns

## Documentation Structure

All specifications are in `docs/` organized as chapters:

| Chapter | Contents | Key Files |
|---------|----------|-----------|
| 01-introduction | Project overview, user experience | overview.md, user-experience.md |
| 02-architecture | System design, data models | components.md, data-model.md |
| 03-core-services | Built-in Rust services | osnova-core.md, osnova-saorsa.md |
| 04-core-screens | Built-in GUI modules | launcher.md, configuration.md |
| 05-components | Component system | component-abi.md, frontend-components.md |
| 06-protocols | OpenRPC contracts | openrpc-conventions.md, manifest-schema.md |
| 07-security | Identity, encryption, keys | identity.md, keys.md, encryption.md |
| 08-networking | Pairing, server ops | pairing.md, server-ops.md |
| 09-ui-ux | Interface design | desktop-ui.md, mobile-ui.md |
| 10-development | Testing, tasks, plans | testing.md, plan.md |
| 11-apps | Application specs | app-launcher-app.md |

**See [docs/README.md](./docs/README.md) for complete documentation index.**

## For AI Agents

### Where to Find Information

- **Project specifications**: `docs/` directory (organized by chapter)
- **Agent instructions**: `.claude/agents/` (agent-specific workflows)
- **Task queue**: `.agents/queue/` (tasks to execute)
- **OpenRPC contracts**: `docs/06-protocols/openrpc-contracts.md` and `docs/03-core-services/osnova-core.md`
- **Dependencies**: `Cargo.toml` (Rust) and `package.json` (TypeScript)
- **Data models**: `docs/02-architecture/data-model.md`
- **Testing requirements**: `docs/10-development/testing.md`
- **Security specs**: `docs/07-security/` (identity, keys, encryption)

### Key Dependencies

**Rust**: autonomi v0.6.1 • saorsa-core (main) • cocoon v0.4.3 • serde • tokio • anyhow • thiserror

**TypeScript**: Svelte 4.x • @tauri-apps/api • Vitest • Playwright

**See `Cargo.toml` and `package.json` for complete dependency lists.**

### Task Execution Pattern

1. Read task from `.agents/queue/task-{id}.json`
2. Review context documents listed in task
3. Write failing tests (TDD)
4. Implement to pass tests
5. Run quality checks (coverage, linting, formatting)
6. Commit with descriptive message
7. Write status to `.agents/status/task-{id}.json`

## Implementation Checklist

When implementing a feature:

- [ ] Read complete specification from `docs/`
- [ ] Understand data model and dependencies
- [ ] Review OpenRPC contracts (if applicable)
- [ ] Write contract tests (must fail initially)
- [ ] Write unit tests (must fail initially)
- [ ] Implement minimal code to pass tests
- [ ] Refactor while keeping tests green
- [ ] Verify coverage ≥85%
- [ ] Fix all clippy/lint warnings
- [ ] Format code (rustfmt/prettier)
- [ ] Add docstrings with examples to all public items
- [ ] No code duplication >3 lines
- [ ] Run full test suite
- [ ] Manual testing if UI changes
- [ ] Update documentation if needed

## Quick Reference

- **Architecture**: `docs/02-architecture/`
- **Core Services**: `docs/03-core-services/` (OpenRPC methods, service specs)
- **Security**: `docs/07-security/` (identity, keys, encryption)
- **Testing**: `docs/10-development/testing.md`
- **Multi-Agent**: `AGENTS.md` (workflow, agent roles, communication)
- **OpenRPC Conventions**: `docs/06-protocols/openrpc-conventions.md`
- **Manifest Schema**: `docs/06-protocols/manifest-schema.md`
- **Component ABI**: `docs/05-components/component-abi.md`

## Getting Help

1. **Search specifications**: `docs/` contains detailed requirements
2. **Check agent specs**: `.claude/agents/` for agent-specific workflows
3. **Review existing code**: Look for similar patterns in codebase
4. **Read contracts**: OpenRPC contracts define all interfaces
5. **Check AGENTS.md**: Multi-agent workflow and communication

---

**Development Principles**: Specification-driven • Test-driven • DRY • Documentation with examples

**Last Updated**: 2025-10-06
