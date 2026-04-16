<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { Category } from '$lib/types'

  export let categories: Category[] = []

  const dispatch = createEventDispatcher()

  // Common icon emojis for categories
  const iconMap: Record<string, string> = {
    '🛒': 'Shopping',
    '🍔': 'Food & Dining',
    '🚗': 'Transportation',
    '🏠': 'Home & Utilities',
    '🎬': 'Entertainment',
    '💪': 'Health & Fitness',
    '📚': 'Education',
    '✈️': 'Travel',
    '🎁': 'Gifts',
    '💼': 'Work',
  }
</script>

<div class="categories-grid">
  {#each categories as category (category.id)}
    <div class="category-card" style="border-top-color: {category.color}">
      <div class="card-icon">{category.icon || '📁'}</div>
      <h3>{category.name}</h3>
      <div class="card-meta">
        <span class="color-badge" style="background: {category.color}"></span>
        <span class="date">
          {new Date(category.created_at).toLocaleDateString('fr-FR')}
        </span>
      </div>
      <button
        class="btn-edit"
        on:click={() => dispatch('edit', { category })}
      >
        Modifier
      </button>
    </div>
  {/each}
</div>

<style>
  .categories-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
  }

  .category-card {
    background: var(--card);
    border: 1px solid var(--border-hi);
    border-top: 3px solid;
    padding: 1.25rem;
    border-radius: 12px;
    text-align: center;
    transition: background 0.15s;
  }

  .category-card:hover {
    background: var(--card-hov);
  }

  .card-icon {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  h3 {
    margin: 0.4rem 0 0.75rem 0;
    color: var(--text);
    font-size: 0.9rem;
    font-weight: 500;
    word-break: break-word;
  }

  .card-meta {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    margin-bottom: 0.75rem;
    font-size: 0.78rem;
  }

  .color-badge {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .date {
    color: var(--text-dim);
  }

  .btn-edit {
    background: rgba(255,255,255,0.06);
    color: var(--text-dim);
    border: 1px solid var(--border);
    padding: 0.45rem 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font);
    font-size: 0.8rem;
    transition: all 0.2s;
    width: 100%;
  }

  .btn-edit:hover {
    background: rgba(255,255,255,0.1);
    color: var(--text);
  }

  @media (max-width: 768px) {
    .categories-grid { grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); }
  }
</style>
