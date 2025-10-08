# Server Mode Implementation Guide

**Last Updated**: 2025-10-08
**Status**: Complete specification for MVP

## Overview

This document provides concrete implementation details for Osnova's server mode, including the HTTP server setup, WebSocket support, TLS configuration, and connection management.

## Technology Stack

### Core Dependencies

```toml
[dependencies]
# HTTP server framework
axum = { version = "0.7", features = ["ws", "macros"] }
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "trace", "compression"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# TLS support
rustls = "0.23"
rustls-pemfile = "2.0"
tokio-rustls = "0.26"
axum-server = { version = "0.6", features = ["tls-rustls"] }

# WebSocket
tokio-tungstenite = "0.21"

# JSON-RPC
jsonrpsee = { version = "0.22", features = ["server", "macros"] }

# Authentication
jsonwebtoken = "9.2"
argon2 = "0.5"

# Tracing and metrics
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
metrics = "0.22"
metrics-exporter-prometheus = "0.14"
```

## Server Architecture

### Main Server Structure

```rust
// core/osnova_lib/src/server/mod.rs

use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct OsnovaServer {
    config: ServerConfig,
    state: Arc<ServerState>,
    rpc_server: Arc<RpcServer>,
}

pub struct ServerConfig {
    pub bind_address: SocketAddr,
    pub max_clients: usize,
    pub tls_config: Option<TlsConfig>,
    pub cors_origins: Vec<String>,
}

pub struct ServerState {
    pub connections: RwLock<ConnectionPool>,
    pub sessions: RwLock<SessionManager>,
    pub metrics: MetricsCollector,
}

pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl OsnovaServer {
    pub fn new(config: ServerConfig) -> Result<Self> {
        let state = Arc::new(ServerState {
            connections: RwLock::new(ConnectionPool::new(config.max_clients)),
            sessions: RwLock::new(SessionManager::new()),
            metrics: MetricsCollector::new(),
        });

        let rpc_server = Arc::new(RpcServer::new());

        Ok(OsnovaServer {
            config,
            state,
            rpc_server,
        })
    }

    pub async fn run(self) -> Result<()> {
        // Build router
        let app = self.build_router().await?;

        // Configure TLS if enabled
        if let Some(tls_config) = &self.config.tls_config {
            self.run_with_tls(app, tls_config).await
        } else {
            self.run_http(app).await
        }
    }

    async fn build_router(&self) -> Result<Router> {
        let cors = CorsLayer::new()
            .allow_origin(
                self.config.cors_origins
                    .iter()
                    .map(|o| o.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

        let app = Router::new()
            // Health check endpoint
            .route("/health", get(health_handler))

            // Status endpoint (read-only)
            .route("/status", get(status_handler))

            // JSON-RPC endpoint
            .route("/rpc", post(rpc_handler))

            // WebSocket endpoint for real-time updates
            .route("/ws", get(websocket_handler))

            // Metrics endpoint
            .route("/metrics", get(metrics_handler))

            // Add middleware
            .layer(cors)
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(Extension(self.state.clone()))
            .layer(Extension(self.rpc_server.clone()));

        Ok(app)
    }

    async fn run_http(self, app: Router) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(&self.config.bind_address).await?;

        tracing::info!("Server listening on {}", self.config.bind_address);

        axum::serve(listener, app)
            .await
            .map_err(|e| e.into())
    }

    async fn run_with_tls(self, app: Router, tls_config: &TlsConfig) -> Result<()> {
        let rustls_config = load_tls_config(tls_config)?;

        let listener = tokio::net::TcpListener::bind(&self.config.bind_address).await?;

        tracing::info!("Server listening on {} (TLS)", self.config.bind_address);

        axum_server::from_tcp_rustls(listener, rustls_config)
            .serve(app.into_make_service())
            .await
            .map_err(|e| e.into())
    }
}
```

## JSON-RPC Implementation

### RPC Server

