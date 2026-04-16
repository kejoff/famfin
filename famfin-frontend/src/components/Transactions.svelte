<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState, Transaction, Category } from '$lib/types'
  import TransactionList from './TransactionList.svelte'
  import TransactionForm from './TransactionForm.svelte'

  export let auth: AuthState

  const dispatch = createEventDispatcher()

  let transactions: Transaction[] = []
  let categories: Category[] = []
  let loading = true
  let error = ''
  let showForm = false
  let editingId: string | null = null
  let limit = 20
  let offset = 0

  onMount(async () => {
    await loadTransactions()
    await loadCategories()
  })

  async function loadTransactions() {
    try {
      loading = true
      transactions = await apiCall(
        'GET',
        `/households/${auth.household_id}/transactions?limit=${limit}&offset=${offset}`,
        auth
      )
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load transactions'
    } finally {
      loading = false
    }
  }

  async function loadCategories() {
    try {
      categories = await apiCall(
        'GET',
        `/households/${auth.household_id}/categories`,
        auth
      )
    } catch (e) {
      console.error('Failed to load categories:', e)
    }
  }

  async function handleCreateUpdate(event: CustomEvent) {
    const { transaction, isEdit } = event.detail
    try {
      if (isEdit) {
        await apiCall(
          'PUT',
          `/households/${auth.household_id}/transactions/${transaction.id}`,
          auth,
          transaction
        )
      } else {
        await apiCall(
          'POST',
          `/households/${auth.household_id}/transactions`,
          auth,
          transaction
        )
      }
      await loadTransactions()
      showForm = false
      editingId = null
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save transaction'
    }
  }

  function handleEdit(event: CustomEvent<{ transaction: Transaction }>) {
    editingId = event.detail.transaction.id
    showForm = true
  }

  function handleCancel() {
    showForm = false
    editingId = null
  }

  function handleNewTransaction() {
    editingId = null
    showForm = true
  }

  function handleGoToImport() {
    dispatch('navigate', 'import')
  }
</script>

<div class="page-wrap"><div class="page">
  <div class="page-top">
    <div class="page-title-block">
      <h2>Transactions</h2>
      <p class="page-count">{transactions.length} transaction{transactions.length !== 1 ? 's' : ''}</p>
    </div>
    <button class="btn-primary" on:click={handleNewTransaction}>
      + Nouvelle transaction
    </button>
  </div>

  {#if error}
    <div class="page-error">{error}</div>
  {/if}

  {#if showForm}
    <TransactionForm
      {auth}
      {categories}
      transactionId={editingId}
      on:save={handleCreateUpdate}
      on:cancel={handleCancel}
    />
  {/if}

  {#if loading}
    <div class="page-loading">Chargement…</div>
  {:else if transactions.length === 0}
    <div class="page-empty">
      <svg class="empty-icon" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="8" y="6" width="32" height="36" rx="3" stroke="currentColor" stroke-width="2"/>
        <path d="M16 16h16M16 23h16M16 30h10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
      <p class="empty-title">Aucune transaction pour le moment.</p>
      <p class="empty-hint">Commencez par importer vos données ou ajoutez une transaction.</p>
      <div class="empty-actions">
        <button class="btn-secondary" on:click={handleGoToImport}>Importer un fichier CSV</button>
        <button class="btn-primary" on:click={handleNewTransaction}>+ Nouvelle transaction</button>
      </div>
    </div>
  {:else}
    <TransactionList
      {transactions}
      {categories}
      on:edit={handleEdit}
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

  .page-title-block {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  h2 {
    margin: 0;
    font-family: var(--font);
    font-style: normal;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text);
    line-height: 1;
    letter-spacing: -0.3px;
  }

  .page-count {
    margin: 0;
    font-size: 11px;
    color: var(--text-dim);
    letter-spacing: 0.3px;
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
    letter-spacing: 0.2px;
    transition: background 0.2s;
    white-space: nowrap;
  }

  .btn-primary:hover {
    background: var(--em-dk);
  }

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
    padding: 5rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0;
  }

  .empty-icon {
    width: 48px;
    height: 48px;
    color: var(--text-dim);
    opacity: 0.4;
    margin-bottom: 1.25rem;
  }

  .empty-title {
    color: var(--text-mid);
    font-size: 0.95rem;
    font-weight: 500;
    margin: 0 0 0.4rem;
  }

  .empty-hint {
    color: var(--text-dim);
    font-size: 0.82rem;
    margin: 0 0 1.75rem;
  }

  .empty-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .btn-secondary {
    background: transparent;
    color: var(--text-mid);
    border: 1px solid var(--border-hi);
    padding: 0.55rem 1.1rem;
    border-radius: 8px;
    cursor: pointer;
    font-family: var(--font);
    font-size: 0.82rem;
    font-weight: 500;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .btn-secondary:hover {
    border-color: rgba(255,255,255,0.25);
    color: var(--text);
  }
</style>
