<script lang="ts">
  import type { Snippet } from 'svelte';

  interface CardProps {
    variant?: 'default' | 'elevated' | 'outlined' | 'flat';
    padding?: 'none' | 'sm' | 'md' | 'lg';
    hoverable?: boolean;
    clickable?: boolean;
    onclick?: (event: MouseEvent) => void;
    children: Snippet;
  }

  let {
    variant = 'default',
    padding = 'md',
    hoverable = false,
    clickable = false,
    onclick,
    children
  }: CardProps = $props();

  let className = $derived(
    [
      'card',
      `card-${variant}`,
      `card-padding-${padding}`,
      (hoverable || clickable) && 'card-hoverable',
      clickable && 'card-clickable'
    ]
      .filter(Boolean)
      .join(' ')
  );

  function handleClick(event: MouseEvent) {
    if (clickable && onclick) {
      onclick(event);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (clickable && (event.key === 'Enter' || event.key === ' ')) {
      event.preventDefault();
      onclick?.(event as unknown as MouseEvent);
    }
  }
</script>

{#if clickable}
  <div
    class={className}
    role="button"
    tabindex="0"
    onclick={handleClick}
    onkeydown={handleKeyDown}
  >
    {@render children()}
  </div>
{:else}
  <div class={className}>
    {@render children()}
  </div>
{/if}

<style>
  .card {
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-lg);
    transition: all var(--transition-base);
  }

  /* Variants */
  .card-default {
    border: 1px solid var(--color-border);
    box-shadow: var(--shadow-sm);
  }

  .card-elevated {
    border: none;
    box-shadow: var(--shadow-lg);
  }

  .card-outlined {
    border: 1px solid var(--color-border);
    box-shadow: none;
  }

  .card-flat {
    border: none;
    box-shadow: none;
  }

  /* Padding */
  .card-padding-none {
    padding: 0;
  }

  .card-padding-sm {
    padding: var(--spacing-md);
  }

  .card-padding-md {
    padding: var(--spacing-lg);
  }

  .card-padding-lg {
    padding: var(--spacing-xl);
  }

  /* Hoverable */
  .card-hoverable:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-lg);
  }

  .card-default.card-hoverable:hover {
    border-color: var(--color-border-hover);
  }

  /* Clickable */
  .card-clickable {
    cursor: pointer;
  }

  .card-clickable:focus {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }

  .card-clickable:active {
    transform: translateY(0);
  }
</style>
