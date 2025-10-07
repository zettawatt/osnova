<script lang="ts">
  import { onMount } from 'svelte';
  import { identityStore } from '$lib/stores/identity';
  import { navigationStore, type BottomMenuTab } from '$lib/stores/navigation';
  import BottomMenu from '$lib/components/BottomMenu.svelte';
  import LauncherScreen from '$lib/screens/LauncherScreen.svelte';
  import WalletScreen from '$lib/screens/WalletScreen.svelte';
  import ConfigScreen from '$lib/screens/ConfigScreen.svelte';
  import DeploymentScreen from '$lib/screens/DeploymentScreen.svelte';
  import OnboardingScreen from '$lib/screens/OnboardingScreen.svelte';
  import CreateIdentityFlow from '$lib/components/CreateIdentityFlow.svelte';
  import ImportIdentityFlow from '$lib/components/ImportIdentityFlow.svelte';

  let hasIdentity = $state(false);
  let activeTab = $state<BottomMenuTab>('launcher');
  let checkingIdentity = $state(true);

  $effect(() => {
    const unsubIdentity = identityStore.subscribe((state) => {
      hasIdentity = state.hasIdentity;
    });

    const unsubNav = navigationStore.subscribe((state) => {
      activeTab = state.activeTab;
    });

    return () => {
      unsubIdentity();
      unsubNav();
    };
  });

  onMount(async () => {
    // Check if identity exists
    await identityStore.checkIdentity();
    checkingIdentity = false;

    // If identity exists, load active tab
    if (hasIdentity) {
      await navigationStore.loadActiveTab();
    }
  });

  async function handleOnboardingComplete() {
    // Identity has been created/imported, reload state
    await identityStore.checkIdentity();

    // Load navigation state
    await navigationStore.loadActiveTab();
  }
</script>

{#if checkingIdentity}
  <!-- Loading screen while checking identity -->
  <div class="loading-screen">
    <div class="spinner"></div>
    <p>Loading...</p>
  </div>
{:else if !hasIdentity}
  <!-- Show onboarding if no identity -->
  <OnboardingScreen oncomplete={handleOnboardingComplete}>
    {#snippet create(handleComplete: () => void)}
      <CreateIdentityFlow onComplete={handleComplete} />
    {/snippet}

    {#snippet importIdentity(handleComplete: () => void)}
      <ImportIdentityFlow onComplete={handleComplete} />
    {/snippet}
  </OnboardingScreen>
{:else}
  <!-- Show main app once identity exists -->
  <div class="app">
    <!-- Screen content based on active tab -->
    {#if activeTab === 'launcher'}
      <LauncherScreen />
    {:else if activeTab === 'wallet'}
      <WalletScreen />
    {:else if activeTab === 'deployment'}
      <DeploymentScreen />
    {:else if activeTab === 'config'}
      <ConfigScreen />
    {/if}

    <!-- Bottom navigation menu -->
    <BottomMenu />
  </div>
{/if}

<style>
  .app {
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .loading-screen {
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    background-color: var(--color-bg-primary);
  }

  .spinner {
    width: 3rem;
    height: 3rem;
    border: 4px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .loading-screen p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
  }
</style>
