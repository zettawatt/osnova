<script lang="ts">
  import type { AppListItem } from '$lib/stores/apps';
  import { appsStore } from '$lib/stores/apps';
  import Button from './Button.svelte';
  import Card from './Card.svelte';

  interface AppUninstallDialogProps {
    app: AppListItem;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { app, onClose, onSuccess }: AppUninstallDialogProps = $props();

  let loading = $state(false);
  let error = $state<string | null>(null);

  async function handleUninstall() {
    loading = true;
    error = null;

    try {
      // Note: This calls the backend which has a stub implementation
      // The actual uninstall logic (deleting cached components) is TODO
      await fetch(`http://localhost:3000/uninstall/${app.id}`, {
        method: 'DELETE'
      });

      // Reload apps list
      await appsStore.loadApps();

      onSuccess();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to uninstall app';
      console.error('Failed to uninstall app:', err);
      loading = false;
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
</script>

<div class="dialog-backdrop" onclick={handleBackdropClick} role="presentation">
  <div class="dialog" role="dialog" aria-labelledby="dialog-title" aria-modal="true">
    <Card variant="elevated" padding="lg">
      <div class="dialog-content">
        <div class="dialog-header">
          <h2 id="dialog-title">Uninstall Application</h2>
          <button class="close-button" onclick={onClose} aria-label="Close dialog">
            ✕
          </button>
        </div>

        <div class="dialog-body">
          <div class="app-info">
            <div class="app-icon-large">
              {#if app.icon_uri}
                <img src={app.icon_uri} alt={app.name} />
              {:else}
                <div class="fallback-icon">
                  {app.name
                    .split(' ')
                    .map((w) => w[0])
                    .join('')
                    .toUpperCase()
                    .slice(0, 2)}
                </div>
              {/if}
            </div>
            <div class="app-details">
              <h3>{app.name}</h3>
              <p class="app-version">Version {app.version}</p>
              <p class="app-id">{app.id}</p>
            </div>
          </div>

          <div class="warning-box">
            <span class="warning-icon">⚠️</span>
            <div>
              <p><strong>Are you sure you want to uninstall this app?</strong></p>
              <p>This will remove the app and all its cached data. This action cannot be undone.</p>
            </div>
          </div>

          {#if error}
            <div class="error-box">
              <span class="error-icon">✕</span>
              <p>{error}</p>
            </div>
          {/if}
        </div>

        <div class="dialog-footer">
          <Button variant="ghost" onclick={onClose} disabled={loading}>
            Cancel
          </Button>
          <Button variant="danger" onclick={handleUninstall} loading={loading}>
            {loading ? 'Uninstalling...' : 'Uninstall'}
          </Button>
        </div>
      </div>
    </Card>
  </div>
</div>

<style>
  .dialog-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: var(--z-modal-backdrop);
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .dialog {
    max-width: 450px;
    width: 90%;
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .dialog-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .dialog-header h2 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .close-button {
    background: none;
    border: none;
    font-size: var(--font-size-xl);
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: var(--spacing-xs);
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast);
  }

  .close-button:hover {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }

  .dialog-body {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .app-info {
    display: flex;
    gap: var(--spacing-md);
    align-items: center;
    padding: var(--spacing-md);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .app-icon-large {
    width: 4rem;
    height: 4rem;
    border-radius: var(--radius-md);
    overflow: hidden;
    flex-shrink: 0;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
  }

  .app-icon-large img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .fallback-icon {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: var(--font-weight-bold);
    font-size: var(--font-size-lg);
    background: linear-gradient(135deg, var(--color-accent), var(--color-accent-hover));
    color: white;
  }

  .app-details {
    flex: 1;
    min-width: 0;
  }

  .app-details h3 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .app-version {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .app-id {
    font-size: var(--font-size-xs);
    font-family: var(--font-mono);
    color: var(--color-text-tertiary);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .warning-box {
    display: flex;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: rgba(251, 191, 36, 0.1);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
  }

  .warning-icon {
    font-size: var(--font-size-xl);
    flex-shrink: 0;
  }

  .warning-box p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .warning-box p:last-child {
    margin-bottom: 0;
  }

  .warning-box strong {
    color: var(--color-text-primary);
  }

  .error-box {
    display: flex;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
  }

  .error-icon {
    font-size: var(--font-size-base);
    color: var(--color-error);
    flex-shrink: 0;
  }

  .error-box p {
    font-size: var(--font-size-sm);
    color: var(--color-error);
    margin: 0;
  }

  .dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--spacing-sm);
  }

  @media (max-width: 768px) {
    .dialog {
      width: 95%;
    }

    .dialog-footer {
      flex-direction: column-reverse;
    }

    .dialog-footer :global(button) {
      width: 100%;
    }
  }
</style>
