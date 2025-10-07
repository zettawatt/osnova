<script lang="ts">
  // Placeholder deployment screen
</script>

<div class="deployment-screen">
  <header class="screen-header">
    <h1>Deployment</h1>
  </header>

  <main class="screen-content">
    <div class="placeholder">
      <div class="placeholder-icon">🚀</div>
      <h2>Deployment</h2>
      <p>Manage application deployments and server connections.</p>
      <p class="coming-soon">Coming soon in Phase 3...</p>

      <div class="features-preview">
        <h3>Planned Features:</h3>
        <ul>
          <li>📦 Deploy apps to Autonomi Network</li>
          <li>🌐 Server mode configuration</li>
          <li>🔗 Client-server pairing</li>
          <li>📊 Deployment status monitoring</li>
          <li>⚙️ Component packaging</li>
        </ul>
      </div>
    </div>
  </main>
</div>

<style>
  .deployment-screen {
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
    max-width: 500px;
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
    margin-bottom: var(--spacing-xl);
  }

  .features-preview {
    text-align: left;
    padding: var(--spacing-lg);
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    margin-top: var(--spacing-xl);
  }

  .features-preview h3 {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--spacing-md);
  }

  .features-preview ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .features-preview li {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: var(--line-height-relaxed);
    margin-bottom: var(--spacing-sm);
    padding-left: var(--spacing-md);
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
