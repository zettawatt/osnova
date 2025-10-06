# osnova-autonomi (built‑in service)

Architecture Update (2025-10-03): This functionality is integrated into the Osnova shell as an in‑process Rust module. The OpenRPC methods listed below describe the service surface when exposed over RPC (stand‑alone or server mode); internally, equivalent Rust APIs are used. Packaging/manifests for this service are no longer required.

## Tokens

All downloads from the Autonomi network are free.
No token exchange is required for these operations.
Initial uploads for all autonomi operations require both Arbitrum ETH tokens to pay for gas and AUTONOMI tokens, which is an Arbitrum L2 ETH token used to pay for Autonomi network storage.
Updates to pointer and scratchpad types are free after the initial payment has been processed.

## Autonomi Data Types

The Autonomi network supports several core data types:

1. **Chunk**: Immutable 4MB data blocks (free to download, paid to upload)
2. **Pointer**: Mutable reference to a chunk address (paid initial upload, free updates)
3. **Scratchpad**: Mutable 4MB data storage (paid initial upload, free updates)
4. **GraphEntry**: Graph node with edges for linked data structures
5. **Archive**: Collection of files (Public or Private)
6. **Register**: Conflict-free replicated data type (CRDT)
7. **Vault**: User-specific encrypted storage

## Payment Integration

All upload operations require payment via the osnova-wallet component:
- **Arbitrum ETH**: For gas fees
- **AUTONOMI tokens**: For network storage costs

The component will call `wallet.requestPayment` before each paid operation.

## OpenRPC methods

The osnova-autonomi backend component provides the following OpenRPC methods:

### Client Management

##### `autonomi.client.connect`
Connect to the Autonomi network.

**Request**:
```json
{
  "method": "autonomi.client.connect",
  "params": {
    "testnet": false,
    "peers": []
  }
}
```

**Response**:
```json
{
  "result": {
    "connected": true,
    "networkType": "mainnet",
    "peerCount": 25
  }
}
```

##### `autonomi.client.disconnect`
Disconnect from the Autonomi network.

**Request**:
```json
{
  "method": "autonomi.client.disconnect",
  "params": {}
}
```

**Response**:
```json
{
  "result": {
    "disconnected": true
  }
}
```

### Chunk Operations (Immutable Data)

##### `autonomi.chunk.upload`
Upload immutable data as a chunk (max 4MB).

**Request**:
```json
{
  "method": "autonomi.chunk.upload",
  "params": {
    "data": "base64_encoded_data",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_chunk_address",
    "size": 1024,
    "cost": {
      "eth": "0.001",
      "autonomi": "10.0"
    },
    "transactionHash": "0x..."
  }
}
```

**Payment Flow**:
1. Estimate cost via Autonomi client
2. Request payment from wallet component
3. Upload chunk with payment proof
4. Return chunk address

##### `autonomi.chunk.download`
Download a chunk by address (free operation).

**Request**:
```json
{
  "method": "autonomi.chunk.download",
  "params": {
    "address": "base64_encoded_chunk_address"
  }
}
```

**Response**:
```json
{
  "result": {
    "data": "base64_encoded_data",
    "size": 1024,
    "address": "base64_encoded_chunk_address"
  }
}
```

### Pointer Operations (Mutable References)

##### `autonomi.pointer.create`
Create a new public pointer (paid operation).

**Request**:
```json
{
  "method": "autonomi.pointer.create",
  "params": {
    "target": "base64_encoded_chunk_address",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_pointer_address",
    "target": "base64_encoded_chunk_address",
    "secretKey": "hex_encoded_secret_key",
    "cost": {
      "eth": "0.001",
      "autonomi": "5.0"
    },
    "transactionHash": "0x..."
  }
}
```

**Notes**:
- Returns secret key for future updates
- Store secret key securely via osnova-core
- Initial creation requires payment

##### `autonomi.pointer.get`
Get the target of a pointer (free operation).

**Request**:
```json
{
  "method": "autonomi.pointer.get",
  "params": {
    "address": "base64_encoded_pointer_address"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_pointer_address",
    "target": "base64_encoded_chunk_address",
    "counter": 5
  }
}
```

##### `autonomi.pointer.update`
Update a pointer to a new target (free operation after initial creation).

**Request**:
```json
{
  "method": "autonomi.pointer.update",
  "params": {
    "address": "base64_encoded_pointer_address",
    "newTarget": "base64_encoded_new_chunk_address",
    "secretKey": "hex_encoded_secret_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_pointer_address",
    "newTarget": "base64_encoded_new_chunk_address",
    "counter": 6,
    "updated": true
  }
}
```

**Notes**: Updates are free but require the secret key from creation

### Scratchpad Operations (Mutable Data Storage)

##### `autonomi.scratchpad.create`
Create a new public scratchpad (paid operation, max 4MB).

**Request**:
```json
{
  "method": "autonomi.scratchpad.create",
  "params": {
    "data": "base64_encoded_data",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_scratchpad_address",
    "secretKey": "hex_encoded_secret_key",
    "size": 2048,
    "cost": {
      "eth": "0.002",
      "autonomi": "15.0"
    },
    "transactionHash": "0x..."
  }
}
```

