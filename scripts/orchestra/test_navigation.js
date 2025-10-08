#!/usr/bin/env node

// Test navigation between screens
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
      }, 10000);
    });
  }

  disconnect() {
    if (this.client) this.client.end();
  }
}

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
  const client = new TauriClient();

  try {
    await client.connect();
    console.log('✓ Connected to Tauri app\n');

    // Test clicking Settings tab
    console.log('=== Clicking Settings Tab ===');
    try {
      await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Settings',
        should_click: true
      });
      console.log('✓ Clicked Settings tab');
      await sleep(500);

      const route = await client.sendCommand('execute_js', {
        window_label: 'main',
        code: 'window.location.pathname'
      });
      console.log(`  New route: ${route}\n`);
    } catch (e) {
      console.log(`✗ Failed: ${e.message}\n`);
    }

    // Test clicking Wallet tab
    console.log('=== Clicking Wallet Tab ===');
    try {
      await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Wallet',
        should_click: true
      });
      console.log('✓ Clicked Wallet tab');
      await sleep(500);

      const route = await client.sendCommand('execute_js', {
        window_label: 'main',
        code: 'window.location.pathname'
      });
      console.log(`  New route: ${route}\n`);
    } catch (e) {
      console.log(`✗ Failed: ${e.message}\n`);
    }

    // Test clicking Deploy tab
    console.log('=== Clicking Deploy Tab ===');
    try {
      await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Deploy',
        should_click: true
      });
      console.log('✓ Clicked Deploy tab');
      await sleep(500);

      const route = await client.sendCommand('execute_js', {
        window_label: 'main',
        code: 'window.location.pathname'
      });
      console.log(`  New route: ${route}\n`);
    } catch (e) {
      console.log(`✗ Failed: ${e.message}\n`);
    }

    // Go back to Launcher
    console.log('=== Clicking Launcher Tab ===');
    try {
      await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Launcher',
        should_click: true
      });
      console.log('✓ Clicked Launcher tab');
      await sleep(500);

      const route = await client.sendCommand('execute_js', {
        window_label: 'main',
        code: 'window.location.pathname'
      });
      console.log(`  New route: ${route}\n`);
    } catch (e) {
      console.log(`✗ Failed: ${e.message}\n`);
    }

    console.log('=== Navigation Test Complete ===');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  } finally {
    client.disconnect();
  }
}

main();
