# Operating Modes

Osnova supports two primary operating modes: Stand-Alone and Client-Server. This flexibility allows users to optimize for their specific use case, whether running entirely on a single device or leveraging a more powerful server for backend operations.

## Stand-Alone Mode

### Overview

Stand-alone mode is the default configuration. All frontend and backend components run locally on the individual device.

### When to Use Stand-Alone Mode

- Desktop or laptop with sufficient resources
- No reliable network connection
- Maximum privacy (all operations local)
- Single-device usage
- Development and testing

### How It Works

In stand-alone mode:
1. User launches application from Launcher
2. All components load and execute on local device
3. Frontend components render in WebView
4. Backend components run as local processes
5. Communication via local IPC (OpenRPC)
6. Data stored in encrypted local storage

### Architecture

```
┌─────────────────────────────────────┐
│         Local Device                │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Frontend Components         │  │
│  │  (WebView)                   │  │
│  └────────────┬─────────────────┘  │
│               │ Local IPC          │
│               │ (OpenRPC)          │
│  ┌────────────▼─────────────────┐  │
│  │  Backend Components          │  │
│  │  (Rust Processes)            │  │
│  └────────────┬─────────────────┘  │
│               │                    │
│  ┌────────────▼─────────────────┐  │
│  │  Core Services               │  │
│  └────────────┬─────────────────┘  │
│               │                    │
│  ┌────────────▼─────────────────┐  │
│  │  Encrypted Local Storage     │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

### Performance Characteristics

**Advantages**:
- Low latency (no network overhead)
- Works offline
- Maximum privacy
- Consistent performance

**Considerations**:
- Limited by device resources
- May not be suitable for resource-intensive tasks on mobile
- Storage limited by device capacity

### Resource Requirements

**Minimum** (for basic usage):
- 4 GB RAM
- 1 GB storage
- Modern CPU (last 5 years)

**Recommended** (for optimal experience):
- 8+ GB RAM
- 10+ GB storage
- Multi-core CPU

## Client-Server Mode

### Overview

Client-server mode splits the workload between a client (typically mobile) and a server (typically a desktop or dedicated hardware). This enables mobile devices to leverage more powerful hardware for backend operations while maintaining a responsive UI.

### When to Use Client-Server Mode

- Mobile device usage
- Resource-intensive backend operations
- Multiple devices in a household
- Consistent access to home server
- Better performance on mobile

### How It Works

In client-server mode:
1. User pairs mobile device with server via QR code or address
2. Secure encrypted channel established (via saorsa-core)
3. Frontend components run on mobile client
4. Backend operations execute on server
5. Client and server communicate via OpenRPC over encrypted channel
6. Each client's data isolated and encrypted on server

### Architecture

```
┌──────────────────────┐         ┌──────────────────────┐
│   Mobile Client      │         │   Server             │
│                      │         │                      │
│  ┌────────────────┐ │         │  ┌────────────────┐  │
│  │  Frontend      │ │         │  │  Backend       │  │
│  │  Components    │ │         │  │  Components    │  │
│  │  (WebView)     │ │         │  │  (Rust)        │  │
│  └────────┬───────┘ │         │  └────────┬───────┘  │
│           │         │         │           │          │
│  ┌────────▼───────┐ │         │  ┌────────▼───────┐  │
│  │  OpenRPC       │◄├────────►│  │  OpenRPC       │  │
│  │  Client        │ │ Encrypt │  │  Server        │  │
│  └────────────────┘ │ Channel │  └────────┬───────┘  │
│                      │         │           │          │
│  ┌────────────────┐ │         │  ┌────────▼───────┐  │
│  │  Local Cache   │ │         │  │  Core Services │  │
│  └────────────────┘ │         │  └────────┬───────┘  │
│                      │         │           │          │
└──────────────────────┘         │  ┌────────▼───────┐  │
                                 │  │  Encrypted     │  │
                                 │  │  Storage       │  │
                                 │  │  (per-user)    │  │
                                 │  └────────────────┘  │
                                 └──────────────────────┘
