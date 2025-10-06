# Quick Start

This guide provides a quick walkthrough to get started with Osnova and validate the core functionality.

## Prerequisites

- Osnova installed on your device
- For server mode: a compatible server instance
- For mobile client-server mode: both mobile app and server configured

## First Launch

### Initial Setup

1. **Launch Osnova** for the first time
2. **Onboarding Wizard** will appear automatically
3. **Enter your display name**
4. **Choose your identity option**:
   - **Import Existing**: Enter your 4-word identity address
   - **Create New**: Generate a new identity

### For Stand-Alone or Server Installations

5. **Seed Phrase Setup**:
   - **Generate New**: System creates a 12-word seed phrase
   - **Import Existing**: Enter your existing 12-word seed phrase
6. **Backup your seed phrase** securely (shown on screen)
7. **Complete setup** and proceed to the main interface

## Basic Usage

### Launching an Application

1. **Open the App Launcher** (default screen on startup)
2. **Browse available applications** in the grid view
3. **Click/tap an application icon** to launch
4. **Application loads** and renders in a new tab or window

Expected behavior:
- Manifest is loaded automatically
- Required assets are fetched (core services are built-in)
- UI renders within approximately 2 seconds
- Application becomes interactive

### Managing Multiple Applications

- **Switch between apps** using tabs (desktop) or the bottom menu (mobile)
- **Close apps** by closing their tab or window
- **Reorder app icons** by clicking/long-pressing and dragging

## Client-Server Mode Setup

### Pairing a Mobile Device

1. **On the server**: Display the pairing QR code or identity address
2. **On the mobile device**:
   - Open Configuration in Osnova
   - Select "Add Server"
   - Scan the QR code OR enter the 4-word address manually
3. **Wait for pairing** (automatic secure channel establishment)
4. **Confirmation** when pairing succeeds

Connection behavior:
- Up to 3 connection attempts
- 5-second timeout per attempt
- Exponential backoff: 1s, 2s, 4s with jitter
- "Server not found" message if all attempts fail
- Retry option available

### Using Client-Server Mode

1. **Launch an application** on your mobile device
2. **Backend operations** execute automatically on the configured server
3. **Frontend remains responsive** on the mobile client
4. **Data is encrypted** end-to-end between device and server

## Testing Core Functionality

### Stand-Alone Mode Test

**Given**: Fresh installation with no server configured

**When**: Launch an Osnova app from the App Launcher

**Then**:
- Manifest loads successfully
- App assets are fetched or retrieved from cache
- UI renders in a new tab/window
- All operations run locally

### Client-Server Mode Test

**Given**: Server address configured and pairing completed

**When**: Use Osnova on a mobile device

**Then**:
- Backend operations execute on the server
- Mobile client remains responsive
- UI updates reflect server-side processing

### Slow Server Fallback Test

**Given**: Server is experiencing high latency (>5 seconds)

**When**: Using an application in client-server mode

**Then**:
- System prompts the user about the delay
- Options provided: retry or temporarily use stand-alone mode
- User can make an informed decision

## Data Isolation and Security

### Verifying Encryption

1. **Check data isolation**: Each client's data is separate
2. **Verify encryption-at-rest**: Local data is encrypted
3. **Confirm E2E encryption** (client-server mode):
   - User data is encrypted on the client
   - Server cannot decrypt user content
   - Only routing/operational metadata remains in plaintext

## Performance Validation

### Launch Performance

**Target**: p95 time from app launch to first meaningful render ≤ 2 seconds

**How to measure**:
1. Close all Osnova apps
2. Launch an app from the Launcher
3. Time from icon click to first interactive render
4. Repeat multiple times to establish p95 metric

### Expected Results
- Most launches complete within 2 seconds
- 95th percentile remains at or below 2 seconds
- Mobile clients remain responsive during backend operations

## Configuration Management

### Viewing App Configuration

1. **Open Configuration Manager**
2. **Select an application**
3. **View current settings**
4. **Modify as needed** (changes require app restart)

### Managing App Cache

1. **Open Configuration Manager**
2. **Navigate to app cache section**
3. **Available actions**:
   - View cache contents
   - Export cache data
   - Reset cache (with confirmation)
   - Delete cache (with warning)

**Note**: Deleting configuration or cache requires an app relaunch to take effect.

## Next Steps

After validating the basic functionality:

1. **Explore core applications**:
   - File Manager
   - Crypto Wallet
   - Search interface

2. **Try advanced features**:
   - Theme switching (light/dark mode)
   - Icon reordering and customization
   - Multi-device setup

3. **Development path**:
   - Review contract tests in contracts/
   - Follow tasks.md for implementation details
   - Refer to component documentation for custom development

## Troubleshooting

### Server Not Found
- Verify server is running
- Check 4-word address is correct
- Ensure network connectivity
- Try manual address entry

### App Won't Launch
- Check manifest URL is valid
- Verify component versions exist
- Review error messages in logs
- Ensure sufficient storage space

### Performance Issues
- Check network latency in client-server mode
- Verify server has adequate resources (5+ concurrent clients)
- Review system resource usage
- Consider switching to stand-alone mode temporarily
