<script lang="ts">
  import { appsStore, type AppListItem } from '$lib/stores/apps';
  import { launcherStore } from '$lib/stores/launcher';
  import AppIcon from './AppIcon.svelte';

  interface AppGridProps {
    onUninstallRequest?: (app: AppListItem) => void;
  }

  let { onUninstallRequest }: AppGridProps = $props();

  let apps = $state<AppListItem[]>([]);
  let layout = $state<string[]>([]);
  let loading = $state(false);
  let draggedAppId = $state<string | null>(null);
  let dragOverAppId = $state<string | null>(null);

  // Mobile touch state
  let touchStartTime = $state(0);
  let touchStartPos = $state<{ x: number; y: number } | null>(null);
  let longPressTimer: number | null = null;
  let isDraggingMobile = $state(false);

  // Subscribe to stores
  $effect(() => {
    const unsubApps = appsStore.subscribe((state) => {
      apps = state.apps;
      loading = state.loading;
    });

    const unsubLayout = launcherStore.subscribe((state) => {
      layout = state.layout;
    });

    return () => {
      unsubApps();
      unsubLayout();
    };
  });

  // Sort apps according to layout, with unlisted apps at the end
  let sortedApps = $derived(() => {
    const appsMap = new Map(apps.map((app) => [app.id, app]));
    const layoutApps = layout
      .map((id) => appsMap.get(id))
      .filter((app): app is AppListItem => app !== undefined);

    const unlistedApps = apps.filter((app) => !layout.includes(app.id));

    return [...layoutApps, ...unlistedApps];
  });

  async function handleLaunchApp(appId: string) {
    try {
      await appsStore.launchApp(appId);
    } catch (error) {
      console.error('Failed to launch app:', error);
      // TODO: Show error toast
    }
  }

  function handleContextMenu(event: MouseEvent, app: AppListItem) {
    event.preventDefault();
    onUninstallRequest?.(app);
  }

  // Drag and drop handlers for desktop
  function handleDragStart(event: DragEvent, appId: string) {
    draggedAppId = appId;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', appId);
    }
  }

  function handleDragEnd() {
    draggedAppId = null;
    dragOverAppId = null;
  }

  function handleDragOver(event: DragEvent, appId: string) {
    event.preventDefault();
    if (draggedAppId && draggedAppId !== appId) {
      dragOverAppId = appId;
      if (event.dataTransfer) {
        event.dataTransfer.dropEffect = 'move';
      }
    }
  }

  function handleDragLeave() {
    dragOverAppId = null;
  }

  async function handleDrop(event: DragEvent, targetAppId: string) {
    event.preventDefault();
    dragOverAppId = null;

    if (!draggedAppId || draggedAppId === targetAppId) {
      draggedAppId = null;
      return;
    }

    // Create new layout by reordering
    const currentApps = sortedApps();
    const draggedIndex = currentApps.findIndex((app) => app.id === draggedAppId);
    const targetIndex = currentApps.findIndex((app) => app.id === targetAppId);

    if (draggedIndex === -1 || targetIndex === -1) {
      draggedAppId = null;
      return;
    }

    // Reorder the apps
    const reorderedApps = [...currentApps];
    const [draggedApp] = reorderedApps.splice(draggedIndex, 1);
    reorderedApps.splice(targetIndex, 0, draggedApp);

    // Update layout
    const newLayout = reorderedApps.map((app) => app.id);
    await launcherStore.saveLayout(newLayout);

    draggedAppId = null;
  }

  // Mobile touch handlers
  function handleTouchStart(event: TouchEvent, appId: string) {
    const touch = event.touches[0];
    touchStartTime = Date.now();
    touchStartPos = { x: touch.clientX, y: touch.clientY };

    // Long press detection (500ms)
    longPressTimer = window.setTimeout(() => {
      isDraggingMobile = true;
      draggedAppId = appId;
      // Haptic feedback if available
      if ('vibrate' in navigator) {
        navigator.vibrate(50);
      }
    }, 500);
  }

  function handleTouchMove(event: TouchEvent) {
    if (!isDraggingMobile || !draggedAppId) return;

    event.preventDefault();
    const touch = event.touches[0];

    // Find element under touch point
    const element = document.elementFromPoint(touch.clientX, touch.clientY);
    const wrapper = element?.closest('.app-wrapper');

    if (wrapper) {
      const targetAppId = wrapper.getAttribute('data-app-id');
      if (targetAppId && targetAppId !== draggedAppId) {
        dragOverAppId = targetAppId;
      }
    }
  }

  async function handleTouchEnd(event: TouchEvent) {
    // Clear long press timer
    if (longPressTimer) {
      clearTimeout(longPressTimer);
      longPressTimer = null;
    }

    // If not dragging, this was a tap - ignore
    if (!isDraggingMobile) {
      touchStartPos = null;
      return;
    }

    event.preventDefault();

    // Perform drop if we have a valid target
    if (draggedAppId && dragOverAppId && draggedAppId !== dragOverAppId) {
      const currentApps = sortedApps();
      const draggedIndex = currentApps.findIndex((app) => app.id === draggedAppId);
      const targetIndex = currentApps.findIndex((app) => app.id === dragOverAppId);

      if (draggedIndex !== -1 && targetIndex !== -1) {
        const reorderedApps = [...currentApps];
        const [draggedApp] = reorderedApps.splice(draggedIndex, 1);
        reorderedApps.splice(targetIndex, 0, draggedApp);

        const newLayout = reorderedApps.map((app) => app.id);
        await launcherStore.saveLayout(newLayout);
      }
    }

    // Reset state
    isDraggingMobile = false;
    draggedAppId = null;
    dragOverAppId = null;
    touchStartPos = null;
  }

  function handleTouchCancel() {
    if (longPressTimer) {
      clearTimeout(longPressTimer);
      longPressTimer = null;
    }
    isDraggingMobile = false;
    draggedAppId = null;
    dragOverAppId = null;
    touchStartPos = null;
  }
