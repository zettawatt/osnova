<script lang="ts">
  import { appsStore, type AppListItem } from '$lib/stores/apps';
  import { launcherStore } from '$lib/stores/launcher';
  import AppIcon from './AppIcon.svelte';

  let apps = $state<AppListItem[]>([]);
  let layout = $state<string[]>([]);
  let loading = $state(false);

  // Subscribe to stores
  $effect(() => {
    const unsubApps = appsStore.subscribe((state) => {
      apps = state.apps;
      loading = state.loading;
    });

    const unsubLayout = launcherStore.subscribe((state) => {
      layout = state.layout;
    });

    return () => {
      unsubApps();
      unsubLayout();
    };
  });

  // Sort apps according to layout, with unlisted apps at the end
  let sortedApps = $derived(() => {
    const appsMap = new Map(apps.map((app) => [app.id, app]));
    const layoutApps = layout
      .map((id) => appsMap.get(id))
      .filter((app): app is AppListItem => app !== undefined);

    const unlistedApps = apps.filter((app) => !layout.includes(app.id));

    return [...layoutApps, ...unlistedApps];
  });

  async function handleLaunchApp(appId: string) {
    try {
      await appsStore.launchApp(appId);
    } catch (error) {
      console.error('Failed to launch app:', error);
      // TODO: Show error toast
    }
  }
</script>

<div class="app-grid-container">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading apps...</p>
    </div>
  {:else if apps.length === 0}
    <div class="empty-state">
      <div class="empty-icon">📦</div>
      <h3>No Apps Installed</h3>
      <p>Install apps from the marketplace to get started.</p>
    </div>
  {:else}
    <div class="app-grid">
      {#each sortedApps() as app (app.id)}
        <AppIcon {app} size="md" onclick={() => handleLaunchApp(app.id)} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .app-grid-container {
    width: 100%;
    min-height: 300px;
    padding: var(--spacing-lg);
  }

  .app-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(5rem, 1fr));
    gap: var(--spacing-xl);
    justify-items: center;
  }

  /* Loading state */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    min-height: 300px;
    color: var(--color-text-secondary);
  }

  .spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Empty state */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    min-height: 300px;
    text-align: center;
    color: var(--color-text-secondary);
  }

  .empty-icon {
    font-size: 4rem;
    opacity: 0.5;
  }

  .empty-state h3 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .empty-state p {
    font-size: var(--font-size-base);
    margin: 0;
    max-width: 400px;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .app-grid {
      grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
      gap: var(--spacing-lg);
    }

    .app-grid-container {
      padding: var(--spacing-md);
    }
  }

  @media (min-width: 1024px) {
    .app-grid {
      grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
      gap: var(--spacing-2xl);
    }
  }
</style>
