# osnova-saorsa backend component

This component is used to interact with the saorsa DHT and network.

**MVP Status**: This component is **REQUIRED for MVP** as it provides the foundational identity and networking layer.

This component should always be running.

## Overview

The osnova-saorsa component provides OpenRPC access to saorsa-core's decentralized identity, presence, messaging, and storage capabilities. It serves as the bridge between Osnova applications and the saorsa-core P2P networking layer.

### Key Features

- **Four-Word Identity**: Human-readable addresses with ML-DSA post-quantum signatures
- **Multi-Device Presence**: Support for multiple devices per identity
- **P2P Messaging**: Direct and group messaging with end-to-end encryption
- **Virtual Disks**: Private and public (website) storage with FEC protection
- **DHT Storage**: Trust-weighted Kademlia distributed hash table
- **Real-Time Media**: Audio/video calling with WebRTC-over-QUIC
- **Group Management**: Create and manage groups with membership control

## Core Concepts

### Identity
- **Four-Word Address**: Human-readable identifiers (e.g., "river-spark-honest-lion")
- **ML-DSA Keypair**: Post-quantum digital signatures (ML-DSA-65)
- **Identity Key**: `blake3(utf8(join(words,'-')))` → 32 bytes

### Presence
- **Multi-Device Support**: Users can register multiple devices
- **Device Types**: Active (user machines) vs Headless (storage nodes)
- **Presence Packets**: DHT-stored signed packets with device lists

### Storage
- **Automatic FEC Strategy**: Based on group size (1=direct, 2=replication, 3-5=FEC(3,2), etc.)
- **saorsa-seal**: Encryption for data at rest
- **saorsa-fec**: Forward error correction for redundancy

### OpenRPC methods

The osnova-saorsa backend component provides the following OpenRPC methods:

## Identity Management

##### `saorsa.identity.register`
Register a new identity with four-word address and ML-DSA keypair.

**Request**:
```json
{
  "method": "saorsa.identity.register",
  "params": {
    "words": ["welfare", "absurd", "king", "ridge"],
    "publicKey": "base64_encoded_ml_dsa_public_key",
    "signature": "base64_encoded_ml_dsa_signature"
  }
}
```

**Response**:
```json
{
  "result": {
    "identityHandle": "opaque_handle_string",
    "identityKey": "base64_encoded_32_byte_key",
    "words": ["welfare", "absurd", "king", "ridge"]
  }
}
```

**Notes**:
- Signature must be over `utf8(join(words,'-'))` using the provided public key
- Words are validated against FWN dictionary
- Identity key is `blake3(utf8(join(words,'-')))`

##### `saorsa.identity.get`
Retrieve an existing identity by key.

**Request**:
```json
{
  "method": "saorsa.identity.get",
  "params": {
    "key": "base64_encoded_identity_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "words": ["welfare", "absurd", "king", "ridge"],
    "identityKey": "base64_encoded_32_byte_key",
    "publicKey": "base64_encoded_ml_dsa_public_key",
    "endpoints": [
      {
        "ipv4": "203.0.113.10:443",
        "ipv6": null,
        "fw4": "word1-word2-word3-word4",
        "fw6": null
      }
    ],
    "websiteRoot": "base64_encoded_key_or_null",
    "deviceSetRoot": "base64_encoded_key"
  }
}
```

##### `saorsa.identity.publishEndpoints`
Publish network endpoints for an identity with signature.

**Request**:
```json
{
  "method": "saorsa.identity.publishEndpoints",
  "params": {
    "identityKey": "base64_encoded_identity_key",
    "endpoints": [
      {
        "ipv4": "203.0.113.10:443",
        "ipv6": null
      }
    ],
    "endpointSignature": "base64_encoded_signature"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "publishedAt": 1696214400
  }
}
```

**Notes**: Signature must be over `(id || pk || CBOR(endpoints))`

