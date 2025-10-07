<script lang="ts">
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';

  let message = $state('');

  function showMessage(text: string) {
    message = text;
    setTimeout(() => {
      message = '';
    }, 3000);
  }
</script>

<main class="container">
  <div class="header">
    <h1>Osnova</h1>
    <ThemeToggle />
  </div>

  <div class="content">
    <p class="welcome">Welcome to the Osnova Application Platform</p>

    <div class="info-grid">
      <div class="info-card">
        <h2>🚀 Getting Started</h2>
        <p>This is a Tauri 2.0 application with Svelte frontend and Rust backend.</p>
      </div>

      <div class="info-card">
        <h2>🎨 Theme System</h2>
        <p>Toggle between light, dark, and system themes using the button above.</p>
      </div>

      <div class="info-card">
        <h2>🔐 Secure by Design</h2>
        <p>All data is encrypted at rest using industry-standard cryptography.</p>
      </div>

      <div class="info-card">
        <h2>📦 Component-Based</h2>
        <p>Modular architecture allows for flexible app composition.</p>
      </div>
    </div>

    {#if message}
      <div class="message">
        {message}
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    min-height: 100vh;
    padding: var(--spacing-xl);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-2xl);
  }

  h1 {
    font-size: var(--font-size-4xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-accent);
    margin: 0;
  }

  .content {
    max-width: 1200px;
    margin: 0 auto;
  }

  .welcome {
    text-align: center;
    font-size: var(--font-size-xl);
    color: var(--color-text-secondary);
    margin-bottom: var(--spacing-2xl);
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: var(--spacing-lg);
    margin-bottom: var(--spacing-xl);
  }

  .info-card {
    padding: var(--spacing-lg);
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    transition: all var(--transition-base);
  }

  .info-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-lg);
    border-color: var(--color-border-hover);
  }

  .info-card h2 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    margin-bottom: var(--spacing-sm);
    color: var(--color-text-primary);
  }

  .info-card p {
    font-size: var(--font-size-sm);
    line-height: var(--line-height-relaxed);
    color: var(--color-text-secondary);
    margin: 0;
  }

  .message {
    position: fixed;
    bottom: var(--spacing-xl);
    right: var(--spacing-xl);
    padding: var(--spacing-md) var(--spacing-lg);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-xl);
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @media (max-width: 768px) {
    .container {
      padding: var(--spacing-md);
    }

    .header {
      flex-direction: column;
      gap: var(--spacing-md);
      align-items: flex-start;
    }

    h1 {
      font-size: var(--font-size-3xl);
    }

    .info-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
