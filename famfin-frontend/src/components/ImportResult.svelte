<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { ImportResponse } from '$lib/types'

  export let result: ImportResponse

  const dispatch = createEventDispatcher()

  function getTotalProcessed(): number {
    return result.imported_count + result.skipped_count
  }

  function getSuccessRate(): number {
    const total = getTotalProcessed()
    return total > 0 ? Math.round((result.imported_count / total) * 100) : 0
  }
</script>

<div class="result-container">
  <div class="result-card" class:success={result.imported_count > 0}>
    <div class="result-header">
      <h3>
        {#if result.imported_count > 0}
          Importation terminée
        {:else}
          Importation traitée
        {/if}
      </h3>
    </div>

    <div class="stats">
      <div class="stat">
        <div class="stat-label">Importées</div>
        <div class="stat-value imported">{result.imported_count}</div>
      </div>
      <div class="stat">
        <div class="stat-label">Ignorées</div>
        <div class="stat-value skipped">{result.skipped_count}</div>
      </div>
      <div class="stat">
        <div class="stat-label">Total</div>
        <div class="stat-value">{getTotalProcessed()}</div>
      </div>
      <div class="stat">
        <div class="stat-label">Réussite</div>
        <div class="stat-value">{getSuccessRate()} %</div>
      </div>
    </div>

    {#if result.imported_count > 0}
      <div class="progress-bar">
        <div class="progress" style="width: {getSuccessRate()}%"></div>
      </div>
    {/if}

    {#if result.errors.length > 0}
      <div class="errors">
        <h4>Problèmes détectés</h4>
        <ul>
          {#each result.errors as err}
            <li>{err}</li>
          {/each}
        </ul>
      </div>
    {/if}

    {#if result.imported_count > 0}
      <div class="success-message">
        <p>{result.imported_count} transaction(s) importée(s) avec succès.{#if result.skipped_count > 0} {result.skipped_count} ignorée(s) (doublons).{/if}</p>
      </div>
    {/if}
  </div>

  <div class="actions">
    <button class="btn-primary" on:click={() => dispatch('done')}>
      Importer un autre fichier
    </button>
  </div>
</div>

<style>
  .result-container {
    margin-bottom: 2rem;
  }

  .result-card {
    background: var(--card);
    border: 1px solid var(--border-hi);
    padding: 1.5rem;
    border-radius: 10px;
    margin-bottom: 1rem;
    border-left: 3px solid var(--amb);
  }

  .result-card.success {
    border-left-color: var(--em);
  }

  .result-header {
    margin-bottom: 1.25rem;
  }

  h3 {
    margin: 0;
    color: var(--text);
    font-size: 1rem;
    font-weight: 500;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .stat {
    background: rgba(255,255,255,0.08);
    border: 1px solid var(--border);
    padding: 0.75rem 0.5rem;
    border-radius: 8px;
    text-align: center;
  }

  .stat-label {
    color: var(--text-dim);
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    margin-bottom: 0.4rem;
  }

  .stat-value {
    font-family: var(--mono);
    font-size: 1.5rem;
    font-weight: 500;
    font-variant-numeric: tabular-nums;
    color: var(--text);
  }

  .stat-value.imported { color: var(--em); }
  .stat-value.skipped  { color: var(--amb); }

  .progress-bar {
    width: 100%;
    height: 4px;
    background: rgba(255,255,255,0.08);
    border-radius: 2px;
    overflow: hidden;
    margin-bottom: 1.25rem;
  }

  .progress {
    height: 100%;
    background: var(--em);
    transition: width 0.5s ease;
  }

  .errors {
    background: rgba(255,133,0,0.08);
    border: 1px solid rgba(255,133,0,0.2);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    margin-bottom: 0.75rem;
  }

  .errors h4 {
    margin: 0 0 0.5rem 0;
    color: var(--amb);
    font-size: 0.85rem;
    font-weight: 600;
  }

  .errors ul {
    margin: 0;
    padding-left: 1.25rem;
  }

  .errors li {
    color: rgba(255,133,0,0.8);
    font-size: 0.82rem;
    margin-bottom: 0.3rem;
  }

  .success-message {
    background: rgba(0,217,126,0.08);
    border: 1px solid rgba(0,217,126,0.2);
    padding: 0.75rem 1rem;
    border-radius: 8px;
  }

  .success-message p {
    margin: 0;
    color: var(--em);
    font-size: 0.9rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }

  .btn-primary {
    flex: 1;
    padding: 0.7rem;
    background: var(--em);
    color: #002010;
    border: none;
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-primary:hover { background: var(--em-dk); }

  @media (max-width: 768px) {
    .stats { grid-template-columns: repeat(2, 1fr); }
  }
</style>
