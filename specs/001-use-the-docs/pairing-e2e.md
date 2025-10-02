# Pairing and End-to-End Encryption (MVP)

## Goals
- Simple mobile-to-server pairing
- Mutual authentication and confidential channel
- Clear failure feedback and retry UI

## QR payload (example)
```json
{
  "server": {
    "address": "osnova://example.com:443",
    "fingerprint": "<optional server public key fingerprint>"
  },
  "pairing": {
    "code": "ABCD-1234",          
    "nonce": "<random>",
    "expiresAt": "2025-10-01T12:00:00Z"
  },
  "client": {
    "deviceIdHint": "<optional>"
  }
}
```
Notes:
- No long-lived secrets in QR. Pairing code is short-lived and single-use.

## Handshake (high level)
1) Client initiates pairing using QR/manual address.
2) Client sends its public key and pairing code to server.
3) Server validates pairing code; responds with its public key and a server nonce.
4) Both derive a session using mutually authenticated key exchange.
5) Establish encrypted, authenticated channel; register device.
6) Persist device/server linkage under the active identity.

## Failure cases and UX
- Server unreachable/invalid: show "Server not found" with retry.
- Expired/invalid pairing code: show specific error; allow re-scan.
- Key mismatch/verification failure: abort pairing; log safely; prompt retry.

## Security considerations
- Keys are derived and stored via saorsa-core identity APIs.
- Channel security (transport, ciphers) is defined in the implementation plan; must provide mutual authentication and forward secrecy.
- Rotation and revocation: devices can be revoked; pairing must be repeatable without data disclosure. Follow the saorsa-core documentation for how to do this.

