<script lang="ts">
  import { appsStore } from '$lib/stores/apps';
  import Button from './Button.svelte';
  import Input from './Input.svelte';
  import Card from './Card.svelte';

  interface AppInstallDialogProps {
    onClose: () => void;
    onSuccess: () => void;
  }

  let { onClose, onSuccess }: AppInstallDialogProps = $props();

  let manifestUri = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function handleInstall() {
    if (!manifestUri.trim()) {
      error = 'Please enter a manifest URI';
      return;
    }

    loading = true;
    error = null;

    try {
      // Call backend to install app
      // Note: This will fail since apps.install is not yet implemented
      // but the UI flow is complete
      await appsStore.launchApp(manifestUri); // Placeholder - should be install

      // Reload apps list
      await appsStore.loadApps();

      onSuccess();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to install app';
      console.error('Failed to install app:', err);
    } finally {
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
          <h2 id="dialog-title">Install Application</h2>
          <button class="close-button" onclick={onClose} aria-label="Close dialog">
            ✕
          </button>
        </div>

        <div class="dialog-body">
          <p class="description">
            Enter the manifest URI to install a new application. The manifest should be hosted
            on the Autonomi Network or accessible via HTTP/HTTPS.
          </p>

          <Input
            label="Manifest URI"
            type="url"
            bind:value={manifestUri}
            placeholder="ant://... or https://..."
            error={error ?? undefined}
            fullWidth
            autofocus
          />

          <div class="info-box">
            <span class="info-icon">ℹ️</span>
            <div class="info-text">
              <p><strong>Supported URI schemes:</strong></p>
              <ul>
                <li><code>ant://</code> - Autonomi Network address</li>
                <li><code>https://</code> - Direct HTTPS URL</li>
                <li><code>file://</code> - Local file path (dev only)</li>
              </ul>
            </div>
          </div>
        </div>

        <div class="dialog-footer">
          <Button variant="ghost" onclick={onClose} disabled={loading}>
            Cancel
          </Button>
          <Button
            variant="primary"
            onclick={handleInstall}
            loading={loading}
            disabled={!manifestUri.trim() || loading}
          >
            {loading ? 'Installing...' : 'Install'}
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
    max-width: 500px;
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

  .description {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  .info-box {
    display: flex;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: var(--color-accent-light);
    border: 1px solid var(--color-accent);
    border-radius: var(--radius-md);
  }

  .info-icon {
    font-size: var(--font-size-lg);
    flex-shrink: 0;
  }

  .info-text {
    flex: 1;
  }

  .info-text p {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    margin: 0 0 var(--spacing-xs) 0;
    font-weight: var(--font-weight-medium);
  }

  .info-text ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .info-text li {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin-bottom: var(--spacing-xs);
  }

  .info-text code {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    padding: 0 var(--spacing-xs);
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-sm);
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
