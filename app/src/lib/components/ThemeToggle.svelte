<script lang="ts">
  import { themeStore, type Theme } from '$lib/stores/theme';

  let currentTheme = $state<Theme>('system');

  $effect(() => {
    const unsubscribe = themeStore.subscribe((state) => {
      currentTheme = state.theme;
    });
    return unsubscribe;
  });

  async function toggleTheme() {
    const themes: Theme[] = ['light', 'dark', 'system'];
    const currentIndex = themes.indexOf(currentTheme);
    const nextTheme = themes[(currentIndex + 1) % themes.length];
    await themeStore.setTheme(nextTheme);
  }

  function getThemeIcon(theme: Theme): string {
    switch (theme) {
      case 'light':
        return '☀️';
      case 'dark':
        return '🌙';
      case 'system':
        return '💻';
    }
  }

  function getThemeLabel(theme: Theme): string {
    return theme.charAt(0).toUpperCase() + theme.slice(1);
  }
</script>

<button
  class="theme-toggle"
  onclick={toggleTheme}
  title={`Current theme: ${getThemeLabel(currentTheme)}`}
  aria-label="Toggle theme"
>
  <span class="icon">{getThemeIcon(currentTheme)}</span>
  <span class="label">{getThemeLabel(currentTheme)}</span>
</button>

<style>
  .theme-toggle {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-secondary);
    color: var(--color-text-primary);
    font-family: var(--font-sans);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    cursor: pointer;
    transition: all var(--transition-fast);
    box-shadow: var(--shadow-sm);
  }

  .theme-toggle:hover {
    border-color: var(--color-border-hover);
    background-color: var(--color-bg-tertiary);
    box-shadow: var(--shadow-md);
  }

  .theme-toggle:active {
    transform: scale(0.98);
  }

  .icon {
    font-size: var(--font-size-lg);
    line-height: 1;
  }

  .label {
    user-select: none;
  }
</style>
