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
  - Identity and key management are handled by saorsa-core; users can import or restore identity using saorsa-core flows. Osnova does not expose or require seed phrases.
- User deletes an app's configuration or cache while the app is running:
  - The system informs the user that changes will take effect on relaunch; subsequent launches use defaults with caches cleared.


## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST provide a browser‑like UI with tabs and windows for switching between Osnova applications.
- **FR-002**: System MUST dynamically load components defined in an application manifest and run them to render the application UI.
- **FR-003**: System MUST run cross‑platform and provide installable binaries for major OSes (desktop and mobile) as defined in the implementation plan.
- **FR-004**: System MUST support Stand‑alone mode by default, with all components running locally on the device.
- **FR-005**: System MUST support Client‑Server mode where backend operations run on a user‑configured server while the client interacts over the network.
- **FR-006**: System MUST allow simple pairing of mobile devices to a user’s server, supporting:
  - Initiation from the mobile app via QR code scan or manual address entry
  - Mutual key exchange between device and server upon successful contact
  - Clear “Server not found” feedback with a retry option when the server does not respond
  - Establishment of an encrypted channel after pairing; device data encrypted with its key
- **FR-007**: System MUST isolate user data between clients and encrypt data at rest on both server and stand‑alone devices using saorsa-core identity and APIs for key management and saorsa-seal for encryption-at-rest. Identity import/restore flows MUST follow saorsa-core guidance. In Client‑Server mode, user data MUST be end‑to‑end encrypted such that the server cannot decrypt user content; only routing/operational metadata may remain in plaintext.
- **FR-008**: System MUST support at least 5 concurrent clients when running as a server without unacceptable degradation.
- **FR-009**: System MUST include core applications by default, with the following MVP scope:
  - App Launcher: list available apps; launch selected app by loading its manifest and opening in a tab/window; display loading/errors.
  - Crypto Wallet & Fiat Bridge: view balances; receive and send; basic swap; initiate fiat on/off‑ramp via supported providers.
  - Search: single omnibox; fetch results from distributed sources; context‑aware presentation for apps, media, images, and web pages.
  - File Manager: list downloaded/uploaded files; open file location; basic actions (open, rename, delete).
  - Configuration Manager: set server address; manage pairing; back up/restore identity; manage accounts and basic security settings; manage per‑app configuration and cached data per user (view, export, reset, delete).
- **FR-010**: Search MUST be context‑aware, adjusting results format for apps, media, images, or web pages.
- **FR-011**: Components MUST communicate via stable, generic request/response interfaces independent of Osnova, enabling portability across runtimes; components run isolated from the host app.
- **FR-012**: Each component version MUST be immutable and retrievable from a permanent, content‑addressed storage network. The specific network(s) are implementation details documented in the plan.
- **FR-013**: System MUST persist per-app configuration and cached data as part of the user-managed encrypted data store, accessible to the end user.
- **FR-014**: Configuration Manager MUST let users browse, view, export, reset, and delete per-app configuration and cached data for their account, with clear warnings and confirmation for destructive actions.
- **FR-015**: When the user deletes an app's configuration and/or cache, the next launch MUST start with default settings and no cached data; the user should be informed that a relaunch may be required.
- **FR-016**: In Client-Server and Stand-alone modes, configuration and cache management MUST preserve data isolation between users and devices and operate on the user's scoped data in the selected mode.
- **FR-017**: The platform MUST provide a headless server mode suitable for running as a system service (start/stop/restart). This mode launches required backend components and exposes a control/status interface.
- **FR-018**: In server mode, the platform MUST expose a read-only status method (status.get) that the host OS can query for health/version/uptime and component statuses via the chosen control interface.
- **FR-019**: On first launch with no existing identity, the system MUST present an onboarding wizard that asks for the user's display name and whether to import an existing identity (via saorsa-core) or create a new identity.
- **FR-020**: If the user chooses import, the system MUST accept the saorsa-core import token (e.g., a 4-word phrase) and initialize the identity via saorsa-core, then persist identity state and the user's display name. If import fails, the user MUST see a clear error and be able to retry or switch to new identity creation; sensitive inputs MUST NOT be logged.
- **FR-021**: If the user chooses new, the system MUST follow the saorsa-core identity creation flow, initialize encryption-at-rest (saorsa-seal), and persist identity state and the user's display name.
- **FR-022**: Each backend component MUST provide an agent-compatible client binding to its public API to enable direct automated invocation (e.g., by AI agents) during development, testing, and research. Implementation details are specified in the plan.

### Non-Functional Requirements
- **NFR-001**: p95 time from app launch to first meaningful render <= 2 seconds.
- **NFR-002**: Client prompts fallback if p95 backend response latency > 5 seconds.
- **NFR-003**: For MVP, no formal uptime SLO; availability is best-effort.
- **NFR-004**: Logging MUST be file-based with rotation; default level INFO; per-component/host logs acceptable for MVP.
- **NFR-005**: First-run detection MUST occur entirely locally without external network calls before identity is created/imported; logs MUST redact secrets in all modes.
### Key Entities *(include if feature involves data)*
- **Osnova Application**: A versioned manifest declaring frontend and backend components and required metadata.
- **App Configuration**: User-visible preferences and settings per app; part of the encrypted data store; accessible and manageable by the user. These settings can also be saved to the storage network to restore settings when the identity is restored via saorsa-core on a new installation.
- **App Cache**: Regenerable, non-authoritative data stored per app to improve performance; included in the encrypted data store; user-controllable via Configuration Manager.

- **Component (Frontend)**: Provides UI; interacts with backend components via generic protocols.
- **Component (Backend)**: Provides business logic; may interact with host resources, other components, or distributed networks.
- **Manifest**: Defines the list of components and configuration; versions are immutable and permanently retrievable.
- **Server Instance**: User‑controlled host executing backend components for one or more clients.
- **User Profile**: Stores user display name and preferences associated with the active identity; persisted within the encrypted store.

