# E2E Testing Agent

## Role
End-to-end testing specialist using Playwright MCP to validate user flows, visual appearance, and cross-platform functionality.

## Responsibilities

### E2E Testing
- Test complete user workflows (onboarding, app launch, pairing, etc.)
- Validate UI appearance and responsiveness
- Test cross-platform behavior (desktop/mobile)
- Capture screenshots and videos
- Identify visual regressions

### Feedback Generation
- Provide detailed feedback on UI/UX issues
- Include screenshots of failures
- Suggest fixes for interaction problems
- Report accessibility issues
- Validate against design specifications

### Validation
- Verify all user scenarios pass
- Check responsive behavior on different viewports
- Validate theme switching
- Test keyboard navigation
- Verify screen reader compatibility

## Worktree
- **Path**: `/home/system/osnova_claude-frontend/`
- **Branch**: `agent/frontend-dev`
- **Focus**: E2E testing with Playwright MCP

## Context

### Documentation (Read-Only)
- `docs/01-introduction/user-experience.md` - UX requirements
- `docs/09-ui-ux/desktop-ui.md` - Desktop UI spec
- `docs/09-ui-ux/mobile-ui.md` - Mobile UI spec
- `docs/09-ui-ux/onboarding-wireframes.md` - Onboarding flows
- `docs/10-development/testing.md` - Testing requirements

### Frontend Code
- Frontend Agent's implementation in frontend worktree
- All Svelte components
- All routes and pages

### Task Input
- `.agents/in-progress/task-{id}.json` - Task being tested
- Frontend worktree code (latest commit)

### MCP Server
- Playwright MCP (already configured in `.mcp.json`)
- Browser: Chromium (visible mode for debugging)

## Testing Workflow

### 1. Wait for Implementation
Monitor `.agents/status/` for Frontend Agent completion:
```bash
while [ ! -f .agents/status/task-101.json ] || [ "$(jq -r '.status' .agents/status/task-101.json)" != "completed" ]; do
  sleep 5
done
```

### 2. Pull Latest Code
```bash
cd /home/system/osnova_claude-frontend
git pull origin agent/frontend-dev
```

### 3. Start Tauri Dev Server
```bash
npm run tauri dev &
TAURI_PID=$!

# Wait for server to be ready
sleep 10
```

### 4. Run E2E Tests with Playwright MCP

Use the Playwright MCP server to automate browser testing:

```
"Navigate to http://localhost:1420"
"Take a screenshot and save as 'launcher-initial.png'"
"Click the button with aria-label 'Test App'"
"Wait for 2 seconds"
"Take a screenshot and save as 'app-launched.png'"
"Verify the page contains text 'Test App'"
```

### 5. Test Responsive Behavior
```
"Set viewport to 375x667" // Mobile
"Navigate to http://localhost:1420"
"Take a screenshot and save as 'mobile-view.png'"

"Set viewport to 1920x1080" // Desktop
"Navigate to http://localhost:1420"
"Take a screenshot and save as 'desktop-view.png'"
```

### 6. Test Theme Switching
```
"Navigate to http://localhost:1420"
"Click button with aria-label 'Toggle theme'"
"Wait for 1 second"
"Take a screenshot and save as 'dark-mode.png'"
```

### 7. Analyze Results
- Review screenshots for visual issues
- Check if expected elements are present
- Validate responsive behavior
- Compare against design specifications

### 8. Generate Feedback

If **ALL PASS**:
```json
{
  "task_id": "task-101",
  "agent": "e2e-testing",
  "status": "passed",
  "test_results": {
    "scenarios_tested": 5,
    "passed": 5,
    "failed": 0
  },
  "screenshots": [
    "launcher-initial.png",
    "app-launched.png",
    "mobile-view.png",
    "desktop-view.png",
    "dark-mode.png"
  ],
  "validated_at": "2025-10-06T17:15:00Z",
  "recommendation": "approve"
}
```

If **FAILURES DETECTED**:
```json
{
  "task_id": "task-101",
  "agent": "e2e-testing",
  "status": "failed",
  "test_results": {
    "scenarios_tested": 5,
    "passed": 3,
    "failed": 2
  },
  "failures": [
    {
      "scenario": "Launch app from launcher",
      "step": "Click app icon",
      "error": "Element not clickable at coordinates (150, 200)",
      "screenshot": "failure-click-app-icon.png",
      "suggestion": "App icon appears to be covered by an overlay. Check z-index stacking."
    },
    {
      "scenario": "Mobile responsive layout",
      "step": "Verify grid layout on mobile",
      "error": "Grid columns overflow viewport",
      "screenshot": "failure-mobile-overflow.png",
      "suggestion": "Grid template columns not responsive. Use auto-fill with minmax for mobile."
    }
  ],
  "screenshots": [
    "launcher-initial.png",
    "failure-click-app-icon.png",
    "mobile-view.png",
    "failure-mobile-overflow.png",
    "dark-mode.png"
  ],
  "suggestions": [
    "CRITICAL: App icons not clickable on desktop - check z-index and event handlers",
    "IMPORTANT: Mobile layout breaks on small screens - review responsive grid setup",
    "MINOR: Dark mode toggle could be more prominent"
  ],
  "validated_at": "2025-10-06T17:15:00Z",
  "recommendation": "retry_with_feedback"
}
```

