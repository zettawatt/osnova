<script lang="ts">
  import { onMount } from 'svelte';
  import { appsStore, type AppListItem } from '$lib/stores/apps';
  import { launcherStore } from '$lib/stores/launcher';
  import AppGrid from '$lib/components/AppGrid.svelte';
  import Button from '$lib/components/Button.svelte';
  import AppInstallDialog from '$lib/components/AppInstallDialog.svelte';
  import AppUninstallDialog from '$lib/components/AppUninstallDialog.svelte';

  let error = $state<string | null>(null);
  let showInstallDialog = $state(false);
  let showUninstallDialog = $state(false);
  let appToUninstall = $state<AppListItem | null>(null);

  onMount(async () => {
    // Load apps and layout on mount
    try {
      await Promise.all([appsStore.loadApps(), launcherStore.loadLayout()]);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load launcher';
      console.error('Failed to load launcher:', err);
    }
  });

  async function handleRefresh() {
    error = null;
    try {
      await Promise.all([appsStore.loadApps(), launcherStore.loadLayout()]);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to refresh';
    }
  }

  function handleDismissError() {
    error = null;
    appsStore.clearError();
    launcherStore.clearError();
  }

  function handleInstallClick() {
    showInstallDialog = true;
  }

  function handleInstallClose() {
    showInstallDialog = false;
  }

  function handleInstallSuccess() {
    showInstallDialog = false;
    // Apps will be reloaded by the dialog
  }

  function handleUninstallRequest(app: AppListItem) {
    appToUninstall = app;
    showUninstallDialog = true;
  }

  function handleUninstallClose() {
    showUninstallDialog = false;
    appToUninstall = null;
  }

  function handleUninstallSuccess() {
    showUninstallDialog = false;
    appToUninstall = null;
    // Apps will be reloaded by the dialog
  }
</script>

<div class="launcher-screen">
  <header class="launcher-header">
    <h1>My Apps</h1>
    <div class="actions">
      <Button variant="primary" size="sm" onclick={handleInstallClick}>
        <span>➕</span>
        Install App
      </Button>
      <Button variant="ghost" size="sm" onclick={handleRefresh}>
        <span>🔄</span>
        Refresh
      </Button>
    </div>
  </header>

  {#if error}
    <div class="error-banner">
      <div class="error-content">
        <span class="error-icon">⚠️</span>
        <span class="error-message">{error}</span>
      </div>
      <Button variant="ghost" size="sm" onclick={handleDismissError}>
        Dismiss
      </Button>
    </div>
  {/if}

  <main class="launcher-content">
    <AppGrid onUninstallRequest={handleUninstallRequest} />
  </main>

  {#if showInstallDialog}
    <AppInstallDialog onClose={handleInstallClose} onSuccess={handleInstallSuccess} />
  {/if}

  {#if showUninstallDialog && appToUninstall}
    <AppUninstallDialog
      app={appToUninstall}
      onClose={handleUninstallClose}
      onSuccess={handleUninstallSuccess}
    />
  {/if}
</div>

<style>
  .launcher-screen {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: var(--color-bg-primary);
    padding-bottom: 70px; /* Space for bottom menu */
  }

  .launcher-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-lg) var(--spacing-xl);
    background-color: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .launcher-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .actions {
    display: flex;
    gap: var(--spacing-sm);
  }

  .error-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md) var(--spacing-xl);
    background-color: rgba(239, 68, 68, 0.1);
    border-bottom: 1px solid var(--color-error);
    animation: slideDown 0.3s ease-out;
  }

  @keyframes slideDown {
    from {
      transform: translateY(-100%);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .error-content {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
  }

  .error-icon {
    font-size: var(--font-size-xl);
  }

  .error-message {
    color: var(--color-error);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
  }

  .launcher-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  /* Mobile adjustments */
  @media (max-width: 768px) {
    .launcher-header {
      padding: var(--spacing-md);
    }

    .launcher-header h1 {
      font-size: var(--font-size-xl);
    }

    .error-banner {
      padding: var(--spacing-sm) var(--spacing-md);
      flex-direction: column;
      gap: var(--spacing-sm);
      align-items: flex-start;
    }
  }
</style>
