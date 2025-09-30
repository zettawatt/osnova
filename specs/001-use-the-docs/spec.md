# Feature Specification: Initialize Osnova feature spec from docs/spec.md

**Feature Branch**: `001-use-the-docs`
**Created**: 2025-09-29
**Status**: Draft
**Input**: User description: "use the @docs/spec.md as the initial specification"
**Source Document**: docs/spec.md

## Execution Flow (main)
```
1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Extract key concepts from description
   → Identify: actors, actions, data, constraints
3. For each unclear aspect:
   → Mark with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   → If no clear user flow: ERROR "Cannot determine user scenarios"
5. Generate Functional Requirements
   → Each requirement must be testable
   → Mark ambiguous requirements
6. Identify Key Entities (if data involved)
7. Run Review Checklist
   → If any [NEEDS CLARIFICATION]: WARN "Spec has uncertainties"
   → If implementation details found: ERROR "Remove tech details"
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

### Section Requirements
- **Mandatory sections**: Must be completed for every feature
- **Optional sections**: Include only when relevant to the feature
- When a section doesn't apply, remove it entirely (don't leave as "N/A")

### For AI Generation
When creating this spec from a user prompt:
1. **Mark all ambiguities**: Use [NEEDS CLARIFICATION: specific question] for any assumption you'd need to make
2. **Don't guess**: If the prompt doesn't specify something (e.g., "login system" without auth method), mark it
3. **Think like a tester**: Every vague requirement should fail the "testable and unambiguous" checklist item
4. **Common underspecified areas**:
   - User types and permissions
   - Data retention/deletion policies
   - Performance targets and scale
   - Error handling behaviors
   - Integration requirements
   - Security/compliance needs

---
## Clarifications

### Session 2025-09-30
- Q: For Client‑Server mode, what is the intended confidentiality model for user data handled by the server? → A: End‑to‑end encryption for user data
- Q: For MVP, how many concurrent mobile clients must a single server support without unacceptable degradation? → A: 5 concurrent clients
- Q: What is the p95 target from app launch to first meaningful render for MVP? → A: ≤ 2 seconds
- Q: At runtime, above what p95 backend response latency should the client prompt the user to retry or temporarily switch to stand-alone mode? A: > 5 seconds
- Q: What monthly uptime target should the server meet for MVP? → A: Best-effort (no formal SLO)


## User Scenarios & Testing *(mandatory)*

### Primary User Story
As an end user, after installing Osnova I can browse and run distributed applications in a modern, browser‑like UI (tabs and windows) across platforms. I can optionally connect a mobile device to my home server to offload backend workloads while keeping the client responsive.

### Acceptance Scenarios
1. **Given** a fresh installation with no server configured, **When** the user launches an Osnova app from the App Launcher, **Then** Osnova loads the app’s manifest, fetches and starts all listed components, and renders the UI in a new tab/window.
2. **Given** a server address is configured and pairing is completed, **When** the user uses Osnova on a mobile device, **Then** backend operations execute on the configured server while the mobile client remains responsive.

### Edge Cases
- During pairing, if the server address is invalid or the server does not respond:
  - The mobile app informs the user “Server not found” and offers a retry option.
- When a previously configured server is temporarily unreachable or slow at runtime:
  - The user is notified and may choose to retry or temporarily run the affected app in stand‑alone mode.
- Multiple concurrent mobile clients connecting to one server:
  - The server must support multiple clients; the specific concurrency strategy is decided in the implementation plan.
- Missing/invalid component version referenced in a manifest:
  - Warn the user and cancel opening the app.
- Key lifecycle (creation, storage, rotation, recovery):
  - A user‑controlled 12‑word seed phrase (industry‑standard mnemonic) establishes the root identity; all keys derive from it. Users can import an existing seed to restore identity and access.
- User deletes an app's configuration or cache while the app is running:
  - The system informs the user that changes will take effect on relaunch; subsequent launches use defaults with caches cleared.


## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST provide a browser‑like UI with tabs and windows for switching between Osnova applications.
- **FR-002**: System MUST dynamically load components defined in an application manifest and run them to render the application UI.
- **FR-003**: System MUST run cross‑platform and provide installable binaries for all major OSes and architectures: Windows, macOS, Android, iOS, and various flavors of Linux.
- **FR-004**: System MUST support Stand‑alone mode by default, with all components running locally on the device.
- **FR-005**: System MUST support Client‑Server mode where backend operations run on a user‑configured server while the client interacts over the network.
- **FR-006**: System MUST allow simple pairing of mobile devices to a user’s server, supporting:
  - Initiation from the mobile app via QR code scan or manual address entry
  - Mutual key exchange between device and server upon successful contact
  - Clear “Server not found” feedback with a retry option when the server does not respond
  - Establishment of an encrypted channel after pairing; device data encrypted with its key
- **FR-007**: System MUST isolate user data between clients and encrypt data at rest on both server and stand‑alone devices, using a user‑controlled root secret (12‑word seed) for key derivation and allowing seed import for recovery. In Client‑Server mode, user data MUST be end‑to‑end encrypted such that the server cannot decrypt user content; only routing/operational metadata may remain in plaintext.
- **FR-008**: System MUST support at least 5 concurrent clients when running as a server without unacceptable degradation.
- **FR-009**: System MUST include core applications by default, with the following MVP scope:
  - App Launcher: list available apps; launch selected app by loading its manifest and opening in a tab/window; display loading/errors.
  - Crypto Wallet & Fiat Bridge: view balances; receive and send; basic swap; initiate fiat on/off‑ramp via supported providers.
  - Search: single omnibox; fetch results from distributed sources; context‑aware presentation for apps, media, images, and web pages.
  - File Manager: list downloaded/uploaded files; open file location; basic actions (open, rename, delete).
  - Configuration Manager: set server address; manage pairing; back up/restore seed phrase; manage accounts and basic security settings; manage per‑app configuration and cached data per user (view, export, reset, delete).

- **FR-010**: Search MUST be context‑aware, adjusting results format for apps, media, images, or web pages.
- **FR-011**: Components MUST communicate via stable, generic request/response interfaces independent of Osnova, enabling portability across runtimes; components run isolated from the host app.
- **FR-012**: Each component version MUST be immutable and retrievable from permanent, content‑addressed storage networks; primary network to be the Autonomi network, with support for alternatives (e.g., other permanent storage networks) to ensure long‑term availability.
- **FR-013**: System MUST persist per-app configuration and cached data as part of the user-managed encrypted data store, accessible to the end user.
- **FR-014**: Configuration Manager MUST let users browse, view, export, reset, and delete per-app configuration and cached data for their account, with clear warnings and confirmation for destructive actions.
- **FR-015**: When the user deletes an app's configuration and/or cache, the next launch MUST start with default settings and no cached data; the user should be informed that a relaunch may be required.


### Non-Functional Requirements
- **NFR-001**: p95 time from app launch to first meaningful render <= 2 seconds.
- **NFR-003**: For MVP, no formal uptime SLO; availability is best-effort.

- **NFR-002**: Client prompts fallback if p95 backend response latency > 5 seconds.


- **FR-016**: In Client-Server and Stand-alone modes, configuration and cache management MUST preserve data isolation between users and devices and operate on the user's scoped data in the selected mode.


### Key Entities *(include if feature involves data)*
- **Osnova Application**: A versioned manifest declaring frontend and backend components and required metadata.
- **App Configuration**: User-visible preferences and settings per app; part of the encrypted data store; accessible and manageable by the user. These settings can also be saved to the storage network to restore settings from the seed phrase when restarting the application on a new installation.
- **App Cache**: Regenerable, non-authoritative data stored per app to improve performance; included in the encrypted data store; user-controllable via Configuration Manager.

- **Component (Frontend)**: Provides UI; interacts with backend components via generic protocols.
- **Component (Backend)**: Provides business logic; may interact with host resources, other components, or distributed networks.
- **Manifest**: Defines the list of components and configuration; versions are immutable and permanently retrievable.
- **Server Instance**: User‑controlled host executing backend components for one or more clients.
- **Client Device**: User device (including mobile) that renders frontends and communicates with the server when configured.

- **Root Identity**: User’s 12‑word seed phrase (industry‑standard mnemonic) from which device and account keys are derived; used for backup and recovery.
- **Pairing Session**: Temporary handshake state exchanging device and server keys to establish a trusted, encrypted channel.

---


## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [ ] No implementation details (languages, frameworks, APIs)
- [ ] Focused on user value and business needs
- [ ] Written for non-technical stakeholders
- [ ] All mandatory sections completed

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain
- [ ] Requirements are testable and unambiguous
- [ ] Success criteria are measurable
- [ ] Scope is clearly bounded
- [ ] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [ ] User description parsed
- [ ] Key concepts extracted
- [ ] Ambiguities marked
- [ ] User scenarios defined
- [ ] Requirements generated
- [ ] Entities identified
- [ ] Review checklist passed

---
