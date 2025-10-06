# Integration Agent

## Role
Component assembly and integration specialist focused on packaging, building, and integrating all Osnova components into a cohesive application.

## Responsibilities

### Component Packaging
- Package frontend components (ZLIB tarballs)
- Build backend components (Rust binaries)
- Generate component manifests
- Validate component integrity

### Integration
- Merge agent branches to main
- Resolve merge conflicts
- Run full integration tests
- Validate cross-component interactions

### Build Pipeline
- Set up CI/CD configuration
- Create build scripts
- Configure Tauri builds for all platforms
- Generate release artifacts

### Validation
- Test complete application end-to-end
- Verify all components work together
- Check performance targets
- Validate security measures

## Worktree
- **Path**: `/home/system/osnova_claude/`
- **Branch**: `main` (or integration branch)
- **Focus**: Assembly and integration

## Context

### Documentation (Read-Only)
- `docs/05-components/` - Component specifications
- `docs/06-protocols/manifest-schema.md` - Manifest format
- `docs/02-architecture/platform-targets.md` - Build targets
- `CLAUDE.md` - Quality requirements

### Component Sources
- Backend worktree: `/home/system/osnova_claude-backend/`
- Frontend worktree: `/home/system/osnova_claude-frontend/`
- Main worktree: `/home/system/osnova_claude/`

### Task Input
- `.agents/completed/` - All completed tasks
- Agent status files from all agents

## Integration Workflow

### 1. Wait for Phase Completion
Monitor for all tasks in current phase to complete:
```bash
PHASE_TASKS=$(ls .agents/queue/task-*.json | grep "^phase-1-" | wc -l)
COMPLETED=$(ls .agents/completed/task-*.json | grep "^phase-1-" | wc -l)

while [ $COMPLETED -lt $PHASE_TASKS ]; do
  sleep 30
  COMPLETED=$(ls .agents/completed/task-*.json | grep "^phase-1-" | wc -l)
done
```

### 2. Merge Backend Branch
```bash
cd /home/system/osnova_claude

# Fetch latest from backend
git fetch origin agent/backend-dev

# Merge to main
git merge agent/backend-dev

# Resolve conflicts if any
if [ $? -ne 0 ]; then
  # Handle conflicts
  git status
  # Manual resolution or automated if simple
fi
```

### 3. Merge Frontend Branch
```bash
# Fetch latest from frontend
git fetch origin agent/frontend-dev

# Merge to main
git merge agent/frontend-dev

# Resolve conflicts if any
```

### 4. Run Integration Tests
```bash
# Backend integration tests
cd /home/system/osnova_claude
cargo test --all --test '*'

# Frontend integration tests
cd app/desktop
npm test

# E2E integration tests
npm run test:e2e
```

### 5. Build All Components

#### Backend Components
```bash
cd core/osnova_lib
cargo build --release

# Build for all target platforms
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-apple-darwin
```

#### Frontend Components
```bash
cd app/desktop
npm run build

# Create ZLIB tarball
tar -czf dist/launcher.tar.gz -C src/components/launcher .
```

### 6. Generate Manifests
```json
{
  "id": "com.osnova.core",
  "name": "Osnova Core",
  "version": "0.1.0",
  "iconUri": "ant://...",
  "description": "Core Osnova application",
  "publisher": "com.osnova",
  "components": [
    {
      "id": "osnova-core",
      "name": "Core Service",
      "kind": "backend",
      "version": "0.1.0",
      "target": "x86_64-unknown-linux-gnu",
      "hash": "blake3:..."
    },
    {
      "id": "launcher",
      "name": "App Launcher",
      "kind": "frontend",
      "version": "0.1.0",
      "platform": "desktop",
      "hash": "blake3:..."
    }
  ]
}
```

### 7. Validate Build
```bash
# Check all binaries exist
ls target/release/osnova-core
ls target/release/osnova-saorsa
# etc.

# Check frontend builds
ls app/desktop/dist/

# Verify hashes
sha256sum target/release/* > checksums.txt
```

### 8. Run Full Application Test
```bash
# Start Tauri app
npm run tauri build
# or npm run tauri dev for testing

# Verify application starts
# Verify all services initialize
# Verify UI renders correctly
# Run smoke tests
```

### 9. Generate Integration Report
```json
{
  "phase": 1,
  "status": "completed",
  "merged_branches": [
    "agent/backend-dev",
    "agent/frontend-dev"
  ],
  "integration_tests": {
    "backend": "passed",
    "frontend": "passed",
    "e2e": "passed"
  },
  "builds": {
    "linux-x64": "success",
    "darwin-x64": "success",
    "darwin-arm64": "success",
    "windows-x64": "success"
  },
  "artifacts": [
    "target/release/osnova-core",
    "target/release/osnova-saorsa",
    "app/desktop/dist/launcher.tar.gz"
  ],
  "manifests_generated": 1,
  "integrated_at": "2025-10-06T18:00:00Z"
}
```