##### `saorsa.identity.setWebsiteRoot`
Set the website root for an identity.

**Request**:
```json
{
  "method": "saorsa.identity.setWebsiteRoot",
  "params": {
    "identityKey": "base64_encoded_identity_key",
    "websiteRoot": "base64_encoded_website_root_key",
    "signature": "base64_encoded_signature"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "websiteRoot": "base64_encoded_website_root_key"
  }
}
```

**Notes**: Canonical signing message: `b"saorsa-identity:website_root:v1" || id || pk || CBOR(website_root)`

## Device & Presence Management

##### `saorsa.device.publishForward`
Publish or update a device forward for an identity.

**Request**:
```json
{
  "method": "saorsa.device.publishForward",
  "params": {
    "identityKey": "base64_encoded_identity_key",
    "forward": {
      "protocol": "quic",
      "address": "203.0.113.10:443",
      "metadata": {}
    },
    "signature": "base64_encoded_signature"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "deviceSetRoot": "base64_encoded_device_set_root"
  }
}
```

##### `saorsa.device.subscribe`
Subscribe to device forward updates for an identity.

**Request**:
```json
{
  "method": "saorsa.device.subscribe",
  "params": {
    "identityKey": "base64_encoded_identity_key"
  }
}
```

**Response** (subscription stream):
```json
{
  "result": {
    "subscriptionId": "unique_subscription_id",
    "forward": {
      "protocol": "quic",
      "address": "203.0.113.10:443",
      "metadata": {},
      "timestamp": 1696214400
    }
  }
}
```

##### `saorsa.presence.register`
Register devices and mark active device for an identity.

**Request**:
```json
{
  "method": "saorsa.presence.register",
  "params": {
    "identityHandle": "opaque_handle_string",
    "devices": [
      {
        "id": "device_id_1",
        "deviceType": "Active",
        "storageGb": 100,
        "endpoint": {
          "protocol": "quic",
          "address": "203.0.113.10:443"
        },
        "capabilities": {
          "storage": true,
          "relay": true
        }
      }
    ],
    "activeDevice": "device_id_1"
  }
}
```

**Response**:
```json
{
  "result": {
    "presenceReceipt": {
      "timestamp": 1696214400,
      "deviceCount": 1,
      "activeDevice": "device_id_1"
    }
  }
}
```

## Group Management

##### `saorsa.group.create`
Create a new group identity with members.

**Request**:
```json
{
  "method": "saorsa.group.create",
  "params": {
    "words": ["team", "alpha", "secure", "chat"],
    "members": [
      {
        "memberId": "base64_encoded_member_id",
        "memberPublicKey": "base64_encoded_member_pk"
      }
    ]
  }
}
```

**Response**:
```json
{
  "result": {
    "groupId": "base64_encoded_group_id",
    "words": ["team", "alpha", "secure", "chat"],
    "groupPublicKey": "base64_encoded_group_pk",
    "groupSignature": "base64_encoded_group_sig",
    "membershipRoot": "base64_encoded_membership_root"
  }
}
```

##### `saorsa.group.publish`
Publish a group identity to the DHT.

**Request**:
```json
{
  "method": "saorsa.group.publish",
  "params": {
    "groupId": "base64_encoded_group_id",
    "words": ["team", "alpha", "secure", "chat"],
    "groupPublicKey": "base64_encoded_group_pk",
    "groupSignature": "base64_encoded_group_sig",
    "members": [
      {
        "memberId": "base64_encoded_member_id",
        "memberPublicKey": "base64_encoded_member_pk"
      }
    ],
    "membershipRoot": "base64_encoded_membership_root"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "publishedAt": 1696214400
  }
}
```

##### `saorsa.group.get`
Retrieve a group identity by ID.

**Request**:
```json
{
  "method": "saorsa.group.get",
  "params": {
    "groupId": "base64_encoded_group_id"
  }
}
```

