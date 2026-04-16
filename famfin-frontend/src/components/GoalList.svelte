<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { Goal } from '$lib/types'

  export let goals: Goal[] = []

  const dispatch = createEventDispatcher()

  function getProgress(goal: Goal): number {
    return goal.target_amount > 0 ? (goal.current_amount / goal.target_amount) * 100 : 0
  }

  function formatDate(dateStr: string | undefined): string {
    if (!dateStr) return 'Sans échéance'
    const date = new Date(dateStr)
    return date.toLocaleDateString('fr-FR', { year: 'numeric', month: 'short', day: 'numeric' })
  }
</script>

<div class="goals-grid">
  {#each goals as goal (goal.id)}
    <div class="goal-card">
      <div class="goal-header">
        <h3>{goal.name}</h3>
        <div class="goal-actions">
          <button
            class="btn-edit"
            on:click={() => dispatch('edit', { goal })}
          >
            Modifier
          </button>
          <button
            class="btn-delete"
            on:click={() => dispatch('delete', { goalId: goal.id })}
          >
            Supprimer
          </button>
        </div>
      </div>

      {#if goal.description}
        <p class="description">{goal.description}</p>
      {/if}

      <div class="goal-progress">
        <div class="progress-bar">
          <div class="progress" style="width: {Math.min(getProgress(goal), 100)}%"></div>
        </div>
        <div class="progress-text">
          <span class="current">{goal.current_amount.toLocaleString('fr-FR', {minimumFractionDigits:2, maximumFractionDigits:2})} €</span>
          <span class="target">/ {goal.target_amount.toLocaleString('fr-FR', {minimumFractionDigits:2, maximumFractionDigits:2})} €</span>
          <span class="percentage">{getProgress(goal).toFixed(0)} %</span>
        </div>
      </div>

      <div class="goal-info">
        <span class="deadline">
          {formatDate(goal.deadline)}
        </span>
        {#if goal.generates_income}
          <span class="tag income">Génère des revenus</span>
        {/if}
        {#if goal.creates_expenses}
          <span class="tag expense">Crée des dépenses</span>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .goals-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }

  .goal-card {
    background: var(--card);
    border: 1px solid var(--border-hi);
    padding: 1.25rem;
    border-radius: 12px;
    transition: background 0.15s;
  }

  .goal-card:hover {
    background: var(--card-hov);
  }

  .goal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.75rem;
  }

  h3 {
    margin: 0;
    color: var(--text);
    font-size: 1rem;
    font-weight: 500;
    flex: 1;
  }

  .goal-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn-edit {
    background: none;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    font-family: var(--font);
    font-size: 0.8rem;
    padding: 0;
    transition: color 0.2s;
  }

  .btn-edit:hover { color: var(--text); }

  .btn-delete {
    background: none;
    border: none;
    color: rgba(255,51,88,0.5);
    cursor: pointer;
    font-family: var(--font);
    font-size: 0.8rem;
    padding: 0;
    transition: color 0.2s;
  }

  .btn-delete:hover { color: var(--cri); }

  .description {
    color: var(--text-dim);
    font-size: 0.85rem;
    margin: 0 0 0.75rem 0;
    line-height: 1.5;
  }

  .goal-progress {
    margin-bottom: 0.75rem;
  }

  .progress-bar {
    width: 100%;
    height: 4px;
    background: rgba(255,255,255,0.08);
    border-radius: 2px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .progress {
    height: 100%;
    background: var(--em);
    transition: width 0.3s ease;
  }

  .progress-text {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    font-size: 0.82rem;
  }

  .current {
    font-family: var(--mono);
    font-variant-numeric: tabular-nums;
    color: var(--em);
    font-weight: 500;
  }

  .target {
    font-family: var(--mono);
    font-variant-numeric: tabular-nums;
    color: var(--text-dim);
  }

  .percentage {
    margin-left: auto;
    color: var(--text-dim);
    font-size: 0.78rem;
  }

  .goal-info {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    font-size: 0.8rem;
    margin-top: 0.5rem;
  }

  .deadline {
    color: var(--text-dim);
  }

  .tag {
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .tag.income {
    background: rgba(0,217,126,0.1);
    border: 1px solid rgba(0,217,126,0.25);
    color: var(--em);
  }

  .tag.expense {
    background: rgba(255,133,0,0.1);
    border: 1px solid rgba(255,133,0,0.25);
    color: var(--amb);
  }

  @media (max-width: 768px) {
    .goals-grid { grid-template-columns: 1fr; }
  }
</style>
