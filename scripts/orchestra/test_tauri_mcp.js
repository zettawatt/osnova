#!/usr/bin/env node

// Test script to interact with Tauri app via MCP server
import net from 'net';
import fs from 'fs';

const SOCKET_PATH = '/tmp/osnova-tauri-mcp.sock';

class TauriClient {
  constructor() {
    this.client = null;
    this.buffer = '';
    this.responseCallbacks = new Map();
  }

  async connect() {
    return new Promise((resolve, reject) => {
      this.client = net.createConnection({ path: SOCKET_PATH }, () => {
        console.log('Connected to Tauri socket');
        this.client.on('data', (data) => this.handleData(data));
        resolve();
      });

      this.client.on('error', reject);
    });
  }

  handleData(data) {
    this.buffer += data.toString();

    let newlineIndex;
    while ((newlineIndex = this.buffer.indexOf('\n')) !== -1) {
      const jsonStr = this.buffer.substring(0, newlineIndex);
      this.buffer = this.buffer.substring(newlineIndex + 1);

      try {
        const response = JSON.parse(jsonStr);
        const callbackIds = Array.from(this.responseCallbacks.keys());

        if (callbackIds.length > 0) {
          callbackIds.sort();
          const callbackId = callbackIds[0];
          const callback = this.responseCallbacks.get(callbackId);

          if (callback) {
            this.responseCallbacks.delete(callbackId);
            if (!response.success) {
              callback.reject(new Error(response.error || 'Command failed'));
            } else {
              callback.resolve(response.data);
            }
          }
        }
      } catch (err) {
        console.error('Error parsing response:', err);
      }
    }
  }

  async sendCommand(command, payload = {}) {
    return new Promise((resolve, reject) => {
      const request = JSON.stringify({ command, payload }) + '\n';
      const requestId = Date.now().toString() + Math.random().toString(36).substring(2);

      this.responseCallbacks.set(requestId, { resolve, reject });

      this.client.write(request, (err) => {
        if (err) {
          this.responseCallbacks.delete(requestId);
          reject(err);
        }
      });

      setTimeout(() => {
        if (this.responseCallbacks.has(requestId)) {
          this.responseCallbacks.delete(requestId);
          reject(new Error('Request timed out'));
        }
      }, 10000);
    });
  }

  disconnect() {
    if (this.client) {
      this.client.end();
    }
  }
}

async function main() {
  const client = new TauriClient();

  try {
    console.log('Connecting to Tauri app...');
    await client.connect();

    // Take initial screenshot
    console.log('\n1. Taking initial screenshot...');
    const screenshot1 = await client.sendCommand('take_screenshot', {
      window_label: 'main',
      quality: 80,
      max_width: 1200
    });
    console.log('Screenshot captured:', screenshot1?.image_data ? `${screenshot1.image_data.length} bytes` : 'no data');

    // Get DOM content
    console.log('\n2. Getting DOM content...');
    const dom = await client.sendCommand('get_dom', { window_label: 'main' });
    console.log('DOM HTML length:', dom?.html ? dom.html.length : 'no data');
    console.log('Page contains "Create Identity":', dom?.html?.includes('Create Identity') ? 'YES' : 'NO');
    console.log('Page contains "Import Identity":', dom?.html?.includes('Import Identity') ? 'YES' : 'NO');

    // Execute JavaScript to check current URL/route
    console.log('\n3. Checking current route...');
    const route = await client.sendCommand('execute_js', {
      code: 'window.location.pathname',
      window_label: 'main'
    });
    console.log('Current route:', route?.result || 'unknown');

    // Resize window
    console.log('\n4. Resizing window to 1000x800...');
    await client.sendCommand('manage_window', {
      operation: 'setSize',
      window_label: 'main',
      width: 1000,
      height: 800
    });
    console.log('Window resized');

    // Take screenshot after resize
    console.log('\n5. Taking screenshot after resize...');
    const screenshot2 = await client.sendCommand('take_screenshot', {
      window_label: 'main',
      quality: 80,
      max_width: 1200
    });
    console.log('Screenshot captured:', screenshot2?.image_data ? `${screenshot2.image_data.length} bytes` : 'no data');

    // Try to click a button using JavaScript
    console.log('\n6. Looking for navigation buttons...');
    const buttons = await client.sendCommand('execute_js', {
      code: `
        const buttons = Array.from(document.querySelectorAll('button'));
        buttons.map(b => ({ text: b.textContent.trim(), id: b.id, class: b.className }))
      `,
      window_label: 'main'
    });
    console.log('Buttons found:', buttons?.result || 'none');

    console.log('\n✅ All tests completed successfully!');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
  } finally {
    client.disconnect();
  }
}

main();