**Response**:
```json
{
  "result": {
    "groupId": "base64_encoded_group_id",
    "words": ["team", "alpha", "secure", "chat"],
    "groupPublicKey": "base64_encoded_group_pk",
    "members": [
      {
        "memberId": "base64_encoded_member_id",
        "memberPublicKey": "base64_encoded_member_pk"
      }
    ],
    "membershipRoot": "base64_encoded_membership_root",
    "createdAt": 1696214400
  }
}
```

##### `saorsa.group.addMember`
Add a member to an existing group.

**Request**:
```json
{
  "method": "saorsa.group.addMember",
  "params": {
    "groupId": "base64_encoded_group_id",
    "member": {
      "memberId": "base64_encoded_new_member_id",
      "memberPublicKey": "base64_encoded_new_member_pk"
    },
    "groupPublicKey": "base64_encoded_group_pk",
    "groupSignature": "base64_encoded_group_sig"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "newMembershipRoot": "base64_encoded_new_membership_root"
  }
}
```

##### `saorsa.group.removeMember`
Remove a member from a group.

**Request**:
```json
{
  "method": "saorsa.group.removeMember",
  "params": {
    "groupId": "base64_encoded_group_id",
    "memberId": "base64_encoded_member_to_remove",
    "groupPublicKey": "base64_encoded_group_pk",
    "groupSignature": "base64_encoded_group_sig"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "newMembershipRoot": "base64_encoded_new_membership_root"
  }
}
```

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
## DHT Operations

##### `saorsa.dht.put`
Store data in the DHT with policy.

**Request**:
```json
{
  "method": "saorsa.dht.put",
  "params": {
    "key": "base64_encoded_32_byte_key",
    "data": "base64_encoded_data",
    "policy": {
      "quorum": 3,
      "ttl": 3600,
      "authType": "Identity"
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "key": "base64_encoded_32_byte_key",
    "timestamp": 1696214400,
    "storingNodes": ["node_id_1", "node_id_2", "node_id_3"]
  }
}
```

##### `saorsa.dht.get`
Retrieve data from the DHT.

**Request**:
```json
{
  "method": "saorsa.dht.get",
  "params": {
    "key": "base64_encoded_32_byte_key",
    "quorum": 2
  }
}
```

**Response**:
```json
{
  "result": {
    "key": "base64_encoded_32_byte_key",
    "data": "base64_encoded_data",
    "timestamp": 1696214400
  }
}
```

##### `saorsa.dht.watch`
Subscribe to updates for a DHT key.

**Request**:
```json
{
  "method": "saorsa.dht.watch",
  "params": {
    "key": "base64_encoded_32_byte_key"
  }
}
```

**Response** (subscription stream):
```json
{
  "result": {
    "subscriptionId": "unique_subscription_id",
    "key": "base64_encoded_32_byte_key",
    "data": "base64_encoded_data",
    "timestamp": 1696214400
  }
}
```

## Messaging

##### `saorsa.messaging.send`
Send an encrypted message to recipients.

**Request**:
```json
{
  "method": "saorsa.messaging.send",
  "params": {
    "recipients": ["river-spark-honest-lion", "ocean-bright-calm-tiger"],
    "content": {
      "type": "text",
      "text": "Hello, world!",
      "attachments": []
    },
    "channelId": "channel_id_string",
    "options": {
      "priority": "normal",
      "requireDeliveryReceipt": true
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "messageId": "unique_message_id",
    "deliveryReceipt": {
      "sent": true,
      "timestamp": 1696214400,
      "recipientCount": 2
    }
  }
}
```

##### `saorsa.messaging.subscribe`
Subscribe to incoming messages.

**Request**:
```json
{
  "method": "saorsa.messaging.subscribe",
  "params": {
    "channelFilter": "channel_id_or_null"
  }
}
```

