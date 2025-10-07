# Autonomi Network Operations

**Status**: Implemented (Phase 3)
**Last Updated**: 2025-10-07

## Overview

This document describes operations for interacting with the Autonomi Network for component storage and retrieval. The Autonomi Network provides decentralized, content-addressed storage for Osnova components.

## Connection Management

### Connect to Network

```rust
use osnova_lib::network::AutonomiClient;

// Local mode (for testing)
let client = AutonomiClient::connect().await?;

// Alphanet mode (for production)
let client = AutonomiClient::connect_alpha().await?;
```

### Health Check

```rust
let is_healthy = client.health_check().await?;
if is_healthy {
    println!("Network connection is healthy");
}
```

### Disconnect

```rust
client.disconnect().await?;
```

## Data Upload

### Upload Public Data

Uploads data to the Autonomi Network and returns an `ant://` URI for retrieval.

```rust
use osnova_lib::network::upload_data;

let client = AutonomiClient::connect().await?;
let data = b"Hello, Autonomi Network!";

let uri = upload_data(&client, data).await?;
println!("Uploaded to: {}", uri);
// Output: "Uploaded to: ant://0123456789abcdef..."
```

**Features:**
- Automatic chunking for files >1MB
- Content addressing (deterministic URIs)
- Self-encryption of data chunks
- Public accessibility (no encryption at network level)

**URI Format:**
```
ant://<hex-encoded-xor-name>
```

### Estimate Upload Cost

Calculate the cost of uploading data before performing the actual upload.

```rust
use osnova_lib::network::estimate_upload_cost;

let client = AutonomiClient::connect().await?;
let data = vec![0u8; 1024 * 1024]; // 1MB

let cost = estimate_upload_cost(&client, &data).await?;
println!("Upload will cost: {} AttoTokens", cost);
```

**Cost Factors:**
- Data size (after chunking)
- Number of chunks
- Network storage fees
- Redundancy requirements

## Data Download

### Download Public Data

Retrieve data from the Autonomi Network using an `ant://` URI.

```rust
use osnova_lib::network::download_data;

let client = AutonomiClient::connect().await?;
let uri = "ant://0123456789abcdef...";

let data = download_data(&client, uri).await?;
println!("Downloaded {} bytes", data.len());
```

**Features:**
- Automatic chunk reassembly for large files
- Content verification (hash checking)
- Progress tracking for large downloads
- Error handling for missing or corrupted data

## Error Handling

All operations return `Result<T, OsnovaError>`. Common error scenarios:

### Network Errors

```rust
match upload_data(&client, data).await {
    Ok(uri) => println!("Success: {}", uri),
    Err(OsnovaError::Network(msg)) => {
        eprintln!("Network error: {}", msg);
        // Handle network failure (retry, fallback, etc.)
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

**Common Network Errors:**
- `"Client not connected"` - Client was disconnected
- `"Failed to upload data: ..."` - Upload operation failed
- `"Failed to download data: ..."` - Download operation failed
- `"Failed to estimate cost: ..."` - Cost estimation failed

### Retry Strategy

For transient network failures, implement exponential backoff:

```rust
use tokio::time::{sleep, Duration};

let mut retries = 0;
let max_retries = 3;

loop {
    match upload_data(&client, data).await {
        Ok(uri) => {
            println!("Uploaded to: {}", uri);
            break;
        }
        Err(e) if retries < max_retries => {
            retries += 1;
            let delay = Duration::from_secs(2_u64.pow(retries));
            eprintln!("Upload failed, retrying in {:?}... ({}/{})", delay, retries, max_retries);
            sleep(delay).await;
        }
        Err(e) => {
            eprintln!("Upload failed after {} retries: {}", max_retries, e);
            return Err(e);
        }
    }
}
```

## Best Practices

### 1. Connection Pooling

Reuse client connections instead of creating new ones for each operation:

```rust
// Good: Reuse client
let client = AutonomiClient::connect_alpha().await?;
for data in datasets {
    let uri = upload_data(&client, data).await?;
    // ...
}

// Bad: Create new client each time
for data in datasets {
    let client = AutonomiClient::connect_alpha().await?;
    let uri = upload_data(&client, data).await?;
    // ...
}
```

### 2. Estimate Costs First

For user-facing operations, estimate costs before uploading:

```rust
let cost = estimate_upload_cost(&client, data).await?;
if user_confirms_cost(cost) {
    let uri = upload_data(&client, data).await?;
}
```

### 3. Validate Data Before Upload

Verify data integrity before uploading to avoid wasting network resources:

```rust
fn validate_component(data: &[u8]) -> bool {
    // Check magic bytes, size limits, format, etc.
    data.len() > 0 && data.len() < 100 * 1024 * 1024 // Max 100MB
}

if validate_component(data) {
    let uri = upload_data(&client, data).await?;
} else {
    return Err(OsnovaError::Other("Invalid component data".to_string()));
}
```

### 4. Cache URIs Locally

Store `ant://` URIs in local database to avoid redundant uploads:

```rust
if let Some(cached_uri) = db.get_component_uri(&component_id) {
    return Ok(cached_uri);
}

let uri = upload_data(&client, data).await?;
db.store_component_uri(&component_id, &uri)?;
Ok(uri)
```

## Implementation Details

### Chunking

Files larger than 1MB are automatically chunked using the Autonomi self-encryption algorithm:

1. Data is split into chunks of ~1MB
2. Each chunk is encrypted using XOR of adjacent chunks
3. Data map is created with chunk addresses
4. Data map is uploaded as root chunk
5. Root chunk address becomes the `ant://` URI

### Content Addressing

URIs are deterministic based on content:
- Same data always produces same URI
- Changes to data produce different URI
- Enables deduplication across network

### Payment

Currently uses `PaymentOption::Receipt(Receipt::default())` for test mode. Production deployment will require:
- EVM wallet integration
- Token balance management
- Payment confirmation

## See Also

- [Component ABI](../05-components/component-abi.md) - Component packaging format
- [Manifest Schema](manifest-schema.md) - Component manifest structure
- [Data Model](../02-architecture/data-model.md) - Data structures and relationships
