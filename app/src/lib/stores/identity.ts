import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

interface IdentityState {
  hasIdentity: boolean;
  identityId: string | null;
  loading: boolean;
  error: string | null;
}

function createIdentityStore() {
  const { subscribe, set, update } = writable<IdentityState>({
    hasIdentity: false,
    identityId: null,
    loading: false,
    error: null
  });

  return {
    subscribe,

    /**
     * Check if identity exists
     */
    async checkIdentity() {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const identityId = (await invoke('identity_get')) as string;
        set({
          hasIdentity: true,
          identityId,
          loading: false,
          error: null
        });
        return true;
      } catch (error) {
        // No identity found is expected on first run
        set({
          hasIdentity: false,
          identityId: null,
          loading: false,
          error: null
        });
        return false;
      }
    },

    /**
     * Create new identity
     * @returns seed phrase for backup
     */
    async createIdentity(): Promise<string> {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const seedPhrase = (await invoke('identity_create')) as string;
        const identityId = (await invoke('identity_get')) as string;

        set({
          hasIdentity: true,
          identityId,
          loading: false,
          error: null
        });

        return seedPhrase;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to create identity';
        update((state) => ({ ...state, loading: false, error: errorMessage }));
        throw error;
      }
    },

    /**
     * Import identity from seed phrase
     */
    async importIdentity(seedPhrase: string): Promise<void> {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const identityId = (await invoke('identity_import', { seedPhrase })) as string;

        set({
          hasIdentity: true,
          identityId,
          loading: false,
          error: null
        });
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to import identity';
        update((state) => ({ ...state, loading: false, error: errorMessage }));
        throw error;
      }
    },

    /**
     * Clear error state
     */
    clearError() {
      update((state) => ({ ...state, error: null }));
    }
  };
}

export const identityStore = createIdentityStore();
