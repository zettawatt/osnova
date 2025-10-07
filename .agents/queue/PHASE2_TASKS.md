# Phase 2: Frontend Implementation + Integration

## Overview
Phase 2 focuses on implementing the Tauri 2.0 desktop/mobile application shell with Svelte frontend, integrating the osnova-core services from Phase 1, and implementing the three core built-in screens: Launcher, Configuration, and Deployment.

## Prerequisites
- ✅ Phase 1 complete (osnova-core library with all services)
- ✅ 159 tests passing
- ✅ Data models, storage layer, and services operational

## Execution Strategy
- **Agents**: Frontend Agent + E2E Testing Agent (Playwright MCP) + Backend Core Agent (for integration)
- **Worktrees**:
  - `/home/system/osnova_claude-frontend/` (Tauri + Svelte)
  - `/home/system/osnova_claude-backend/` (osnova-core integration)
- **Estimated Tasks**: 35 tasks
- **Estimated Duration**: 4-6 days with parallel execution
- **Estimated Agent Invocations**: ~55-70

## Task List

### Group 1: Tauri Project Setup (Sequential)
**Dependencies**: Phase 1 complete

#### Task 029: Initialize Tauri 2.0 Project
- **Type**: frontend-setup
- **Agent**: frontend
- **Priority**: P0
- **Description**: Create Tauri 2.0 desktop + mobile project with Svelte
- **Deliverables**:
  - `app/desktop/` directory structure
  - `app/mobile/` directory structure for iOS/Android
  - `package.json` with Svelte + TypeScript dependencies
  - Tauri configuration files
  - Initial Svelte app scaffold
- **Dependencies**: []
- **Context**: `CLAUDE.md`, `docs/10-development/plan.md`, `docs/09-ui-ux/desktop-ui.md`

#### Task 030: Configure Tauri-Rust Integration
- **Type**: frontend-setup
- **Agent**: frontend + backend-core
- **Priority**: P0
- **Description**: Wire Tauri commands to osnova-core library
- **Deliverables**:
  - Tauri command handlers in Rust
  - TypeScript bindings for Tauri commands
  - Error handling bridge
  - Initial smoke test
- **Dependencies**: [029]
- **Context**: `docs/02-architecture/components.md`

#### Task 031: Setup Theme System
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P0
- **Description**: Implement light/dark theme system with OS sync
- **Deliverables**:
  - Theme store (Svelte writable store)
  - CSS variables for light/dark modes
  - OS theme detection
  - Theme toggle component (desktop: top-right corner)
  - Persistence via `ui.setTheme` / `ui.getTheme`
  - Tests
- **Dependencies**: [030]
- **Context**: `docs/09-ui-ux/desktop-ui.md`, `docs/09-ui-ux/mobile-ui.md`

---

### Group 2: Core UI Components (Parallel After Group 1)
**Dependencies**: Tasks 029-031

#### Task 032: Implement Layout Container Component
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P1
- **Description**: Create responsive layout container for desktop/mobile
- **Deliverables**:
  - `src/lib/components/LayoutContainer.svelte`
  - Responsive grid system
  - Mobile/desktop detection
  - Tests (Vitest component tests)
- **Dependencies**: [031]
- **Context**: `docs/09-ui-ux/desktop-ui.md`, `docs/09-ui-ux/mobile-ui.md`

#### Task 033: Implement Bottom Navigation (Mobile)
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P1
- **Description**: Create configurable 5-icon bottom menu for mobile
- **Deliverables**:
  - `src/lib/components/BottomNav.svelte`
  - Tab switching logic
  - Active tab indicator
  - Persistence via `nav.setBottomMenu`
  - Tests
- **Dependencies**: [031]
- **Context**: `docs/09-ui-ux/mobile-ui.md`, `docs/03-core-services/osnova-core.md`

#### Task 034: Implement Icon Component
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P1
- **Description**: Create reusable app icon component with fallback
- **Deliverables**:
  - `src/lib/components/AppIcon.svelte`
  - Image loading with fallback
  - Loading spinner state
  - Error state handling
  - Icon scaling logic
  - Tests
- **Dependencies**: [031]
- **Context**: `docs/04-core-screens/launcher.md`

