<script lang="ts">
  // Placeholder wallet screen
</script>

<div class="wallet-screen">
  <header class="screen-header">
    <h1>Wallet</h1>
  </header>

  <main class="screen-content">
    <div class="placeholder">
      <div class="placeholder-icon">💰</div>
      <h2>Wallet</h2>
      <p>Manage your digital assets and transactions.</p>
      <p class="coming-soon">Coming soon...</p>
    </div>
  </main>
</div>

<style>
  .wallet-screen {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: var(--color-bg-primary);
    padding-bottom: 70px;
  }

  .screen-header {
    padding: var(--spacing-lg) var(--spacing-xl);
    background-color: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .screen-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .screen-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder {
    text-align: center;
    max-width: 400px;
    padding: var(--spacing-2xl);
  }

  .placeholder-icon {
    font-size: 6rem;
    margin-bottom: var(--spacing-lg);
    opacity: 0.5;
  }

  .placeholder h2 {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--spacing-md);
  }

  .placeholder p {
    font-size: var(--font-size-base);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin-bottom: var(--spacing-sm);
  }

  .coming-soon {
    font-weight: var(--font-weight-medium);
    color: var(--color-accent);
  }

  @media (max-width: 768px) {
    .screen-header {
      padding: var(--spacing-md);
    }

    .screen-header h1 {
      font-size: var(--font-size-xl);
    }

    .placeholder {
      padding: var(--spacing-xl);
    }

    .placeholder-icon {
      font-size: 4rem;
    }

    .placeholder h2 {
      font-size: var(--font-size-2xl);
    }
  }
</style>