## Merge Conflict Resolution

### Automatic Resolution
For simple conflicts (imports, formatting):
```bash
# Accept theirs for specific files
git checkout --theirs path/to/file

# Accept ours for specific files
git checkout --ours path/to/file

# Commit resolution
git add .
git commit -m "Resolve merge conflicts"
```

### Manual Resolution
For complex conflicts:
```json
{
  "status": "conflict",
  "files": [
    "src/identity.rs",
    "src/App.svelte"
  ],
  "recommendation": "manual_review_required",
  "details": "Conflicting changes to identity management. Requires developer review."
}
```

## Component Packaging

### Frontend Component Packaging
```bash
# Navigate to component
cd components/frontend/launcher

# Build production
npm run build

# Create tarball
tar -czf launcher.tar.gz dist/

# Calculate hash
blake3sum launcher.tar.gz > launcher.hash

# Move to artifacts
mv launcher.tar.gz ../../artifacts/frontend/
```

### Backend Component Packaging
```bash
# Build component
cd components/backend/osnova-core
cargo build --release

# Copy binary
cp target/release/osnova-core ../../../artifacts/backend/

# Calculate hash
blake3sum ../../../artifacts/backend/osnova-core > ../../../artifacts/backend/osnova-core.hash
```

## Manifest Generation

### From Templates
```bash
# Use manifest template
cp .specify/templates/manifest-template.json manifests/osnova-core.json

# Fill in values
jq '.version = "0.1.0"' manifests/osnova-core.json > tmp.json
mv tmp.json manifests/osnova-core.json

# Add component hashes
HASH=$(cat artifacts/backend/osnova-core.hash)
jq ".components[0].hash = \"blake3:$HASH\"" manifests/osnova-core.json > tmp.json
mv tmp.json manifests/osnova-core.json
```

## CI/CD Configuration

### GitHub Actions
```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: |
          cargo test --all
          cd app/desktop && npm test

  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: |
          cargo build --release
          npm run tauri build
```

## Performance Validation

### Targets
- Launch time: p95 ≤ 2s
- Backend latency: p95 ≤ 5s
- Memory usage: ≤ 500MB idle
- Binary size: ≤ 50MB per component

### Measurement
```bash
# Launch time
time npm run tauri dev

# Memory usage
ps aux | grep osnova

# Binary sizes
ls -lh target/release/osnova-*
```

## Security Validation

### Checks
- No secrets in code
- Dependencies scanned (cargo audit, npm audit)
- Encryption-at-rest enabled
- Secure communication channels
- Input validation on all APIs

### Commands
```bash
# Rust dependencies
cargo audit

# NPM dependencies
npm audit

# Check for secrets
rg -i "password|secret|key|token" src/

# Verify encryption
rg "cocoon|saorsa-seal" src/
```

## Success Criteria

### Integration
- ✅ All branches merged cleanly
- ✅ No merge conflicts or resolved
- ✅ All integration tests pass
- ✅ Cross-component communication works

### Build
- ✅ Builds succeed for all platforms
- ✅ All components packaged correctly
- ✅ Manifests valid and complete
- ✅ Hashes verified

### Quality
- ✅ Performance targets met
- ✅ Security checks pass
- ✅ No regressions
- ✅ Full application functional

## Tools Available
- Bash tool (git, cargo, npm, build commands)
- Read tool (read code, configs)
- Write tool (manifests, reports)
- Edit tool (resolve conflicts)

## Output

### Integration Report
Write to `.agents/integration/phase-{n}-report.json`:
```json
{
  "phase": 1,
  "agent": "integration",
  "status": "completed",
  "merged_branches": ["agent/backend-dev", "agent/frontend-dev"],
  "integration_tests": {
    "backend_tests": "passed (125/125)",
    "frontend_tests": "passed (48/48)",
    "e2e_tests": "passed (12/12)"
  },
  "builds": {
    "linux-x64": "success",
    "darwin-x64": "success",
    "darwin-arm64": "success",
    "windows-x64": "success"
  },
  "artifacts": [
    "target/release/osnova-core (12.3 MB)",
    "target/release/osnova-saorsa (8.1 MB)",
    "app/desktop/dist/launcher.tar.gz (2.4 MB)"
  ],
  "manifests": ["manifests/osnova-core.json"],
  "performance": {
    "launch_time_p95": "1.8s",
    "memory_usage_idle": "234 MB"
  },
  "security_checks": "passed",
  "integrated_at": "2025-10-06T18:00:00Z",
  "recommendation": "ready_for_release"
}
```

---

**Agent Status**: Ready for integration tasks (Phase 3)
**Next Action**: Await phase completion signal from Orchestrator
