# osnova-wallet backend component

This component interacts with osnova-core to handle crypto wallet functionality for osnova apps.
It is also used to interact with the osnova server in the case of the client-server model.

**MVP Status**: This component is **REQUIRED for MVP** as it provides payment functionality for Autonomi network uploads.

This component should always be running.

## Overview

The osnova-wallet component provides Ethereum-compatible wallet functionality with support for ETH and ERC-20 tokens on multiple networks. It is specifically designed to support payments for Autonomi network operations using Arbitrum ETH and AUTONOMI tokens.

### Key Features

- **Ethereum Wallet Management**: Create and manage Ethereum wallets derived from the master key
- **Multi-Network Support**: Support for Ethereum mainnet, Arbitrum, and other L2 networks
- **Token Support**: Native ETH and ERC-20 tokens (specifically AUTONOMI token)
- **Key Derivation**: BIP-44/BIP-32 compliant key derivation from master key via osnova-core
- **Payment Authorization**: Component-based payment authorization system
- **Balance Tracking**: Real-time balance information for ETH and tokens
- **Import Support**: Import external private keys outside the derivation chain
- **User Consent**: All payments require explicit user approval via UI dialog

## Key Derivation

The wallet uses standard Ethereum key derivation (BIP-44) from the master key stored in osnova-core:

```
Derivation Path: m/44'/60'/0'/0/{index}
- 44' = BIP-44 purpose
- 60' = Ethereum coin type
- 0' = Account 0
- 0 = External chain (receiving addresses)
- {index} = Address index (0, 1, 2, ...)
```

Keys are derived via osnova-core's `keys.deriveAtIndex` method with component ID `com.osnova.wallet` and the appropriate index.

## Payment Authorization Flow

1. Backend component requests payment via `wallet.requestPayment`
2. If component is not linked, request is rejected
3. Wallet displays payment dialog in shell UI with:
   - Requesting component name
   - Payment amount and token
   - Current balance
   - Gas estimate
   - Accept/Reject buttons
4. User reviews and approves/rejects
5. On approval, wallet signs and broadcasts transaction
6. Transaction hash returned to requesting component

## Supported Networks (MVP)

- **Arbitrum One** (Chain ID: 42161) - Primary network for AUTONOMI token
- **Ethereum Mainnet** (Chain ID: 1) - For ETH operations

Post-MVP: Additional L2 networks (Optimism, Base, etc.)

## Supported Tokens (MVP)

- **ETH** - Native Ether on all supported networks
- **AUTONOMI** - ERC-20 token on Arbitrum (contract address TBD)

Post-MVP: Additional ERC-20 tokens

### OpenRPC methods

The osnova-wallet backend component provides the following OpenRPC methods:

#### Wallet Management

##### `wallet.create`
Create a new wallet address derived from the master key at the next available index.

**Request**:
```json
{
  "method": "wallet.create",
  "params": {
    "label": "My Wallet" // Optional user-friendly label
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "index": 0,
    "derivationPath": "m/44'/60'/0'/0/0",
    "label": "My Wallet"
  }
}
```

##### `wallet.list`
List all wallet addresses managed by this component.

**Request**:
```json
{
  "method": "wallet.list",
  "params": {}
}
```

**Response**:
```json
{
  "result": {
    "wallets": [
      {
        "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        "index": 0,
        "derivationPath": "m/44'/60'/0'/0/0",
        "label": "My Wallet",
        "isImported": false
      },
      {
        "address": "0x8ba1f109551bD432803012645Ac136ddd64DBA72",
        "index": null,
        "derivationPath": null,
        "label": "Imported Wallet",
        "isImported": true
      }
    ]
  }
}
```

##### `wallet.import`
Import an external private key outside the derivation chain.

**Request**:
```json
{
  "method": "wallet.import",
  "params": {
    "privateKey": "0x...", // Hex-encoded private key
    "label": "Imported Wallet" // Optional label
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "0x8ba1f109551bD432803012645Ac136ddd64DBA72",
    "label": "Imported Wallet",
    "isImported": true
  }
}
```

**Security**: Private key is stored encrypted in osnova-core keystore with special marker indicating it's imported.

##### `wallet.export`
Export a wallet's private key (requires user confirmation).

