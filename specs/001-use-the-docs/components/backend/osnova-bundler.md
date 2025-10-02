# osnova-bundler backend component

This component is used to perform several functions:
- compile osnova backend components to various targets and uploads them to the autonomi network
- package frontend components into a ZLIB compressed tarball and uploads them to the autonomi network
- generate component and application manifest files and uploads them to the autonomi network

This component leverages the osnova-autonomi component to perform the autonomi upload operations and the osnova-wallet to pay for these uploads.

### OpenRPC methods

The osnova-bundler backend component provides the following OpenRPC methods:

FIXME: This component implements all of the functionality described in other sections for manifests and component structures. Derive an OpenRPC set of functions to implement these structures using your best guess of what is needed.

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
