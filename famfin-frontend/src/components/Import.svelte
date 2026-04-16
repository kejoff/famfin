<script lang="ts">
  import { apiCall } from '$lib/stores'
  import type { AuthState, ImportResponse } from '$lib/types'
  import ImportForm from './ImportForm.svelte'
  import ImportResult from './ImportResult.svelte'

  export let auth: AuthState

  let result: ImportResponse | null = null
  let loading = false
  let error = ''

  async function handleImportFile(event: CustomEvent) {
    const { file, data } = event.detail

    loading = true
    error = ''
    result = null

    try {
      result = await apiCall(
        'POST',
        `/households/${auth.household_id}/import-file`,
        auth,
        {
          filename: file.name,
          data: data,
        }
      )
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to import file'
    } finally {
      loading = false
    }
  }

  function handleReset() {
    result = null
    error = ''
  }
</script>

<div class="page-wrap"><div class="page">
  <div class="page-top">
    <div class="page-title-block">
      <h2>Importer</h2>
      <p class="page-sub">Fichiers CSV ou OFX de votre banque</p>
    </div>
  </div>

  {#if error}
    <div class="page-error">{error}</div>
  {/if}

  {#if result}
    <ImportResult {result} on:done={handleReset} />
  {:else}
    <ImportForm {auth} {loading} on:import={handleImportFile} />
  {/if}
</div></div>

<style>
  .page-wrap {
    background: var(--surface);
    min-height: calc(100vh - 50px);
  }

  .page {
    padding: 2.5rem;
    max-width: 760px;
    margin: 0 auto;
  }

  .page-top {
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

  .page-sub { margin: 0; font-size: 11px; color: var(--text-dim); }

  .page-error {
    background: rgba(255,51,88,0.08);
    border: 1px solid rgba(255,51,88,0.2);
    color: var(--cri);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    margin-bottom: 1.25rem;
    font-size: 0.85rem;
  }
</style>
