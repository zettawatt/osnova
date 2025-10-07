import { writable } from 'svelte/store';
import { invoke } from '$lib/utils/tauri';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  resolvedTheme: 'light' | 'dark';
}

function createThemeStore() {
  const { subscribe, set, update } = writable<ThemeState>({
    theme: 'system',
    resolvedTheme: 'light'
  });

  // Detect system theme preference
  function getSystemTheme(): 'light' | 'dark' {
    if (typeof window !== 'undefined' && window.matchMedia) {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return 'light';
  }

  // Resolve theme to actual light/dark value
  function resolveTheme(theme: Theme): 'light' | 'dark' {
    return theme === 'system' ? getSystemTheme() : theme;
  }

  // Apply theme to document
  function applyTheme(resolvedTheme: 'light' | 'dark') {
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', resolvedTheme);
    }
  }

  return {
    subscribe,

    /**
     * Initialize theme from backend
     */
    async init() {
      try {
        const theme = (await invoke('ui_get_theme')) as Theme;
        const resolvedTheme = resolveTheme(theme);
        applyTheme(resolvedTheme);
        set({ theme, resolvedTheme });

        // Listen for system theme changes
        if (typeof window !== 'undefined' && window.matchMedia) {
          const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
          mediaQuery.addEventListener('change', (e) => {
            update((state) => {
              if (state.theme === 'system') {
                const newResolved = e.matches ? 'dark' : 'light';
                applyTheme(newResolved);
                return { ...state, resolvedTheme: newResolved };
              }
              return state;
            });
          });
        }
      } catch (error) {
        console.error('Failed to load theme:', error);
        // Fall back to system theme
        const resolvedTheme = getSystemTheme();
        applyTheme(resolvedTheme);
        set({ theme: 'system', resolvedTheme });
      }
    },

    /**
     * Set theme and persist to backend
     */
    async setTheme(theme: Theme) {
      try {
        await invoke('ui_set_theme', { theme });
        const resolvedTheme = resolveTheme(theme);
        applyTheme(resolvedTheme);
        set({ theme, resolvedTheme });
      } catch (error) {
        console.error('Failed to set theme:', error);
      }
    },

    /**
     * Get current resolved theme (light or dark)
     */
    getResolved(): 'light' | 'dark' {
      let resolved: 'light' | 'dark' = 'light';
      subscribe((state) => {
        resolved = state.resolvedTheme;
      })();
      return resolved;
    }
  };
}

export const themeStore = createThemeStore();
