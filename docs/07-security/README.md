# Chapter 7: Security

This chapter covers Osnova's security model, including identity management, encryption, and access control.

## Contents

- [Keys](./keys.md) - Key management and derivation
- [Client-Server Authentication](./client-server-auth.md) - Authentication model
- [Component Access Control](./component-access-control.md) - Component permissions
- [Cocoon Unlock](./cocoon-unlock.md) - Encryption-at-rest with cocoon

## Key Concepts

- **Master Key**: Derived from 12-word seed phrase
- **Key Derivation**: HKDF-SHA256 per component
- **Encryption at Rest**: saorsa-seal via cocoon
- **End-to-End Encryption**: Client-server mode
- **Access Control**: Component-based permissions
