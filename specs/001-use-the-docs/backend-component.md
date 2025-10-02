# Backend Components

This document describes the structure and lifecycle of backend components.

## Lifecycle

Backend components are constructed from Rust and have the standard rust project data structure and semantics.
Most are independent projects that are not part of the osnova shell itself, but follow the proper semantics to run under the osnova framework.
Core backend components may be developed within the osnova shell project itself under the 'components/backend' directory tree.
In this sub-hierarchy, they are constructed the same as any independent backend component project.

### Development

During development it does not make sense to make releases of backend components as these will be under constant flux.
In the osnova app schema a directory path may be provided that points to the osnova backend component project.
When ready to execute, the backend component developer will compile the backend component into a compatible binary for the target running the osnova shell application.
Osnova shell, when launching an osnova app that points to this backend component, will look to the proper target releease directory and run the backend component binary within the plugin loader framework.

### Release

A backend component is released when a set of backend component binaries is written to the Autonomi network.
There may be multiple component binaries for each version based on the desired target implementations.
Each binary will be uploaded to the Autonomi network as a public file and the cooresponding Autonomi address location will be recorded in the component manifest.

### Production

After release, a production binary of that version may not be used again.
Versions on particular targets are immutable.
Osnova apps pointing to a binary of a backend component will always pull from the backend component manifest address ensuring that the data has not been tampered with and was signed/uploaded by the project maintainer as only the maintainer has the keys to upload content to the manifest address.

## Backend component manifest schema

Each version contains a manifest that has the following skeleton schema that is loaded as a public file to the Autonomi network:

```json
{
  "title": "Osnova Backend Component Manifest",
  "type": "object",
  "required": ["id", "name", "version", "description"],
  "properties": {
    "id": {"type": "string", "description": "Autonomi content address of the manifest itself or a path on the local filesystem for development purposes"},
    "name": {"type": "string"},
    "version": {"type": "string", "pattern": "^\d+\.\d+\.\d+$", "description": "Semver; exact pinned version"},
    "description": {"type": "string"},
    "publisher": {"type": "string", "description": "Publisher identifier"},
    "signature": {"type": "string", "description": "Detached signature over canonical manifest"},
    "targets": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "target"],
        "properties": {
          "id": {"type": "string", "description": "Autonomi address of the backend component binary"},
          "target": {"type": "string", "description": "Target for compiled backend components following Rust's official target triple format (e.g., x86_64-unknown-linux-gnu)." },
          "hash": {"type": "string", "description": "Hash (e.g., blake3 base64) of the fetched artifact"},
          "config": {"type": "object", "additionalProperties": true},
        }
      }
    },
    "metadata": {"type": "object", "additionalProperties": true}
  }
}
```

Notes:
- The 'targets' field is required for production releases. For development the target is assumed to be the system running the development. This field is required for cross-platform compilation strategies

## Storage on the Autonomi Network

Compiled backend component binaries are always uploaded as public files, each target is its own file.
Each version's binary collateral is listed in a manifest file as described above.
The address for the component is a pointer that points to a graph entry.
Each time a new version is added, a new manifest file is created along with a graph entry.
The graph entry contains links to all of the previous version manifest files as well as the latest entry.
The pointer is updated to point to the latest graph entry.
In this way, we build up an immutable list of collateral for each component specified by version.

## Backend Component Installation

To install a new osnova app, it will need to download the necessary backend components. To read a backend component, it follows these steps:
- read the component address id from the osnova app manifest
- download the pointer at that address to get the graph entry address
- download the graph entry
- walk through all of the entries and download the version manifest files
- check if there is a newer compatible version in the manifest than is cached on the local machine or osnova server
  - if so, download this version compatible with the target architecture and store it in the backend component cache, shutdown the older compatible version if it is running, and execute the newly downloaded version
  - if there is no newer version than is already cached, move on to the next component
  
After an osnova app is installed, the backend component is stored in the component cache directory.

## Backend Component Loading

