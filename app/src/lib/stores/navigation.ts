import { writable } from 'svelte/store';
import { invoke } from '$lib/utils/tauri';

export type BottomMenuTab = 'launcher' | 'wallet' | 'config' | 'deployment';

interface NavigationState {
  activeTab: BottomMenuTab;
  loading: boolean;
}

function createNavigationStore() {
  const { subscribe, set, update } = writable<NavigationState>({
    activeTab: 'launcher',
    loading: false
  });

  return {
    subscribe,

    /**
     * Load active tab from backend
     */
    async loadActiveTab() {
      update((state) => ({ ...state, loading: true }));

      try {
        const tab = (await invoke('navigation_get_bottom_menu')) as BottomMenuTab;
        set({ activeTab: tab, loading: false });
      } catch (error) {
        console.error('Failed to load active tab:', error);
        set({ activeTab: 'launcher', loading: false });
      }
    },

    /**
     * Set active tab and persist to backend
     */
    async setActiveTab(tab: BottomMenuTab) {
      try {
        await invoke('navigation_set_bottom_menu', { tab });
        update((state) => ({ ...state, activeTab: tab }));
      } catch (error) {
        console.error('Failed to set active tab:', error);
      }
    }
  };
}

export const navigationStore = createNavigationStore();
