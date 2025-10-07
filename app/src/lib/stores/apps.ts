import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AppListItem {
  id: string;
  name: string;
  version: string;
  icon_uri: string;
  manifest_uri: string;
}

interface AppsState {
  apps: AppListItem[];
  loading: boolean;
  error: string | null;
}

function createAppsStore() {
  const { subscribe, set, update } = writable<AppsState>({
    apps: [],
    loading: false,
    error: null
  });

  return {
    subscribe,

    /**
     * Load all installed applications from backend
     */
    async loadApps() {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const appsJson = (await invoke('apps_list')) as string;
        const apps = JSON.parse(appsJson) as AppListItem[];
        set({ apps, loading: false, error: null });
      } catch (error) {
        console.error('Failed to load apps:', error);
        set({
          apps: [],
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to load apps'
        });
      }
    },

    /**
     * Launch an application by ID
     */
    async launchApp(appId: string) {
      try {
        await invoke('apps_launch', { appId });
      } catch (error) {
        console.error('Failed to launch app:', error);
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

export const appsStore = createAppsStore();
