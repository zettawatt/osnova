<script lang="ts">
  import { onMount } from 'svelte';
  import { themeStore } from '$lib/stores/theme';
  import '$lib/styles/theme.css';

  onMount(() => {
    // Initialize theme from backend on app start
    themeStore.init();
  });
</script>

<slot />
