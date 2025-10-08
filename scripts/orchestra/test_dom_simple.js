#!/usr/bin/env node

// Simplified test for just DOM access
import net from 'net';

const SOCKET_PATH = '/tmp/osnova-tauri-mcp.sock';

async function testDom() {
  const client = net.createConnection({ path: SOCKET_PATH }, () => {
    console.log('✓ Connected to socket');

    // Send get_dom command
    const request = JSON.stringify({
      command: 'get_dom',
      payload: { window_label: 'main' }
    }) + '\n';

    console.log('→ Sending get_dom command...');
    client.write(request);
  });

  client.on('data', (data) => {
    console.log('← Received response:', data.toString().substring(0, 200));
    client.end();
    process.exit(0);
  });

  client.on('error', (err) => {
    console.error('✗ Socket error:', err.message);
    process.exit(1);
  });

  client.on('timeout', () => {
    console.error('✗ Timeout waiting for response');
    client.end();
    process.exit(1);
  });

  client.setTimeout(5000);
}

testDom();
