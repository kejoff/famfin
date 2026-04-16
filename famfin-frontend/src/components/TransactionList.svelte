<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { Transaction, Category } from '$lib/types'

  export let transactions: Transaction[] = []
  export let categories: Category[] = []

  const dispatch = createEventDispatcher<{ edit: { transaction: Transaction } }>()

  function getCategoryName(categoryId: string | undefined): string {
    if (!categoryId) return 'Non catégorisé'
    const cat = categories.find(c => c.id === categoryId)
    return cat?.name || 'Inconnu'
  }

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr + 'T00:00:00')
    return date.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
  }
</script>

<div class="transactions-list">
  <table>
    <thead>
      <tr>
        <th>Date</th>
        <th>Marchand</th>
        <th>Catégorie</th>
        <th>Montant</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each transactions as txn (txn.id)}
        <tr>
          <td class="date">{formatDate(txn.date)}</td>
          <td class="merchant">
            <div class="merchant-name">{txn.merchant_name}</div>
            {#if txn.description}
              <div class="description">{txn.description}</div>
            {/if}
          </td>
          <td class="category">{getCategoryName(txn.category_id)}</td>
          <td class="amount" class:expense={txn.amount < 0}>
            {Math.abs(txn.amount).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} €
          </td>
          <td class="actions">
            <button
              class="btn-edit"
              on:click={() => dispatch('edit', { transaction: txn })}
            >
              Modifier
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .transactions-list {
    background: var(--card);
    border: 1px solid var(--border-hi);
    border-radius: 12px;
    overflow: hidden;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  thead {
    background: rgba(0,0,0,0.2);
    border-bottom: 1px solid var(--border-hi);
  }

  th {
    padding: 0.8rem 1.1rem;
    text-align: left;
    font-weight: 700;
    color: var(--text-dim);
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 1.5px;
  }

  tbody tr {
    transition: background 0.15s;
  }

  tbody tr:hover {
    background: var(--card-hov);
  }

  td {
    padding: 0.9rem 1.1rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.88rem;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  .date {
    color: var(--text-mid);
    font-size: 0.8rem;
    width: 120px;
    white-space: nowrap;
  }

  .merchant-name {
    font-weight: 500;
    color: var(--text);
  }

  .description {
    font-size: 0.78rem;
    color: var(--text-dim);
    margin-top: 3px;
  }

  .category {
    color: var(--em);
    font-size: 0.83rem;
  }

  .amount {
    font-family: var(--mono);
    font-variant-numeric: tabular-nums;
    font-weight: 500;
    text-align: right;
    width: 120px;
    color: var(--text);
  }

  .amount.expense {
    color: #FF6B6B;
  }

  .actions {
    text-align: right;
    width: 80px;
    padding-right: 1rem;
  }

  .btn-edit {
    background: none;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    font-size: 0.78rem;
    font-family: var(--font);
    padding: 3px 8px;
    border-radius: 4px;
    transition: all 0.15s;
  }

  .btn-edit:hover {
    color: var(--text);
    background: rgba(255,255,255,0.07);
  }

  @media (max-width: 768px) {
    th, td { padding: 0.65rem 0.75rem; }
    .description { display: none; }
  }
</style>
