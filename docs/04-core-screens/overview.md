# Core Screens Overview

Osnova's core screens are built-in frontend interfaces that provide essential functionality for managing the system. These screens are part of the Osnova shell GUI and are always available.

## The Three Core Screens

### 1. Launcher
The default screen displaying installed applications in a grid layout.

**Key features**:
- Grid-based icon layout (desktop: continuous scroll, mobile: paginated)
- Icon reordering (desktop: click-drag, mobile: long-press-drag)
- Per-identity layout persistence
- App launching with manifest loading
- Loading states and error handling

**Details**: [launcher.md](./launcher.md)

### 2. Configuration
Comprehensive settings and system management interface.

**Key features**:
- Identity and security management
- Server pairing and mode selection
- Application and component management
- Theme and appearance customization
- Advanced settings and logging

**Details**: [configuration.md](./configuration.md)

### 3. Deployment
Developer tool for building, packaging, and deploying Osnova applications.

**Key features**:
- Project management and initialization
- Backend compilation (multi-target)
- Frontend packaging
- Manifest editor and validator
- Upload manager with cost estimation
- Complete build and deploy workflows

**Details**: [deployment.md](./deployment.md)

## Architecture

Core screens are integrated into the Osnova shell as Svelte components. They communicate with core services via:
- **Built-in services**: Direct in-process Rust APIs
- **External components**: OpenRPC when needed

## Common Design Principles

All core screens follow consistent design principles:
- **Responsive**: Adapt to desktop and mobile layouts
- **Accessible**: Keyboard navigation, screen readers, high contrast
- **Theme-aware**: Follow system light/dark mode
- **Performance-focused**: Fast loading and smooth interactions
- **User-friendly**: Clear error messages and helpful feedback
