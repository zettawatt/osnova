<script lang="ts">
  import { onMount } from 'svelte';
  import { themeStore } from '$lib/stores/theme';
  import '$lib/styles/theme.css';

  onMount(async () => {
    // Initialize theme from backend on app start
    themeStore.init();

    // Setup MCP plugin listeners for E2E testing (debug builds only)
    if (import.meta.env.DEV) {
      try {
        const { setupPluginListeners } = await import('tauri-plugin-mcp');
        await setupPluginListeners();
        console.log('MCP plugin listeners initialized');
      } catch (e) {
        console.warn('Failed to initialize MCP plugin listeners:', e);
      }
    }
  });
</script>

<slot />