**Request**:
```json
{
  "method": "wallet.export",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "privateKey": "0x...",
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Security**: Triggers user confirmation dialog before exporting.

#### Balance Operations

##### `wallet.getBalance`
Get balance for a wallet address on a specific network.

**Request**:
```json
{
  "method": "wallet.getBalance",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "network": "arbitrum", // "ethereum" | "arbitrum"
    "token": null // null for ETH, or token contract address for ERC-20
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "network": "arbitrum",
    "token": null,
    "balance": "1.5", // Human-readable balance
    "balanceWei": "1500000000000000000", // Wei/smallest unit
    "symbol": "ETH",
    "decimals": 18
  }
}
```

##### `wallet.getBalances`
Get all balances for a wallet address across all supported networks and tokens.

**Request**:
```json
{
  "method": "wallet.getBalances",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  }
}
```

**Response**:
```json
{
  "result": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "balances": [
      {
        "network": "ethereum",
        "token": null,
        "balance": "0.5",
        "balanceWei": "500000000000000000",
        "symbol": "ETH",
        "decimals": 18
      },
      {
        "network": "arbitrum",
        "token": null,
        "balance": "1.5",
        "balanceWei": "1500000000000000000",
        "symbol": "ETH",
        "decimals": 18
      },
      {
        "network": "arbitrum",
        "token": "0x...AUTONOMI_CONTRACT_ADDRESS",
        "balance": "1000.0",
        "balanceWei": "1000000000000000000000",
        "symbol": "AUTONOMI",
        "decimals": 18
      }
    ]
  }
}
```

#### Payment Operations

##### `wallet.requestPayment`
Request payment from the wallet (called by other backend components).

**Request**:
```json
{
  "method": "wallet.requestPayment",
  "params": {
    "componentId": "com.osnova.autonomi",
    "fromAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "toAddress": "0x...", // Recipient address
    "amount": "0.1", // Human-readable amount
    "network": "arbitrum",
    "token": null, // null for ETH, or token contract address
    "purpose": "Upload 10MB to Autonomi network", // User-visible description
    "metadata": {
      "uploadSize": "10MB",
      "estimatedCost": "0.1 ETH"
    }
  }
}
```

**Response** (after user approval):
```json
{
  "result": {
    "approved": true,
    "transactionHash": "0x...",
    "fromAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "toAddress": "0x...",
    "amount": "0.1",
    "network": "arbitrum",
    "gasUsed": "21000",
    "gasPriceGwei": "0.1"
  }
}
```

**Response** (if user rejects):
```json
{
  "error": {
    "code": -32000,
    "message": "Payment rejected by user"
  }
}
```

**Response** (if component not linked):
```json
{
  "error": {
    "code": -32001,
    "message": "Component not authorized for wallet access"
  }
}
```

**Behavior**:
1. Check if component is linked (authorized)
2. If not linked, reject immediately
3. If linked, display payment dialog in shell UI
4. Wait for user approval/rejection
5. On approval, sign and broadcast transaction
6. Return transaction hash or error

##### `wallet.estimateGas`
Estimate gas cost for a transaction.

**Request**:
```json
{
  "method": "wallet.estimateGas",
  "params": {
    "fromAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "toAddress": "0x...",
    "amount": "0.1",
    "network": "arbitrum",
    "token": null
  }
}
```

**Response**:
```json
{
  "result": {
    "gasLimit": "21000",
    "gasPriceGwei": "0.1",
    "estimatedCostEth": "0.0021",
    "estimatedCostUsd": "5.25" // Optional, if price oracle available
  }
}
```

#### Component Authorization

##### `wallet.linkComponent`
Link a backend component to the wallet for payment authorization.

**Request**:
```json
{
  "method": "wallet.linkComponent",
  "params": {
    "componentId": "com.osnova.autonomi",
    "componentName": "Autonomi Network",
    "permissions": ["payment"], // Future: could include "read_balance", etc.
    "maxAmountPerTransaction": "1.0", // Optional spending limit
    "maxAmountPerDay": "10.0" // Optional daily limit
  }
}
```

**Response** (after user approval):
```json
{
  "result": {
    "linked": true,
    "componentId": "com.osnova.autonomi",
    "permissions": ["payment"],
    "maxAmountPerTransaction": "1.0",
    "maxAmountPerDay": "10.0",
    "linkedAt": 1696214400
  }
}
```

**Behavior**: Displays authorization dialog to user with component details and requested permissions.

##### `wallet.unlinkComponent`
Revoke wallet access for a component.

**Request**:
```json
{
  "method": "wallet.unlinkComponent",
  "params": {
    "componentId": "com.osnova.autonomi"
  }
}
```

**Response**:
```json
{
  "result": {
    "unlinked": true,
    "componentId": "com.osnova.autonomi"
  }
}
```

##### `wallet.listLinkedComponents`
List all components with wallet access.

**Request**:
```json
{
  "method": "wallet.listLinkedComponents",
  "params": {}
}
```

**Response**:
```json
{
  "result": {
    "components": [
      {
        "componentId": "com.osnova.autonomi",
        "componentName": "Autonomi Network",
        "permissions": ["payment"],
        "maxAmountPerTransaction": "1.0",
        "maxAmountPerDay": "10.0",
        "linkedAt": 1696214400,
        "totalSpent": "2.5",
        "transactionCount": 25
      }
    ]
  }
}
```

#### Transaction History

##### `wallet.getTransactions`
Get transaction history for a wallet address.

**Request**:
```json
{
  "method": "wallet.getTransactions",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "network": "arbitrum",
    "limit": 50,
    "offset": 0
  }
}
```

**Response**:
```json
{
  "result": {
    "transactions": [
      {
        "hash": "0x...",
        "from": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        "to": "0x...",
        "amount": "0.1",
        "token": null,
        "network": "arbitrum",
        "timestamp": 1696214400,
        "status": "confirmed",
        "gasUsed": "21000",
        "gasPriceGwei": "0.1",
        "purpose": "Upload to Autonomi network"
      }
    ],
    "total": 125,
    "limit": 50,
    "offset": 0
  }
}
```

## Integration with osnova-core

The wallet component relies on osnova-core for:

1. **Key Derivation**: Uses `keys.deriveAtIndex` with component ID `com.osnova.wallet`
2. **Key Storage**: Stores derived and imported keys in encrypted cocoon
3. **Key Retrieval**: Uses `keys.getByPublicKey` to retrieve private keys for signing

## Integration with osnova-autonomi

The osnova-autonomi component uses `wallet.requestPayment` to pay for uploads:

```javascript
// Example: Autonomi component requesting payment for upload
const paymentResult = await openrpc.call("wallet.requestPayment", {
  componentId: "com.osnova.autonomi",
  fromAddress: userWalletAddress,
  toAddress: autonomiPaymentAddress,
  amount: "0.05",
  network: "arbitrum",
  token: autonomiTokenAddress,
  purpose: "Upload 5MB file to Autonomi network",
  metadata: {
    uploadSize: "5MB",
    fileName: "document.pdf"
  }
});
```

## Security Considerations

1. **User Consent**: All payments require explicit user approval via UI dialog
2. **Component Authorization**: Components must be linked before requesting payments
3. **Spending Limits**: Optional per-transaction and daily limits
4. **Key Protection**: Private keys never leave the encrypted keystore
5. **Audit Trail**: All transactions logged with purpose and requesting component
6. **Secrets Not Logged**: Private keys and sensitive data never logged

## Error Codes

- `-32000`: Payment rejected by user
- `-32001`: Component not authorized for wallet access
- `-32002`: Insufficient balance
- `-32003`: Invalid address
- `-32004`: Network error
- `-32005`: Transaction failed
- `-32006`: Spending limit exceeded
- `-32007`: Invalid private key (import)

## MVP Implementation Notes

1. **RPC Provider**: Use public RPC endpoints for Ethereum and Arbitrum (e.g., Infura, Alchemy)
2. **Gas Estimation**: Use eth_estimateGas RPC call
3. **Balance Queries**: Use eth_getBalance and ERC-20 balanceOf calls
4. **Transaction Broadcasting**: Use eth_sendRawTransaction
5. **AUTONOMI Token**: Contract address to be determined and configured
6. **Price Oracle**: Optional USD pricing via CoinGecko or similar API

## Post-MVP Enhancements

- Support for additional L2 networks (Optimism, Base, Polygon)
- Hardware wallet integration
- Multi-sig wallet support
- Token swap functionality
- NFT support
- Transaction batching
- Custom gas price selection
- Address book management

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
