/**
 * Tauri API wrapper with automatic fallback to mock for browser testing
 *
 * This module provides a unified invoke() function that:
 * - Uses real Tauri API when running in Tauri
 * - Falls back to mock implementation when running in browser (E2E tests)
 */

import { mockInvoke, installTauriMock } from './tauri-mock';

// Install mock immediately if not in Tauri
if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
  installTauriMock();
}

/**
 * Check if we're running in REAL Tauri environment (not mock)
 *
 * The mock creates __TAURI__ but it won't have __TAURI_INTERNALS__
 * which is created by the real Tauri runtime
 */
function isTauri(): boolean {
  return (
    typeof window !== 'undefined' &&
    '__TAURI_INTERNALS__' in window &&
    window.__TAURI_INTERNALS__ !== undefined
  );
}

/**
 * Invoke a Tauri command
 *
 * Automatically uses mock implementation when running in browser
 */
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  // Use mock for browser testing (must check first before any imports)
  if (!isTauri()) {
    return mockInvoke(command, args) as Promise<T>;
  }

  // Use real Tauri API
  const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
  return tauriInvoke<T>(command, args);
}
