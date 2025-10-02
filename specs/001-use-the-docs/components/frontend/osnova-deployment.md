# osnova-deployment component

It is written in Svelte as a static web app.
It is a single page application.

**MVP Status**: This component is **REQUIRED for MVP** as it provides the developer interface for building and deploying Osnova applications.

## Overview

The osnova-deployment component is a comprehensive developer tool for building, packaging, and deploying Osnova components and applications. It provides an intuitive GUI interface to the osnova-bundler backend component, making it easy for developers to:

- Create new Osnova projects from templates
- Compile backend components to multiple targets
- Package frontend components
- Generate and edit manifests
- Upload to Autonomi network
- Manage deployments and versions

## Layout

The application uses a tabbed interface with the following main sections:

### Main Navigation (Left Sidebar)
- **Projects**: List of local projects
- **Build**: Compilation and packaging
- **Manifests**: Manifest editor and validator
- **Deploy**: Upload and deployment
- **Settings**: Configuration and preferences

### Content Area (Main Panel)
- Dynamic content based on selected navigation item
- Split view for editor/preview when applicable
- Progress indicators for long-running operations
- Real-time logs and output

### Status Bar (Bottom)
- Connection status (Autonomi network)
- Wallet balance (ETH and AUTONOMI tokens)
- Current project info
- Build/deploy status

## Functionality

### 1. Project Management

#### Project List View
- **Display**: Grid or list view of local projects
- **Actions**:
  - Create new project
  - Open existing project
  - Import project from Autonomi
  - Delete project
- **Info Display**: Project name, type, version, last modified

#### New Project Dialog
- **Fields**:
  - Project name
  - Application ID (auto-generated from name, editable)
  - Project type: Application, Frontend Component, Backend Component
  - Template selection: Svelte+Rust, Svelte Only, Rust Only
  - Project location (file picker)
- **Actions**:
  - Create project
  - Cancel
- **Backend Call**: `bundler.project.init`

#### Project Validation
- **Display**: Validation results with errors/warnings
- **Actions**:
  - Validate project structure
  - Fix common issues (auto-fix button)
- **Backend Call**: `bundler.project.validate`

### 2. Build Interface

#### Backend Compilation Panel
- **Target Selection**: Checkboxes for each target
  - Linux x64
  - Linux ARM64
  - macOS x64 (Intel)
  - macOS ARM64 (Apple Silicon)
  - Windows x64
  - Android ARM64
  - iOS ARM64
- **Build Options**:
  - Release/Debug toggle
  - Feature flags (multi-select)
  - Custom build flags (text input)
- **Actions**:
  - Compile selected targets
  - Compile all targets
  - Clean build artifacts
- **Output**:
  - Real-time compilation logs
  - Build status per target
  - Binary sizes and hashes
  - Error messages with line numbers
- **Backend Call**: `bundler.backend.compile`

#### Frontend Packaging Panel
- **Build Configuration**:
  - Build command (default: `npm run build`)
  - Dist directory (default: `dist`)
  - Node version selection
- **Actions**:
  - Run build
  - Package to tarball
  - Preview packaged files
- **Output**:
  - Build logs
  - Package size (original vs compressed)
  - Compression ratio
  - File list in tarball
- **Backend Call**: `bundler.frontend.package`

#### Build Progress
- **Display**:
  - Overall progress bar
  - Per-target progress
  - Current step description
  - Estimated time remaining
- **Actions**:
  - Cancel build
  - View detailed logs

### 3. Manifest Editor

#### Component Manifest Editor
- **Form Fields**:
  - Component ID (read-only if editing)
  - Name
  - Version (semver validation)
  - Type (backend/frontend)
  - Description (textarea)
  - Author
  - License (dropdown with common licenses)
  - Icon (image upload, 512x512 recommended)
  - Dependencies (multi-select from available components)
- **Backend Binaries Section** (for backend components):
  - List of compiled binaries
  - Target, Autonomi address, hash, size
  - Upload status indicator
- **Frontend Package Section** (for frontend components):
  - Tarball info
  - Autonomi address
  - Upload status
- **Actions**:
  - Save manifest
  - Validate manifest
  - Upload manifest
  - Preview JSON
- **Backend Calls**:
  - `bundler.manifest.createComponent`
  - `bundler.manifest.validate`
  - `bundler.manifest.upload`

#### Application Manifest Editor
- **Form Fields**:
  - Application ID
  - Name
  - Version
  - Description
  - Author
  - License
  - Icon (image upload)
  - Permissions (multi-select checkboxes)
- **Components Section**:
  - List of included components
  - Add component button (opens component selector)
  - Remove component button
  - Component details: ID, type, Autonomi address
- **Actions**:
  - Save manifest
  - Validate manifest
  - Upload manifest
  - Preview JSON
- **Backend Calls**:
  - `bundler.manifest.createApplication`
  - `bundler.manifest.validate`
  - `bundler.manifest.upload`

#### Manifest Validator
- **Display**:
  - Validation status (valid/invalid)
  - List of errors (red, with line numbers)
  - List of warnings (yellow)
  - Suggestions for fixes
- **Actions**:
  - Re-validate
  - Auto-fix (for fixable issues)
  - Jump to error in editor

### 4. Deployment Interface

#### Upload Manager
- **Upload Queue**:
  - List of items to upload
  - Type (binary, tarball, manifest)
  - Size
  - Estimated cost
  - Status (pending, uploading, complete, failed)
- **Cost Summary**:
  - Total ETH required
  - Total AUTONOMI tokens required
  - Current wallet balance
  - Sufficient funds indicator
- **Actions**:
  - Upload all
  - Upload selected
  - Remove from queue
  - Retry failed
