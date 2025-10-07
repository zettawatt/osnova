<script lang="ts">
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';
  import Card from '$lib/components/Card.svelte';

  // Placeholder config screen with theme toggle
</script>

<div class="config-screen">
  <header class="screen-header">
    <h1>Settings</h1>
  </header>

  <main class="screen-content">
    <div class="settings-container">
      <Card variant="default" padding="lg">
        <h2>Appearance</h2>
        <div class="setting-item">
          <div class="setting-info">
            <h3>Theme</h3>
            <p>Choose your preferred color scheme</p>
          </div>
          <ThemeToggle />
        </div>
      </Card>

      <Card variant="default" padding="lg">
        <h2>Coming Soon</h2>
        <p>More settings will be available here:</p>
        <ul>
          <li>Account management</li>
          <li>Privacy & security</li>
          <li>Network settings</li>
          <li>Storage & cache</li>
          <li>About & version info</li>
        </ul>
      </Card>
    </div>
  </main>
</div>

<style>
  .config-screen {
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
    padding: var(--spacing-xl);
  }

  .settings-container {
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  h2 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--spacing-md) 0;
  }

  h3 {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
    margin: 0;
  }

  p {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    margin: var(--spacing-xs) 0 0 0;
    line-height: var(--line-height-relaxed);
  }

  ul {
    margin: var(--spacing-md) 0 0 0;
    padding-left: var(--spacing-lg);
    color: var(--color-text-secondary);
  }

  li {
    margin-bottom: var(--spacing-xs);
    line-height: var(--line-height-relaxed);
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--spacing-lg);
  }

  .setting-info {
    flex: 1;
  }

  @media (max-width: 768px) {
    .screen-header {
      padding: var(--spacing-md);
    }

    .screen-header h1 {
      font-size: var(--font-size-xl);
    }

    .screen-content {
      padding: var(--spacing-md);
    }

    .setting-item {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
