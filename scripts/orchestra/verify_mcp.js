#!/usr/bin/env node

// Verification script showing MCP webview operations working
import net from 'net';

const SOCKET_PATH = '/tmp/osnova-tauri-mcp.sock';

class TauriClient {
  constructor() {
    this.client = null;
    this.buffer = '';
    this.requestId = 0;
    this.callbacks = new Map();
  }

  async connect() {
    return new Promise((resolve, reject) => {
      this.client = net.createConnection({ path: SOCKET_PATH }, () => {
        console.log('✓ Connected to Tauri MCP socket');
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
        const callbackIds = Array.from(this.callbacks.keys());
        if (callbackIds.length > 0) {
          const cbId = callbackIds[0];
          const { resolve, reject } = this.callbacks.get(cbId);
          this.callbacks.delete(cbId);

          if (!response.success) {
            reject(new Error(response.error || 'Command failed'));
          } else {
            resolve(response.data);
          }
        }
      } catch (err) {
        console.error('Parse error:', err.message);
      }
    }
  }

  async sendCommand(command, payload = {}) {
    return new Promise((resolve, reject) => {
      const id = ++this.requestId;
      const request = JSON.stringify({ command, payload }) + '\n';

      this.callbacks.set(id, { resolve, reject });
      this.client.write(request);

      setTimeout(() => {
        if (this.callbacks.has(id)) {
          this.callbacks.delete(id);
          reject(new Error('Timeout'));
        }
      }, 5000);
    });
  }

  disconnect() {
    if (this.client) this.client.end();
  }
}

async function main() {
  const client = new TauriClient();

  try {
    await client.connect();

    // Test 1: Get DOM
    console.log('\n1. Testing DOM access...');
    const dom = await client.sendCommand('get_dom', { window_label: 'main' });
    const domStr = typeof dom === 'string' ? dom : JSON.stringify(dom);
    const parsedDom = domStr.includes('"<html') ? JSON.parse(domStr) : domStr;
    console.log(`   ✓ DOM retrieved (${parsedDom.length} chars)`);
    console.log(`   ✓ Contains "Create Identity": ${parsedDom.includes('Create Identity')}`);

    // Test 2: Execute JavaScript
    console.log('\n2. Testing JavaScript execution...');
    const url = await client.sendCommand('execute_js', {
      window_label: 'main',
      code: 'window.location.pathname',
      timeout_ms: 3000
    });
    console.log(`   ✓ Current route: ${JSON.stringify(url)}`);

    // Test 3: Window management
    console.log('\n3. Testing window management...');
    await client.sendCommand('manage_window', {
      operation: 'setSize',
      window_label: 'main',
      width: 1200,
      height: 900
    });
    console.log('   ✓ Window resized to 1200x900');

    console.log('\n✅ All MCP webview operations working!\n');

  } catch (error) {
    console.error('\n❌ Test failed:', error.message);
    process.exit(1);
  } finally {
    client.disconnect();
  }
}

main();
