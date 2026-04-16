<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { AuthState } from '$lib/types'

  export let auth: AuthState
  export let categoryId: string | null = null

  const dispatch = createEventDispatcher()

  let name = ''
  let icon = '📁'
  let color = '#667eea'
  let loading = false
  let error = ''
  let isEdit = false

  // Common emojis for quick selection
  const commonEmojis = ['🛒', '🍔', '🚗', '🏠', '🎬', '💪', '📚', '✈️', '🎁', '💼', '📱', '👕', '💰', '🏥', '📖']

  // Common colors
  const commonColors = ['#667eea', '#764ba2', '#f093fb', '#4facfe', '#43e97b', '#fa709a', '#feca57', '#ff6348']

  async function handleSubmit(e: Event) {
    e.preventDefault()

    if (!name) {
      error = 'Category name is required'
      return
    }

    loading = true
    error = ''

    try {
      const categoryData = {
        name,
        icon,
        color,
      }

      dispatch('save', { category: categoryData, isEdit })
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save category'
    } finally {
      loading = false
    }
  }
</script>

<form on:submit={handleSubmit} class="form">
  <h3>{isEdit ? 'Modifier la catégorie' : 'Nouvelle catégorie'}</h3>

  {#if error}
    <div class="error-msg">{error}</div>
  {/if}

  <div class="form-group">
    <label for="name">Nom de la catégorie</label>
    <input
      type="text"
      id="name"
      bind:value={name}
      placeholder="ex: Courses alimentaires"
      required
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label>Icône</label>
    <div class="emoji-picker">
      {#each commonEmojis as emoji}
        <button
          type="button"
          class="emoji-btn"
          class:selected={icon === emoji}
          on:click={() => (icon = emoji)}
          disabled={loading}
        >
          {emoji}
        </button>
      {/each}
    </div>
    <input
      type="text"
      placeholder="Or type an emoji"
      bind:value={icon}
      maxlength="2"
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label>Couleur</label>
    <div class="color-picker">
      {#each commonColors as colorOption}
        <button
          type="button"
          class="color-btn"
          style="background: {colorOption}; box-shadow: {color === colorOption ? '0 0 0 2px #F2F2F7' : 'none'}"
          on:click={() => (color = colorOption)}
          disabled={loading}
        ></button>
      {/each}
    </div>
    <div class="color-input">
      <label for="color">Couleur personnalisée</label>
      <input type="color" id="color" bind:value={color} disabled={loading} />
    </div>
  </div>

  <div class="preview">
    <p>Aperçu :</p>
    <div class="preview-card" style="border-top-color: {color}">
      <div class="preview-icon">{icon}</div>
      <p>{name || 'Nom de la catégorie'}</p>
    </div>
  </div>

  <div class="form-actions">
    <button type="submit" class="btn-primary" disabled={loading}>
      {loading ? '…' : isEdit ? 'Mettre à jour' : 'Créer'}
    </button>
    <button
      type="button"
      class="btn-secondary"
      on:click={() => dispatch('cancel')}
      disabled={loading}
    >
      Annuler
    </button>
  </div>
</form>

<style>
  .form {
    background: var(--card);
    border: 1px solid var(--border-hi);
    padding: 1.5rem;
    border-radius: 10px;
    margin-bottom: 1.5rem;
    max-width: 600px;
  }

  h3 {
    margin: 0 0 1.25rem 0;
    font-family: var(--serif);
    font-style: italic;
    font-size: 1.1rem;
    font-weight: 400;
    color: var(--text);
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  label {
    display: block;
    margin-bottom: 5px;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: rgba(242,242,247,0.35);
  }

  input[type='text'] {
    width: 100%;
    padding: 0.6rem 0.8rem;
    background: rgba(255,255,255,0.08);
    border: 1px solid rgba(255,255,255,0.15);
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.9rem;
    color: var(--text);
    box-sizing: border-box;
  }

  input[type='text']::placeholder { color: rgba(242,242,247,0.2); }

  input[type='text']:focus {
    outline: none;
    border-color: rgba(0,217,126,0.4);
    box-shadow: 0 0 0 3px rgba(0,217,126,0.1);
  }

  input[type='text']:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .emoji-picker {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.75rem;
  }

  .emoji-btn {
    font-size: 1.3rem;
    padding: 0.4rem;
    border: 1px solid rgba(255,255,255,0.15);
    background: rgba(255,255,255,0.04);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .emoji-btn:hover:not(:disabled) {
    border-color: rgba(0,217,126,0.3);
    background: rgba(0,217,126,0.08);
  }

  .emoji-btn.selected {
    border-color: var(--em);
    background: rgba(0,217,126,0.1);
  }

  .emoji-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .color-picker {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
  }

  .color-btn {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    transition: transform 0.15s;
  }

  .color-btn:hover:not(:disabled) {
    transform: scale(1.15);
  }

  .color-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .color-input {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .color-input label {
    margin: 0;
    white-space: nowrap;
  }

  .color-input input {
    height: 36px;
    width: 64px;
    cursor: pointer;
    border-radius: 6px;
    border: 1px solid rgba(255,255,255,0.15);
    background: transparent;
  }

  .preview {
    background: rgba(255,255,255,0.02);
    border: 1px solid var(--border);
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1.25rem;
  }

  .preview > p {
    margin: 0 0 0.75rem 0;
    color: var(--text-dim);
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
  }

  .preview-card {
    background: rgba(255,255,255,0.04);
    padding: 0.75rem;
    border-radius: 8px;
    border-top: 3px solid;
    text-align: center;
  }

  .preview-icon {
    font-size: 1.75rem;
    margin-bottom: 0.3rem;
  }

  .preview-card p {
    margin: 0;
    color: var(--text);
    font-weight: 500;
    font-size: 0.85rem;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .btn-primary,
  .btn-secondary {
    flex: 1;
    padding: 0.65rem;
    border: none;
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-primary {
    background: var(--em);
    color: #002010;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--em-dk);
  }

  .btn-secondary {
    background: transparent;
    color: var(--text-dim);
    border: 1px solid var(--border);
  }

  .btn-secondary:hover:not(:disabled) {
    background: #e0e0e0;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error-msg {
    background: #fee;
    color: #c33;
    padding: 0.75rem;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  @media (max-width: 768px) {
    .form {
      padding: 1.5rem;
    }

    .form-actions {
      flex-direction: column;
    }
  }
</style>
