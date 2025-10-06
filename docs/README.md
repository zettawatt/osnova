# Osnova Documentation

Welcome to the Osnova documentation. This documentation is organized as a technical book with each chapter covering a specific aspect of the system.

## Table of Contents

### Part I: Getting Started

#### [Chapter 1: Introduction](./01-introduction/README.md)
- [Overview](./01-introduction/overview.md) - What is Osnova and why it exists
- [User Experience](./01-introduction/user-experience.md) - The end-user experience
- [Quick Start](./01-introduction/quick-start.md) - Get up and running quickly

### Part II: Architecture

#### [Chapter 2: System Architecture](./02-architecture/README.md)
- [Architecture Overview](./02-architecture/overview.md) - High-level system design
- [Components](./02-architecture/components.md) - Component-based architecture
- [Operating Modes](./02-architecture/modes.md) - Stand-alone vs Client-Server
- [Data Model](./02-architecture/data-model.md) - Core entities and relationships
- [Platform Targets](./02-architecture/platform-targets.md) - Supported platforms and targets

### Part III: Core System

#### [Chapter 3: Core Services](./03-core-services/README.md)
Built-in Rust services that provide core functionality
- [Overview](./03-core-services/overview.md) - Core services introduction
- [osnova-core](./03-core-services/osnova-core.md) - Shell services
- [osnova-saorsa](./03-core-services/osnova-saorsa.md) - Identity management
- [osnova-wallet](./03-core-services/osnova-wallet.md) - Cryptocurrency wallet
- [osnova-autonomi](./03-core-services/osnova-autonomi.md) - Autonomi network integration
- [osnova-bundler](./03-core-services/osnova-bundler.md) - Component packaging

#### [Chapter 4: Core Screens](./04-core-screens/README.md)
Built-in GUI modules for the Osnova shell
- [Overview](./04-core-screens/overview.md) - Core screens introduction
- [Launcher](./04-core-screens/launcher.md) - Application launcher
- [Configuration](./04-core-screens/configuration.md) - System configuration
- [Deployment](./04-core-screens/deployment.md) - Application deployment

### Part IV: Component System

#### [Chapter 5: Components](./05-components/README.md)
External components that extend Osnova applications
- [Overview](./05-components/overview.md) - Component system introduction
- [Frontend Components](./05-components/frontend-components.md) - Frontend component specification
- [Backend Components](./05-components/backend-components.md) - Backend component specification
- [Component ABI](./05-components/component-abi.md) - Component application binary interface
- [External Component ABI](./05-components/external-component-abi.md) - External component interface
- [Component Access Control](./05-components/component-access-control.md) - Security and permissions

### Part V: Communication

#### [Chapter 6: Protocols](./06-protocols/README.md)
Communication protocols and contracts
- [OpenRPC Conventions](./06-protocols/openrpc-conventions.md) - OpenRPC standards and best practices
- [OpenRPC Contracts](./06-protocols/openrpc-contracts.md) - Contract definitions
- [Manifest Schema](./06-protocols/manifest-schema.md) - Application manifest format

### Part VI: Security

#### [Chapter 7: Security & Identity](./07-security/README.md)
Security, encryption, and identity management
- [Identity Management](./07-security/identity.md) - Identity and addressing
- [Key Management](./07-security/keys.md) - Key derivation and storage
- [Encryption at Rest](./07-security/encryption.md) - Data encryption
- [Client-Server Authentication](./07-security/client-server-auth.md) - Authentication protocol
- [Component Access Control](./07-security/component-access-control.md) - Authorization model
- [Cocoon Encryption](./07-security/cocoon-unlock.md) - Cocoon-based encryption

### Part VII: Networking

#### [Chapter 8: Networking](./08-networking/README.md)
Networking, pairing, and server operations
- [Device Pairing](./08-networking/pairing.md) - Pairing flow and E2E encryption
- [Storage Backend](./08-networking/storage-backend.md) - Storage network integration
- [Server Operations](./08-networking/server-ops.md) - Running as a server

### Part VIII: User Interface

#### [Chapter 9: UI/UX](./09-ui-ux/README.md)
User interface design and implementation
- [Desktop UI](./09-ui-ux/desktop-ui.md) - Desktop interface specification
- [Mobile UI](./09-ui-ux/mobile-ui.md) - Mobile interface specification
- [Tauri WebView Configuration](./09-ui-ux/tauri-webview-config.md) - WebView setup
- [Onboarding Wireframes](./09-ui-ux/onboarding-wireframes.md) - Onboarding flow design

