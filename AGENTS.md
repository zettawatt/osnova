# Multi-Agent Development Guide

## Overview

Osnova uses a multi-agent development approach to automate implementation. This document explains the agent architecture, workflow, and how to work with the system.

## Architecture

### Agent Types

**1. Orchestrator Agent**
- **Role**: Main coordinator
- **Location**: Main worktree
- **Responsibilities**:
  - Task distribution
  - Progress tracking
  - Integration management
  - Failure handling

**2. Backend Core Agent**
- **Role**: Rust backend implementation
- **Location**: Backend worktree (`/home/system/osnova_claude-backend/`)
- **Responsibilities**:
  - Data model implementation
  - Service implementation
  - OpenRPC servers
  - Cryptographic operations

**3. Rust Testing Agent**
- **Role**: Quality assurance for Rust code
- **Location**: Backend worktree
- **Responsibilities**:
  - Run tests and measure coverage
  - Code quality review
  - Feedback generation
  - Validation

**4. Frontend Agent** (Phase 2)
- **Role**: Svelte/TypeScript UI implementation
- **Location**: Frontend worktree (`/home/system/osnova_claude-frontend/`)
- **Responsibilities**:
  - Component implementation
  - Responsive design
  - OpenRPC client integration
  - User interactions

**5. E2E Testing Agent** (Phase 2)
- **Role**: End-to-end testing with Playwright MCP
- **Location**: Frontend worktree
- **Responsibilities**:
  - User flow testing
  - Visual validation
  - Cross-platform testing
  - Screenshot capture

**6. Integration Agent** (Phase 3)
- **Role**: Component assembly and builds
- **Location**: Main worktree
- **Responsibilities**:
  - Branch merging
  - Component packaging
  - Build pipeline
  - Release artifacts

## Worktree Structure

```
/home/system/osnova_claude/              # Main worktree (Orchestrator, Integration)
/home/system/osnova_claude-backend/      # Backend worktree (Backend Core, Rust Testing)
/home/system/osnova_claude-frontend/     # Frontend worktree (Frontend, E2E Testing)
```

### Branch Strategy

- **main**: Production-ready code
- **agent/backend-dev**: Backend agent work
- **agent/frontend-dev**: Frontend agent work

## Communication Protocol

### Directory Structure

```
.agents/
├── queue/          # Tasks waiting for execution
├── in-progress/    # Currently executing tasks
├── completed/      # Finished tasks
├── feedback/       # Test results and feedback
└── status/         # Agent status tracking
```

### Task Format

Tasks are JSON files with this structure:

```json
{
  "id": "task-001",
  "type": "backend-implementation",
  "title": "Implement RootIdentity model",
  "description": "Create RootIdentity struct with seed phrase handling",
  "dependencies": ["task-002"],
  "agent": "backend-core",
  "worktree": "backend",
  "context": [
    "docs/02-architecture/data-model.md",
    "docs/07-security/identity.md",
    "CLAUDE.md"
  ],
  "success_criteria": [
    "RootIdentity struct implemented",
    "from_seed() method functional",
    "Tests with ≥85% coverage",
    "Documentation with examples"
  ],
  "status": "queued",
  "created_at": "2025-10-06T15:50:00Z"
}
```

### Status Format

Agents write status to `.agents/status/task-{id}.json`:

```json
{
  "task_id": "task-001",
  "agent": "backend-core",
  "status": "completed",
  "worktree": "backend",
  "branch": "agent/backend-dev",
  "files_changed": ["src/models/identity.rs", "tests/identity_test.rs"],
  "lines_added": 245,
  "tests_added": 8,
  "coverage": 89.2,
  "commit_hash": "abc123def",
  "duration_seconds": 180,
  "completed_at": "2025-10-06T16:30:00Z"
}
```

### Feedback Format

Testing agents write feedback to `.agents/feedback/task-{id}.json`:

```json
{
  "task_id": "task-001",
  "agent": "rust-testing",
  "status": "passed",
  "test_results": {
    "total": 8,
    "passed": 8,
    "failed": 0,
    "coverage": 89.2
  },
  "validated_at": "2025-10-06T16:35:00Z",
  "recommendation": "approve"
}
```

## Workflow

### Task Execution Flow

```
1. Orchestrator creates task in .agents/queue/
2. Orchestrator spawns appropriate agent
3. Agent reads task and context
4. Agent implements solution (TDD)
5. Agent commits to its branch
6. Agent writes status to .agents/status/
7. Testing agent validates
8. Testing agent writes feedback
9. If pass: Orchestrator merges to main
10. If fail: Agent retries with feedback
```