```rust
// core/osnova_lib/src/server/rpc.rs

use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use serde_json::Value;

#[rpc(server)]
pub trait OsnovaRpc {
    #[method(name = "apps.list")]
    async fn apps_list(&self) -> RpcResult<Vec<Application>>;

    #[method(name = "apps.install")]
    async fn apps_install(&self, manifest_uri: String) -> RpcResult<String>;

    #[method(name = "apps.uninstall")]
    async fn apps_uninstall(&self, app_id: String) -> RpcResult<bool>;

    #[method(name = "config.getServer")]
    async fn config_get_server(&self) -> RpcResult<Option<String>>;

    #[method(name = "config.setServer")]
    async fn config_set_server(&self, address: String) -> RpcResult<bool>;

    #[method(name = "identity.status")]
    async fn identity_status(&self) -> RpcResult<IdentityStatus>;

    #[method(name = "pairing.start")]
    async fn pairing_start(&self) -> RpcResult<PairingToken>;

    #[method(name = "pairing.complete")]
    async fn pairing_complete(&self, token: String) -> RpcResult<bool>;
}

pub struct RpcServer {
    services: Arc<Services>,
}

#[async_trait]
impl OsnovaRpcServer for RpcServer {
    async fn apps_list(&self) -> RpcResult<Vec<Application>> {
        self.services.apps_service
            .list_applications()
            .await
            .map_err(|e| jsonrpsee::core::Error::Custom(e.to_string()))
    }

    async fn apps_install(&self, manifest_uri: String) -> RpcResult<String> {
        self.services.apps_service
            .install_application(&manifest_uri)
            .await
            .map_err(|e| jsonrpsee::core::Error::Custom(e.to_string()))
    }

    // ... implement other methods
}
```

### RPC Handler

```rust
async fn rpc_handler(
    Extension(rpc_server): Extension<Arc<RpcServer>>,
    Extension(state): Extension<Arc<ServerState>>,
    headers: HeaderMap,
    body: String,
) -> Response<Body> {
    // Authenticate request
    if let Err(e) = authenticate_request(&headers, &state).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from(e.to_string()))
            .unwrap();
    }

    // Rate limiting
    if let Err(e) = check_rate_limit(&headers, &state).await {
        return Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from(e.to_string()))
            .unwrap();
    }

    // Process RPC request
    let response = rpc_server.handle_request(body).await;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(response))
        .unwrap()
}
```

## WebSocket Support

### WebSocket Handler

```rust
// core/osnova_lib/src/server/websocket.rs

use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<ServerState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: Arc<ServerState>) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));

    // Register connection
    let conn_id = Uuid::new_v4().to_string();
    state.connections.write().await.add(conn_id.clone(), sender.clone());

    // Spawn task to handle incoming messages
    let state_clone = state.clone();
    let conn_id_clone = conn_id.clone();
    tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                handle_ws_message(msg, &state_clone, &conn_id_clone).await;
            } else {
                break;
            }
        }

        // Clean up connection
        state_clone.connections.write().await.remove(&conn_id_clone);
    });

    // Send initial status
    let status = json!({
        "type": "connected",
        "connection_id": conn_id,
        "timestamp": Utc::now().to_rfc3339(),
    });

    let _ = sender.lock().await.send(Message::Text(status.to_string())).await;
}

async fn handle_ws_message(msg: Message, state: &Arc<ServerState>, conn_id: &str) {
    match msg {
        Message::Text(text) => {
            // Parse as JSON-RPC request
            if let Ok(request) = serde_json::from_str::<RpcRequest>(&text) {
                let response = process_rpc_request(request, state).await;
                send_ws_response(conn_id, response, state).await;
            }
        }
        Message::Binary(_) => {
            // Handle binary messages if needed
        }
        Message::Ping(data) => {
            // Auto-handled by axum
        }
        Message::Pong(_) => {
            // Auto-handled by axum
        }
        Message::Close(_) => {
            // Connection closing
        }
    }
}
```

## Connection Management

### Connection Pool

