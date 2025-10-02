# Tauri WebView Configuration

**Decision Date**: 2025-10-02
**Status**: Approved for MVP
**Decision**: Use Tauri defaults with single window and tab-based navigation; enable isolation post-MVP

## Overview

The Tauri WebView configuration determines how frontend components are isolated and secured. For MVP, we use a single WebView with tab-based navigation and Tauri's default security settings.

## MVP Configuration

### Single Window, Multiple Tabs

**Architecture**:
- One Tauri window
- One WebView instance
- Tab-based navigation between frontend components
- Svelte 5 for UI framework

**Rationale**:
- Simpler implementation
- Faster development
- Lower memory footprint
- Sufficient for MVP

### Tauri Configuration

`src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Osnova",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": false
      },
      "fs": {
        "all": false,
        "readFile": false,
        "writeFile": false,
        "scope": []
      },
      "http": {
        "all": false,
        "request": false,
        "scope": []
      },
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "notification": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.osnova.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; connect-src 'self' http://localhost:*"
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Osnova",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

### Content Security Policy (CSP)

**MVP CSP**:
```
default-src 'self';
script-src 'self' 'unsafe-inline';
style-src 'self' 'unsafe-inline';
img-src 'self' data: blob:;
connect-src 'self' http://localhost:*;
```

**Explanation**:
- `default-src 'self'`: Only load resources from app origin
- `script-src 'self' 'unsafe-inline'`: Allow inline scripts (needed for Svelte)
- `style-src 'self' 'unsafe-inline'`: Allow inline styles (needed for Tailwind)
- `img-src 'self' data: blob:`: Allow images from app, data URIs, and blobs
- `connect-src 'self' http://localhost:*`: Allow connections to local OpenRPC servers

**Post-MVP**: Remove `'unsafe-inline'` by using nonces

### WebView Security Settings

**MVP**: Use Tauri defaults
- JavaScript enabled
- WebGL enabled
- Local storage enabled
- Session storage enabled
- IndexedDB enabled

**Post-MVP**: Tighten based on needs

## Component Loading

### Tab-Based Navigation

```svelte
<!-- App.svelte -->
<script lang="ts">
  import { writable } from 'svelte/store';
  
  let activeTab = writable('launcher');
  let tabs = writable([
    { id: 'launcher', title: 'Launcher', component: 'osnova-launcher' },
    { id: 'config', title: 'Settings', component: 'osnova-config' },
  ]);
  
  function switchTab(tabId: string) {
    activeTab.set(tabId);
  }
  
  function openApp(appId: string) {
    // Add new tab for app
    tabs.update(t => [...t, {
      id: appId,
      title: appId,
      component: appId,
    }]);
    activeTab.set(appId);
  }
</script>

<div class="app">
  <TabBar {tabs} {activeTab} on:switch={e => switchTab(e.detail)} />
  
  <div class="content">
    {#if $activeTab === 'launcher'}
      <LauncherApp on:openApp={e => openApp(e.detail)} />
    {:else if $activeTab === 'config'}
      <ConfigApp />
    {:else}
      <DynamicComponent componentId={$activeTab} />
    {/if}
  </div>
</div>
```

### Dynamic Component Loading

```typescript
// Load frontend component from cache
async function loadComponent(componentId: string): Promise<string> {
  // Get component manifest
  const manifest = await invoke('component_get_manifest', { componentId });
  
  // Get cached tarball
  const tarballPath = await invoke('component_get_cached_frontend', { componentId });
  
  // Extract and load
  const html = await invoke('component_load_frontend', { tarballPath });
  
  return html;
}

// Render in iframe for isolation (post-MVP)
function renderComponent(html: string, containerId: string) {
  const iframe = document.createElement('iframe');
  iframe.sandbox = 'allow-scripts allow-same-origin';
  iframe.srcdoc = html;
  document.getElementById(containerId).appendChild(iframe);
}
```

## Post-MVP: WebView Isolation

### Multiple Windows

Create separate windows for each component:

```rust
use tauri::{Manager, WindowBuilder, WindowUrl};

pub fn open_component_window(
    app: &tauri::AppHandle,
    component_id: &str,
) -> Result<(), Error> {
    let window = WindowBuilder::new(
        app,
        component_id,
        WindowUrl::App(format!("component/{}", component_id).into())
    )
    .title(component_id)
    .inner_size(800.0, 600.0)
    .build()?;
    
    Ok(())
}
```

**Benefits**:
- Process isolation (on some platforms)
- Independent crash domains
- Better security

**Drawbacks**:
- Higher memory usage
- More complex window management
- Platform differences

### Iframe Isolation

Use iframes with sandbox attribute:

```html
<iframe
  sandbox="allow-scripts allow-same-origin"
  src="component.html"
  style="width: 100%; height: 100%; border: none;"
></iframe>
```

