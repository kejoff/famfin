<script lang="ts">
  import { onMount } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState, Goal } from '$lib/types'
  import GoalForm from './GoalForm.svelte'
  import GoalList from './GoalList.svelte'

  export let auth: AuthState

  let goals: Goal[] = []
  let loading = true
  let error = ''
  let showForm = false
  let editingId: string | null = null

  onMount(async () => {
    await loadGoals()
  })

  async function loadGoals() {
    try {
      loading = true
      goals = await apiCall(
        'GET',
        `/households/${auth.household_id}/goals`,
        auth
      )
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load goals'
    } finally {
      loading = false
    }
  }

  async function handleCreateUpdate(event: CustomEvent) {
    const { goal, isEdit } = event.detail
    try {
      if (isEdit) {
        await apiCall(
          'PUT',
          `/households/${auth.household_id}/goals/${goal.id}`,
          auth,
          goal
        )
      } else {
        await apiCall(
          'POST',
          `/households/${auth.household_id}/goals`,
          auth,
          goal
        )
      }
      await loadGoals()
      showForm = false
      editingId = null
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save goal'
    }
  }

  async function handleDelete(event: CustomEvent<{ goalId: string }>) {
    if (!confirm('Are you sure?')) return

    try {
      await apiCall(
        'DELETE',
        `/households/${auth.household_id}/goals/${event.detail.goalId}`,
        auth
      )
      await loadGoals()
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to delete goal'
    }
  }

  function handleEdit(event: CustomEvent<{ goal: Goal }>) {
    editingId = event.detail.goal.id
    showForm = true
  }

  function handleCancel() {
    showForm = false
    editingId = null
  }

  function handleNewGoal() {
    editingId = null
    showForm = true
  }
</script>

<div class="page-wrap"><div class="page">
  <div class="page-top">
    <div class="page-title-block">
      <h2>Projets de vie</h2>
      <p class="page-count">{goals.length} projet{goals.length !== 1 ? 's' : ''}</p>
    </div>
    <button class="btn-primary" on:click={handleNewGoal}>
      + Nouveau projet
    </button>
  </div>

  {#if error}
    <div class="page-error">{error}</div>
  {/if}

  {#if showForm}
    <GoalForm
      {auth}
      goalId={editingId}
      on:save={handleCreateUpdate}
      on:cancel={handleCancel}
    />
  {/if}

  {#if loading}
    <div class="page-loading">Chargement…</div>
  {:else if goals.length === 0}
    <div class="page-empty">
      <div class="empty-icon">◎</div>
      <p>Aucun projet pour le moment.</p>
      <p class="empty-hint">Définissez un objectif de vie — achat, voyage, épargne.</p>
    </div>
  {:else}
    <GoalList
      {goals}
      on:edit={handleEdit}
      on:delete={handleDelete}
    />
  {/if}
</div></div>

<style>
  .page-wrap {
    background: var(--surface);
    min-height: calc(100vh - 50px);
  }

  .page {
    padding: 2.5rem;
    max-width: 1100px;
    margin: 0 auto;
  }

  .page-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-bottom: 1.75rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .page-title-block { display: flex; flex-direction: column; gap: 3px; }

  h2 {
    margin: 0;
    font-family: var(--serif);
    font-style: italic;
    font-size: 1.75rem;
    font-weight: 400;
    color: var(--text);
    line-height: 1;
  }

  .page-count {
    margin: 0;
    font-size: 11px;
    color: var(--text-dim);
  }

  .btn-primary {
    background: var(--em);
    color: #002010;
    border: none;
    padding: 0.55rem 1.1rem;
    border-radius: 8px;
    cursor: pointer;
    font-family: var(--font);
    font-size: 0.82rem;
    font-weight: 600;
    transition: background 0.2s;
    white-space: nowrap;
  }

  .btn-primary:hover { background: var(--em-dk); }

  .page-error {
    background: rgba(255,51,88,0.08);
    border: 1px solid rgba(255,51,88,0.2);
    color: var(--cri);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    margin-bottom: 1.25rem;
    font-size: 0.85rem;
  }

  .page-loading {
    text-align: center;
    color: var(--text-dim);
    padding: 3rem;
    font-size: 0.9rem;
  }

  .page-empty {
    text-align: center;
    padding: 4rem 2rem;
  }

  .empty-icon {
    font-size: 2rem;
    color: var(--text-dim);
    margin-bottom: 1rem;
    opacity: 0.4;
  }

  .page-empty p {
    color: var(--text-mid);
    font-size: 0.95rem;
    margin: 0 0 0.4rem;
  }

  .empty-hint {
    color: var(--text-dim) !important;
    font-size: 0.82rem !important;
  }
</style>
