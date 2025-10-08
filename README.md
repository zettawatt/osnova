# Osnova

A cross-platform distributed application framework built with Tauri 2.0, Svelte, and Rust.

## Project Structure

```
osnova/
├── core/
│   └── osnova_lib/          # Core Rust library
│       ├── src/
│       │   ├── lib.rs       # Library entry point
│       │   ├── models/      # Data models (identity, apps, config)
│       │   ├── crypto/      # Cryptographic operations
│       │   ├── storage/     # Storage layer (SQLite, files)
│       │   └── services/    # Core services
│       └── Cargo.toml
├── docs/                    # Comprehensive specifications
├── .claude/                 # Agent specifications
├── .agents/                 # Agent coordination (queue, status)
└── Cargo.toml               # Workspace configuration
```

## Development Status

**Phase 1** (In Progress): Data Models + osnova-core
- Implementing foundational data models
- Building cryptographic layer (key derivation, encryption)
- Creating storage layer (SQLite, encrypted files)
- Implementing core services (identity, keys, config)

## Setup

```bash
# .gitignore should include:
/target/
**/*.rs.bk
*.pdb
```

## Building

```bash
cd app
npm install
npm run tauri dev
```

## Documentation

Comprehensive specifications available in `docs/`:
- Architecture: `docs/02-architecture/`
- Core Services: `docs/03-core-services/`
- Security: `docs/07-security/`
- Development: `docs/10-development/`

See `CLAUDE.md` for development guidelines and `AGENTS.md` for multi-agent workflow.

## License

AGPL3