```rust
// core/osnova_lib/src/server/connections.rs

pub struct ConnectionPool {
    connections: HashMap<String, ClientConnection>,
    max_connections: usize,
}

pub struct ClientConnection {
    id: String,
    address: SocketAddr,
    authenticated: bool,
    user_id: Option<String>,
    websocket: Option<Arc<Mutex<SplitSink<WebSocket, Message>>>>,
    created_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        ConnectionPool {
            connections: HashMap::new(),
            max_connections,
        }
    }

    pub fn add(&mut self, id: String, ws: Arc<Mutex<SplitSink<WebSocket, Message>>>) -> Result<()> {
        if self.connections.len() >= self.max_connections {
            return Err("Maximum connections reached".into());
        }

        let conn = ClientConnection {
            id: id.clone(),
            address: SocketAddr::from(([127, 0, 0, 1], 0)), // Get from context
            authenticated: false,
            user_id: None,
            websocket: Some(ws),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };

        self.connections.insert(id, conn);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) {
        self.connections.remove(id);
    }

    pub fn get(&self, id: &str) -> Option<&ClientConnection> {
        self.connections.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ClientConnection> {
        self.connections.get_mut(id)
    }

    pub fn cleanup_inactive(&mut self, timeout: Duration) {
        let cutoff = Utc::now() - timeout;
        self.connections.retain(|_, conn| conn.last_activity > cutoff);
    }
}
```

## Authentication & Authorization

### JWT Authentication

```rust
// core/osnova_lib/src/server/auth.rs

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub device_id: String,  // Device identifier
}

pub struct AuthManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthManager {
    pub fn new(secret: &[u8]) -> Self {
        AuthManager {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn create_token(&self, user_id: &str, device_id: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
            device_id: device_id.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| e.into())
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| e.into())
    }
}

async fn authenticate_request(
    headers: &HeaderMap,
    state: &Arc<ServerState>,
) -> Result<String> {
    // Extract bearer token
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .ok_or("Missing authorization header")?;

    let auth_str = auth_header.to_str()
        .map_err(|_| "Invalid authorization header")?;

    if !auth_str.starts_with("Bearer ") {
        return Err("Invalid authorization format".into());
    }

    let token = &auth_str[7..];

    // Verify token
    let auth_manager = &state.auth_manager;
    let claims = auth_manager.verify_token(token)?;

    Ok(claims.sub)
}
```

## Rate Limiting

### Token Bucket Implementation

```rust
// core/osnova_lib/src/server/rate_limit.rs

use std::time::Instant;

pub struct RateLimiter {
    limits: HashMap<String, TokenBucket>,
}

pub struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_rate: u32,  // Tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        TokenBucket {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    pub fn try_consume(&mut self, tokens: u32) -> bool {
        self.refill();

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f32();
        let tokens_to_add = (elapsed * self.refill_rate as f32) as u32;

        self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
        self.last_refill = now;
    }
}

async fn check_rate_limit(
    headers: &HeaderMap,
    state: &Arc<ServerState>,
) -> Result<()> {
    // Get client identifier (IP or user ID)
    let client_id = get_client_id(headers)?;

    let mut limiters = state.rate_limiters.write().await;

    let bucket = limiters
        .entry(client_id)
        .or_insert_with(|| TokenBucket::new(100, 100)); // 100 req/sec

    if !bucket.try_consume(1) {
        return Err("Rate limit exceeded".into());
    }

    Ok(())
}
```

## TLS Configuration

### Loading TLS Certificates

```rust
// core/osnova_lib/src/server/tls.rs

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

fn load_tls_config(config: &TlsConfig) -> Result<ServerConfig> {
    // Load certificate
    let cert_file = std::fs::File::open(&config.cert_path)?;
    let mut cert_reader = std::io::BufReader::new(cert_file);
    let certs = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect::<Vec<_>>();

    // Load private key
    let key_file = std::fs::File::open(&config.key_path)?;
    let mut key_reader = std::io::BufReader::new(key_file);
    let keys = pkcs8_private_keys(&mut key_reader)?;

    if keys.is_empty() {
        return Err("No private key found".into());
    }

    let key = PrivateKey(keys[0].clone());

    // Build TLS config
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(config)
}
```

### Generating Self-Signed Certificates (Development)