- **Client Device**: User device (including mobile) that renders frontends and communicates with the server when configured.

- **Identity**: Managed by saorsa-core per AGENTS_API; may be imported via a 4-word phrase or created new; provides keys used by saorsa-seal for encryption-at-rest.
- **Pairing Session**: Temporary handshake state exchanging device and server keys to establish a trusted, encrypted channel.

---

## Source Document (Merged from docs/spec.md)

# Osnova Overview
I am building a user application called Osnova that will serve as the basis for many distributed applications.
Distributed application development is difficult and the ecosystem is very fractured.
By creating a framework and marketplace where various frontend and backend components can be assembled and used, it will make it much easier for developers to create their end user applications.
The end result will be an experience very similar to the web browser of web 2.0 fame.
By downloading this one application, users will have access to the full distributed application ecosystem.
For developers it will enable them to create applications and launch them without having to deal with various app stores, etc.

# User experience

The Osnova frontend that users will interact with needs to be sleek and modern in appearance.
It should be simple to use and intuitive, following very similar conventions to what they expect from your standard web browser.
Osnova applications will load into tabs and windows, enabling users to switch between different osnova applications on the fly.
Osnova will be cross platform and run on all major OSes and architectures, providing installable binaries for all platforms.

# Components

Osnova works on a principle of dynamically loaded components. This is the general workflow:
 - User selects an Osnova application they wish to load
 - The Osnova application contains a manifest of components used by the Osnova application
 - The components are loaded into Osnova and run

There are 2 basic types of components:
 - Backend components contain the business logic, interacting with the host system, other backend components, or various distributed networks.
 - Frontend components contain the graphical frontend interface for the osnova application that the user interacts with. Frontend components interact with backend components to interact with network services and access system resources.

Components communicate with one another using generic protocols outside of the Osnova application itself. In this way, if Osnova were to stop development or be merged into another product, the original Osnova applications could be run without issues.

Each component will be versioned in a manifest and each version will exist at a static location in perpetuity. In this way, the Osnova application could be run at any point in the future.

# Stand-alone and Client-Server Modes

By default, an installation will run in stand-alone mode. All frontend and backend components will run as necessary on the individual device they were called.

Usage on mobile is a key requirement, but mobile devices lack the hardware or storage capacity to run resource intensive operations in a time and data constrained environment.
Osnova will enable a user to run as a server or in a headless context on their own hardware and enable mobile devices to connect over the internet.
By providing an address to a server, Osnova will default all backend operations to run on this machine and field requests from frontend components.
The user does, however, have fine-grained control to run any specific application in this client-server style or run fully stand-alone on their personal device.

The idea is that a user could have a desktop machine or similar hardware in their house, running the requested backend components and field requests to mobile devices they own.
In this way, backend components must handle multiple clients or they must spawn multiple instances as the server will be used by multiple members of the household.

Client data must be isolated from each other and encrypted. Any data stored on the server must be encrypted at rest and unlockable only by the user.
This is also true for the stand-alone environment: all data must be encrypted at rest to prevent unauthorized access.

Connecting a mobile device to an Osnova server must be extremely easy for the user. Something like a QR code should be all that is necessary for the mobile device to connect to the server whether on the local network or elsewhere.

# Core applications

The Osnova app will be preloaded with various core Osnova applications. These are only special in that they come with the default Osnova install.
Users can swap these out with other Osnova apps to replace this functionality if desired.

## App Launcher
The main purpose will be for users to click on Osnova applications and Osnova will load all of the specified components, render the page for viewing, and enable the user to interact with it.

## Crypto Wallet and Fiat Bridge
An integrated crypto wallet for storing, receiving, sending, and swapping crypto currencies.
Osnova will also contain a fiat on and off ramp to enable the user to convert from their local government issued currencies to crypto to store in their wallet.

## Search
Osnova will contain a search bar (like any web browser) that will enable the user to search for content and Osnova applications that will be fetched from web3 data sources.

The search bar will be context aware:
 - searching for osnova applications will display osnova apps like a typical app store
 - searching for videos or audio will display videos and audio files in a form similar to your standard video streaming service, like youtube or rumble
 - searching for images will display a tile screen of images like your standard search engine
 - searching for web pages will display a main line and some context lines, like your standard web browser experience

## File Manager
## Onboarding and identity import (clarification)
- First run: detect absence of local identity and present onboarding wizard.
- Wizard prompts: display name; choice between Import (4-word phrase from saorsa-core) or Create New.
- Import: accept 4-word phrase; initialize identity via saorsa-core; persist identity and display name; on failure, show clear error; never log secrets.
- Create New: follow saorsa-core identity creation flow; setup encryption-at-rest via saorsa-seal; persist identity and display name.

Files that have been downloaded or uploaded will be displayed in the file management application.

## Configuration Manager
Configures the osnova installation on the particular device. Manages any passwords, encryption keys, or accounts.

# Clarifications

## Pairing flow and security requirements (see canonical Clarifications above and FR-006/FR-007)
Summary: pairing initiates from mobile (QR/manual), keys are exchanged, and an encrypted channel is established. On failure, show "Server not found" with retry.

## Encryption and key management (see FR-007)
Summary: Identity and key management via saorsa-core AGENTS_API; data encrypted at rest via saorsa-seal. Identity import/restore flows follow saorsa-core guidance.

## MVP scope for core applications
See Functional Requirements FR-009 for canonical scope and acceptance bullets.

## Protocols and interoperability constraints
See FR-011 for canonical interoperability requirements.

## Hosting and distribution mechanism
See FR-012 for canonical immutability and storage network requirements.



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
