# Test Strategy

## Overview

Osnova follows Test-Driven Development (TDD) as mandated by the Constitution. All tests must pass before code is merged, with a minimum coverage target of ≥85% across lines and branches.

## Testing Stack

### Rust Backend Testing
- **Formatter**: `cargo fmt` - Enforce consistent code style
- **Linter**: `cargo clippy` - Catch common mistakes and enforce best practices
- **Unit Tests**: `cargo test` - Test individual functions and modules
- **Integration Tests**: `cargo test --test '*'` - Test service interactions (built-in) and component interactions (app-supplied)
- **Coverage**: `cargo tarpaulin` or `cargo llvm-cov` - Measure code coverage

**Pre-commit Requirements**:
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all-features
```

### Frontend Testing (Svelte/TypeScript)
- **Framework**: Vitest - Fast unit test runner for Vite-based projects
- **Component Testing**: @testing-library/svelte - Test Svelte components
- **E2E Testing**: Playwright - Cross-browser end-to-end testing
- **Linter**: ESLint with TypeScript support
- **Formatter**: Prettier

**Pre-commit Requirements**:
```bash
npm run lint
npm run format:check
npm run test
npm run test:e2e
```

**Vitest Configuration** (`vitest.config.ts`):
```typescript
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  test: {
    globals: true,
    environment: 'jsdom',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      lines: 85,
      functions: 85,
      branches: 85,
      statements: 85
    }
  }
});
```

### OpenRPC Contract Testing
- **Tool**: Custom test harness using OpenRPC schema validation
- **Approach**: Generate test stubs from `contracts/openrpc.json`
- **Validation**: Assert request/response schemas match OpenRPC definitions
- **Location**: `tests/contract/` directory

**Contract Test Pattern**:
```rust
#[test]
fn test_apps_list_contract() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "apps.list",
        "params": [],
        "id": 1
    });

    let response = call_openrpc_method(request);

    // Validate response schema matches OpenRPC definition
    assert_valid_openrpc_response(&response, "apps.list");
}
```

### Service/Component Integration Testing
- **Scope**: For built-in services, test in-process service lifecycle and inter-service behavior. For app-supplied components, test full lifecycle (fetch → start → call → stop).
- **Mock Dependencies**: Mock Autonomi network for CI environments
- **Test Components**: Create minimal test components (e.g., echo server) for app-supplied flows
- **Location**: `tests/integration/` directory

**Mock Autonomi Pattern**:
```rust
struct MockAutonomi {
    storage: HashMap<String, Vec<u8>>, // address -> content
}

impl MockAutonomi {
    fn get(&self, address: &str) -> Result<Vec<u8>> {
        self.storage.get(address)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn put(&mut self, content: Vec<u8>) -> String {
        let address = blake3::hash(&content).to_hex();
        self.storage.insert(address.clone(), content);
        format!("ant://{}", address)
    }
}
```

### Mobile Testing (Android/iOS)
**MVP Scope**: Manual testing only - automated mobile testing is deferred post-MVP due to complexity.

**Post-MVP Approach**:
- **Android**: GitHub Actions with Android emulator
- **iOS**: GitHub Actions with iOS simulator (macOS runners)
- **Framework**: Appium or native platform test frameworks
- **CI Integration**: Automated on PR for mobile-specific changes

**Manual Test Checklist (MVP)**:
- [ ] App launches successfully
- [ ] Onboarding wizard completes
- [ ] QR code pairing works
- [ ] Bottom navigation menu functions
- [ ] Theme toggle works
- [ ] App launcher grid displays and reorders
- [ ] Backend operations execute on server (Client-Server mode)

## Test Organization

```
tests/
├── contract/           # OpenRPC contract tests
│   ├── test_apps.rs
│   ├── test_config.rs
│   ├── test_pairing.rs
│   └── test_identity.rs
├── integration/        # Service/Component integration tests
│   ├── test_component_lifecycle.rs
│   ├── test_manifest_loading.rs
│   ├── test_pairing_flow.rs
│   └── test_encryption.rs
└── unit/              # Unit tests (co-located with source)
    └── (in src/ alongside implementation)

app/desktop/tests/     # Frontend tests
├── unit/              # Component unit tests
│   ├── Launcher.test.ts
│   ├── ConfigManager.test.ts
│   └── ThemeToggle.test.ts
└── e2e/               # Playwright E2E tests
    ├── onboarding.spec.ts
    ├── app-launch.spec.ts
    └── pairing.spec.ts
```

## CI/CD Pipeline

### GitHub Actions Workflow (`.github/workflows/ci.yml`)

```yaml
name: CI

on: [push, pull_request]

jobs:
  rust-tests:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test --all-features
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3