```bash
#!/bin/bash
# Generate self-signed certificate for development

# Create private key
openssl genrsa -out server.key 2048

# Create certificate signing request
openssl req -new -key server.key -out server.csr \
    -subj "/C=US/ST=State/L=City/O=Osnova/CN=localhost"

# Generate self-signed certificate
openssl x509 -req -days 365 -in server.csr \
    -signkey server.key -out server.crt

# Convert to PKCS8 format
openssl pkcs8 -topk8 -inform PEM -outform PEM \
    -in server.key -out server.pk8 -nocrypt
```

## Health & Status Endpoints

### Health Check

```rust
async fn health_handler() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
    }))
}
```

### Status Endpoint

```rust
async fn status_handler(
    Extension(state): Extension<Arc<ServerState>>,
) -> impl IntoResponse {
    let connections = state.connections.read().await;
    let sessions = state.sessions.read().await;

    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": state.start_time.elapsed().as_secs(),
        "connections": connections.len(),
        "max_connections": state.config.max_clients,
        "active_sessions": sessions.count_active(),
        "mode": "server",
    }))
}
```

## Metrics Collection

### Prometheus Metrics

```rust
// core/osnova_lib/src/server/metrics.rs

use metrics::{counter, gauge, histogram};

pub struct MetricsCollector {
    // Track various metrics
}

impl MetricsCollector {
    pub fn new() -> Self {
        // Register metrics
        describe_counter!("rpc_requests_total", "Total number of RPC requests");
        describe_histogram!("rpc_request_duration", "RPC request duration");
        describe_gauge!("active_connections", "Number of active connections");

        MetricsCollector {}
    }

    pub fn record_rpc_request(&self, method: &str, duration: Duration) {
        counter!("rpc_requests_total", "method" => method.to_string()).increment(1);
        histogram!("rpc_request_duration", "method" => method.to_string())
            .record(duration.as_secs_f64());
    }

    pub fn set_active_connections(&self, count: usize) {
        gauge!("active_connections").set(count as f64);
    }
}

async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap()
}
```

## Configuration

### Server Configuration File

```toml
# config/server.toml

[server]
bind_address = "0.0.0.0:8080"
max_clients = 5

[tls]
enabled = true
cert_path = "/etc/osnova/certs/server.crt"
key_path = "/etc/osnova/certs/server.pk8"

[cors]
allowed_origins = [
    "http://localhost:3000",
    "https://app.osnova.io"
]

[auth]
jwt_secret = "your-secret-key-here"  # Use environment variable in production
token_expiry = 86400  # 24 hours

[rate_limit]
requests_per_second = 100
burst_size = 200

[logging]
level = "info"
format = "json"
file = "/var/log/osnova/server.log"
```

## Testing

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let config = ServerConfig::default();
        let server = OsnovaServer::new(config).unwrap();
        let app = server.build_router().await.unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_rpc_authentication() {
        let config = ServerConfig::default();
        let server = OsnovaServer::new(config).unwrap();
        let app = server.build_router().await.unwrap();

        // Request without auth header
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/rpc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"jsonrpc":"2.0","method":"apps.list","id":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
```

## Production Deployment

### systemd Service

```ini
# /etc/systemd/system/osnova-server.service

[Unit]
Description=Osnova Server
After=network.target

[Service]
Type=simple
User=osnova
Group=osnova
WorkingDirectory=/opt/osnova
Environment="RUST_LOG=info"
ExecStart=/opt/osnova/bin/osnova-server --config /etc/osnova/server.toml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Docker Container

```dockerfile
# Dockerfile.server

FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin osnova-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/osnova-server /usr/local/bin/
EXPOSE 8080
CMD ["osnova-server"]
```

## Security Considerations

1. **TLS Required**: Always use TLS in production
2. **Authentication**: All RPC calls require valid JWT
3. **Rate Limiting**: Prevent DoS attacks
4. **Input Validation**: Sanitize all inputs
5. **CORS**: Restrict to known origins
6. **Secrets Management**: Use environment variables or secret managers
7. **Audit Logging**: Log all authentication attempts and RPC calls

## Next Steps

1. Implement session persistence with Redis
2. Add horizontal scaling with load balancing
3. Implement circuit breakers for resilience
4. Add request tracing with OpenTelemetry
5. Create admin dashboard for monitoring