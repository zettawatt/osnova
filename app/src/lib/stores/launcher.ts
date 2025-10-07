import { writable } from 'svelte/store';
import { invoke } from '$lib/utils/tauri';

interface LauncherState {
  layout: string[];
  loading: boolean;
  error: string | null;
}

function createLauncherStore() {
  const { subscribe, set, update } = writable<LauncherState>({
    layout: [],
    loading: false,
    error: null
  });

  return {
    subscribe,

    /**
     * Load launcher layout from backend
     */
    async loadLayout() {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const layoutJson = (await invoke('launcher_get_layout')) as string;
        const layout = JSON.parse(layoutJson) as string[];
        set({ layout, loading: false, error: null });
      } catch (error) {
        console.error('Failed to load layout:', error);
        set({
          layout: [],
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to load layout'
        });
      }
    },

    /**
     * Save launcher layout to backend
     */
    async saveLayout(layout: string[]) {
      try {
        await invoke('launcher_set_layout', { appIds: layout });
        update((state) => ({ ...state, layout }));
      } catch (error) {
        console.error('Failed to save layout:', error);
        throw error;
      }
    },

    /**
     * Reorder apps in the layout
     */
    async reorderApps(fromIndex: number, toIndex: number) {
      update((state) => {
        const newLayout = [...state.layout];
        const [removed] = newLayout.splice(fromIndex, 1);
        newLayout.splice(toIndex, 0, removed);

        // Save to backend asynchronously
        invoke('launcher_set_layout', { appIds: newLayout }).catch((error) => {
          console.error('Failed to save reordered layout:', error);
        });

        return { ...state, layout: newLayout };
      });
    },

    /**
     * Add app to layout
     */
    async addToLayout(appId: string) {
      update((state) => {
        if (state.layout.includes(appId)) {
          return state;
        }

        const newLayout = [...state.layout, appId];

        // Save to backend asynchronously
        invoke('launcher_set_layout', { appIds: newLayout }).catch((error) => {
          console.error('Failed to save layout:', error);
        });

        return { ...state, layout: newLayout };
      });
    },

    /**
     * Remove app from layout
     */
    async removeFromLayout(appId: string) {
      update((state) => {
        const newLayout = state.layout.filter((id) => id !== appId);

        // Save to backend asynchronously
        invoke('launcher_set_layout', { appIds: newLayout }).catch((error) => {
          console.error('Failed to save layout:', error);
        });

        return { ...state, layout: newLayout };
      });
    },

    /**
     * Clear error state
     */
    clearError() {
      update((state) => ({ ...state, error: null }));
    }
  };
}

export const launcherStore = createLauncherStore();
