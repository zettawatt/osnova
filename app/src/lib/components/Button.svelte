<script lang="ts">
  import type { Snippet } from 'svelte';

  interface ButtonProps {
    variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    fullWidth?: boolean;
    loading?: boolean;
    onclick?: (event: MouseEvent) => void;
    type?: 'button' | 'submit' | 'reset';
    children: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    fullWidth = false,
    loading = false,
    onclick,
    type = 'button',
    children
  }: ButtonProps = $props();

  let className = $derived(
    [
      'btn',
      `btn-${variant}`,
      `btn-${size}`,
      fullWidth && 'btn-full-width',
      loading && 'btn-loading',
      disabled && 'btn-disabled'
    ]
      .filter(Boolean)
      .join(' ')
  );
</script>

<button
  class={className}
  {type}
  disabled={disabled || loading}
  {onclick}
  aria-busy={loading}
>
  {#if loading}
    <span class="spinner"></span>
  {/if}
  <span class="btn-content" class:hidden={loading}>
    {@render children()}
  </span>
</button>

<style>
  .btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    font-family: var(--font-sans);
    font-weight: var(--font-weight-medium);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    white-space: nowrap;
    user-select: none;
  }

  /* Sizes */
  .btn-sm {
    padding: var(--spacing-xs) var(--spacing-md);
    font-size: var(--font-size-sm);
    min-height: 2rem;
  }

  .btn-md {
    padding: var(--spacing-sm) var(--spacing-lg);
    font-size: var(--font-size-base);
    min-height: 2.5rem;
  }

  .btn-lg {
    padding: var(--spacing-md) var(--spacing-xl);
    font-size: var(--font-size-lg);
    min-height: 3rem;
  }

  /* Variants */
  .btn-primary {
    background-color: var(--color-accent);
    color: white;
    border-color: var(--color-accent);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
    border-color: var(--color-accent-hover);
    box-shadow: var(--shadow-md);
  }

  .btn-secondary {
    background-color: var(--color-bg-secondary);
    color: var(--color-text-primary);
    border-color: var(--color-border);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-bg-tertiary);
    border-color: var(--color-border-hover);
  }

  .btn-outline {
    background-color: transparent;
    color: var(--color-accent);
    border-color: var(--color-accent);
  }

  .btn-outline:hover:not(:disabled) {
    background-color: var(--color-accent-light);
  }

  .btn-ghost {
    background-color: transparent;
    color: var(--color-text-primary);
    border-color: transparent;
  }

  .btn-ghost:hover:not(:disabled) {
    background-color: var(--color-bg-secondary);
  }

  .btn-danger {
    background-color: var(--color-error);
    color: white;
    border-color: var(--color-error);
  }

  .btn-danger:hover:not(:disabled) {
    background-color: #dc2626;
    box-shadow: var(--shadow-md);
  }

  /* States */
  .btn:active:not(:disabled) {
    transform: scale(0.98);
  }

  .btn:disabled,
  .btn-disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-full-width {
    width: 100%;
  }

  .btn-loading {
    cursor: wait;
  }

  /* Loading spinner */
  .spinner {
    width: 1em;
    height: 1em;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .hidden {
    opacity: 0;
  }

  .btn-content {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm);
    transition: opacity var(--transition-fast);
  }
</style>