Backend components are treated as binary plugins in the Tauri framework.
Backend components are executed by the Tauri backend when an osnova app requiring them is started.

FIXME: the intent here is to start each component with a common set of Tauri commands that can start, stop, and return the status of individual components. I do NOT want to use the Tauri shell option because this is not cross platform compatible. The solution must run on all platforms. The status for the components can be returned through an OpenRPC call, same with stop. Only the start method is special in that it actually starts the process or somehow runs it in the same Tauri process via FFI/dynamic linking. Come up with the best solution here that meets these requirements and describe it here.

## Backend Component Lifecycle

If a component crashes, its restart policy is defined by the setting in the configuration page.
By default, backend components will restart automatically and throw up a warning 'toast' in the osnova shell GUI describing which component failed and the message that occurred.
The component configuration may also set the restart policy to not restart the component on failure.
By default, backend components, once started, will halt once all users' apps utilizing them are closed within the osnova shell app.
The component configuration may also disable the auto-shutdown policy and keep the process running.

## Versioning and Usage in Osnova Shell

FIXME: I want to use identical semantics to rust's Cargo.toml semantic versioning rules. Please update the app manifest documentation to allow for all of this functionality when describing component versions. There is no reason to reinvent the wheel here, let's just use what works and is proven.

Different osnova apps may utilize the same backend components.
It is undesirable to start a backend component OpenRPC server more than one time when multiple frontend components and users can share one service.
To alleviate this issue, frontend components and other backend components specifying different versions in their manifests may substitute with the running version if it is newer but on the same semantic version as that specified.
If an older but compatible version of the backend component is currently running and a newer and compatible version is downloaded for use in a different application, the newer version will run and be used.
The latest cached compatible version of the backend component will be used unless a specific version is forced through the configuration app.

From the osnova configuration application, a user sees the list of all installed osnova apps.
Selecting an app will display the various frontend and backend components used and their versions.
At the top of the menu there is a toggle selector that will enable viewing by components instead of apps.
This view will show components used, not the apps that use them.
Having both views gives the user the option the best view for whatever operation they are wanting to undertake.
By default, the app will use the latest compatible component version.
However, if there are incompatibilties, a user can force a component to be an older version for the system.
When a different version is requested, the user can click a checkbox that says 'force version' and a drop down for the component will be displayed with all of the compatible versions for that component.
The user selects which component version they want to use.
For standalone installations of osnova, it will tell the user to restart osnova to use this newly specified version in a dialog pop up.
For server-client installations of osnova, if the client has admin privileges, they will be given the option to restart the osnova server remotely.
If the client does not have admin privileges, a popup will be displayed telling them that they need to contact a server administrator to restart the server.

## Local caching and data storage

FIXME: as with the 'FIXME' in the 'Versioning and Usage in Osnova Shell' section, follow the same versioning semantics in the description below. If the running component is incompatible with a different app, create a new component user data directory and shared data directory as required.

Backend components can store data on the local device or server running the component.
By default, the configuration app will specify a local data directory.
All component data will be stored in a sub-directory by user and component version.
Components may also share data for the installation that is not user specific, this will be placed under a 'shared' sub directory.
For example, if Bob is running an osnova app that contains contains backend component foo version 0.1.2 and Alice is running an osnova app that contains backend component bar version 1.2.1, the directory structure will look like this:

<DATA_DIR>/
 |-> Bob/
   |-> foo/
       |-> v0.1.0/
           |-> Bob's 'foo' component data
 |-> Alice/
   |-> bar/
       |-> v1.0.0/
           |-> Alice's 'bar' component data
 |-> shared/
   |-> foo/
       |-> v0.1.0/
           |-> shared 'foo' component data
   |-> bar/
       |-> v1.0.0/
           |-> shared 'bar' component data

Notes:
- The most significant version specifies compatibility. So if Bob updated 'foo' to v0.1.2, the data directory stays the same. If foo is updated to v0.2.3, a new data directory is required. Likewise if Alice updated 'bar' to version v1.5.2, the same v1.0.0 directory is OK as it is compatible with the latest version.

