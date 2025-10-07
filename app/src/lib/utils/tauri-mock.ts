/**
 * Mock implementation of Tauri API for browser-based E2E testing
 *
 * This allows the app to run in a regular browser during Playwright tests
 * by providing stub implementations of Tauri commands.
 */

// In-memory storage for mock data
const mockStorage = {
  theme: 'system' as 'light' | 'dark' | 'system',
  activeTab: 'launcher' as string,
  layout: ['test-app-1', 'test-app-2'] as string[],
  apps: [
    {
      id: 'test-app-1',
      name: 'Test App',
      version: '1.0.0',
      icon_uri: null,
      manifest_uri: 'test://manifest1'
    },
    {
      id: 'test-app-2',
      name: 'Another App',
      version: '2.0.0',
      icon_uri: null,
      manifest_uri: 'test://manifest2'
    },
    {
      id: 'test-app-3',
      name: 'Sample Application',
      version: '3.1.4',
      icon_uri: null,
      manifest_uri: 'test://manifest3'
    }
  ],
  hasIdentity: true,  // Start with identity for testing launcher
  identityId: 'mock-test-identity-123' as string | null,
  seedPhrase: null as string | null
};

/**
 * Mock invoke function that simulates Tauri backend responses
 */
export async function mockInvoke(command: string, args?: Record<string, unknown>): Promise<unknown> {
  console.log('[MOCK] Tauri command:', command, args);

  switch (command) {
    // Theme commands
    case 'ui_get_theme':
      return mockStorage.theme;

    case 'ui_set_theme':
      if (args?.theme) {
        mockStorage.theme = args.theme as typeof mockStorage.theme;
      }
      return null;

    // Navigation commands
    case 'navigation_get_active_tab':
      return mockStorage.activeTab;

    case 'navigation_set_active_tab':
      if (args?.tab) {
        mockStorage.activeTab = args.tab as string;
      }
      return null;

    // Launcher commands
    case 'launcher_get_layout':
      return JSON.stringify(mockStorage.layout);

    case 'launcher_save_layout':
      if (args?.layout && Array.isArray(args.layout)) {
        mockStorage.layout = args.layout as string[];
      }
      return null;

    // Apps commands
    case 'apps_list':
      return JSON.stringify(mockStorage.apps);

    case 'apps_launch':
      console.log('[MOCK] Launching app:', args?.app_id);
      return null;

    // Identity commands
    case 'identity_check':
      return mockStorage.hasIdentity;

    case 'identity_create':
      mockStorage.hasIdentity = true;
      mockStorage.identityId = 'mock-identity-' + Date.now();
      mockStorage.seedPhrase = 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
      return mockStorage.seedPhrase;

    case 'identity_import':
      if (args?.seed_phrase) {
        mockStorage.hasIdentity = true;
        mockStorage.identityId = 'mock-identity-imported-' + Date.now();
        mockStorage.seedPhrase = args.seed_phrase as string;
      }
      return null;

    case 'identity_get':
      return mockStorage.identityId;

    // Status commands
    case 'status_server_info':
      return null; // Not running in server mode

    default:
      console.warn('[MOCK] Unhandled Tauri command:', command);
      return null;
  }
}

/**
 * Install mock Tauri API for browser testing
 */
export function installTauriMock() {
  // Check if we're in a browser (not Tauri)
  if (typeof window !== 'undefined') {
    // Always install the mock, even if __TAURI__ exists (it might be incomplete)
    console.log('[MOCK] Installing Tauri API mock for browser testing');

    // Create mock Tauri API structure
    (window as any).__TAURI__ = {
      core: {
        invoke: mockInvoke
      },
      invoke: mockInvoke  // Also at top level for compatibility
    };

    // Also expose for direct imports
    (window as any).__TAURI_INVOKE__ = mockInvoke;
  }
}

/**
 * Reset mock storage to initial state
 */
export function resetMockStorage() {
  mockStorage.theme = 'system';
  mockStorage.activeTab = 'launcher';
  mockStorage.layout = [];
  mockStorage.apps = [
    {
      id: 'test-app-1',
      name: 'Test App',
      version: '1.0.0',
      icon_uri: null,
      manifest_uri: 'test://manifest1'
    },
    {
      id: 'test-app-2',
      name: 'Another App',
      version: '2.0.0',
      icon_uri: null,
      manifest_uri: 'test://manifest2'
    }
  ];
  mockStorage.hasIdentity = false;
  mockStorage.identityId = null;
  mockStorage.seedPhrase = null;
}

/**
 * Expose mock storage for test assertions
 */
export function getMockStorage() {
  return mockStorage;
}
