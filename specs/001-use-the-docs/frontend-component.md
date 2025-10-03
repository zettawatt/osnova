# Frontend Components

This document describes the structure and lifecycle of external frontend components.
This document does not apply to core graphical elements included as part of the osnova shell application

## Lifecycle

Frontend components are constructed from JavaScript/TypeScript, HTML, and CSS and may or may not use a framework.
These frontend components will run within the osnova shell application WebKit within their own tab.
Most are independent projects that are not part of the osnova shell itself, but follow the proper semantics to run under the osnova framework.
Core frontend components may be developed within the osnova shell project itself under the 'components/frontend' directory tree.
In this sub-hierarchy, they are constructed the same as any independent frontend component project.

### Development

During development it does not make sense to make releases of frontend components as these will be under constant flux.
In the osnova app schema a directory path may be provided that points to the osnova frontend component project.
When ready to execute, the WebKit will render the index.html file, which will specify all of the components necessary for the frontend application.

### Release

A frontend component is released when the frontend project is compiled into a single ZLIB compressed package and written to the Autonomi network.
There may be multiple packages for each version based on the desired target platform.
Each package will be uploaded to the Autonomi network as a public file and the corresponding Autonomi address location will be recorded in the component manifest.

### Production

After release, a production binary of that version may not be used again.
Versions on particular targets are immutable.
Osnova apps pointing to a package of a frontend component will always pull from the frontend component manifest address ensuring that the data has not been tampered with and was signed/uploaded by the project maintainer as only the maintainer has the keys to upload content to the manifest address.

## Frontend component manifest schema

Each version contains a manifest that has the following skeleton schema that is loaded as a public file to the Autonomi network:

```json
{
  "title": "Osnova Frontend Component Manifest",
  "type": "object",
  "required": ["id", "name", "version", "description"],
  "properties": {
    "id": {"type": "string", "description": "Autonomi content address of the manifest itself or a path on the local filesystem for development purposes"},
    "name": {"type": "string"},
    "version": {"type": "string", "pattern": "^\d+\.\d+\.\d+$", "description": "Semver; exact pinned version"},
    "description": {"type": "string"},
    "publisher": {"type": "string", "description": "Publisher identifier"},
    "signature": {"type": "string", "description": "Detached signature over canonical manifest"},
    "platforms": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "platform"],
        "properties": {
          "id": {"type": "string", "description": "Autonomi address of the frontend component package"},
          "platform": {"type": "string", "description": "Platform that the frontend will run on. Could be Android, iOS, desktop, or all which is platform independent" },
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
- The 'platform' field is required for production releases. For development the target is assumed to be the system running the development.
- The 'any' platform can run on any platform. If a specific platform is defined and matches the target machine, the specific platform will be higher priority with the 'any' used as a default.
- The frontend component developer may choose to build a single 'any' platform component and pass in variables to change the component behavior based on platform.

## Storage on the Autonomi Network

Compiled frontend component packages are always uploaded as public files, each target is its own file.
Each version's package collateral is listed in a manifest file as described above.
The address for the component is a pointer that points to a graph entry.
Each time a new version is added, a new manifest file is created along with a graph entry.
The graph entry contains links to all of the previous version manifest files as well as the latest entry.
The pointer is updated to point to the latest graph entry.
In this way, we build up an immutable list of collateral for each component specified by version.

## Frontend Component Installation

To install a new osnova app, it will need to download the necessary frontend components. To read a frontend component, it follows these steps:
- read the component address id from the osnova app manifest
- download the pointer at that address to get the graph entry address
- download the graph entry
- walk through all of the entries and download the version manifest files
- check if there is a newer compatible version in the manifest than is cached on the local machine or osnova server
  - if so, download this version compatible with the target architecture and store it in the frontend component cache, shutdown the older compatible version if it is running, and execute the newly downloaded version
  - if there is no newer version than is already cached, move on to the next component
  
After an osnova app is installed, the frontend component is stored in the component cache directory.

## Local caching and data storage

Frontend components can store data on the local device.
By default, the configuration app will specify a local data directory.
All component data will be stored in a sub-directory by user and component version.
