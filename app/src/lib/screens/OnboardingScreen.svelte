<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Snippet } from 'svelte';
  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';

  interface OnboardingScreenProps {
    oncomplete: () => void;
    create: Snippet<[handleComplete: () => void]>;
    importIdentity: Snippet<[handleComplete: () => void]>;
  }

  let { oncomplete, create, importIdentity }: OnboardingScreenProps = $props();

  const dispatch = createEventDispatcher<{ complete: void }>();

  let step = $state<'welcome' | 'choose' | 'create' | 'import'>('welcome');

  function handleGetStarted() {
    step = 'choose';
  }

  function handleCreateNew() {
    step = 'create';
  }

  function handleImportExisting() {
    step = 'import';
  }

  function handleBack() {
    if (step === 'create' || step === 'import') {
      step = 'choose';
    } else if (step === 'choose') {
      step = 'welcome';
    }
  }

  function handleComplete() {
    oncomplete();
  }
</script>

<div class="onboarding-screen">
  <div class="onboarding-container">
    {#if step === 'welcome'}
      <div class="welcome-content">
        <div class="logo">
          <div class="logo-icon">🚀</div>
          <h1>Welcome to Osnova</h1>
        </div>

        <Card variant="elevated" padding="lg">
          <div class="intro">
            <h2>Your Personal Application Platform</h2>
            <p>
              Osnova is a secure, distributed application platform that puts you in control of
              your data and digital identity.
            </p>

            <div class="features">
              <div class="feature">
                <span class="feature-icon">🔐</span>
                <div>
                  <h3>Secure by Design</h3>
                  <p>All data encrypted at rest with your personal keys</p>
                </div>
              </div>

              <div class="feature">
                <span class="feature-icon">🌐</span>
                <div>
                  <h3>Distributed</h3>
                  <p>Run standalone or connect to a server for sync</p>
                </div>
              </div>

              <div class="feature">
                <span class="feature-icon">📦</span>
                <div>
                  <h3>Modular Apps</h3>
                  <p>Install only the applications you need</p>
                </div>
              </div>
            </div>

            <Button variant="primary" size="lg" fullWidth onclick={handleGetStarted}>
              Get Started
            </Button>
          </div>
        </Card>
      </div>
    {:else if step === 'choose'}
      <div class="choice-content">
        <h1>Setup Your Identity</h1>
        <p class="subtitle">Choose how you'd like to begin</p>

        <div class="options">
          <Card variant="outlined" padding="lg" hoverable clickable onclick={handleCreateNew}>
            <div class="option">
              <span class="option-icon">✨</span>
              <h3>Create New Identity</h3>
              <p>Generate a fresh identity with a secure 12-word backup phrase</p>
            </div>
          </Card>

          <Card variant="outlined" padding="lg" hoverable clickable onclick={handleImportExisting}>
            <div class="option">
              <span class="option-icon">🔑</span>
              <h3>Import Existing Identity</h3>
              <p>Restore your identity from a backup phrase</p>
            </div>
          </Card>
        </div>

        <Button variant="ghost" onclick={handleBack}>
          Back
        </Button>
      </div>
    {:else if step === 'create'}
      <div class="create-content">
        <div class="back-button">
          <Button variant="ghost" onclick={handleBack}>
            ← Back
          </Button>
        </div>

        <h1>Create New Identity</h1>

        {@render create(handleComplete)}
      </div>
    {:else if step === 'import'}
      <div class="import-content">
        <div class="back-button">
          <Button variant="ghost" onclick={handleBack}>
            ← Back
          </Button>
        </div>

        <h1>Import Identity</h1>

        {@render importIdentity(handleComplete)}
      </div>
    {/if}
  </div>
</div>

<style>
  .onboarding-screen {
    width: 100%;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-xl);
    background: linear-gradient(
      135deg,
      var(--color-bg-primary) 0%,
      var(--color-bg-secondary) 100%
    );
  }

  .onboarding-container {
    width: 100%;
    max-width: 600px;
  }

  /* Welcome step */
  .welcome-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xl);
  }

  .logo {
    text-align: center;
    margin-bottom: var(--spacing-lg);
  }

  .logo-icon {
    font-size: 5rem;
    margin-bottom: var(--spacing-md);
    animation: bounce 2s ease-in-out infinite;
  }

  @keyframes bounce {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  .logo h1 {
    font-size: var(--font-size-4xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-accent);
    margin: 0;
  }

  .intro {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .intro h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
    text-align: center;
  }

  .intro > p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    text-align: center;
    margin: 0;
  }

  .features {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    margin: var(--spacing-md) 0;
  }

  .feature {
    display: flex;
    gap: var(--spacing-md);
    align-items: flex-start;
  }

  .feature-icon {
    font-size: var(--font-size-2xl);
    flex-shrink: 0;
  }

  .feature h3 {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--spacing-xs) 0;
  }

  .feature p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  /* Choice step */
  .choice-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
    text-align: center;
  }

  .choice-content h1 {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    margin: 0;
  }

  .options {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    margin: var(--spacing-lg) 0;
  }

  .option {
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .option-icon {
    font-size: var(--font-size-3xl);
  }

  .option h3 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .option p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin: 0;
  }

  /* Create/Import steps */
  .create-content,
  .import-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .back-button {
    align-self: flex-start;
  }

  .create-content h1,
  .import-content h1 {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin: 0;
    text-align: center;
  }

  /* Mobile adjustments */
  @media (max-width: 768px) {
    .onboarding-screen {
      padding: var(--spacing-md);
    }

    .logo-icon {
      font-size: 4rem;
    }

    .logo h1 {
      font-size: var(--font-size-3xl);
    }

    .intro h2 {
      font-size: var(--font-size-xl);
    }

    .choice-content h1,
    .create-content h1,
    .import-content h1 {
      font-size: var(--font-size-2xl);
    }
  }
</style>
