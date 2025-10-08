# E2E Testing with Tauri MCP Plugin

**Last Updated**: 2025-10-08

## Overview

Osnova uses the **tauri-plugin-mcp** to enable AI-powered E2E testing of the Tauri desktop application. This plugin allows Claude Code and other AI agents to directly interact with the running Tauri app through an MCP (Model Context Protocol) server.

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│  Claude Code    │◄────────┤  MCP Server      │◄────────┤  Tauri App      │
│  (AI Agent)     │  stdio  │  (TypeScript)    │  socket │  (with plugin)  │
└─────────────────┘         └──────────────────┘         └─────────────────┘
```

### Components

1. **tauri-plugin-mcp** (Rust)
   - Runs inside the Tauri application
   - Creates a Unix domain socket (IPC) or TCP socket server
   - Exposes commands for screenshot, DOM access, JS execution, etc.
   - **Only enabled in debug builds**

2. **MCP Server** (TypeScript)
   - Standalone process that connects to the Tauri plugin socket
   - Implements MCP protocol for Claude Code
   - Translates MCP tool calls to Tauri socket commands
   - Located at: `/home/system/tauri-mcp-server/mcp-server-ts/`

3. **Claude Code**
   - Connects to MCP server via stdio
   - Uses MCP tools to interact with Tauri app
   - Can take screenshots, read DOM, execute JavaScript, etc.

## Available MCP Tools

The following tools are available to Claude Code for testing:

| Tool | Purpose | Key Parameters |
|------|---------|----------------|
| `take_screenshot` | Capture window screenshot | `window_label`, `quality`, `max_width` |
| `execute_js` | Run JavaScript in webview | `code`, `window_label`, `timeout_ms` |
| `get_dom` | Get HTML DOM content | `window_label` |
| `manage_window` | Control window (focus, resize, move) | `operation`, `x`, `y`, `width`, `height` |
| `manage_local_storage` | Read/write localStorage | `action`, `key`, `value` |

## Setup Instructions

### 1. Tauri App Configuration

The plugin is already integrated in `app/src-tauri/src/lib.rs`:

```rust
// Enable MCP plugin for AI-powered testing (debug builds only)
#[cfg(debug_assertions)]
{
    use std::path::PathBuf;
    builder = builder.plugin(tauri_plugin_mcp::init_with_config(
        tauri_plugin_mcp::PluginConfig::new("osnova".to_string())
            .start_socket_server(true)
            // Use IPC socket (Unix domain socket on Linux/macOS)
            .socket_path(PathBuf::from("/tmp/osnova-tauri-mcp.sock")),
    ));
}
```

**Socket Path**: `/tmp/osnova-tauri-mcp.sock`

### 2. MCP Server Setup

The MCP server is located at `/home/system/tauri-mcp-server/mcp-server-ts/`.

**Build the server** (already done during setup):
```bash
cd /home/system/tauri-mcp-server/mcp-server-ts
npm install
npm run build
```

**Binary location**: `/home/system/tauri-mcp-server/mcp-server-ts/build/index.js`

### 3. Claude Code MCP Configuration

Add the following to your MCP server configuration:

**File**: `~/.config/claude-code/mcp.json` (or equivalent for your platform)

```json
{
  "mcpServers": {
    "osnova-tauri": {
      "command": "node",
      "args": ["/home/system/tauri-mcp-server/mcp-server-ts/build/index.js"],
      "env": {
        "TAURI_MCP_IPC_PATH": "/tmp/osnova-tauri-mcp.sock"
      }
    }
  }
}
```

**Environment Variables**:
- `TAURI_MCP_IPC_PATH`: Path to the socket file (must match Tauri config)
- `TAURI_MCP_CONNECTION_TYPE`: Set to `"tcp"` for TCP mode (optional)
- `TAURI_MCP_TCP_HOST`: TCP host (if using TCP mode)
- `TAURI_MCP_TCP_PORT`: TCP port (if using TCP mode)

## Testing Workflow

### Starting the Environment

1. **Start Tauri app in dev mode**:
   ```bash
   cd app
   npm run tauri dev
   ```

   This will:
   - Start Vite dev server at `http://localhost:1420`
   - Launch Tauri desktop window
   - Create socket at `/tmp/osnova-tauri-mcp.sock` (debug builds only)

2. **Verify socket created**:
   ```bash
   ls -l /tmp/osnova-tauri-mcp.sock
   ```

3. **Claude Code connects automatically** when you request testing

### Example Test Scenarios

#### Test 1: Identity Creation Flow

```
User: "Test the identity creation flow in the Tauri app"

Claude Code will:
1. Take screenshot to see current state
2. Click "Create Identity" button (via execute_js or DOM manipulation)
3. Wait for UI update
4. Take screenshot to verify seed phrase displayed
5. Read DOM to extract and validate seed phrase format
6. Click "Continue" button
7. Verify navigation to next screen
8. Report success/failure
```