- **Backend Calls**:
  - `bundler.backend.upload`
  - `bundler.frontend.upload`
  - `bundler.manifest.upload`

#### Complete Workflow
- **One-Click Deploy**:
  - Single button to build and deploy everything
  - Progress through all stages
  - Automatic payment handling
- **Workflow Steps Display**:
  1. Validate project ✓
  2. Compile backends ✓
  3. Package frontend ✓
  4. Upload binaries (in progress...)
  5. Generate manifests (pending)
  6. Upload manifests (pending)
  7. Complete (pending)
- **Actions**:
  - Start deployment
  - Cancel deployment
  - View detailed logs
- **Backend Call**: `bundler.workflow.buildAndDeploy`

#### Deployment History
- **Display**: Table of past deployments
  - Date/time
  - Version
  - Status (success/failed)
  - Autonomi address
  - Cost
- **Actions**:
  - View details
  - Copy install URL
  - Rollback (post-MVP)
  - Delete

### 5. Settings

#### Autonomi Network Settings
- **Fields**:
  - Network type: Mainnet / Testnet (toggle)
  - Custom peers (optional, textarea)
- **Display**:
  - Connection status
  - Peer count
  - Network health indicator

#### Wallet Settings
- **Display**:
  - Connected wallet address
  - ETH balance
  - AUTONOMI token balance
- **Actions**:
  - Link wallet component
  - View transaction history
  - Refresh balances

#### Build Settings
- **Fields**:
  - Default build targets (checkboxes)
  - Rust toolchain version
  - Node.js version
  - Build cache location
- **Actions**:
  - Clear build cache
  - Reset to defaults

#### Editor Settings
- **Fields**:
  - Theme (light/dark)
  - Font size
  - Tab size
  - Auto-save (toggle)

## UI Components

### Reusable Components

#### TargetSelector
- Checkbox list of compilation targets
- Visual indicators for supported platforms
- Tooltips with target details

#### CostEstimator
- Real-time cost calculation
- ETH and AUTONOMI token breakdown
- Wallet balance comparison
- Warning if insufficient funds

#### ProgressTracker
- Multi-step progress indicator
- Current step highlight
- Completed steps checkmark
- Failed steps error icon
- Estimated time remaining

#### LogViewer
- Scrollable log output
- Syntax highlighting for errors/warnings
- Filter by log level
- Search functionality
- Copy to clipboard
- Clear logs

#### ManifestPreview
- JSON syntax highlighting
- Collapsible sections
- Copy to clipboard
- Download as file

#### FileTree
- Hierarchical file browser
- Icons for file types
- File size display
- Context menu (open, delete, rename)

## User Flows

### Flow 1: Create and Deploy New Application

1. Click "New Project" in Projects view
2. Fill in project details, select "Svelte+Rust" template
3. Click "Create" → Project initialized
4. Navigate to Build tab
5. Select Linux x64 and macOS ARM64 targets
6. Click "Compile All" → Compilation starts
7. Wait for compilation to complete
8. Click "Package Frontend" → Frontend packaged
9. Navigate to Manifests tab
10. Fill in application details
11. Click "Save Manifest"
12. Navigate to Deploy tab
13. Review upload queue and costs
14. Click "Deploy All" → Payment dialog appears
15. Approve payment in wallet
16. Wait for uploads to complete
17. Copy install URL from success message

### Flow 2: Update Existing Application

1. Open project from Projects list
2. Make code changes
3. Navigate to Build tab
4. Click "Compile All" → Recompilation
5. Navigate to Manifests tab
6. Increment version number
7. Click "Save Manifest"
8. Navigate to Deploy tab
9. Click "Deploy All"
10. Approve payment
11. New version deployed

### Flow 3: Validate and Fix Project

1. Open project
2. Click "Validate Project"
3. Review errors and warnings
4. Click "Auto-Fix" for fixable issues
5. Manually fix remaining errors
6. Re-validate until clean

## Integration with Backend Components

### osnova-bundler Integration
- All build, package, and upload operations
- Manifest generation and validation
- Project initialization and validation

### osnova-wallet Integration
- Display wallet balance
- Request payment for uploads
- Show transaction history

### osnova-autonomi Integration
- Network connection status
- Upload progress tracking
- Download deployed applications

### osnova-core Integration
- Store project preferences
- Cache build artifacts
- Manage component dependencies

## Error Handling

### User-Friendly Error Messages
- **Compilation Error**: "Failed to compile backend: missing dependency 'serde'. Run 'cargo add serde' to fix."
- **Upload Error**: "Upload failed: insufficient AUTONOMI tokens. You need 50 AUTONOMI but have 30."
- **Validation Error**: "Invalid manifest: version must follow semver format (e.g., 1.0.0)"

### Error Recovery
- Retry button for failed operations
- Auto-save drafts to prevent data loss
- Detailed error logs for debugging

## Accessibility

- Keyboard navigation support
- Screen reader compatible
- High contrast mode
- Adjustable font sizes
- Focus indicators

## Performance Considerations

- Lazy load project list
- Virtual scrolling for large logs
- Debounced validation
- Background compilation
- Incremental builds

## MVP Implementation Notes

1. **Svelte 5**: Use latest Svelte with runes
2. **Styling**: Tailwind CSS for rapid development
3. **Icons**: Lucide icons or similar
4. **State Management**: Svelte stores
5. **OpenRPC Client**: Auto-generated from bundler spec
6. **File Picker**: Tauri file dialog API
7. **Notifications**: Toast notifications for success/error

## Post-MVP Enhancements

- Drag-and-drop file upload
- Visual manifest editor (form-based)
- Integrated code editor
- Git integration
- Collaborative editing
- Deployment analytics
- A/B testing support
- Automated testing integration
- CI/CD pipeline integration
- Marketplace integration
