# Osnova Overview
I am building a user application called Osnova that will serve as the basis for many distributed applications.
Distributed application development is difficult and the ecosystem is very fractured.
By creating a framework and marketplace where various frontend and backend components can be assembled and used, it will make it much easier for developers to create their end user applications.
The end result will be an experience very similar to the web browser of web 2.0 fame.
By downloading this one application, users will have access to the full distributed application ecosystem.
For developers it will enable them to create applications and launch them without having to deal with various app stores, etc.

# User experience

The Osnova frontend that users will interact with needs to be sleek and modern in appearance.
It should be simple to use and intuitive, following very similar conventions to what they expect from your standard web browser.
Osnova applications will load into tabs and windows, enabling users to switch between different osnova applications on the fly.
Osnova will be cross platform and run on all major OSes and architectures, providing installable binaries for all platforms.

# Components

Osnova works on a principle of dynamically loaded components. This is the general workflow:
 - User selects an Osnova application they wish to load
 - The Osnova application contains a manifest of components used by the Osnova application
 - The components are loaded into Osnova and run

There are 2 basic types of components:
 - Backend components contain the business logic, interacting with the host system, other backend components, or various distributed networks.
 - Frontend components contain the graphical frontend interface for the osnova application that the user interacts with. Frontend components interact with backend components to interact with network services and access system resources.
 
Components communicate with one another using generic protocols outside of the Osnova application itself. In this way, if Osnova were to stop development or be merged into another product, the original Osnova applications could be run without issues.

Each component will be versioned in a manifest and each version will exist at a static location in perpetuity. In this way, the Osnova application could be run at any point in the future.

# Stand-alone and Client-Server Modes

By default, an installation will run in stand-alone mode. All frontend and backend components will run as necessary on the individual device they were called.

Usage on mobile is a key requirement, but mobile devices lack the hardware or storage capacity to run resource intensive operations in a time and data constrained environment.
Osnova will enable a user to run as a server or in a headless context on their own hardware and enable mobile devices to connect over the internet.
By providing an address to a server, Osnova will default all backend operations to run on this machine and field requests from frontend components.
The user does, however, have fine-grained control to run any specific application in this client-server style or run fully stand-alone on their personal device.

The idea is that a user could have a desktop machine or similar hardware in their house, running the requested backend components and field requests to mobile devices they own.
In this way, backend components must handle multiple clients or they must spawn multiple instances as the server will be used by multiple members of the household.

Client data must be isolated from each other and encrypted. Any data stored on the server must be encrypted at rest and unlockable only by the user.
This is also true for the stand-alone environment: all data must be encrypted at rest to prevent unauthorized access.

Connecting a mobile device to an Osnova server must be extremely easy for the user. Something like a QR code should be all that is necessary for the mobile device to connect to the server whether on the local network or elsewhere.

# Core applications

The Osnova app will be preloaded with various core Osnova applications. These are only special in that they come with the default Osnova install.
Users can swap these out with other Osnova apps to replace this functionality if desired.

## App Launcher
The main purpose will be for users to click on Osnova applications and Osnova will load all of the specified components, render the page for viewing, and enable the user to interact with it.

## Crypto Wallet and Fiat Bridge
An integrated crypto wallet for storing, receiving, sending, and swapping crypto currencies.
Osnova will also contain a fiat on and off ramp to enable the user to convert from their local government issued currencies to crypto to store in their wallet.

## Search
Osnova will contain a search bar (like any web browser) that will enable the user to search for content and Osnova applications that will be fetched from web3 data sources.

The search bar will be context aware:
 - searching for osnova applications will display osnova apps like a typical app store
 - searching for videos or audio will display videos and audio files in a form similar to your standard video streaming service, like youtube or rumble
 - searching for images will display a tile screen of images like your standard search engine
 - searching for web pages will display a main line and some context lines, like your standard web browser experience

## File Manager
Files that have been downloaded or uploaded will be displayed in the file management application.

## Configuration Manager
Configures the osnova installation on the particular device. Manages any passwords, encryption keys, or accounts.

# Clarifications

## Pairing flow and security requirements
Pairing a mobile device with a server uses a bi-directional handshake:
- In the mobile Osnova app, the user starts pairing
- The app opens a QR code scanner or allows manual server address entry
- After entering/scanning the address, the device sends its public key to the server. If the server does not respond, show "Server not found" with a retry option
- The server responds with its public key
- The device and server establish an encrypted channel
- All device data is encrypted with its key. Multiple mobile clients can be paired with a server

## Encryption and key management
- A user-controlled 12-word seed phrase establishes the root identity (industry-standard mnemonic)
- Device and account keys are derived from the seed
- Data at rest is encrypted on both the server and stand-alone devices
- Users can import an existing seed to restore identity and access

## MVP scope for core applications
- App Launcher: list available apps; launch selected app by loading its manifest and opening in a tab/window; display loading/errors
- Crypto Wallet & Fiat Bridge: view balances; receive and send; basic swap; initiate fiat on/off-ramp via supported providers
- Search: single omnibox; fetch results from distributed sources; context-aware presentation for apps, media, images, and web pages
- File Manager: list downloaded/uploaded files; open file location; basic actions (open, rename, delete)
- Configuration Manager: set server address; manage pairing; back up/restore seed phrase; manage accounts and basic security settings; manage per-app configuration and cached data per user (view, export, reset, delete)

## Protocols and interoperability constraints
Components communicate via stable, generic request/response interfaces independent of Osnova (e.g., JSON-RPC 2.0 or REST). Components run isolated from the host app to enable portability.

## Hosting and distribution mechanism
Each component version is immutable and retrievable from a permanent, content-addressed storage network. The primary target is the Autonomi network, with support for alternatives (e.g., Arweave or IPFS) to ensure long-term availability.
