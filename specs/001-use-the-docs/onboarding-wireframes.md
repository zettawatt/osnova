# Onboarding Wizard Wireframes

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Purpose**: Visual specification for onboarding user experience

## Overview

The onboarding wizard guides users through initial setup of Osnova, including identity creation, seed phrase backup, and deployment mode selection.

## Onboarding Flow

```
Welcome
  ↓
Deployment Mode Selection
  ↓
Identity Creation/Import
  ↓
Seed Phrase Display (if new)
  ↓
Seed Phrase Verification (if new)
  ↓
Server Configuration (if client mode)
  ↓
Complete
```

## Screen 1: Welcome

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│                         OSNOVA                             │
│                                                            │
│              Decentralized Application Platform            │
│                                                            │
│                                                            │
│                    [Osnova Logo/Icon]                      │
│                                                            │
│                                                            │
│              Welcome to Osnova                             │
│                                                            │
│     A secure, decentralized platform for running           │
│     applications with end-to-end encryption and            │
│     distributed storage.                                   │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                      [Get Started]                         │
│                                                            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 2: Deployment Mode Selection

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 1/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│              Choose Your Deployment Mode                   │
│                                                            │
│  How would you like to use Osnova?                        │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  ○  Stand-Alone Desktop                              │ │
│  │                                                      │ │
│  │     Run everything locally on this device.          │ │
│  │     Best for: Desktop users, full control           │ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  ○  Client-Server (Mobile)                          │ │
│  │                                                      │ │
│  │     Connect to a server for backend processing.     │ │
│  │     Best for: Mobile devices, shared resources      │ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  ○  Server                                           │ │
│  │                                                      │ │
│  │     Run as a server for client devices.             │ │
│  │     Best for: Always-on machines, shared access     │ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│                                                            │
│                                      [Continue]            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 3: Identity Creation/Import

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 2/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│                  Create Your Identity                      │
│                                                            │
│  Your identity is secured by a 12-word seed phrase.       │
│  This phrase is the master key to all your data.          │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  ●  Create New Identity                              │ │
│  │                                                      │ │
│  │     Generate a new 12-word seed phrase              │ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  ○  Import Existing Identity                        │ │
│  │                                                      │ │
│  │     Restore from a seed phrase backup               │ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│                                                            │
│  ⚠️  Important Security Notice:                           │
│                                                            │
│  • Never share your seed phrase with anyone              │
│  • Store it securely offline (paper, metal backup)       │
│  • Losing it means losing access to all your data        │
│  • No one can recover it for you                         │
│                                                            │
│                                                            │
│                                      [Continue]            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 4a: Seed Phrase Display (New Identity)

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 3/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│                  Your Seed Phrase                          │
│                                                            │
│  Write down these 12 words in order. You'll need them     │
│  to verify in the next step.                              │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                                                      │ │
│  │   1. abandon    2. ability    3. able    4. about   │ │
│  │                                                      │ │
│  │   5. above      6. absent     7. absorb  8. abstract│ │
│  │                                                      │ │
│  │   9. absurd    10. abuse     11. access 12. accident│ │
│  │                                                      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  [📋 Copy to Clipboard]  [🖨️ Print]  [💾 Download]        │
│                                                            │
│                                                            │
│  ⚠️  Security Checklist:                                  │
│                                                            │
│  □  I have written down all 12 words                      │
│  □  I have verified the words are correct                 │
│  □  I have stored the backup in a secure location         │
│  □  I understand this cannot be recovered if lost         │
│                                                            │
│                                                            │
│                                      [I've Saved It]       │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 4b: Seed Phrase Import (Existing Identity)

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 3/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│                Import Your Seed Phrase                     │
│                                                            │
│  Enter your 12-word seed phrase to restore your identity. │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Word 1:  [abandon        ▼]                        │ │
│  │  Word 2:  [ability        ▼]                        │ │
│  │  Word 3:  [able           ▼]                        │ │
│  │  Word 4:  [about          ▼]                        │ │
│  │  Word 5:  [above          ▼]                        │ │
│  │  Word 6:  [absent         ▼]                        │ │
│  │  Word 7:  [absorb         ▼]                        │ │
│  │  Word 8:  [abstract       ▼]                        │ │
│  │  Word 9:  [absurd         ▼]                        │ │
│  │  Word 10: [abuse          ▼]                        │ │
│  │  Word 11: [access         ▼]                        │ │
│  │  Word 12: [accident       ▼]                        │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  [📋 Paste from Clipboard]                                │
│                                                            │
│  ℹ️  Each word will auto-complete from the BIP-39 wordlist│
│                                                            │
│                                                            │
│                                      [Import]              │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 5: Seed Phrase Verification (New Identity Only)

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 4/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│                Verify Your Seed Phrase                     │
│                                                            │
│  To ensure you've saved it correctly, please enter        │
│  the following words from your seed phrase:               │
│                                                            │
│                                                            │
│  Word #3:  [____________]                                 │
│                                                            │
│  Word #7:  [____________]                                 │
│                                                            │
│  Word #11: [____________]                                 │
│                                                            │
│                                                            │
│  ℹ️  Type the words exactly as shown in the previous step │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                                      [Verify]              │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 6: Server Configuration (Client Mode Only)

```
┌────────────────────────────────────────────────────────────┐
│  ← Back                                            Step 4/4 │
├────────────────────────────────────────────────────────────┤
│                                                            │
│                Connect to Server                           │
│                                                            │
│  Enter your server's 4-word identity address:             │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  [________]  [________]  [________]  [________]      │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ℹ️  Example: apple banana cherry dragon                  │
│                                                            │
│                                                            │
│  Or scan QR code:                                         │
│                                                            │
│  ┌──────────────────┐                                     │
│  │                  │                                     │
│  │   [QR Scanner]   │                                     │
│  │                  │                                     │
│  └──────────────────┘                                     │
│                                                            │
│                                                            │
│  [Test Connection]                                         │
│                                                            │
│                                      [Connect]             │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Screen 7: Setup Complete

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│                                                            │
│                                                            │
│                      ✓  All Set!                           │
│                                                            │
│                                                            │
│              Your Osnova is ready to use                   │
│                                                            │
│                                                            │
│  Next steps:                                              │
│                                                            │
│  • Browse and install applications                        │
│  • Configure your preferences                             │
│  • Explore the Autonomi network                           │
│                                                            │
│                                                            │
│  ℹ️  You can change these settings later in Configuration │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                                                            │
│                      [Launch Osnova]                       │
│                                                            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Error States

### Invalid Seed Phrase

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│  ⚠️  Invalid Seed Phrase                                  │
│                                                            │
│  The seed phrase you entered is not valid. Please check:  │
│                                                            │
│  • All 12 words are from the BIP-39 wordlist              │
│  • Words are spelled correctly                            │
│  • Words are in the correct order                         │
│                                                            │
│  [Try Again]  [Cancel]                                    │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### Verification Failed

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│  ⚠️  Verification Failed                                  │
│                                                            │
│  The words you entered don't match your seed phrase.      │
│                                                            │
│  Please go back and review your backup, then try again.   │
│                                                            │
│  [Go Back]  [Try Again]                                   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### Server Connection Failed

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│  ⚠️  Connection Failed                                    │
│                                                            │
│  Unable to connect to the server. Please check:           │
│                                                            │
│  • The 4-word address is correct                          │
│  • The server is online and reachable                     │
│  • Your network connection is working                     │
│                                                            │
│  [Try Again]  [Change Address]                            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Mobile Adaptations

### Smaller Screens

- Reduce padding and margins
- Stack elements vertically
- Use bottom sheets for dialogs
- Larger touch targets (min 44x44 points)

### Landscape Orientation

- Side-by-side layout where appropriate
- Maintain readability
- Adjust seed phrase grid to 4x3 or 6x2

## Accessibility

- **Screen Reader**: All elements properly labeled
- **Keyboard Navigation**: Full keyboard support
- **High Contrast**: Ensure visibility in all themes
- **Font Scaling**: Respect system font size
- **Focus Indicators**: Clear focus states

## Implementation Notes

### Svelte Components

```typescript
// Onboarding.svelte
<script lang="ts">
  import { writable } from 'svelte/store';
  
  let step = writable(1);
  let deploymentMode = writable('standalone');
  let identityMode = writable('create');
  let seedPhrase = writable([]);
  
  // ... component logic
</script>
```

### State Management

Store onboarding state in memory only:
- Don't persist until complete
- Clear on cancel
- Validate at each step

### Security

- Mask seed phrase by default
- Clear clipboard after paste
- Warn before printing
- No screenshots (platform-specific)

## Testing

- Test all paths (create/import, standalone/client/server)
- Test error states
- Test back navigation
- Test cancellation
- Test accessibility
- Test on all target platforms

## Summary

These wireframes provide a clear visual specification for implementing the onboarding wizard. The flow is designed to be:

✅ Simple and intuitive
✅ Secure by default
✅ Accessible to all users
✅ Consistent across platforms
✅ Error-tolerant with clear messaging