**Notes**:
- Max 4MB data size
- Returns secret key for future updates
- Initial creation requires payment

##### `autonomi.scratchpad.get`
Get data from a scratchpad (free operation).

**Request**:
```json
{
  "method": "autonomi.scratchpad.get",
  "params": {
    "address": "base64_encoded_scratchpad_address"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_scratchpad_address",
    "data": "base64_encoded_data",
    "size": 2048,
    "counter": 3
  }
}
```

##### `autonomi.scratchpad.update`
Update scratchpad data (free operation after initial creation).

**Request**:
```json
{
  "method": "autonomi.scratchpad.update",
  "params": {
    "address": "base64_encoded_scratchpad_address",
    "data": "base64_encoded_new_data",
    "secretKey": "hex_encoded_secret_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "base64_encoded_scratchpad_address",
    "size": 2560,
    "counter": 4,
    "updated": true
  }
}
```

**Notes**: Updates are free but require the secret key from creation

### Archive Operations (File Collections)

##### `autonomi.archive.uploadPublic`
Upload a public archive (collection of files).

**Request**:
```json
{
  "method": "autonomi.archive.uploadPublic",
  "params": {
    "files": [
      {
        "path": "document.pdf",
        "data": "base64_encoded_file_data"
      },
      {
        "path": "image.png",
        "data": "base64_encoded_image_data"
      }
    ],
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "archiveAddress": "base64_encoded_archive_address",
    "fileCount": 2,
    "totalSize": 5242880,
    "cost": {
      "eth": "0.005",
      "autonomi": "50.0"
    },
    "transactionHash": "0x..."
  }
}
```

##### `autonomi.archive.uploadPrivate`
Upload a private (encrypted) archive.

**Request**:
```json
{
  "method": "autonomi.archive.uploadPrivate",
  "params": {
    "files": [
      {
        "path": "secret.txt",
        "data": "base64_encoded_file_data"
      }
    ],
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "archiveAddress": "base64_encoded_archive_address",
    "accessKey": "hex_encoded_access_key",
    "fileCount": 1,
    "totalSize": 1024,
    "cost": {
      "eth": "0.002",
      "autonomi": "20.0"
    },
    "transactionHash": "0x..."
  }
}
```

**Notes**: Returns access key required for downloading private archive

##### `autonomi.archive.download`
Download an archive (public or private).

**Request**:
```json
{
  "method": "autonomi.archive.download",
  "params": {
    "archiveAddress": "base64_encoded_archive_address",
    "accessKey": "hex_encoded_access_key_or_null"
  }
}
```

**Response**:
```json
{
  "result": {
    "archiveAddress": "base64_encoded_archive_address",
    "files": [
      {
        "path": "document.pdf",
        "data": "base64_encoded_file_data",
        "size": 2621440
      },
      {
        "path": "image.png",
        "data": "base64_encoded_image_data",
        "size": 2621440
      }
    ],
    "fileCount": 2,
    "totalSize": 5242880
  }
}
```

**Notes**: accessKey required for private archives, null for public archives

### Register Operations (CRDT)

##### `autonomi.register.create`
Create a new register (conflict-free replicated data type).

**Request**:
```json
{
  "method": "autonomi.register.create",
  "params": {
    "initialValue": "base64_encoded_initial_value",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "registerAddress": "base64_encoded_register_address",
    "secretKey": "hex_encoded_secret_key",
    "cost": {
      "eth": "0.001",
      "autonomi": "8.0"
    },
    "transactionHash": "0x..."
  }
}
```

##### `autonomi.register.get`
Get current value(s) from a register.

**Request**:
```json
{
  "method": "autonomi.register.get",
  "params": {
    "registerAddress": "base64_encoded_register_address"
  }
}
```

**Response**:
```json
{
  "result": {
    "registerAddress": "base64_encoded_register_address",
    "values": ["base64_encoded_value_1", "base64_encoded_value_2"],
    "version": 5
  }
}
```

**Notes**: May return multiple values if concurrent updates occurred

##### `autonomi.register.write`
Write a new value to a register.

**Request**:
```json
{
  "method": "autonomi.register.write",
  "params": {
    "registerAddress": "base64_encoded_register_address",
    "value": "base64_encoded_new_value",
    "secretKey": "hex_encoded_secret_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "registerAddress": "base64_encoded_register_address",
    "version": 6,
    "written": true
  }
}
```

### GraphEntry Operations (Linked Data)

##### `autonomi.graph.createEntry`
Create a graph entry with edges.

**Request**:
```json
{
  "method": "autonomi.graph.createEntry",
  "params": {
    "data": "base64_encoded_node_data",
    "edges": [
      {
        "label": "next",
        "target": "base64_encoded_target_address"
      }
    ],
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "entryAddress": "base64_encoded_entry_address",
    "edgeCount": 1,
    "cost": {
      "eth": "0.001",
      "autonomi": "7.0"
    },
    "transactionHash": "0x..."
  }
}
```

