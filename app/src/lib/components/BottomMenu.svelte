<script lang="ts">
  import { navigationStore, type BottomMenuTab } from '$lib/stores/navigation';

  let activeTab = $state<BottomMenuTab>('launcher');

  $effect(() => {
    const unsubscribe = navigationStore.subscribe((state) => {
      activeTab = state.activeTab;
    });
    return unsubscribe;
  });

  interface MenuItem {
    id: BottomMenuTab;
    label: string;
    icon: string;
  }

  const menuItems: MenuItem[] = [
    { id: 'launcher', label: 'Launcher', icon: '🚀' },
    { id: 'wallet', label: 'Wallet', icon: '💰' },
    { id: 'config', label: 'Settings', icon: '⚙️' }
  ];

  async function handleTabClick(tab: BottomMenuTab) {
    await navigationStore.setActiveTab(tab);
  }

  function handleKeyDown(event: KeyboardEvent, tab: BottomMenuTab) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleTabClick(tab);
    }
  }
</script>

<nav class="bottom-menu" aria-label="Main navigation">
  <div class="menu-container">
    {#each menuItems as item (item.id)}
      <button
        class="menu-item"
        class:active={activeTab === item.id}
        onclick={() => handleTabClick(item.id)}
        onkeydown={(e) => handleKeyDown(e, item.id)}
        role="tab"
        aria-selected={activeTab === item.id}
        aria-label={item.label}
      >
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </button>
    {/each}
  </div>
</nav>

<style>
  .bottom-menu {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: var(--color-bg-secondary);
    border-top: 1px solid var(--color-border);
    box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
    z-index: var(--z-fixed);
  }

  .menu-container {
    display: flex;
    justify-content: space-around;
    align-items: center;
    max-width: 600px;
    margin: 0 auto;
    padding: var(--spacing-xs) var(--spacing-md);
  }

  .menu-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-sm) var(--spacing-lg);
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    color: var(--color-text-secondary);
    min-width: 80px;
  }

  .menu-item:hover {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }

  .menu-item:focus {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }

  .menu-item.active {
    color: var(--color-accent);
    font-weight: var(--font-weight-semibold);
  }

  .menu-item.active .icon {
    transform: scale(1.1);
  }

  .icon {
    font-size: var(--font-size-2xl);
    transition: transform var(--transition-fast);
  }

  .label {
    font-size: var(--font-size-xs);
    text-align: center;
  }

  /* Mobile adjustments */
  @media (max-width: 480px) {
    .menu-item {
      min-width: 60px;
      padding: var(--spacing-xs) var(--spacing-sm);
    }

    .icon {
      font-size: var(--font-size-xl);
    }

    .label {
      font-size: 0.625rem;
    }
  }
</style>
