<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState, Goal } from '$lib/types'

  export let auth: AuthState
  export let goalId: string | null = null

  const dispatch = createEventDispatcher()

  let name = ''
  let description = ''
  let targetAmount = ''
  let currentAmount = ''
  let deadline = ''
  let generatesIncome = false
  let createsExpenses = false
  let loading = false
  let error = ''
  let isEdit = false

  onMount(async () => {
    if (goalId) {
      await loadGoal()
    }
  })

  async function loadGoal() {
    try {
      const goal: Goal = await apiCall(
        'GET',
        `/households/${auth.household_id}/goals/${goalId}`,
        auth
      )
      name = goal.name
      description = goal.description || ''
      targetAmount = goal.target_amount.toString()
      currentAmount = goal.current_amount.toString()
      deadline = goal.deadline || ''
      generatesIncome = goal.generates_income
      createsExpenses = goal.creates_expenses
      isEdit = true
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load goal'
    }
  }

  async function handleSubmit(e: Event) {
    e.preventDefault()

    if (!name || !targetAmount) {
      error = 'Name and target amount are required'
      return
    }

    loading = true
    error = ''

    try {
      const goalData = {
        name,
        description: description || null,
        target_amount: parseFloat(targetAmount),
        current_amount: parseFloat(currentAmount) || 0,
        deadline: deadline || null,
        generates_income: generatesIncome,
        creates_expenses: createsExpenses,
      }

      dispatch('save', { goal: goalData, isEdit })
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save goal'
    } finally {
      loading = false
    }
  }
</script>

<form on:submit={handleSubmit} class="form">
  <h3>{isEdit ? 'Modifier le projet' : 'Nouveau projet de vie'}</h3>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="form-group">
    <label for="name">Nom du projet</label>
    <input
      type="text"
      id="name"
      bind:value={name}
      placeholder="ex: Achat appartement à Lyon"
      required
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="description">Description (optionnel)</label>
    <textarea
      id="description"
      bind:value={description}
      placeholder="À quoi sert ce projet ?"
      disabled={loading}
      rows="2"
    ></textarea>
  </div>

  <div class="form-row">
    <div class="form-group">
      <label for="target">Objectif</label>
      <div class="amount-input">
        <span class="currency">€</span>
        <input
          type="number"
          id="target"
          bind:value={targetAmount}
          placeholder="0.00"
          step="0.01"
          min="0"
          required
          disabled={loading}
        />
      </div>
    </div>

    <div class="form-group">
      <label for="current">Montant actuel</label>
      <div class="amount-input">
        <span class="currency">€</span>
        <input
          type="number"
          id="current"
          bind:value={currentAmount}
          placeholder="0.00"
          step="0.01"
          min="0"
          disabled={loading}
        />
      </div>
    </div>
  </div>

  <div class="form-group">
    <label for="deadline">Échéance (optionnel)</label>
    <input
      type="date"
      id="deadline"
      bind:value={deadline}
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label class="checkbox">
      <input type="checkbox" bind:checked={generatesIncome} disabled={loading} />
      Ce projet génère des revenus
    </label>
  </div>

  <div class="form-group">
    <label class="checkbox">
      <input type="checkbox" bind:checked={createsExpenses} disabled={loading} />
      Ce projet crée des dépenses récurrentes
    </label>
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

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
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

  input,
  textarea {
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

  textarea {
    resize: vertical;
  }

  input::placeholder,
  textarea::placeholder { color: rgba(242,242,247,0.2); }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: rgba(0,217,126,0.4);
    box-shadow: 0 0 0 3px rgba(0,217,126,0.1);
  }

  input:disabled,
  textarea:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .amount-input {
    display: flex;
    align-items: center;
    position: relative;
  }

  .currency {
    position: absolute;
    left: 0.75rem;
    color: var(--text-dim);
    font-family: var(--mono);
  }

  .amount-input input {
    padding-left: 2rem;
  }

  .checkbox {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    font-weight: normal;
    text-transform: none;
    letter-spacing: 0;
    color: var(--text-dim);
    cursor: pointer;
  }

  input[type='checkbox'] {
    width: auto;
    padding: 0;
    cursor: pointer;
    accent-color: var(--em);
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
    background: rgba(255,255,255,0.08);
    color: var(--text);
  }

  button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .error {
    background: rgba(255,51,88,0.1);
    border: 1px solid rgba(255,51,88,0.25);
    color: var(--cri);
    padding: 0.65rem 0.8rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    font-size: 0.85rem;
  }

  @media (max-width: 768px) {
    .form {
      padding: 1.5rem;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .form-actions {
      flex-direction: column;
    }
  }
</style>
