# osnova-autonomi backend component

This component is used to interact with the autonomi network, specifically the 'autonomi' rust crate version 0.6.1 as released on crates.io.
It exposes the public autonomi library APIs as OpenRPC functions to be utilized by frontend and other backend components within the osnova ecosystem.
This component is part of the osnova shell application stack, like the osnova-core backend component.
By default this component will operate on the autonomi 'mainnet'.
By adding the 'testnet = true' option, the component may operate on a local testnet instead of the mainnet.
This component should always be running.

## Tokens

All downloads from the Autonomi network are free.
No token exchange is required for these operations.
Initial uploads for all autonomi operations require both Arbitrum ETH tokens to pay for gas and AUTONOMI tokens, which is an Arbitrum L2 ETH token used to pay for Autonomi network storage.
Updates to pointer and scratchpad types are free after the initial payment has been processed.

## OpenRPC methods

The osnova-autonomi backend component provides the following OpenRPC methods for interacting with the autonomi network:

FIXME: Read through and understand the documentation from this page: https://docs.autonomi.com/developers/core-concepts/data-types . After understanding the autonomi client methods, create the necessary documentation here to implement all of the client methods as OpenRPC functions. In addition to the types specified here, create functions to create and update public scratchpads and public pointers. These are not part of the core data types, but are useful additions that most people use so should be added here as well. The function names should be orthogonal and follow the same function name pattern as used in private vs public files. The implementation of these methods can be found in the colonylib github project starting in the pod.rs file here: https://github.com/zettawatt/colonylib/blob/main/src/pod.rs#L2088 . The 'create_pointer' creates a public pointer. The 'create_scratchpad' creates a public scratchpad. The update_scratchpad and update_pointer methods are used to update these publicly readable pointers and scartchpads. Use this implementation in the OpenRPC function descriptions that will be added below. Also create an `examples/autonomi-upload-download.rs` file with example code to guide the implementation with what we want to use here. We will also need to pay for uploads using the necessary tokens. Use the osnova-wallet backend component API to retrieve the wallet private key for payment processing.

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
