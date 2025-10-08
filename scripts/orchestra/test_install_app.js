#!/usr/bin/env node

// Test Install App button behavior
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

async function getPageContent(client) {
  const dom = await client.sendCommand('get_dom', { window_label: 'main' });
  const domStr = typeof dom === 'string' ? dom : JSON.stringify(dom);
  return domStr.includes('"<html') ? JSON.parse(domStr) : domStr;
}

async function main() {
  const client = new TauriClient();

  try {
    await client.connect();
    console.log('✓ Connected to Tauri app\n');

    // Get initial state
    console.log('=== Before Clicking "Install App" ===');
    const dom1 = await getPageContent(client);
    console.log(`Page contains "Install App" dialog: ${dom1.includes('Are you sure') || dom1.includes('install this app')}`);
    console.log(`Page has dialogs: ${dom1.includes('dialog')}`);
    console.log();

    // Click "Install App" button
    console.log('=== Clicking "Install App" Button ===');
    try {
      const result = await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Install App',
        should_click: true
      });
      console.log('✓ Successfully clicked "Install App" button');
      console.log(`  Element clicked: ${result.element.tag}`);
    } catch (e) {
      console.log(`✗ Failed to click: ${e.message}`);
    }

    // Wait for UI to update
    await sleep(1000);

    // Check if dialog appeared
    console.log('\n=== After Clicking "Install App" ===');
    const dom2 = await getPageContent(client);

    const hasDialog = dom2.includes('dialog-backdrop') || dom2.includes('Install Application');
    const hasInstallDialog = dom2.includes('Install Application') || dom2.includes('Are you sure');
    const hasInput = dom2.includes('<input') && dom2.includes('placeholder');

    console.log(`Dialog appeared: ${hasDialog ? 'YES ✓' : 'NO ✗'}`);
    console.log(`Install dialog visible: ${hasInstallDialog ? 'YES ✓' : 'NO ✗'}`);
    console.log(`Has input fields: ${hasInput ? 'YES ✓' : 'NO ✗'}`);

    if (hasInstallDialog) {
      console.log('\n✓ CONFIRMED: Dialog opened when "Install App" was clicked!');

      // Try to find the input field
      if (hasInput) {
        console.log('\n=== Dialog has input field for app manifest URL ===');
      }
    } else {
      console.log('\n✗ WARNING: No dialog appeared after clicking "Install App"');
    }

    console.log('\n=== Test Complete ===');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  } finally {
    client.disconnect();
  }
}

main();
