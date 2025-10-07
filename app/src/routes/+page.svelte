<script lang="ts">
  import { onMount } from 'svelte';
  import { navigationStore, type BottomMenuTab } from '$lib/stores/navigation';
  import BottomMenu from '$lib/components/BottomMenu.svelte';
  import LauncherScreen from '$lib/screens/LauncherScreen.svelte';
  import WalletScreen from '$lib/screens/WalletScreen.svelte';
  import ConfigScreen from '$lib/screens/ConfigScreen.svelte';

  let activeTab = $state<BottomMenuTab>('launcher');

  $effect(() => {
    const unsubscribe = navigationStore.subscribe((state) => {
      activeTab = state.activeTab;
    });
    return unsubscribe;
  });

  onMount(async () => {
    // Load active tab from backend
    await navigationStore.loadActiveTab();
  });
</script>

<div class="app">
  <!-- Screen content based on active tab -->
  {#if activeTab === 'launcher'}
    <LauncherScreen />
  {:else if activeTab === 'wallet'}
    <WalletScreen />
  {:else if activeTab === 'config'}
    <ConfigScreen />
  {/if}

  <!-- Bottom navigation menu -->
  <BottomMenu />
</div>

<style>
  .app {
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }
</style>