</script>

<div class="app-grid-container">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading apps...</p>
    </div>
  {:else if apps.length === 0}
    <div class="empty-state">
      <div class="empty-icon">📦</div>
      <h3>No Apps Installed</h3>
      <p>Install apps from the marketplace to get started.</p>
    </div>
  {:else}
    <div class="app-grid">
      {#each sortedApps() as app (app.id)}
        <div
          class="app-wrapper"
          class:dragging={draggedAppId === app.id}
          class:drag-over={dragOverAppId === app.id}
          data-app-id={app.id}
          draggable="true"
          ondragstart={(e) => handleDragStart(e, app.id)}
          ondragend={handleDragEnd}
          ondragover={(e) => handleDragOver(e, app.id)}
          ondragleave={handleDragLeave}
          ondrop={(e) => handleDrop(e, app.id)}
          ontouchstart={(e) => handleTouchStart(e, app.id)}
          ontouchmove={handleTouchMove}
          ontouchend={handleTouchEnd}
          ontouchcancel={handleTouchCancel}
          role="button"
          tabindex="0"
        >
          <AppIcon
            {app}
            size="md"
            onclick={() => handleLaunchApp(app.id)}
            oncontextmenu={(e) => handleContextMenu(e, app)}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .app-grid-container {
    width: 100%;
    min-height: 300px;
    padding: var(--spacing-lg);
  }

  .app-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(5rem, 1fr));
    gap: var(--spacing-xl);
    justify-items: center;
  }

  .app-wrapper {
    cursor: grab;
    transition: all var(--transition-fast);
    border-radius: var(--radius-lg);
    padding: var(--spacing-xs);
  }

  .app-wrapper:active {
    cursor: grabbing;
  }

  .app-wrapper.dragging {
    opacity: 0.5;
    transform: scale(0.95);
    cursor: grabbing;
  }

  .app-wrapper.drag-over {
    background-color: var(--color-accent-light);
    transform: scale(1.05);
  }

  /* Mobile touch feedback */
  @media (hover: none) and (pointer: coarse) {
    .app-wrapper {
      touch-action: none;
      user-select: none;
      -webkit-user-select: none;
    }

    .app-wrapper.dragging {
      opacity: 0.7;
      transform: scale(1.1);
      z-index: 1000;
      box-shadow: var(--shadow-lg);
    }
  }

  /* Loading state */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    min-height: 300px;
    color: var(--color-text-secondary);
  }

  .spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Empty state */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    min-height: 300px;
    text-align: center;
    color: var(--color-text-secondary);
  }

  .empty-icon {
    font-size: 4rem;
    opacity: 0.5;
  }

  .empty-state h3 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }

  .empty-state p {
    font-size: var(--font-size-base);
    margin: 0;
    max-width: 400px;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .app-grid {
      grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
      gap: var(--spacing-lg);
    }

    .app-grid-container {
      padding: var(--spacing-md);
    }

    /* Disable desktop drag-and-drop on touch devices */
    .app-wrapper {
      cursor: default;
    }
  }

  @media (min-width: 1024px) {
    .app-grid {
      grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
      gap: var(--spacing-2xl);
    }
  }
</style>