```

### Pairing Process

The pairing process is designed to be simple and secure:

1. **Server Setup**: Server displays QR code or 4-word identity address
2. **Client Initiation**: Mobile app scans QR or enters address manually
3. **Connection Attempts**:
   - Up to 3 attempts
   - 5-second timeout per attempt
   - Exponential backoff: 1s, 2s, 4s (with jitter)
4. **Key Exchange**: Mutual key exchange via saorsa-core
5. **Channel Establishment**: End-to-end encrypted channel created
6. **Confirmation**: User receives success confirmation

If connection fails:
- Clear "Server not found" message
- Retry option available
- Manual address entry option

### Data Security

**End-to-End Encryption**:
- User data encrypted on client before transmission
- Server cannot decrypt user content
- Only routing/operational metadata in plaintext
- Each device has unique encryption keys

**Data Isolation**:
- Each client's data completely isolated
- Per-user encrypted storage on server
- No cross-client data access

### Performance Characteristics

**Advantages**:
- Mobile device remains responsive
- Leverage more powerful server hardware
- Suitable for resource-intensive operations
- Shared resources across household devices

**Considerations**:
- Network latency added to operations
- Requires reliable network connection
- Server must be available
- Setup slightly more complex

### Server Requirements

**Minimum** (5 concurrent clients):
- 8 GB RAM
- 100 GB storage
- 4-core CPU
- Stable network connection

**Recommended** (10+ concurrent clients):
- 16+ GB RAM
- 500+ GB storage
- 8+ core CPU
- High-speed network connection

### Client Requirements

**Mobile**:
- 2 GB RAM
- 500 MB storage (for frontend components and cache)
- Modern mobile device (last 3 years)
- WiFi or mobile data connection

### Fallback Behavior

When server becomes slow or unreachable:

**High Latency** (>5 seconds p95):
- Client detects slow responses
- Prompts user with options:
  - Retry connection
  - Temporarily switch to stand-alone mode

**Server Unavailable**:
- Clear "Server not found" message
- Option to retry
- Option to work in stand-alone mode
- Cached data remains available

## Mode Selection and Switching

### Initial Configuration

- **Default**: Stand-alone mode
- **Server Mode**: Configure via Configuration Manager
- **Pairing**: Use QR code or manual entry

### Switching Modes

Users can switch modes per-application:
- **Global default**: Set in Configuration Manager
- **Per-app override**: Choose mode for specific app
- **Temporary switch**: Fallback when server issues occur

### Mode Persistence

Mode selection persists across:
- Application restarts
- Device reboots
- System updates

## Multi-Client Scenarios

### Household Setup

Typical household setup:
1. Server running on desktop or dedicated hardware
2. Multiple mobile devices paired to server
3. Each family member has their own identity
4. Shared backend resources
5. Isolated per-user data

### Concurrent Client Support

**MVP Requirements**:
- Server must support ≥5 concurrent clients
- No unacceptable degradation
- Fair resource allocation
- Independent execution contexts

**Scaling Beyond MVP**:
- Additional clients supported based on hardware
- Load balancing for optimal performance
- Monitoring and health checks

## Headless Server Mode

### Overview

Server mode can run headless (no GUI):
- Suitable for system service deployment
- Managed via systemd (Linux) or equivalent
- Exposes control/status interface

### Server Operations

**Lifecycle**:
- Start: Launch backend components
- Stop: Graceful shutdown
- Restart: Stop and start
- Status: Query health and component status

**Status Interface**:
- Read-only status method: `status.get`
- Returns: health, version, uptime, component statuses
- Used by host OS for monitoring

### Deployment

Headless server deployment:
```bash
# Start server
osnova --server start

# Check status
osnova --server status

# Stop server
osnova --server stop
```

### Logging

File-based logging with rotation:
- Default level: INFO
- Per-component logs acceptable for MVP
- Secrets redacted in all modes
- Platform-specific log locations

## Best Practices

### For Stand-Alone Users

- Ensure sufficient local storage
- Regular backups of encrypted data
- Keep application cache manageable
- Monitor resource usage

### For Client-Server Users

- Maintain reliable network connection
- Keep server hardware adequate for load
- Monitor server resource usage
- Regular server maintenance
- Backup server data regularly

### For Server Operators

- Use headless mode for dedicated servers
- Configure appropriate resource limits
- Monitor concurrent client count
- Implement log rotation
- Regular security updates

## Choosing the Right Mode

**Use Stand-Alone When**:
- On a capable desktop/laptop
- Privacy is paramount
- Network unavailable or unreliable
- Single device usage

**Use Client-Server When**:
- Primary device is mobile
- Need access to powerful backend
- Multiple household devices
- Consistent home network access

**Consider Hybrid**:
- Critical apps in stand-alone mode
- Others in client-server mode
- Switch based on context (home vs. away)
