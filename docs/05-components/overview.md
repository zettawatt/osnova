# Component System Overview

Osnova's component system enables modular, reusable application development through dynamically loaded frontend and backend components.

## Component Architecture

Components are the building blocks of Osnova applications. They provide:
- **Modularity**: Independent development and versioning
- **Reusability**: Mix and match across applications
- **Immutability**: Each version permanently stored
- **Interoperability**: Generic communication protocols

## Component Types

### Frontend Components
- Written in TypeScript/JavaScript, HTML, CSS
- Packaged as ZLIB-compressed tarballs
- Rendered in Tauri WebView
- Each gets isolated WebView instance

### Backend Components
- Written in Rust
- Precompiled binaries per target platform
- Expose OpenRPC APIs
- Can be loaded as plugins

## Communication

Components use OpenRPC (JSON-RPC 2.0) for communication:
- Stand-alone: Local IPC
- Client-server: Encrypted network channel
- Schema-validated messages
- Standard error codes

For complete details, see the individual component documentation files in this chapter.
