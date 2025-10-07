<script lang="ts">
  import { identityStore } from '$lib/stores/identity';
  import Button from './Button.svelte';
  import Card from './Card.svelte';
  import Input from './Input.svelte';

  interface ImportIdentityFlowProps {
    onComplete: () => void;
  }

  let { onComplete }: ImportIdentityFlowProps = $props();

  let seedPhrase = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);

  // Validate seed phrase format (should be 12 words)
  let isValidFormat = $derived(() => {
    const words = seedPhrase.trim().split(/\s+/);
    return words.length === 12 && words.every((word) => word.length > 0);
  });

  async function handleImport() {
    if (!isValidFormat()) {
      error = 'Seed phrase must contain exactly 12 words';
      return;
    }

    loading = true;
    error = null;

    try {
      await identityStore.importIdentity(seedPhrase.trim());
      onComplete();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to import identity';
      console.error('Failed to import identity:', err);
    } finally {
      loading = false;
    }
  }

  function handlePaste() {
    navigator.clipboard.readText().then(
      (text) => {
        seedPhrase = text;
      },
      (err) => {
        console.error('Failed to read clipboard:', err);
      }
    );
  }
</script>

<div class="import-flow">
  <Card variant="default" padding="lg">
    <div class="import-content">
      <div class="info-banner">
        <span class="info-icon">🔑</span>
        <div>
          <h3>Restore Your Identity</h3>
          <p>
            Enter your 12-word seed phrase to restore your identity. Make sure you're in a private
            location.
          </p>
        </div>
      </div>

      <div class="input-section">
        <label for="seed-input" class="input-label">
          Seed Phrase
          <span class="required">*</span>
        </label>

        <textarea
          id="seed-input"
          class="seed-input"
          bind:value={seedPhrase}
          placeholder="Enter your 12-word seed phrase..."
          rows="4"
          disabled={loading}
          aria-describedby="seed-hint"
        ></textarea>

        <div id="seed-hint" class="hint">
          {#if seedPhrase.trim()}
            {seedPhrase.trim().split(/\s+/).length} / 12 words
          {:else}
            Separate words with spaces
          {/if}
        </div>

        <Button variant="outline" size="sm" onclick={handlePaste} disabled={loading}>
          📋 Paste from Clipboard
        </Button>
      </div>

      {#if error}
        <div class="error-message">
          <span class="error-icon">⚠️</span>
          {error}
        </div>
      {/if}

      <div class="security-notice">
        <div class="notice-item">
          <span class="notice-icon">🔐</span>
          <p>Your seed phrase is encrypted and never leaves your device</p>
        </div>
        <div class="notice-item">
          <span class="notice-icon">⚠️</span>
          <p>Never share your seed phrase with anyone</p>
        </div>
      </div>

      <Button
        variant="primary"
        size="lg"
        fullWidth
        disabled={!isValidFormat() || loading}
        loading={loading}
        onclick={handleImport}
      >
        {loading ? 'Importing...' : 'Import Identity'}
      </Button>
    </div>
  </Card>
</div>

<style>
  .import-flow {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .import-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .info-banner {
    display: flex;
    gap: var(--spacing-md);
    padding: var(--spacing-md);
    background-color: var(--color-accent-light);
    border: 1px solid var(--color-accent);
    border-radius: var(--radius-md);
  }

  .info-icon {
    font-size: var(--font-size-2xl);
    flex-shrink: 0;
  }

  .info-banner h3 {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .info-banner p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  .input-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .input-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .required {
    color: var(--color-error);
  }

  .seed-input {
    width: 100%;
    padding: var(--spacing-md);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    resize: vertical;
    transition: all var(--transition-fast);
    line-height: var(--line-height-relaxed);
  }

  .seed-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px var(--color-accent-light);
  }

  .seed-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .seed-input::placeholder {
    color: var(--color-text-tertiary);
  }

  .hint {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }

  .security-notice {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .notice-item {
    display: flex;
    gap: var(--spacing-sm);
    align-items: center;
  }

  .notice-icon {
    font-size: var(--font-size-base);
    flex-shrink: 0;
  }

  .notice-item p {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    margin: 0;
    line-height: var(--line-height-normal);
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
    color: var(--color-error);
    font-size: var(--font-size-sm);
  }

  .error-icon {
    flex-shrink: 0;
  }

  /* Mobile adjustments */
  @media (max-width: 768px) {
    .info-banner {
      flex-direction: column;
    }
  }
</style>