---

### Group 3: Launcher Screen (Sequential After Group 2)
**Dependencies**: Tasks 032-034

#### Task 035: Implement Launcher Grid (Desktop)
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Create continuously scrolling grid for desktop
- **Deliverables**:
  - `src/lib/screens/Launcher.svelte` (desktop variant)
  - Grid layout with CSS Grid
  - Scrolling behavior
  - Icon positioning
  - Tests
- **Dependencies**: [032, 034]
- **Context**: `docs/04-core-screens/launcher.md`

#### Task 036: Implement Launcher Grid (Mobile)
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Create paginated grid with swipe navigation for mobile
- **Deliverables**:
  - `src/lib/screens/LauncherMobile.svelte`
  - Paginated grid layout
  - Swipe gesture handling
  - Page indicator dots
  - Tests
- **Dependencies**: [032, 034]
- **Context**: `docs/04-core-screens/launcher.md`

#### Task 037: Implement Drag-and-Drop Reordering (Desktop)
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Add click-and-drag reordering for desktop
- **Deliverables**:
  - Drag event handlers
  - Visual feedback during drag
  - Snap-to-grid logic
  - Reshuffle animation
  - Tests
- **Dependencies**: [035]
- **Context**: `docs/04-core-screens/launcher.md`

#### Task 038: Implement Drag-and-Drop Reordering (Mobile)
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Add long-press + drag reordering for mobile
- **Deliverables**:
  - Long-press detection (≥500ms)
  - Drag event handlers
  - Visual feedback (reorder mode)
  - Snap-to-grid logic
  - Reshuffle animation
  - Tests
- **Dependencies**: [036]
- **Context**: `docs/04-core-screens/launcher.md`

#### Task 039: Integrate Launcher with OpenRPC
- **Type**: frontend-implementation
- **Agent**: frontend + backend-core
- **Priority**: P2
- **Description**: Wire launcher to apps.list, apps.launch, launcher.getLayout, launcher.setLayout
- **Deliverables**:
  - OpenRPC client calls from Svelte
  - App list fetching
  - App launch handling
  - Layout persistence (debounced save within 1s)
  - Layout restoration on load
  - Error handling
  - Tests
- **Dependencies**: [037, 038]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/04-core-screens/launcher.md`

#### Task 040: Test Launcher E2E (Desktop)
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P2
- **Description**: End-to-end tests for launcher functionality on desktop
- **Deliverables**:
  - Playwright tests for app list display
  - Tests for app launch
  - Tests for drag-and-drop reordering
  - Tests for layout persistence
  - Screenshot comparisons
- **Dependencies**: [039]
- **Context**: `docs/04-core-screens/launcher.md`, `docs/10-development/testing.md`

#### Task 041: Test Launcher E2E (Mobile)
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P2
- **Description**: End-to-end tests for launcher functionality on mobile
- **Deliverables**:
  - Playwright tests for paginated grid
  - Tests for swipe navigation
  - Tests for long-press + drag reordering
  - Tests for layout persistence
  - Screenshot comparisons
- **Dependencies**: [039]
- **Context**: `docs/04-core-screens/launcher.md`, `docs/10-development/testing.md`

---

### Group 4: Configuration Screen (Parallel with Launcher)
**Dependencies**: Tasks 032-034

#### Task 042: Implement Configuration Screen UI
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Create configuration screen with settings UI
- **Deliverables**:
  - `src/lib/screens/Configuration.svelte`
  - Settings list UI
  - Form components (text input, toggle, select)
  - Save/cancel buttons
  - Tests
- **Dependencies**: [032]
- **Context**: `docs/04-core-screens/configuration.md`, `docs/09-ui-ux/desktop-ui.md`

#### Task 043: Integrate Configuration with OpenRPC
- **Type**: frontend-implementation
- **Agent**: frontend + backend-core
- **Priority**: P2
- **Description**: Wire configuration screen to config.* OpenRPC methods
- **Deliverables**:
  - Calls to config.getLauncherManifest / config.setLauncherManifest
  - Calls to config.setServer / config.getServer
  - Calls to config.getAppConfig / config.setAppConfig
  - Calls to ui.setTheme / ui.getTheme
  - Error handling
  - Tests
- **Dependencies**: [042]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/04-core-screens/configuration.md`

