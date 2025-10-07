<script lang="ts">
  import type { AppListItem } from '$lib/stores/apps';

  interface AppIconProps {
    app: AppListItem;
    size?: 'sm' | 'md' | 'lg';
    onclick?: () => void;
  }

  let { app, size = 'md', onclick }: AppIconProps = $props();

  let sizeClass = $derived(`icon-${size}`);

  // Handle image loading errors by showing fallback
  let imageError = $state(false);

  function handleImageError() {
    imageError = true;
  }

  function handleClick() {
    onclick?.();
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onclick?.();
    }
  }

  // Generate initials from app name for fallback
  let initials = $derived(
    app.name
      .split(' ')
      .map((word) => word[0])
      .join('')
      .toUpperCase()
      .slice(0, 2)
  );
</script>

<div
  class="app-icon {sizeClass}"
  role="button"
  tabindex="0"
  onclick={handleClick}
  onkeydown={handleKeyDown}
  aria-label={`Launch ${app.name}`}
>
  <div class="icon-container">
    {#if !imageError && app.icon_uri}
      <img src={app.icon_uri} alt={app.name} onerror={handleImageError} />
    {:else}
      <div class="fallback-icon">
        {initials}
      </div>
    {/if}
  </div>
  <div class="app-name">{app.name}</div>
</div>

<style>
  .app-icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xs);
    cursor: pointer;
    user-select: none;
    transition: transform var(--transition-fast);
  }

  .app-icon:hover {
    transform: scale(1.05);
  }

  .app-icon:active {
    transform: scale(0.95);
  }

  .app-icon:focus {
    outline: 2px solid var(--color-accent);
    outline-offset: 4px;
    border-radius: var(--radius-md);
  }

  .icon-container {
    position: relative;
    border-radius: var(--radius-lg);
    overflow: hidden;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    box-shadow: var(--shadow-md);
    transition: all var(--transition-fast);
  }

  .app-icon:hover .icon-container {
    box-shadow: var(--shadow-lg);
    border-color: var(--color-border-hover);
  }

  /* Sizes */
  .icon-sm .icon-container {
    width: 3rem;
    height: 3rem;
  }

  .icon-md .icon-container {
    width: 4rem;
    height: 4rem;
  }

  .icon-lg .icon-container {
    width: 5rem;
    height: 5rem;
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .fallback-icon {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: var(--font-weight-bold);
    color: var(--color-text-primary);
    background: linear-gradient(135deg, var(--color-accent), var(--color-accent-hover));
    color: white;
  }

  .icon-sm .fallback-icon {
    font-size: var(--font-size-sm);
  }

  .icon-md .fallback-icon {
    font-size: var(--font-size-base);
  }

  .icon-lg .fallback-icon {
    font-size: var(--font-size-lg);
  }

  .app-name {
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    text-align: center;
    max-width: 5rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .icon-sm .app-name {
    font-size: 0.625rem;
    max-width: 3rem;
  }

  .icon-lg .app-name {
    font-size: var(--font-size-sm);
    max-width: 6rem;
  }
</style>
