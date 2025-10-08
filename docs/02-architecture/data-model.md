# Data Model

This document defines the conceptual data model for Osnova, including entities, relationships, and validation rules. For concrete SQL implementation, see [data-model-sql.md](./data-model-sql.md).


Architecture Update (2025-10-03): Built-in core services and core screens are part of the Osnova shell and are not represented as ComponentRef entries. ComponentRef applies to app-supplied components only (frontend/backend) referenced by application manifests.

## Entities

### OsnovaApplication
- id: string (content address / manifest hash)
- name: string
- version: semver
- manifestUrl: string (permanent storage URL)
- components: [ComponentRef] (app-supplied; excludes built-in core services/screens)
- metadata: map<string,string>

### ComponentRef
- id: string (content address)
- kind: enum [frontend, backend]
- version: semver (immutable)
- configSchemaRef: string (optional)

### AppConfiguration
- appId: string (FK -> OsnovaApplication.id)
- userId: string (scoped to RootIdentity)
- settings: map<string,any>
- updatedAt: timestamp

### AppCache
- appId: string (FK -> OsnovaApplication.id)
- userId: string (scoped to RootIdentity)
- entries: opaque blob (regenerable)
- updatedAt: timestamp

### RootIdentity
- seedMnemonic: string (12-word; never stored in plaintext)
- rootKey: bytes (derived; secure enclave/keystore)
- deviceKeys: [DeviceKey]

### DeviceKey
- deviceId: string
- publicKey: bytes
- createdAt: timestamp
- revokedAt: timestamp | null

### PairingSession
- sessionId: string
- serverPublicKey: bytes
- devicePublicKey: bytes
- establishedAt: timestamp
- expiresAt: timestamp
- status: enum [pending, established, failed]

### ServerInstance
- serverId: string
- hostname: string
- concurrentClientsCapacity: integer (>= 5 for MVP)
- status: enum [online, degraded, offline]
- lastHeartbeat: timestamp

### ClientDevice
- deviceId: string
- platform: enum [windows, macos, linux, android, ios]
- pairedServerId: string | null (FK -> ServerInstance.serverId)

## Relationships
- OsnovaApplication 1..* ComponentRef
- OsnovaApplication 1..* AppConfiguration (per user)
- OsnovaApplication 1..* AppCache (per user)
- RootIdentity 1..* DeviceKey
- ClientDevice 0..1 <-> 1 ServerInstance (paired)

## Validation & Constraints
- ComponentRef.version immutable; content-addressed ids must match resolved content hash.
- AppConfiguration and AppCache are encrypted at rest and scoped per user/device.
- Client-Server mode: user data is end-to-end encrypted; server cannot decrypt user content.
- Concurrency: server must support at least 5 concurrent mobile clients without unacceptable degradation.

## State Transitions (selected)
- PairingSession: pending -> established | failed
- ServerInstance.status: online -> degraded -> offline (based on health/latency)