#### Task 044: Test Configuration Screen E2E
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P2
- **Description**: End-to-end tests for configuration screen
- **Deliverables**:
  - Tests for launcher manifest change
  - Tests for server address configuration
  - Tests for theme toggle
  - Tests for setting persistence
  - Screenshot comparisons
- **Dependencies**: [043]
- **Context**: `docs/04-core-screens/configuration.md`, `docs/10-development/testing.md`

---

### Group 5: Identity Onboarding (Sequential After Group 4)
**Dependencies**: Tasks 042-044

#### Task 045: Implement Identity Wizard UI
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P2
- **Description**: Create first-run onboarding wizard for identity setup
- **Deliverables**:
  - `src/lib/screens/IdentityWizard.svelte`
  - Multi-step wizard component
  - Display name input
  - Import vs Create choice
  - 12-word seed phrase input (12 boxes)
  - 4-word address input
  - Backup reminder UI
  - Tests
- **Dependencies**: [042]
- **Context**: `docs/07-security/identity.md`, `docs/10-development/plan.md`

#### Task 046: Integrate Identity Wizard with OpenRPC
- **Type**: frontend-implementation
- **Agent**: frontend + backend-core
- **Priority**: P2
- **Description**: Wire identity wizard to identity.* OpenRPC methods
- **Deliverables**:
  - Calls to identity.status
  - Calls to identity.create
  - Calls to identity.importWithPhrase
  - Seed phrase validation
  - Error handling (invalid phrase, network errors)
  - Navigation flow (wizard -> launcher on success)
  - Tests
- **Dependencies**: [045]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/07-security/identity.md`

#### Task 047: Test Identity Wizard E2E
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P2
- **Description**: End-to-end tests for identity onboarding
- **Deliverables**:
  - Tests for create new identity flow
  - Tests for import identity flow (12-word seed)
  - Tests for import identity flow (4-word address)
  - Tests for validation errors
  - Tests for wizard-to-launcher navigation
  - Screenshot comparisons
- **Dependencies**: [046]
- **Context**: `docs/07-security/identity.md`, `docs/10-development/testing.md`

---

### Group 6: Deployment Screen (Stub for MVP)
**Dependencies**: Tasks 032-034

#### Task 048: Implement Deployment Screen Stub
- **Type**: frontend-implementation
- **Agent**: frontend
- **Priority**: P3
- **Description**: Create placeholder deployment screen (minimal for MVP)
- **Deliverables**:
  - `src/lib/screens/Deployment.svelte`
  - Simple UI showing "Deployment screen - coming soon"
  - Tests
- **Dependencies**: [032]
- **Context**: `docs/04-core-screens/deployment.md`

---

### Group 7: Application Management Integration
**Dependencies**: Tasks 039, 043, 046

#### Task 049: Implement App Installation Flow
- **Type**: frontend-implementation
- **Agent**: frontend + backend-core
- **Priority**: P2
- **Description**: Create UI flow for installing apps from manifest URIs
- **Deliverables**:
  - App install dialog/modal
  - Manifest URI input
  - Progress indicator
  - Calls to apps.install
  - Success/error feedback
  - Tests
- **Dependencies**: [039]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 050: Implement App Uninstallation Flow
- **Type**: frontend-implementation
- **Agent**: frontend + backend-core
- **Priority**: P2
- **Description**: Create UI flow for uninstalling apps
- **Deliverables**:
  - Uninstall confirmation dialog
  - Calls to apps.uninstall
  - Launcher refresh after uninstall
  - Tests
- **Dependencies**: [039]
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 051: Test App Management E2E
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P2
- **Description**: End-to-end tests for app install/uninstall
- **Deliverables**:
  - Tests for app installation flow
  - Tests for app uninstallation flow
  - Tests for error cases (invalid manifest, network errors)
  - Screenshot comparisons
- **Dependencies**: [049, 050]
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/10-development/testing.md`

---

### Group 8: Backend Service Implementations (Missing from Phase 1)
**Dependencies**: Phase 1 tasks complete