### Feedback Loop

```
Backend Core Agent
    ↓ (implements code)
Rust Testing Agent
    ↓ (runs tests, checks coverage)
    ├─ PASS → approve → Orchestrator merges
    └─ FAIL → feedback → Backend Core Agent retries
```

## Development Phases

### Phase 1: Data Models + osnova-core
- **Agents**: Backend Core, Rust Testing
- **Tasks**: 28 tasks
- **Duration**: 3-5 days (parallel execution)
- **Deliverables**:
  - Complete data models
  - osnova-core service
  - Identity and key management
  - Storage layer
  - Configuration service

### Phase 2: Frontend Implementation
- **Agents**: Frontend, E2E Testing
- **Tasks**: ~20-30 tasks
- **Duration**: 3-4 days
- **Deliverables**:
  - Launcher component
  - Configuration component
  - Deployment component
  - Responsive UI (desktop/mobile)
  - OpenRPC client integration

### Phase 3: Integration
- **Agents**: Integration
- **Tasks**: ~10-15 tasks
- **Duration**: 1-2 days
- **Deliverables**:
  - Component packaging
  - Manifest generation
  - Build pipeline
  - Release artifacts

## Agent Specifications

Detailed agent specifications are in `.claude/agents/`:
- `orchestrator.md` - Orchestrator agent
- `backend-core.md` - Backend implementation
- `rust-testing.md` - Rust testing and QA
- `frontend.md` - Frontend implementation
- `e2e-testing.md` - E2E testing with Playwright
- `integration.md` - Integration and builds

## Running Agents

### Manual Agent Execution

To spawn an agent manually for a specific task:

```bash
# Create task file
cat > .agents/queue/task-test.json << EOF
{
  "id": "task-test",
  "type": "backend-implementation",
  "title": "Test Task",
  "description": "Implement test feature",
  "agent": "backend-core",
  "worktree": "backend",
  "context": ["CLAUDE.md"],
  "success_criteria": ["Tests pass"]
}
EOF

# Agent will pick up task from queue
```

### Monitoring Progress

Check agent status:

```bash
# List all task statuses
ls -la .agents/status/

# View specific task status
cat .agents/status/task-001.json | jq

# Check feedback
cat .agents/feedback/task-001.json | jq
```

## Best Practices

### For Orchestrator
- Maximize parallel execution where possible
- Keep agent contexts minimal
- Handle failures gracefully
- Maintain clean git history

### For Implementation Agents
- Always follow TDD (tests before code)
- Write comprehensive documentation
- Follow DRY principle
- Handle all error cases
- Commit frequently with clear messages

### For Testing Agents
- Provide actionable feedback
- Include specific error messages
- Reference exact locations (file:line)
- Suggest concrete fixes
- Prioritize issues (critical/important/minor)

## Troubleshooting

### Agent Not Starting
- Check task file format (valid JSON)
- Verify worktree exists
- Check agent specification file

### Tests Failing
- Review feedback in `.agents/feedback/`
- Check test output in worktree
- Run tests locally for debugging

### Merge Conflicts
- Orchestrator will attempt automatic resolution
- Complex conflicts escalate to manual review
- Check git status in worktrees

### High Agent Invocation Count
- Review task granularity (too fine?)
- Check retry logic (excessive retries?)
- Optimize agent context (too much documentation?)

## Metrics and Monitoring

### Success Metrics
- Task completion rate
- Test pass rate
- Code coverage percentage
- Agent invocation efficiency
- Time to completion

### Phase 1 Targets
- 28 tasks completed
- ≥85% code coverage
- ~40-50 agent invocations
- 3-5 days duration
- All tests passing

## Cost Optimization

### Reduce Agent Invocations
- Batch related tasks
- Reuse agent outputs
- Optimize context size
- Implement smart caching

### Improve Efficiency
- Minimize retry loops
- Better error messages
- Clearer specifications
- Strategic manual checkpoints

## Integration with Claude Code

This multi-agent system is designed to work with Claude Code:
- Agents use Claude Code's Task tool
- Specifications follow Claude Code conventions
- TDD and documentation requirements align
- Git workflow optimized for Claude Code

## References

- [CLAUDE.md](./CLAUDE.md) - Development guidelines
- [docs/](./docs/) - Complete specifications
- [.mcp.md](./.mcp.md) - MCP server configuration
- [.claude/agents/](./.claude/agents/) - Agent specifications

---

**Status**: Phase 1 Ready
**Last Updated**: 2025-10-06