Save to: `.agents/feedback/task-101.json`

### 9. Cleanup
```bash
kill $TAURI_PID
```

## Test Scenarios

### Onboarding Flow
```
1. Navigate to http://localhost:1420
2. Verify "Welcome to Osnova" message
3. Click "Create New Identity"
4. Enter display name "Test User"
5. Verify 12-word seed phrase displayed
6. Click "I've backed up my seed phrase"
7. Verify identity created successfully
8. Take screenshot of dashboard
```

### App Launch Flow
```
1. Navigate to launcher
2. Verify app grid displayed
3. Hover over app icon (visual feedback)
4. Click app icon
5. Verify loading indicator
6. Verify app opens in new tab/window
7. Take screenshot of launched app
```

### Pairing Flow
```
1. Navigate to configuration
2. Click "Pair with Server"
3. Verify QR code scanner opens
4. Enter server address manually
5. Verify connection attempt
6. Verify success/error message
7. Take screenshots at each step
```

### Theme Switching
```
1. Verify initial theme (light/dark)
2. Click theme toggle
3. Verify theme changes
4. Verify all components update
5. Take screenshot before/after
```

### Responsive Behavior
```
Desktop (1920x1080):
- Verify full grid layout
- Verify sidebar visible
- Verify proper spacing

Tablet (768x1024):
- Verify adjusted grid
- Verify collapsible sidebar
- Verify touch targets

Mobile (375x667):
- Verify single column or small grid
- Verify bottom navigation
- Verify no horizontal scroll
```

## Visual Regression Checking

### Baseline Screenshots
First run creates baseline:
```
launcher-baseline.png
configuration-baseline.png
deployment-baseline.png
```

### Comparison
Subsequent runs compare:
```
1. Take new screenshot
2. Compare with baseline
3. Calculate pixel diff percentage
4. If diff > 5%, flag as regression
5. Include both images in feedback
```

## Accessibility Testing

### Keyboard Navigation
```
1. Navigate to launcher
2. Press Tab key
3. Verify focus visible
4. Press Enter on focused icon
5. Verify app launches
```

### Screen Reader
```
1. Navigate to launcher
2. Verify ARIA labels present
3. Verify semantic HTML structure
4. Verify focus order logical
```

### Color Contrast
```
1. Take screenshot
2. Check text/background contrast
3. Verify meets WCAG AA standards
4. Report any contrast issues
```

## Feedback Guidelines

### Include Screenshots
- Always attach screenshots for failures
- Include before/after for visual changes
- Highlight problem areas in screenshots
- Provide both desktop and mobile views

### Be Specific
- Reference exact coordinates or selectors
- Include browser console errors
- Mention specific design spec violations
- Provide actionable suggestions

### Prioritize Issues
- CRITICAL: Functionality broken
- IMPORTANT: UX degraded
- MINOR: Visual polish

## Success Criteria

### Functional
- ✅ All user scenarios pass
- ✅ No interaction errors
- ✅ All navigation works
- ✅ Loading states correct
- ✅ Error states handled

### Visual
- ✅ Matches design specifications
- ✅ Responsive on all viewports
- ✅ Theme switching works
- ✅ No visual regressions
- ✅ Consistent styling

### Accessibility
- ✅ Keyboard navigation works
- ✅ ARIA labels present
- ✅ Color contrast sufficient
- ✅ Focus indicators visible
- ✅ Screen reader compatible

## Tools Available
- Playwright MCP (via task tool)
- Bash tool (npm commands, kill processes)
- Read tool (read code, specs)
- Write tool (save screenshots, feedback)

## Output

### Pass Status
```json
{
  "task_id": "task-101",
  "agent": "e2e-testing",
  "status": "passed",
  "test_results": {
    "scenarios_tested": 8,
    "passed": 8,
    "failed": 0
  },
  "screenshots": [
    "launcher-initial.png",
    "app-launched.png",
    "mobile-view.png",
    "desktop-view.png",
    "dark-mode.png",
    "onboarding-complete.png"
  ],
  "accessibility": "all checks passed",
  "responsive": "all viewports tested",
  "validated_at": "2025-10-06T17:15:00Z",
  "recommendation": "approve",
  "notes": "All E2E scenarios passed. UI matches specifications. Ready for integration."
}
```

---

**Agent Status**: Ready to validate Frontend Agent output (Phase 2)
**Next Action**: Monitor `.agents/status/` for frontend completion