#### Task 052: Implement Application Management Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement apps.* OpenRPC methods (list, launch, install, uninstall)
- **Deliverables**:
  - `src/services/apps.rs`
  - AppsService struct
  - list(), launch(), install(), uninstall() methods
  - Manifest parsing and validation
  - Component caching logic
  - Tests
  - Documentation
- **Dependencies**: []
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/06-protocols/manifest-schema.md`

#### Task 053: Test Application Management Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [052]
- **Feedback Target**: Task 052

#### Task 054: Implement Launcher Layout Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement launcher.* OpenRPC methods (getLayout, setLayout)
- **Deliverables**:
  - `src/services/launcher.rs`
  - LauncherService struct
  - getLayout(), setLayout() methods
  - Per-identity layout persistence
  - Tests
  - Documentation
- **Dependencies**: []
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/04-core-screens/launcher.md`

#### Task 055: Test Launcher Layout Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [054]
- **Feedback Target**: Task 054

#### Task 056: Implement UI Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement ui.* OpenRPC methods (setTheme, getTheme)
- **Deliverables**:
  - `src/services/ui.rs`
  - UiService struct
  - setTheme(), getTheme() methods
  - Theme persistence
  - Tests
  - Documentation
- **Dependencies**: []
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 057: Test UI Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [056]
- **Feedback Target**: Task 056

#### Task 058: Implement Navigation Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement nav.* OpenRPC methods (setBottomMenu, switchTab)
- **Deliverables**:
  - `src/services/nav.rs`
  - NavService struct
  - setBottomMenu(), switchTab() methods
  - Mobile menu configuration persistence
  - Tests
  - Documentation
- **Dependencies**: []
- **Context**: `docs/03-core-services/osnova-core.md`

#### Task 059: Test Navigation Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [058]
- **Feedback Target**: Task 058

#### Task 060: Implement Status Service
- **Type**: backend-implementation
- **Agent**: backend-core
- **Priority**: P2
- **Description**: Implement status.get OpenRPC method for server mode
- **Deliverables**:
  - `src/services/status.rs`
  - StatusService struct
  - get() method (returns status, version, uptime, component statuses)
  - Tests
  - Documentation
- **Dependencies**: []
- **Context**: `docs/03-core-services/osnova-core.md`, `docs/08-networking/server-ops.md`

#### Task 061: Test Status Service
- **Type**: rust-testing
- **Agent**: rust-testing
- **Priority**: P2
- **Dependencies**: [060]
- **Feedback Target**: Task 060

---

### Group 9: Integration Testing
**Dependencies**: All frontend and backend tasks

#### Task 062: Full Application Integration Tests
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P3
- **Description**: End-to-end tests for complete user journeys
- **Deliverables**:
  - Test: New user onboarding -> create identity -> launcher displays -> install app -> launch app
  - Test: Import identity -> configuration -> change theme -> verify persistence
  - Test: Desktop drag-and-drop reordering -> verify layout persistence -> restart -> verify restored
  - Test: Mobile long-press reordering -> swipe pages -> verify layout persistence
  - Performance tests (p95 launch < 2s)
  - Screenshot comparisons
- **Dependencies**: [040, 041, 044, 047, 051]
- **Context**: `docs/10-development/testing.md`, `docs/10-development/plan.md`

#### Task 063: Cross-Platform Testing
- **Type**: e2e-testing
- **Agent**: e2e-testing (Playwright MCP)
- **Priority**: P3
- **Description**: Verify functionality across Windows, macOS, Linux, iOS, Android
- **Deliverables**:
  - Platform-specific test runs
  - Platform-specific UI/UX verification
  - Performance metrics per platform
  - Bug reports for platform-specific issues
- **Dependencies**: [062]
- **Context**: `docs/10-development/testing.md`

---

## Dependency Graph

