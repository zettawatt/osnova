# Quickstart - Initialize Osnova feature spec from docs/spec.md

This Quickstart outlines how to validate the feature at a documentation and design level for MVP.

## Prerequisites
- Active feature branch: 001-use-the-docs
- Spec with Clarifications: /home/system/osnova/specs/001-use-the-docs/spec.md

## Manual walkthrough
1) App Launcher flow
- Given a fresh install, launch an app from the Launcher
- Expect: manifest is loaded, any app assets are fetched (core services are built‑in), UI renders in a new tab or window

2) Client-Server mode
- Configure server address and complete pairing
- On mobile, use the app
- Expect: backend operations execute on server; client remains responsive

3) Fallback on slow server
- Simulate backend p95 latency greater than 5 seconds
- Expect: client prompts to retry or use stand-alone mode

4) Data isolation and E2E
- Verify that user data is isolated per client
- Confirm data at rest is encrypted and server cannot decrypt user content in Client-Server mode

## Performance check (MVP)
- Target: p95 launch to first meaningful render is less than or equal to 2 seconds

## Next steps
- Implement contract tests per contracts/
- Follow tasks.md to incrementally deliver functionality with TDD

