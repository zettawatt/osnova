# High Level Plan

Osnova is a Tauri 2.0 app.

# Frontend Details

The UI will be written in TypeScript, HTML, and CSS leveraging the Svelte framework.
The UI should be responsive and able to run in both a desktop and mobile context.

## Desktop View

The desktop application should look like a standard desktop web browser experience.
A light/dark mode selection button should be at the top right corner of the screen with the application automatically switching when the core desktop switches modes.

## Mobile View

For mobile OSes, the application should have a clean interface that works for both iOS and Android.
The bottom of the screen should have a 5 icon menu that is user configurable to select a tab which is running an Osnova app.
A light/dark mode selection button should be located in the configuration window with the application automatically switching when the OS switches modes.

# Backend Details

The backend business logic will be written in Rust. The core Osnova logic will be packaged into a library that can be used by other projects if desired.
The Tauri commands will simply call the Osnova library public functions.

# Component Architecture

All Osnova applications are constructed of components.
Core application component source code is stored within the Osnova repository itself under a components hierarchy, with frontend components and backend components in separate sub-directories.
Each backend component is a separate rust sub project with Cargo.toml, its own life cycle, etc.
Each frontend component is the uncompressed static web application.
Components are dynamically loadable into Osnova and operate like plugins.
The backend components are precompiled Rust binaries to the host architecture.
The frontend comoponents are static web applications, comprised of a ZLIB compressed TypeScript/JavaScript, HTML, and CSS tarball for easy distribution.
The backend and frontend core components can be compiled and packaged by running 

Each backend component is loaded via a plugin architecture by Tauri.
Components communicate using OpenRPC.
When components are downloaded from the Autonomi network they are stored in the user's cache so they do not need to be fetched again.

## Frontend Component Details

Frontend components are written in TypeScript or JavaScript, HTML, and CSS. These are essentially just static web pages that are rendered within Tauri's WebKit in a tab in the frontend application.
For distribution, the webapp is compressed using ZLIB into a tarball that can be distributed as a single file.
When started, the web app is uncompressed and loaded into Tauri's WebKit, optionally passing configuration arguments from the Osnova application manifest.
The webapp will use OpenRPC calls to interact with backend components' respective OpenRPC servers.

## Backend Component Details

The backend components are written in Rust and are precompiled binaries matching the host architecture.
Tauri will treat these components as plugins using a simple API:
 - **component_configure** - create a component configuration JSON object from the user's configuration cache
 - **component_start** - start the component OpenRPC server, passing in a configuration JSON object, and register the component so that it can be managed by the Osnova
 - **component_stop** - stop the component OpenRPC server, unregister the component from the management system since it has been halted
 - **component_status** - returns a JSON object reporting on the component's status if it is alive and running

When started, the backend component binary is executed by the Tauri plugin loader, using the configuration JSON object and optionally, any configuration options from the Osnova application manifest.
The user's configuration cache contains the highest priority options, followed by whatever configuration is specified by the manifest.
Each backend component will leverage a consistent ABI to support the above mentioned commands.

Backend components field requests from frontend components, but can also interact with other backend components over OpenRPC.

### MPC Client

Each backend component will run its own OpenRPC server to communicate to the outside world.
In addition, it will provide an MPC client to enable direct connection of its public API to AI agents.
AI agents will be able to leverage this functionality to iterate on ideas leveraging real world outputs from the component itself, not relying on just code and documentation.

## Manifest Schema

The Osnova application manifest contains references to all of the components and their default configurations required to run that application under the Osnova framework.
The manifest is encoded in JSON format.
References to components for production applications should use Autonomi address URI's prefixed with 'ant://'.
References to components for applications under development can point to local directories.
Local development takes source code as is without compression to enable easier debug.

# Stand-alone and Client-Server Modes

In stand-alone mode, all backend and frontend components for Osnova apps are run on the local device with inter-process communication occuring locally using the most efficient local only OpenRPC transport.

In client-server mode, the workload is split across the client and the server.
The client will run (and cache) the frontend components on the local device.
The server will run the backend component OpenRPC servers and make a direct encrypted connection to the client to field any OpenRPC requests from the client run frontend components.