##### `autonomi.graph.getEntry`
Get a graph entry with its edges.

**Request**:
```json
{
  "method": "autonomi.graph.getEntry",
  "params": {
    "entryAddress": "base64_encoded_entry_address"
  }
}
```

**Response**:
```json
{
  "result": {
    "entryAddress": "base64_encoded_entry_address",
    "data": "base64_encoded_node_data",
    "edges": [
      {
        "label": "next",
        "target": "base64_encoded_target_address"
      }
    ]
  }
}
```

### Vault Operations (User Storage)

##### `autonomi.vault.create`
Create a user vault for encrypted storage.

**Request**:
```json
{
  "method": "autonomi.vault.create",
  "params": {
    "secretKey": "hex_encoded_user_secret_key",
    "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "vaultAddress": "base64_encoded_vault_address",
    "cost": {
      "eth": "0.002",
      "autonomi": "12.0"
    },
    "transactionHash": "0x..."
  }
}
```

##### `autonomi.vault.put`
Store data in a vault.

**Request**:
```json
{
  "method": "autonomi.vault.put",
  "params": {
    "vaultAddress": "base64_encoded_vault_address",
    "key": "user_data_key",
    "value": "base64_encoded_value",
    "secretKey": "hex_encoded_user_secret_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "vaultAddress": "base64_encoded_vault_address",
    "key": "user_data_key",
    "stored": true
  }
}
```

##### `autonomi.vault.get`
Retrieve data from a vault.

**Request**:
```json
{
  "method": "autonomi.vault.get",
  "params": {
    "vaultAddress": "base64_encoded_vault_address",
    "key": "user_data_key",
    "secretKey": "hex_encoded_user_secret_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "vaultAddress": "base64_encoded_vault_address",
    "key": "user_data_key",
    "value": "base64_encoded_value"
  }
}
```

## Integration with osnova-wallet

All paid operations follow this flow:

1. **Estimate Cost**: Calculate ETH and AUTONOMI token costs
2. **Request Payment**: Call `wallet.requestPayment` with:
   - Component ID: `com.osnova.autonomi`
   - Amount and token details
   - Purpose description (e.g., "Upload 2MB file to Autonomi")
3. **User Approval**: Wallet displays payment dialog
4. **Execute Operation**: On approval, perform Autonomi operation with payment
5. **Return Result**: Include transaction hash and addresses

### Example Payment Flow

```javascript
// 1. Estimate cost
const cost = await estimateUploadCost(dataSize);

// 2. Request payment from wallet
const payment = await openrpc.call("wallet.requestPayment", {
  componentId: "com.osnova.autonomi",
  fromAddress: userWalletAddress,
  toAddress: autonomiPaymentAddress,
  amount: cost.autonomi,
  network: "arbitrum",
  token: autonomiTokenAddress,
  purpose: `Upload ${dataSize}MB to Autonomi network`,
  metadata: {
    uploadSize: `${dataSize}MB`,
    estimatedCost: `${cost.autonomi} AUTONOMI + ${cost.eth} ETH`
  }
});

// 3. Upload with payment proof
const result = await autonomiClient.upload(data, payment.privateKey);
```

## Integration with osnova-core

The autonomi component uses osnova-core for:
- **Secret Key Storage**: Store pointer/scratchpad secret keys encrypted
- **Access Key Storage**: Store private archive access keys
- **Component Key Derivation**: Use component ID `com.osnova.autonomi`

## Error Codes

- `-50000`: Network connection error
- `-50001`: Upload failed
- `-50002`: Download failed
- `-50003`: Payment required
- `-50004`: Payment failed
- `-50005`: Insufficient balance
- `-50006`: Invalid address
- `-50007`: Data too large (>4MB for chunk/scratchpad)
- `-50008`: Secret key required
- `-50009`: Access key required
- `-50010`: Pointer/Scratchpad not found

## Security Considerations

1. **Secret Key Protection**: All secret keys stored encrypted via osnova-core
2. **Payment Authorization**: All uploads require explicit user approval
3. **Access Control**: Private archives require access keys
4. **Data Encryption**: Private data encrypted before upload
5. **Immutability**: Chunks are immutable; use pointers for mutable references

## MVP Implementation Notes

1. **Autonomi Client**: Use autonomi crate v0.6.1
2. **Payment Integration**: Integrate with osnova-wallet for all paid operations
3. **Key Management**: Store all secret/access keys via osnova-core
4. **Error Handling**: Comprehensive error types with user-friendly messages
5. **Progress Tracking**: Emit progress events for large uploads/downloads
6. **Testnet Support**: Support testnet flag for development/testing

## Post-MVP Enhancements

- Batch upload/download operations
- Resume interrupted uploads
- Automatic chunking for files >4MB
- Content addressing and deduplication
- Caching layer for frequently accessed data
- Bandwidth optimization
- Multi-part uploads for large archives
- Compression before upload

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
