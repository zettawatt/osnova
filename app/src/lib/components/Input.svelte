<script lang="ts">
  interface InputProps {
    type?: 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' | 'search';
    value?: string;
    placeholder?: string;
    label?: string;
    error?: string;
    hint?: string;
    disabled?: boolean;
    required?: boolean;
    readonly?: boolean;
    autofocus?: boolean;
    size?: 'sm' | 'md' | 'lg';
    fullWidth?: boolean;
    oninput?: (event: Event) => void;
    onchange?: (event: Event) => void;
    onfocus?: (event: FocusEvent) => void;
    onblur?: (event: FocusEvent) => void;
  }

  let {
    type = 'text',
    value = $bindable(''),
    placeholder,
    label,
    error,
    hint,
    disabled = false,
    required = false,
    readonly = false,
    autofocus = false,
    size = 'md',
    fullWidth = false,
    oninput,
    onchange,
    onfocus,
    onblur
  }: InputProps = $props();

  let inputId = $state(`input-${Math.random().toString(36).substring(7)}`);
  let isFocused = $state(false);

  let inputClass = $derived(
    [
      'input',
      `input-${size}`,
      fullWidth && 'input-full-width',
      error && 'input-error',
      disabled && 'input-disabled',
      isFocused && 'input-focused'
    ]
      .filter(Boolean)
      .join(' ')
  );

  function handleFocus(event: FocusEvent) {
    isFocused = true;
    onfocus?.(event);
  }

  function handleBlur(event: FocusEvent) {
    isFocused = false;
    onblur?.(event);
  }
</script>

<div class="input-wrapper" class:full-width={fullWidth}>
  {#if label}
    <label for={inputId} class="label">
      {label}
      {#if required}
        <span class="required">*</span>
      {/if}
    </label>
  {/if}

  <input
    id={inputId}
    class={inputClass}
    {type}
    bind:value
    {placeholder}
    {disabled}
    {required}
    {readonly}
    {autofocus}
    {oninput}
    {onchange}
    onfocus={handleFocus}
    onblur={handleBlur}
    aria-invalid={!!error}
    aria-describedby={error ? `${inputId}-error` : hint ? `${inputId}-hint` : undefined}
  />

  {#if error}
    <div id={`${inputId}-error`} class="error-message">
      {error}
    </div>
  {:else if hint}
    <div id={`${inputId}-hint`} class="hint-message">
      {hint}
    </div>
  {/if}
</div>

<style>
  .input-wrapper {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .input-wrapper.full-width {
    width: 100%;
  }

  .label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-primary);
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .required {
    color: var(--color-error);
  }

  .input {
    font-family: var(--font-sans);
    font-size: var(--font-size-base);
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    outline: none;
  }

  /* Sizes */
  .input-sm {
    padding: var(--spacing-xs) var(--spacing-md);
    min-height: 2rem;
    font-size: var(--font-size-sm);
  }

  .input-md {
    padding: var(--spacing-sm) var(--spacing-md);
    min-height: 2.5rem;
  }

  .input-lg {
    padding: var(--spacing-md) var(--spacing-lg);
    min-height: 3rem;
    font-size: var(--font-size-lg);
  }

  /* States */
  .input:hover:not(:disabled):not(:readonly) {
    border-color: var(--color-border-hover);
  }

  .input:focus,
  .input-focused {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px var(--color-accent-light);
  }

  .input:disabled,
  .input-disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .input:readonly {
    background-color: var(--color-bg-secondary);
    cursor: default;
  }

  .input-error {
    border-color: var(--color-error);
  }

  .input-error:focus {
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
  }

  .input-full-width {
    width: 100%;
  }

  /* Messages */
  .error-message {
    font-size: var(--font-size-sm);
    color: var(--color-error);
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .hint-message {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }

  /* Placeholder */
  .input::placeholder {
    color: var(--color-text-tertiary);
  }
</style>