**Response** (subscription stream):
```json
{
  "result": {
    "subscriptionId": "unique_subscription_id",
    "message": {
      "messageId": "unique_message_id",
      "from": "river-spark-honest-lion",
      "content": {
        "type": "text",
        "text": "Hello, world!"
      },
      "channelId": "channel_id_string",
      "timestamp": 1696214400
    }
  }
}
```

##### `saorsa.messaging.getStatus`
Get delivery status for a message.

**Request**:
```json
{
  "method": "saorsa.messaging.getStatus",
  "params": {
    "messageId": "unique_message_id"
  }
}
```

**Response**:
```json
{
  "result": {
    "messageId": "unique_message_id",
    "status": "delivered",
    "deliveredTo": ["river-spark-honest-lion"],
    "failedTo": [],
    "timestamp": 1696214400
  }
}
```

## Storage & Data Management

##### `saorsa.storage.store`
Store data with automatic FEC strategy selection.

**Request**:
```json
{
  "method": "saorsa.storage.store",
  "params": {
    "identityHandle": "opaque_handle_string",
    "data": "base64_encoded_data",
    "groupSize": 5
  }
}
```

**Response**:
```json
{
  "result": {
    "storageHandle": "opaque_storage_handle",
    "fecStrategy": "FEC(3,2)",
    "shardCount": 5,
    "timestamp": 1696214400
  }
}
```

**Notes**: FEC strategy automatically selected based on group size:
- 1: Direct storage
- 2: Full replication
- 3-5: FEC(3,2)
- 6-10: FEC(4,3)
- 11-20: FEC(6,4)
- 20+: FEC(8,5)

##### `saorsa.storage.retrieve`
Retrieve stored data.

**Request**:
```json
{
  "method": "saorsa.storage.retrieve",
  "params": {
    "storageHandle": "opaque_storage_handle"
  }
}
```

**Response**:
```json
{
  "result": {
    "data": "base64_encoded_data",
    "fecStrategy": "FEC(3,2)",
    "retrievedAt": 1696214400
  }
}
```

##### `saorsa.storage.placeShards`
Get optimal shard placement for an object.

**Request**:
```json
{
  "method": "saorsa.storage.placeShards",
  "params": {
    "objectId": "base64_encoded_32_byte_object_id",
    "shardCount": 5
  }
}
```

**Response**:
```json
{
  "result": {
    "objectId": "base64_encoded_32_byte_object_id",
    "placements": [
      {
        "shardIndex": 0,
        "nodeId": "base64_encoded_node_id",
        "trustScore": 0.95
      }
    ]
  }
}
```

## Virtual Disk Operations

##### `saorsa.disk.create`
Create a virtual disk for an entity.

**Request**:
```json
{
  "method": "saorsa.disk.create",
  "params": {
    "entityId": "base64_encoded_entity_id",
    "diskType": "Private",
    "config": {
      "fecParams": {
        "k": 3,
        "m": 2,
        "shardSize": 65536
      },
      "encryption": {
        "algorithm": "ChaCha20-Poly1305"
      }
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "diskHandle": "opaque_disk_handle",
    "diskRoot": "base64_encoded_disk_root_key",
    "diskType": "Private"
  }
}
```

**Notes**: `diskType` can be "Private" or "Website"

##### `saorsa.disk.write`
Write a file to a virtual disk.

**Request**:
```json
{
  "method": "saorsa.disk.write",
  "params": {
    "diskHandle": "opaque_disk_handle",
    "path": "/documents/hello.txt",
    "content": "base64_encoded_file_content",
    "metadata": {
      "mimeType": "text/plain",
      "permissions": "rw-r--r--",
      "tags": ["important"]
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "path": "/documents/hello.txt",
    "objectKey": "base64_encoded_object_key",
    "size": 1024,
    "writtenAt": 1696214400
  }
}
```

##### `saorsa.disk.read`
Read a file from a virtual disk.

**Request**:
```json
{
  "method": "saorsa.disk.read",
  "params": {
    "diskHandle": "opaque_disk_handle",
    "path": "/documents/hello.txt"
  }
}
```

