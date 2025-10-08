#!/usr/bin/env node

// Test button clicking and screen changes
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
        console.log('✓ Connected to Tauri app');
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

async function getPageButtons(client) {
  const dom = await client.sendCommand('get_dom', { window_label: 'main' });
  const domStr = typeof dom === 'string' ? dom : JSON.stringify(dom);
  const parsedDom = domStr.includes('"<html') ? JSON.parse(domStr) : domStr;

  // Extract button text using regex
  const buttonRegex = /<button[^>]*>(.*?)<\/button>/gi;
  const buttons = [];
  let match;
  while ((match = buttonRegex.exec(parsedDom)) !== null) {
    buttons.push(match[1].trim());
  }

  return { dom: parsedDom, buttons };
}

async function getCurrentRoute(client) {
  const result = await client.sendCommand('execute_js', {
    window_label: 'main',
    code: 'window.location.pathname',
    timeout_ms: 3000
  });
  // Handle different response formats
  if (typeof result === 'string') {
    try {
      const parsed = JSON.parse(result);
      return parsed.result || parsed;
    } catch {
      return result;
    }
  }
  return result.result || result;
}

async function main() {
  const client = new TauriClient();

  try {
    await client.connect();

    // Check initial state
    console.log('\n=== Initial State ===');
    const route1 = await getCurrentRoute(client);
    console.log(`Route: ${route1}`);

    const page1 = await getPageButtons(client);
    console.log(`Buttons found: ${page1.buttons.join(', ')}`);
    console.log(`Contains "Create Identity": ${page1.dom.includes('Create Identity')}`);
    console.log(`Contains "Import Identity": ${page1.dom.includes('Import Identity')}`);
    console.log(`Contains "Install App": ${page1.dom.includes('Install App')}`);

    // Try to find and click "Create Identity" button
    console.log('\n=== Clicking "Create Identity" ===');
    try {
      const clickResult = await client.sendCommand('get_element_position', {
        window_label: 'main',
        selector_type: 'text',
        selector_value: 'Create Identity',
        should_click: true
      });
      console.log('✓ Clicked "Create Identity"');
      console.log(`  Click result: ${JSON.stringify(clickResult, null, 2)}`);
    } catch (e) {
      console.log(`✗ Failed to click "Create Identity": ${e.message}`);
    }

    // Wait for UI to update
    await sleep(1000);

    // Check state after click
    console.log('\n=== State After Click ===');
    const route2 = await getCurrentRoute(client);
    console.log(`Route: ${route2}`);

    const page2 = await getPageButtons(client);
    console.log(`Buttons found: ${page2.buttons.join(', ')}`);
    console.log(`Contains seed phrase area: ${page2.dom.includes('seed') || page2.dom.includes('phrase')}`);

    // Try to find "Install App" button
    console.log('\n=== Looking for "Install App" button ===');
    if (page2.dom.includes('Install App')) {
      console.log('✓ "Install App" button found');

      try {
        const clickResult = await client.sendCommand('get_element_position', {
          window_label: 'main',
          selector_type: 'text',
          selector_value: 'Install App',
          should_click: true
        });
        console.log('✓ Clicked "Install App"');
        console.log(`  Click result: ${JSON.stringify(clickResult, null, 2)}`);

        // Wait for navigation
        await sleep(1000);

        const route3 = await getCurrentRoute(client);
        console.log(`\nRoute after "Install App": ${route3}`);

        const page3 = await getPageButtons(client);
        console.log(`New page buttons: ${page3.buttons.join(', ')}`);
      } catch (e) {
        console.log(`✗ Failed to click "Install App": ${e.message}`);
      }
    } else {
      console.log('✗ "Install App" button NOT found on current page');
    }

    console.log('\n=== Test Complete ===\n');

  } catch (error) {
    console.error('\n❌ Test failed:', error.message);
    process.exit(1);
  } finally {
    client.disconnect();
  }
}

main();
