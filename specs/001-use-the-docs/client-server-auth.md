# Client-Server Authentication

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Use saorsa-core secure channels for authentication; session managed by saorsa-core transport layer

## Overview

In Client-Server deployment mode, mobile clients connect to a server for backend processing. Authentication is handled through saorsa-core's secure channel mechanism, which provides mutual authentication, E2E encryption, and session management.

## Authentication Architecture

### Components

```
Mobile Client (Osnova)
    ↓
saorsa-core (Client)
    ↓
Secure Channel (QUIC + ML-DSA)
    ↓
saorsa-core (Server)
    ↓
Server Backend (Osnova)
```

### Identity-Based Authentication

Both client and server have saorsa-core identities:
- **Client Identity**: 4-word address derived from client's seed phrase
- **Server Identity**: 4-word address derived from server's seed phrase

Authentication is mutual:
- Client verifies server's identity
- Server verifies client's identity

## Pairing Flow

### Initial Pairing

```
1. Server Setup
   ├─ Generate server identity (4-word address)
   ├─ Register with saorsa-core DHT
   ├─ Publish device forward for NAT traversal
   └─ Display QR code with server address

2. Client Setup
   ├─ Complete onboarding (create/import identity)
   ├─ Scan server QR code or enter 4-word address
   ├─ Resolve server identity via DHT
   └─ Initiate secure channel

3. Secure Channel Establishment
   ├─ QUIC connection with TLS 1.3
   ├─ ML-DSA signature verification (mutual)
   ├─ Derive session keys
   └─ Establish encrypted channel

4. Authorization
   ├─ Server prompts: "Allow client <4-word-addr> to connect?"
   ├─ User approves on server
   ├─ Server stores client in authorized list
   └─ Connection established
```

### Subsequent Connections

```
1. Client Startup
   ├─ Load server address from config
   ├─ Resolve server identity via DHT
   └─ Initiate secure channel

2. Secure Channel Establishment
   ├─ QUIC connection
   ├─ ML-DSA signature verification
   ├─ Server checks authorized list
   └─ Connection established (no user prompt)

3. Session Active
   ├─ All OpenRPC calls routed through secure channel
   ├─ E2E encryption for all data
   └─ Heartbeat for connection monitoring
```

## Session Management

### Session Lifecycle

**Session Start**: When secure channel is established
**Session Active**: While connection is maintained
**Session End**: When connection is closed or times out

### Session State

Managed by saorsa-core transport layer:
- Connection ID
- Session keys (ephemeral)
- Last activity timestamp
- Heartbeat interval

**No explicit session tokens** - the secure channel itself is the session

### Heartbeat

```rust
// Client sends heartbeat every 30 seconds
pub async fn maintain_session(transport: &Transport) -> Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        
        // Send heartbeat
        transport.send_heartbeat().await?;
        
        // Check for response
        if !transport.is_connected().await {
            return Err(Error::ConnectionLost);
        }
    }
}
```

### Reconnection

If connection is lost:
1. Client detects disconnection
2. Wait 5 seconds
3. Attempt to re-establish secure channel
4. If successful, resume session
5. If failed, retry with exponential backoff (max 5 minutes)

## Authorization

### Client Authorization

Server maintains list of authorized clients:

```rust
pub struct AuthorizedClients {
    clients: HashMap<String, ClientInfo>, // 4-word address -> info
}

pub struct ClientInfo {
    address: String, // 4-word address
    public_key: Vec<u8>, // ML-DSA public key
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
    nickname: Option<String>, // User-assigned name
}
```

### Authorization Flow

```rust
pub async fn authorize_client(
    server: &Server,
    client_address: &str,
) -> Result<bool> {
    // Check if already authorized
    if server.is_authorized(client_address).await {
        return Ok(true);
    }
    
    // Prompt user
    let approved = server.prompt_user_authorization(client_address).await?;
    
    if approved {
        // Add to authorized list
        server.add_authorized_client(client_address).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
```

### Revocation

User can revoke client authorization:
1. View list of authorized clients in server UI
2. Select client to revoke
3. Confirm revocation
4. Remove from authorized list
5. Close any active connections from that client

## Security Considerations

### Mutual Authentication

Both parties verify each other's identity:
- Client verifies server's ML-DSA signature
- Server verifies client's ML-DSA signature
- Prevents man-in-the-middle attacks

### End-to-End Encryption

All data encrypted with session keys:
- Derived from QUIC handshake
- Ephemeral (not persisted)
- Forward secrecy

### No Password Required

Authentication is cryptographic:
- Based on ML-DSA signatures
- No passwords to remember or leak
- No password reset mechanism needed

### Server Cannot Decrypt Client Data

Even though client uses server for backend:
- Client's seed phrase never sent to server
- Client's cocoon never sent to server
- All sensitive operations use client's keys
- Server only processes encrypted data

## API Routing

### Client-Side Routing

```typescript
// Client OpenRPC client
export class ClientServerRpcClient {
  constructor(private transport: Transport) {}
  
  async call(method: string, params: any): Promise<any> {
    // All calls go through secure channel
    const request = {
      jsonrpc: '2.0',
      method,
      params,
      id: Math.random(),
    };
    
    const response = await this.transport.send(request);
    
    if (response.error) {
      throw new Error(response.error.message);
    }
    
    return response.result;
  }
}
```

