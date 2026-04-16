<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState, Category, Transaction } from '$lib/types'

  export let auth: AuthState
  export let categories: Category[] = []
  export let transactionId: string | null = null

  const dispatch = createEventDispatcher()

  let date = new Date().toISOString().split('T')[0]
  let amount = ''
  let merchantName = ''
  let description = ''
  let categoryId = ''
  let loading = false
  let error = ''
  let isEdit = false

  onMount(async () => {
    if (transactionId) {
      await loadTransaction()
    }
  })

  async function loadTransaction() {
    try {
      const txn: Transaction = await apiCall(
        'GET',
        `/households/${auth.household_id}/transactions/${transactionId}`,
        auth
      )
      date = txn.date
      amount = Math.abs(txn.amount).toString()
      merchantName = txn.merchant_name
      description = txn.description || ''
      categoryId = txn.category_id || ''
      isEdit = true
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load transaction'
    }
  }

  async function handleSubmit(e: Event) {
    e.preventDefault()

    if (!date || !amount || !merchantName) {
      error = 'Date, amount, and merchant name are required'
      return
    }

    loading = true
    error = ''

    try {
      const transactionData = {
        date,
        amount: -Math.abs(parseFloat(amount)),
        merchant_name: merchantName,
        description: description || null,
        category_id: categoryId || null,
        import_fingerprint: `manual_${Date.now()}_${Math.random()}`,
      }

      dispatch('save', { transaction: transactionData, isEdit })
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save transaction'
    } finally {
      loading = false
    }
  }
</script>

<form on:submit={handleSubmit} class="form">
  <h3>{isEdit ? 'Modifier la transaction' : 'Nouvelle transaction'}</h3>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="form-group">
    <label for="date">Date</label>
    <input
      type="date"
      id="date"
      bind:value={date}
      required
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="merchant">Marchand</label>
    <input
      type="text"
      id="merchant"
      bind:value={merchantName}
      placeholder="ex: Carrefour"
      required
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="amount">Montant</label>
    <div class="amount-input">
      <span class="currency">€</span>
      <input
        type="number"
        id="amount"
        bind:value={amount}
        placeholder="0.00"
        step="0.01"
        min="0"
        required
        disabled={loading}
      />
    </div>
  </div>

  <div class="form-group">
    <label for="category">Catégorie</label>
    <select id="category" bind:value={categoryId} disabled={loading}>
      <option value="">Non catégorisé</option>
      {#each categories as cat}
        <option value={cat.id}>{cat.name}</option>
      {/each}
    </select>
  </div>

  <div class="form-group">
    <label for="description">Description (optionnel)</label>
    <input
      type="text"
      id="description"
      bind:value={description}
      placeholder="ex: Courses hebdomadaires"
      disabled={loading}
    />
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
  select {
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

  select option {
    background: #1A1A24;
    color: var(--text);
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: rgba(0,217,126,0.4);
    box-shadow: 0 0 0 3px rgba(0,217,126,0.1);
  }

  input:disabled,
  select:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  input::placeholder { color: rgba(242,242,247,0.2); }

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

    .form-actions {
      flex-direction: column;
    }
  }
</style>