  frontend-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm ci
      - run: npm run lint
      - run: npm run format:check
      - run: npm run test -- --coverage
      - run: npx playwright install
      - run: npm run test:e2e
      - uses: codecov/codecov-action@v3

  contract-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --test contract_*
```

## Coverage Requirements

- **Minimum Coverage**: 85% for lines, branches, functions, and statements
- **Exceptions**: Must be documented in `plan.md` with justification
- **Enforcement**: CI fails if coverage drops below threshold
- **Reports**: Generated on every CI run and uploaded to Codecov

## Test-Driven Development Workflow

1. **Write Failing Test**: Create test that defines expected behavior
2. **Run Test**: Verify test fails (Red)
3. **Implement**: Write minimal code to make test pass
4. **Run Test**: Verify test passes (Green)
5. **Refactor**: Improve code while keeping tests green
6. **Repeat**: Continue for next feature/requirement

## Performance Testing

### Acceptance Criteria
- **p95 Launch Time**: ≤ 2 seconds from app launch to first meaningful render
- **p95 Backend Latency**: ≤ 5 seconds (prompt fallback if exceeded)
- **Concurrent Clients**: Server must support ≥ 5 concurrent mobile clients

### Performance Test Approach
```rust
#[test]
fn test_app_launch_performance() {
    let start = Instant::now();

    // Launch app and wait for first render
    let app = launch_app("com.osnova.launcher");
    app.wait_for_first_render();

    let duration = start.elapsed();

    // p95 target: 2 seconds
    assert!(duration.as_secs_f64() < 2.0,
        "Launch took {:?}, exceeds 2s target", duration);
}
```

## Security Testing

### Encryption Validation
- Verify end-to-end encryption in Client-Server mode
- Validate encryption-at-rest using saorsa-seal
- Ensure secrets are never logged
- Test key derivation from 12-word seed phrase

### Security Test Examples
```rust
#[test]
fn test_user_data_encrypted_at_rest() {
    let user_data = b"sensitive user data";
    let encrypted = encrypt_user_data(user_data);

    // Verify data is encrypted
    assert_ne!(encrypted, user_data);

    // Verify raw data not in storage
    let storage_contents = read_storage_file();
    assert!(!storage_contents.contains(user_data));
}

#[test]
fn test_secrets_not_logged() {
    let seed_phrase = "word1 word2 ... word12";

    // Perform operation that uses seed phrase
    import_identity(seed_phrase);

    // Verify seed phrase not in logs
    let logs = read_log_file();
    assert!(!logs.contains(seed_phrase));
}
```

## Test Data Management

### Mock Data
- Store test fixtures in `tests/fixtures/`
- Use consistent test data across test suites
- Generate test manifests, components, and configurations

### Test Isolation
- Each test runs in isolated environment
- Clean up resources after test completion
- Use temporary directories for file operations
- Mock external dependencies (Autonomi, saorsa-core)

## Documentation Testing

### Quickstart Validation
- `quickstart.md` serves as executable documentation
- Each step must be testable and reproducible
- Automated tests validate quickstart instructions

### Example Testing
- All code examples in documentation must be tested
- Use `cargo test --doc` for Rust doc examples
- Maintain example projects that demonstrate usage