**Response**:
```json
{
  "result": {
    "path": "/documents/hello.txt",
    "content": "base64_encoded_file_content",
    "metadata": {
      "mimeType": "text/plain",
      "size": 1024,
      "modifiedAt": 1696214400
    }
  }
}
```

##### `saorsa.disk.list`
List files in a virtual disk directory.

**Request**:
```json
{
  "method": "saorsa.disk.list",
  "params": {
    "diskHandle": "opaque_disk_handle",
    "path": "/documents",
    "recursive": false
  }
}
```

**Response**:
```json
{
  "result": {
    "path": "/documents",
    "entries": [
      {
        "path": "/documents/hello.txt",
        "fileType": "file",
        "size": 1024,
        "modifiedAt": 1696214400,
        "permissions": "rw-r--r--"
      },
      {
        "path": "/documents/subfolder",
        "fileType": "directory",
        "modifiedAt": 1696214400
      }
    ]
  }
}
```

##### `saorsa.website.setHome`
Set the home page for a website disk.

**Request**:
```json
{
  "method": "saorsa.website.setHome",
  "params": {
    "diskHandle": "opaque_disk_handle",
    "markdownContent": "# Welcome\n\nThis is my website.",
    "assets": [
      {
        "path": "style.css",
        "content": "base64_encoded_css",
        "mimeType": "text/css"
      }
    ]
  }
}
```

**Response**:
```json
{
  "result": {
    "homeKey": "base64_encoded_home_md_key",
    "assetKeys": {
      "style.css": "base64_encoded_css_key"
    }
  }
}
```

##### `saorsa.website.publish`
Publish a website to make it publicly accessible.

**Request**:
```json
{
  "method": "saorsa.website.publish",
  "params": {
    "entityId": "base64_encoded_entity_id",
    "websiteRoot": "base64_encoded_website_root_key"
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "websiteRoot": "base64_encoded_website_root_key",
    "publishedAt": 1696214400,
    "url": "river-spark-honest-lion"
  }
}
```

## Real-Time Media (Audio/Video Calls)

##### `saorsa.call.initiate`
Initiate an audio/video call with a recipient.

**Request**:
```json
{
  "method": "saorsa.call.initiate",
  "params": {
    "recipient": "river-spark-honest-lion",
    "mediaConfig": {
      "audio": true,
      "video": true,
      "screenShare": false,
      "quality": "High"
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "callHandle": "opaque_call_handle",
    "callId": "unique_call_id",
    "status": "ringing"
  }
}
```

##### `saorsa.call.answer`
Answer an incoming call.

**Request**:
```json
{
  "method": "saorsa.call.answer",
  "params": {
    "callId": "unique_call_id",
    "mediaConfig": {
      "audio": true,
      "video": true,
      "screenShare": false,
      "quality": "Auto"
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "callHandle": "opaque_call_handle",
    "status": "connected"
  }
}
```

##### `saorsa.call.hangup`
End an active call.

**Request**:
```json
{
  "method": "saorsa.call.hangup",
  "params": {
    "callHandle": "opaque_call_handle"
  }
}
```

**Response**:
```json
{
  "result": {
    "callSummary": {
      "duration": 300,
      "qualityMetrics": {
        "avgLatency": 45,
        "packetLoss": 0.01
      },
      "bytesTransferred": 15728640
    }
  }
}
```

##### `saorsa.call.toggleMedia`
Toggle audio, video, or screen share during a call.

**Request**:
```json
{
  "method": "saorsa.call.toggleMedia",
  "params": {
    "callHandle": "opaque_call_handle",
    "mediaType": "audio",
    "enabled": false
  }
}
```

**Response**:
```json
{
  "result": {
    "success": true,
    "mediaType": "audio",
    "enabled": false
  }
}
```

**Notes**: `mediaType` can be "audio", "video", or "screenShare"

