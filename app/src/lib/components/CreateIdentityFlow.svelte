<script lang="ts">
  import { identityStore } from '$lib/stores/identity';
  import Button from './Button.svelte';
  import Card from './Card.svelte';

  interface CreateIdentityFlowProps {
    onComplete: () => void;
  }

  let { onComplete }: CreateIdentityFlowProps = $props();

  let step = $state<'generating' | 'backup' | 'confirm'>('generating');
  let seedPhrase = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);

  let confirmedBackup = $state(false);
  let understandsRisk = $state(false);

  // Generate identity on mount
  $effect(() => {
    if (step === 'generating') {
      generateIdentity();
    }
  });

  async function generateIdentity() {
    loading = true;
    error = null;

    try {
      seedPhrase = await identityStore.createIdentity();
      step = 'backup';
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to create identity';
      console.error('Failed to create identity:', err);
    } finally {
      loading = false;
    }
  }

  function handleCopySeedPhrase() {
    navigator.clipboard.writeText(seedPhrase).then(
      () => {
        // Show success feedback
        console.log('Seed phrase copied to clipboard');
      },
      (err) => {
        console.error('Failed to copy seed phrase:', err);
      }
    );
  }

  function handleContinueToConfirm() {
    if (!confirmedBackup) {
      error = 'Please confirm you have backed up your seed phrase';
      return;
    }
    step = 'confirm';
  }

  function handleComplete() {
    if (!understandsRisk) {
      error = 'Please confirm you understand the risks';
      return;
    }
    onComplete();
  }

  function handleRetry() {
    step = 'generating';
    error = null;
    confirmedBackup = false;
    understandsRisk = false;
  }
</script>

<div class="create-flow">
  {#if step === 'generating'}
    <Card variant="default" padding="lg">
      <div class="generating">
        <div class="spinner"></div>
        <h2>Generating Your Identity</h2>
        <p>Creating secure keys and backup phrase...</p>

        {#if error}
          <div class="error">
            <p>{error}</p>
            <Button variant="primary" onclick={handleRetry}>
              Try Again
            </Button>
          </div>
        {/if}
      </div>
    </Card>
  {:else if step === 'backup'}
    <Card variant="default" padding="lg">
      <div class="backup">
        <div class="warning-banner">
          <span class="warning-icon">⚠️</span>
          <div>
            <h3>Important: Backup Your Seed Phrase</h3>
            <p>
              This 12-word phrase is the ONLY way to recover your identity. Store it safely and
              never share it with anyone.
            </p>
          </div>
        </div>

        <div class="seed-phrase-container">
          <div class="seed-phrase">
            {#each seedPhrase.split(' ') as word, index}
              <div class="seed-word">
                <span class="word-number">{index + 1}</span>
                <span class="word-text">{word}</span>
              </div>
            {/each}
          </div>

          <Button variant="outline" size="sm" onclick={handleCopySeedPhrase}>
            📋 Copy to Clipboard
          </Button>
        </div>

        <div class="checklist">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={confirmedBackup} />
            <span>
              I have written down my seed phrase and stored it in a safe place
            </span>
          </label>
        </div>

        {#if error}
          <div class="error-message">{error}</div>
        {/if}

        <Button
          variant="primary"
          size="lg"
          fullWidth
          disabled={!confirmedBackup}
          onclick={handleContinueToConfirm}
        >
          Continue
        </Button>
      </div>
    </Card>
  {:else if step === 'confirm'}
    <Card variant="default" padding="lg">
      <div class="confirm">
        <h2>Final Confirmation</h2>

        <div class="confirm-info">
          <div class="info-item">
            <span class="info-icon">🔐</span>
            <p>
              Your identity has been created. Without your seed phrase, you cannot recover your
              account if you lose access to this device.
            </p>
          </div>

          <div class="info-item">
            <span class="info-icon">⚠️</span>
            <p>
              Never share your seed phrase with anyone. Osnova developers will NEVER ask for
              your seed phrase.
            </p>
          </div>

          <div class="info-item">
            <span class="info-icon">📝</span>
            <p>
              Store your seed phrase offline in a secure location. Digital storage can be
              compromised.
            </p>
          </div>
        </div>

        <div class="checklist">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={understandsRisk} />
            <span>
              I understand that I am solely responsible for keeping my seed phrase safe
            </span>
          </label>
        </div>

        {#if error}
          <div class="error-message">{error}</div>
        {/if}

        <Button
          variant="primary"
          size="lg"
          fullWidth
          disabled={!understandsRisk}
          onclick={handleComplete}
        >
          Complete Setup
        </Button>
      </div>
    </Card>
  {/if}
</div>

<style>
  .create-flow {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  /* Generating step */
  .generating {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-xl) 0;
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

  .generating h2 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .generating p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    margin: 0;
  }

  /* Backup step */
  .backup {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .warning-banner {
    display: flex;
    gap: var(--spacing-md);
    padding: var(--spacing-md);
    background-color: rgba(251, 191, 36, 0.1);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
  }

  .warning-icon {
    font-size: var(--font-size-2xl);
    flex-shrink: 0;
  }

  .warning-banner h3 {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .warning-banner p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  .seed-phrase-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .seed-phrase {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .seed-word {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-xs) var(--spacing-sm);
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-sm);
  }

  .word-number {
    font-size: var(--font-size-xs);
    color: var(--color-text-tertiary);
    font-weight: var(--font-weight-medium);
    min-width: 1.5rem;
  }

  .word-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    font-weight: var(--font-weight-medium);
    font-family: var(--font-mono);
  }

  /* Confirm step */
  .confirm {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .confirm h2 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
    text-align: center;
  }

  .confirm-info {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .info-item {
    display: flex;
    gap: var(--spacing-md);
    padding: var(--spacing-md);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .info-icon {
    font-size: var(--font-size-xl);
    flex-shrink: 0;
  }

  .info-item p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  /* Checklist */
  .checklist {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .checkbox-label {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-sm);
    cursor: pointer;
    user-select: none;
  }

  .checkbox-label input[type='checkbox'] {
    margin-top: 2px;
    cursor: pointer;
    width: 1.25rem;
    height: 1.25rem;
    flex-shrink: 0;
  }

  .checkbox-label span {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
  }

  /* Error states */
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-md);
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
  }

  .error p {
    color: var(--color-error);
    margin: 0;
    font-size: var(--font-size-sm);
  }

  .error-message {
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
    color: var(--color-error);
    font-size: var(--font-size-sm);
    text-align: center;
  }

  /* Mobile adjustments */
  @media (max-width: 768px) {
    .seed-phrase {
      grid-template-columns: repeat(2, 1fr);
    }

    .warning-banner {
      flex-direction: column;
    }
  }
</style>