**Sandbox Attributes**:
- `allow-scripts`: Allow JavaScript
- `allow-same-origin`: Allow same-origin access (needed for Tauri commands)
- `allow-forms`: Allow form submission (if needed)
- `allow-popups`: Allow popups (if needed)

**Benefits**:
- DOM isolation
- Limited API access
- Single window

**Drawbacks**:
- Same process
- Can be bypassed with `allow-same-origin`

### WebView2 Isolation (Windows)

Use WebView2 user data folders for isolation:

```rust
#[cfg(target_os = "windows")]
pub fn create_isolated_webview(component_id: &str) -> Result<WebView, Error> {
    let user_data_folder = format!("webview-{}", component_id);
    
    // WebView2 with separate user data folder
    // Provides cookie/storage isolation
    // Implementation depends on Tauri's WebView2 support
    
    Ok(webview)
}
```

## Communication Between Components

### Via Tauri Commands

All inter-component communication goes through Tauri backend:

```typescript
// Component A
await invoke('storage_set', { key: 'shared.data', value: 'hello' });

// Component B
const value = await invoke('storage_get', { key: 'shared.data' });
```

**Benefits**:
- Centralized control
- Access control enforcement
- Audit logging

### No Direct Communication

Components cannot:
- Access each other's DOM
- Share JavaScript objects
- Use postMessage (unless explicitly enabled)

## Mobile Considerations

### iOS WebView

Uses WKWebView:
- Good security
- Process isolation
- Limited customization

**Configuration**:
```json
{
  "tauri": {
    "ios": {
      "webview": {
        "allowsInlineMediaPlayback": true,
        "allowsBackForwardNavigationGestures": false
      }
    }
  }
}
```

### Android WebView

Uses Android System WebView:
- Security depends on Android version
- Less isolation than iOS
- More customization

**Configuration**:
```json
{
  "tauri": {
    "android": {
      "webview": {
        "allowFileAccess": false,
        "allowContentAccess": false
      }
    }
  }
}
```

## Performance Optimization

### WebView Caching

Enable HTTP caching for faster loads:

```rust
// Configure WebView caching
pub fn configure_webview_cache(webview: &WebView) {
    // Enable HTTP cache
    // Cache frontend components after first load
    // Implementation depends on platform
}
```

### Preloading

Preload common components:

```typescript
// Preload launcher and config on startup
async function preloadComponents() {
  await loadComponent('osnova-launcher');
  await loadComponent('osnova-config');
}
```

## Debugging

### DevTools

Enable DevTools in development:

```json
{
  "tauri": {
    "windows": [
      {
        "devtools": true
      }
    ]
  }
}
```

**Production**: Disable DevTools

### Console Logging

Forward console logs to Rust:

```typescript
// Intercept console.log
const originalLog = console.log;
console.log = (...args) => {
  originalLog(...args);
  invoke('log_from_frontend', { level: 'info', message: args.join(' ') });
};
```

## Testing

### WebView Testing

Use Playwright for E2E tests:

```typescript
import { test, expect } from '@playwright/test';

test('component loads correctly', async ({ page }) => {
  await page.goto('http://localhost:5173');
  await page.click('[data-testid="launcher-icon"]');
  await expect(page.locator('.app-content')).toBeVisible();
});
```

### Isolation Testing

Test that components cannot access each other:

```typescript
test('components are isolated', async ({ page }) => {
  // Load component A
  await page.goto('http://localhost:5173/component-a');
  await page.evaluate(() => {
    window.testData = 'secret';
  });
  
  // Load component B
  await page.goto('http://localhost:5173/component-b');
  const data = await page.evaluate(() => window.testData);
  
  // Should not have access to component A's data
  expect(data).toBeUndefined();
});
```

## Security Checklist

- [ ] CSP configured correctly
- [ ] Tauri allowlist minimized
- [ ] DevTools disabled in production
- [ ] No `eval()` or `Function()` in code
- [ ] No inline event handlers
- [ ] All user input sanitized
- [ ] HTTPS for external resources (if any)
- [ ] Subresource integrity for CDN resources (if any)

## Migration Path

### Phase 1: MVP (Single WebView)
- Single window with tabs
- Tauri default security
- CSP with `unsafe-inline`

### Phase 2: Enhanced Security
- Remove `unsafe-inline` from CSP
- Use nonces for inline scripts
- Tighten Tauri allowlist

### Phase 3: Isolation
- Iframe-based component isolation
- Separate WebView contexts
- Enhanced access control

### Phase 4: Advanced
- Multiple windows for components
- Process isolation (where supported)
- Hardware-backed security

## Summary

**MVP Configuration**:
✅ Single window with tab-based navigation
✅ Tauri default security settings
✅ CSP with necessary inline permissions
✅ Minimal Tauri allowlist
✅ DevTools enabled in development only

**Post-MVP Enhancements**:
- Iframe or window-based isolation
- Stricter CSP without `unsafe-inline`
- Enhanced WebView security settings
- Platform-specific optimizations

This configuration provides a good balance of security, performance, and development velocity for MVP.