##### `saorsa.groupCall.create`
Create a group call within an existing group.

**Request**:
```json
{
  "method": "saorsa.groupCall.create",
  "params": {
    "groupId": "base64_encoded_group_id",
    "mediaConfig": {
      "audio": true,
      "video": true,
      "screenShare": false,
      "quality": "Auto"
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "groupCallHandle": "opaque_group_call_handle",
    "groupCallId": "unique_group_call_id",
    "invitedCount": 5
  }
}
```

## Transport Operations

##### `saorsa.transport.connect`
Establish a QUIC connection to an endpoint.

**Request**:
```json
{
  "method": "saorsa.transport.connect",
  "params": {
    "endpoint": {
      "address": "203.0.113.10:443"
    }
  }
}
```

**Response**:
```json
{
  "result": {
    "connectionId": "unique_connection_id",
    "peerId": "base64_encoded_peer_id",
    "connected": true
  }
}
```

##### `saorsa.transport.openStream`
Open a typed stream on an existing connection.

**Request**:
```json
{
  "method": "saorsa.transport.openStream",
  "params": {
    "connectionId": "unique_connection_id",
    "streamClass": "File"
  }
}
```

**Response**:
```json
{
  "result": {
    "streamId": "unique_stream_id",
    "streamClass": "File"
  }
}
```

**Notes**: `streamClass` can be "Control", "Mls", "File", or "Media"

## Integration with Other Components

### Integration with osnova-core

The saorsa component uses osnova-core for:
- **Key Derivation**: ML-DSA keypair derivation from master key
- **Key Storage**: Encrypted storage of identity keys and group keys
- **Component ID**: Uses `com.osnova.saorsa` for key derivation

### Integration with Autonomi

The saorsa component provides the identity layer for Autonomi operations:
- Four-word addresses used for Autonomi network identity
- DHT operations can complement Autonomi storage
- Virtual disks can reference Autonomi-stored content

### Usage in Osnova Applications

Applications use saorsa for:
1. **User Identity**: Four-word address registration and management
2. **P2P Communication**: Direct messaging and calls between users
3. **Group Collaboration**: Group creation and management
4. **Decentralized Storage**: Virtual disks for private and public content
5. **Website Publishing**: Markdown-based websites accessible via four-word addresses

## Error Codes

- `-40000`: Invalid four-word address
- `-40001`: Signature verification failed
- `-40002`: Identity not found
- `-40003`: Group not found
- `-40004`: Device not found
- `-40005`: DHT operation failed
- `-40006`: Storage operation failed
- `-40007`: Messaging operation failed
- `-40008`: Call operation failed
- `-40009`: Insufficient permissions
- `-40010`: Network error

## Security Considerations

1. **Post-Quantum Cryptography**: ML-DSA-65 for signatures
2. **End-to-End Encryption**: All messages and calls encrypted
3. **Trust-Weighted DHT**: Reputation-based node selection
4. **FEC Protection**: Forward error correction for data redundancy
5. **Canonical Signing**: Prevents signature malleability
6. **Four-Word Validation**: Anti-phishing through constrained dictionary

## MVP Implementation Notes

1. **saorsa-core Integration**: Use saorsa-core Rust library directly
2. **OpenRPC Bridge**: Implement OpenRPC server wrapping saorsa-core API
3. **Async Operations**: All operations are async (tokio runtime)
4. **Error Handling**: Comprehensive error types with structured logging
5. **Subscription Management**: WebSocket-based subscriptions for watch/subscribe operations
6. **Key Management**: Integrate with osnova-core for key derivation and storage

## Post-MVP Enhancements

- MLS (Message Layer Security) for group messaging
- Advanced FEC strategies for large files
- Friend mesh backup for enhanced redundancy
- WebRTC data channels for file transfer
- Advanced call features (recording, transcription)
- Virtual disk snapshots and versioning
- Collaborative editing with CRDT support

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
```