```
Phase 1 Complete (Tasks 001-028)
 ├─> 029 (Tauri Project Setup)
      ├─> 030 (Tauri-Rust Integration)
           ├─> 031 (Theme System)
                ├─> 032 (Layout Container)
                ├─> 033 (Bottom Nav)
                ├─> 034 (Icon Component)
                     ├─> 035 (Launcher Desktop Grid)
                     │    ├─> 037 (Desktop Drag-Drop)
                     │         └─> 039 (Launcher OpenRPC)
                     │              ├─> 040 (E2E Desktop)
                     │              ├─> 049 (App Install)
                     │              └─> 050 (App Uninstall)
                     │                   └─> 051 (E2E App Mgmt)
                     ├─> 036 (Launcher Mobile Grid)
                     │    └─> 038 (Mobile Drag-Drop)
                     │         └─> 039 (Launcher OpenRPC)
                     │              └─> 041 (E2E Mobile)
                     └─> 042 (Config Screen UI)
                          ├─> 043 (Config OpenRPC)
                          │    └─> 044 (E2E Config)
                          └─> 045 (Identity Wizard UI)
                               └─> 046 (Identity OpenRPC)
                                    └─> 047 (E2E Identity)
                                         └─> 048 (Deployment Stub)

Parallel Backend Tasks:
 ├─> 052 (Apps Service) ─> 053 (Test)
 ├─> 054 (Launcher Service) ─> 055 (Test)
 ├─> 056 (UI Service) ─> 057 (Test)
 ├─> 058 (Nav Service) ─> 059 (Test)
 └─> 060 (Status Service) ─> 061 (Test)

Integration:
 └─> 062 (Full Integration Tests)
      └─> 063 (Cross-Platform Tests)
```

## Parallel Execution Opportunities

**Wave 1** (After 031):
- Tasks 032, 033, 034 (UI components in parallel)

**Wave 2** (After 034):
- Tasks 035, 036, 042, 048 (screens in parallel)

**Wave 3** (After 037, 038):
- Task 039 (launcher integration)

**Wave 4** (After 039):
- Tasks 040, 041, 049, 050 (testing and app management in parallel)

**Wave 5** (After 042):
- Tasks 043, 045 (config and identity in parallel)

**Backend Wave** (Parallel with frontend):
- Tasks 052, 054, 056, 058, 060 (all backend services in parallel)

**Final Wave** (After all previous):
- Tasks 062, 063 (integration tests)

## Success Criteria

### Phase 2 Complete When:
- ✅ All 35 tasks (029-063) completed
- ✅ All tests passing (Vitest unit tests + Playwright E2E tests)
- ✅ Overall coverage ≥85%
- ✅ No TypeScript/ESLint errors
- ✅ All components documented
- ✅ Launcher functional (desktop + mobile)
- ✅ Configuration screen functional
- ✅ Identity onboarding functional
- ✅ App install/uninstall working
- ✅ Theme system working (light/dark with OS sync)
- ✅ Performance: p95 launch < 2s

### Deliverables:
- Complete Tauri 2.0 application
- Three built-in screens (Launcher, Configuration, Deployment stub)
- Identity onboarding wizard
- ~25-30 Svelte components
- ~80-100 frontend tests (Vitest + Playwright)
- ~20-25 additional Rust tests (backend services)
- Comprehensive documentation
- Ready for Phase 3 (pairing, server mode, component system)

## Estimated Timeline

**With sequential execution**: ~12-15 days
**With parallel execution (multi-agent)**: ~4-6 days

**Agent invocation breakdown**:
- Frontend Agent: ~25-30 invocations
- E2E Testing Agent: ~10-15 invocations
- Backend Core Agent: ~10-12 invocations
- Rust Testing Agent: ~10-12 invocations
- **Total**: ~55-70 invocations

## Notes

### Mobile Testing Considerations
- Playwright MCP may require additional configuration for iOS/Android testing
- Consider using Tauri's mobile dev server for testing
- Screenshot comparisons should account for different screen sizes

### Performance Testing
- Use Playwright's performance metrics API
- Monitor p95 launch time (target: < 2s)
- Test on low-end devices for mobile

### Theme System
- Ensure theme changes propagate to all open tabs
- Test OS theme change detection
- Verify theme persistence across restarts

## Next Phase

After Phase 2 completion, proceed to **Phase 3**:
- Pairing implementation (client-server mode)
- Server mode operations (headless, status endpoint)
- Component packaging system (ZLIB tarballs, plugin binaries)
- OpenRPC server implementation for external components
- MPC client implementation

---

**Status**: Ready for review
**Generated**: 2025-10-07
