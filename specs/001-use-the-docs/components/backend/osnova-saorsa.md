# osnova-saorsa backend component

This component is used to interact with the saorsa DHT and network.
This component should always be running.

### OpenRPC methods

The osnova-saorsa backend component provides the following OpenRPC methods for interacting with the osnova shell application:

FIXME: Create OpenRPC functions for all of the saorsa-core public facing API as described on the page here: https://github.com/dirvine/saorsa-core/blob/main/AGENTS_API.md . Fully internalize and understand this page before creating the content on this markdown page. The intent of this component is to create an OpenRPC entrypoint to manage the saorsa-core networking components and enable connections to other backend and frontend components leveraging this technology. Take your best guess in creating the necessary functions to get this done. 

Note: All methods follow OpenRPC conventions with standard error codes and authentication via the established secure channel in Client-Server mode.
