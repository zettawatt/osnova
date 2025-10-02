# osnova-wallet backend component

This component interacts with osnova-core to handle cryto wallet functionality for osnova apps.
It is also used to interact with the osnova server in the case of the client-server model.
This component should always be running.

### OpenRPC methods

The osnova-wallet backend component provides the following OpenRPC methods for interacting with the osnova shell application:

FIXME: add an Ethereum crypto wallet and management functions to this component. This will be necessary when transacting with the osnova-autonomi component. Add functions for displaying balance information for various ETH and L2 tokens on the ETH blockchain. In particular, we need support for Arbitrum ETH and AUTONOMI tokens to pay for uploads on autonomi. Update the rest of the documentation to state that this component is required for MVP. Leverage the osnova-core component for key storage and derivation from the master key. Derivation should follw ETH key derivation standards from the master key. When another backend component requests payment, this component will send a request in the in the osnova shell app that will display a dialog with the payment request along with any balance information. Clicking accept in the dialog will process the payment and send the wallet key to the backend component making the request. Also add methods to support importing a separate private key outside of the key derivation chain and store this additional key in the keystore. Add a method to link the wallet to a backend component. The idea here is that a component can request access to the wallet and unless approved by the user, all payment requests will be rejected. This is a security feature. Add all this information below and make your best guess on proper implementation.

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