### Part IX: Development

#### [Chapter 10: Development](./10-development/README.md)
Development workflow, testing, and implementation
- [Testing Strategy](./10-development/testing.md) - TDD approach and testing
- [Research Notes](./10-development/research.md) - Research and design decisions
- [Implementation Tasks](./10-development/tasks.md) - Current implementation tasks
- [Development Plan](./10-development/plan.md) - Implementation plan

### Part X: Applications

#### [Chapter 11: Applications](./11-apps/README.md)
Core application specifications
- [App Launcher](./11-apps/app-launcher-app.md) - Launcher application
- [Configuration App](./11-apps/configuration-app.md) - Configuration application
- [Deployment App](./11-apps/app-deployment-app.md) - Deployment application

## How to Use This Documentation

### For Developers

1. **Getting Started**: Read Chapter 1 to understand what Osnova is and how it works
2. **Architecture**: Review Chapter 2 to understand the system design
3. **Core System**: Read Chapters 3-4 to understand built-in services and screens
4. **Component Development**: Read Chapter 5 to learn how to create components
5. **Protocols**: Review Chapter 6 to understand communication protocols
6. **Security**: Read Chapter 7 for security and identity management
7. **Implementation**: Follow Chapter 10 for development workflow

### For AI Agents

See the [CLAUDE.md](../CLAUDE.md) file in the root directory for AI-specific development guidelines, including:
- Spec-driven development workflow
- DRY principles and code quality requirements
- Testing requirements (TDD, ≥85% coverage)
- Common patterns and examples
- OpenRPC method reference
- File organization and naming conventions

### For Designers

1. **User Experience**: Read Chapter 1 for UX vision
2. **UI Design**: Review Chapter 9 for interface specifications
3. **Onboarding**: See Chapter 9 for onboarding flows

### For Security Auditors

1. **Security Model**: Read Chapter 7 for security architecture
2. **Identity**: Review identity management and key derivation
3. **Encryption**: Understand encryption-at-rest and E2E encryption
4. **Access Control**: Review component isolation and permissions

## Documentation Conventions

### File Organization

- Each chapter has its own directory (e.g., `01-introduction/`)
- Each chapter has a README.md overview
- Individual topics are separate markdown files
- Cross-references use relative links

### Document Structure

Each document follows this structure:

1. **Title**: Clear, descriptive title
2. **Overview**: Brief introduction (1-2 paragraphs)
3. **Content**: Main content with clear sections
4. **Examples**: Code examples where applicable
5. **References**: Links to related documentation

### Code Examples

Code examples use appropriate language tags:

```rust
// Rust examples
```

```typescript
// TypeScript examples
```

```json
// JSON examples
```

```bash
# Shell commands
```

### Diagrams and Visuals

- ASCII diagrams for simple flows
- Mermaid diagrams for complex flows
- Screenshots for UI/UX documentation

## Contributing to Documentation

When adding or updating documentation:

1. **Maintain Structure**: Keep the chapter-based organization
2. **Follow Conventions**: Use the same formatting and style
3. **Cross-Reference**: Link to related documents
4. **Examples**: Include practical code examples
5. **Keep Current**: Update documentation when code changes
6. **Test Examples**: Ensure code examples actually work

## Specification Status

This documentation represents the current specification for Osnova. Key features:

- ✅ **Core Architecture**: Defined and stable
- ✅ **Component Model**: Specified and under implementation
- ✅ **Security Model**: Defined (identity, encryption, pairing)
- ✅ **OpenRPC Contracts**: Specified for core services
- ✅ **UI/UX Design**: Specified for MVP
- 🚧 **Implementation**: In progress
- 📋 **Post-MVP Features**: Documented but not yet prioritized

Legend:
- ✅ Complete and stable
- 🚧 In progress
- 📋 Planned but not started

## Version History

- **2025-10-06**: Documentation reorganized into book format
- **2025-10-03**: Architecture update (core services/screens built-in)
- **2025-09-30**: Initial specification from docs/spec.md
- **2025-09-29**: Feature branch created (001-use-the-docs)

## Getting Help

If you have questions or need clarification:

1. Search this documentation for relevant topics
2. Check the [CLAUDE.md](../CLAUDE.md) for development guidelines
3. Review existing code for examples
4. Check the specification files for detailed requirements

---

**Next**: Start with [Chapter 1: Introduction](./01-introduction/README.md) to understand what Osnova is and why it exists.