### Server-Side Routing

```rust
pub async fn handle_client_request(
    server: &Server,
    client_address: &str,
    request: JsonRpcRequest,
) -> Result<JsonRpcResponse> {
    // Verify client is authorized
    if !server.is_authorized(client_address).await {
        return Err(Error::Unauthorized);
    }
    
    // Route to appropriate service or app-supplied component
    let response = server.route_request(request).await?;

    Ok(response)
}
```

## Connection Monitoring

### Client-Side

```typescript
// Monitor connection status
export class ConnectionMonitor {
  private connected = writable(false);
  private lastHeartbeat = writable<Date | null>(null);
  
  async start(transport: Transport) {
    setInterval(async () => {
      try {
        await transport.sendHeartbeat();
        this.connected.set(true);
        this.lastHeartbeat.set(new Date());
      } catch (error) {
        this.connected.set(false);
        // Attempt reconnection
        await this.reconnect(transport);
      }
    }, 30000); // 30 seconds
  }
  
  async reconnect(transport: Transport) {
    // Exponential backoff
    for (let i = 0; i < 10; i++) {
      await sleep(Math.min(5000 * Math.pow(2, i), 300000));
      
      try {
        await transport.connect();
        this.connected.set(true);
        return;
      } catch (error) {
        console.error('Reconnection failed:', error);
      }
    }
  }
}
```

### Server-Side

```rust
pub struct ClientConnectionMonitor {
    clients: Arc<RwLock<HashMap<String, ClientConnection>>>,
}

pub struct ClientConnection {
    address: String,
    transport: Transport,
    last_heartbeat: DateTime<Utc>,
}

impl ClientConnectionMonitor {
    pub async fn monitor(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            
            let mut clients = self.clients.write().await;
            let now = Utc::now();
            
            // Remove stale connections
            clients.retain(|_, conn| {
                let elapsed = now - conn.last_heartbeat;
                elapsed.num_seconds() < 120 // 2 minutes timeout
            });
        }
    }
}
```

## Error Handling

### Connection Errors

- **Network Unreachable**: Show offline indicator, retry
- **Server Not Found**: Show error, allow address change
- **Authentication Failed**: Show error, check identities
- **Unauthorized**: Show error, request server authorization

### Session Errors

- **Session Expired**: Automatically reconnect
- **Connection Lost**: Show reconnecting indicator
- **Heartbeat Timeout**: Attempt reconnection

## UI Indicators

### Connection Status

```
┌────────────────────────────────────┐
│  ● Connected to server             │
│    Last sync: 2 seconds ago        │
└────────────────────────────────────┘

┌────────────────────────────────────┐
│  ○ Connecting to server...         │
│    Attempt 2 of 10                 │
└────────────────────────────────────┘

┌────────────────────────────────────┐
│  ⚠ Offline                          │
│    Reconnecting in 30 seconds      │
└────────────────────────────────────┘
```

## Configuration

### Client Configuration

```json
{
  "deployment": {
    "mode": "client",
    "server": {
      "address": "apple banana cherry dragon",
      "autoConnect": true,
      "reconnectInterval": 5000,
      "heartbeatInterval": 30000
    }
  }
}
```

### Server Configuration

```json
{
  "deployment": {
    "mode": "server",
    "server": {
      "requireAuthorization": true,
      "maxClients": 10,
      "sessionTimeout": 3600
    }
  }
}
```

## Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_client_authorization() {
    let server = Server::new();
    let client_address = "test client address word";
    
    // Initially not authorized
    assert!(!server.is_authorized(client_address).await);
    
    // Authorize
    server.add_authorized_client(client_address).await.unwrap();
    
    // Now authorized
    assert!(server.is_authorized(client_address).await);
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_secure_channel_establishment() {
    let server = start_test_server().await;
    let client = create_test_client().await;
    
    // Establish connection
    client.connect(&server.address).await.unwrap();
    
    // Verify connection
    assert!(client.is_connected().await);
    
    // Make RPC call
    let result = client.call("keys.derive", json!({
        "componentId": "test",
        "keyType": "ml_dsa"
    })).await.unwrap();
    
    assert!(result.is_object());
}
```

## Migration from Standalone

If user wants to switch from standalone to client-server:

1. **Export Data**: Backup all data from standalone
2. **Setup Server**: Install Osnova in server mode
3. **Pair Client**: Connect client to server
4. **Import Data**: Restore data on server
5. **Verify**: Ensure all data accessible from client

**Note**: Seed phrases remain separate (client and server have different identities)

## Summary

**MVP Authentication**:
✅ saorsa-core secure channels for mutual authentication
✅ ML-DSA signatures for identity verification
✅ QUIC transport with TLS 1.3
✅ Session managed by transport layer (no explicit tokens)
✅ Heartbeat for connection monitoring
✅ Automatic reconnection with exponential backoff
✅ Server-side client authorization

**Security Properties**:
✅ Mutual authentication
✅ End-to-end encryption
✅ Forward secrecy
✅ No passwords required
✅ Server cannot decrypt client data

**Post-MVP Enhancements**:
- Multi-server support (failover)
- Session persistence across app restarts
- Advanced connection policies
- Bandwidth optimization
- Offline mode with sync