#### Test 2: Verify Initial State

```
User: "What does the app look like right now?"

Claude Code will:
1. Take screenshot of main window
2. Return image for you to review
```

#### Test 3: Check localStorage

```
User: "What's stored in localStorage?"

Claude Code will:
1. Use manage_local_storage with action="keys"
2. List all keys
3. Use action="get" to retrieve values
4. Report findings
```

## Troubleshooting

### Socket Connection Issues

**Problem**: MCP server can't connect to socket

**Solutions**:
1. Verify Tauri app is running in **debug mode** (`npm run tauri dev`)
2. Check socket exists: `ls -l /tmp/osnova-tauri-mcp.sock`
3. Verify socket path matches in both configs
4. Check Tauri app logs for plugin initialization

### Permission Denied

**Problem**: Socket permission errors

**Solution**:
```bash
sudo chmod 777 /tmp/osnova-tauri-mcp.sock
```

### MCP Server Not Found

**Problem**: Claude Code can't find MCP server

**Solutions**:
1. Verify MCP server is built: `ls /home/system/tauri-mcp-server/mcp-server-ts/build/index.js`
2. Test manually:
   ```bash
   cd /home/system/tauri-mcp-server/mcp-server-ts
   TAURI_MCP_IPC_PATH=/tmp/osnova-tauri-mcp.sock node build/index.js
   ```
3. Update MCP configuration path in Claude Code

### Plugin Not Loading

**Problem**: Tauri app builds but plugin doesn't work

**Verify**:
1. You're running **debug build** (not release)
2. Check `Cargo.toml` has: `tauri-plugin-mcp = { git = "https://github.com/P3GLEG/tauri-plugin-mcp", branch = "main" }`
3. Rebuild: `cd app && cargo build`

## Platform Support

| Platform | IPC Support | Notes |
|----------|-------------|-------|
| Linux | ✅ Unix socket | Default, works out of the box |
| macOS | ✅ Unix socket | Use `/tmp/` or `/private/tmp/` path |
| Windows | ✅ Named pipe | Auto-converted to `\\.\pipe\tmp\...` |

**TCP Mode**: Works on all platforms, useful for:
- Remote debugging
- Docker containers
- Network-isolated environments

## Production Builds

**IMPORTANT**: The MCP plugin is **automatically disabled in production builds**.

The `#[cfg(debug_assertions)]` guard ensures:
- ✅ Available during development (`cargo build` or `npm run tauri dev`)
- ❌ Removed from release builds (`cargo build --release` or `npm run tauri build`)
- ❌ No security risk in distributed apps
- ❌ No performance overhead in production

## Security Considerations

### Development Only

- Plugin provides **full control** over the application
- Can execute arbitrary JavaScript
- Can read all DOM content and localStorage
- Should **never** be enabled in production

### Socket File Permissions

Default socket (`/tmp/osnova-tauri-mcp.sock`) is:
- Accessible by local user only
- Not exposed to network
- Automatically cleaned up on app exit

### TCP Mode Security

If using TCP mode:
- Bind to `127.0.0.1` (localhost) only
- Use firewall rules to block external access
- Consider authentication if needed

## Advanced Configuration

### Custom Socket Path

Modify `app/src-tauri/src/lib.rs`:

```rust
.socket_path(PathBuf::from("/custom/path/to/socket.sock"))
```

### TCP Mode

Change configuration to use TCP:

```rust
tauri_plugin_mcp::PluginConfig::new("osnova".to_string())
    .start_socket_server(true)
    .tcp("127.0.0.1".to_string(), 9999)
```

Then update MCP server config:

```json
{
  "env": {
    "TAURI_MCP_CONNECTION_TYPE": "tcp",
    "TAURI_MCP_TCP_HOST": "127.0.0.1",
    "TAURI_MCP_TCP_PORT": "9999"
  }
}
```

### Multiple Windows

To interact with specific windows, use the `window_label` parameter:

```javascript
// Screenshot specific window
take_screenshot({ window_label: "settings", quality: 90 })

// Execute JS in specific window
execute_js({ code: "document.title", window_label: "main" })
```

## References

- **Plugin Source**: https://github.com/P3GLEG/tauri-plugin-mcp
- **MCP Protocol**: https://modelcontextprotocol.io/
- **Tauri Documentation**: https://v2.tauri.app/
- **Local MCP Server**: `/home/system/tauri-mcp-server/`

## Related Documentation

- **Testing Strategy**: `docs/10-development/testing.md`
- **Development Guide**: `CLAUDE.md`
- **Tauri App Structure**: `app/README.md`
