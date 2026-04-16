<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import type { AuthState } from '$lib/types'

  export let auth: AuthState
  export let loading = false

  const dispatch = createEventDispatcher()

  let fileInput: HTMLInputElement
  let selectedFile: File | null = null
  let preview: string[] = []
  let error = ''

  function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement
    const file = target.files?.[0]

    if (!file) return

    // Check file type
    const validTypes = ['.csv', '.ofx', '.qfx']
    const isValid = validTypes.some(type => file.name.toLowerCase().endsWith(type))

    if (!isValid) {
      error = 'Please select a CSV, OFX, or QFX file'
      selectedFile = null
      preview = []
      return
    }

    error = ''
    selectedFile = file
    previewFile(file)
  }

  function previewFile(file: File) {
    const reader = new FileReader()

    reader.onload = (e) => {
      const content = e.target?.result as string
      const lines = content.split('\n').slice(0, 5)
      preview = lines.filter(line => line.trim())
    }

    reader.onerror = () => {
      error = 'Failed to read file'
    }

    reader.readAsText(file)
  }

  async function handleSubmit(e: Event) {
    e.preventDefault()

    if (!selectedFile) {
      error = 'Please select a file'
      return
    }

    const reader = new FileReader()

    reader.onload = async (e) => {
      try {
        const content = e.target?.result as string
        const base64 = btoa(content)

        dispatch('import', {
          file: selectedFile,
          data: base64,
        })
      } catch (err) {
        error = 'Failed to process file'
      }
    }

    reader.onerror = () => {
      error = 'Failed to read file'
    }

    reader.readAsText(selectedFile)
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault()
    e.dataTransfer!.dropEffect = 'copy'
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault()
    const file = e.dataTransfer?.files[0]
    if (file) {
      const input = fileInput as HTMLInputElement
      const dataTransfer = new DataTransfer()
      dataTransfer.items.add(file)
      input.files = dataTransfer.files
      handleFileSelect({ target: input } as any)
    }
  }
</script>

<form on:submit={handleSubmit} class="form">
  <div
    class="file-upload"
    on:dragover={handleDragOver}
    on:drop={handleDrop}
  >
    <input
      type="file"
      bind:this={fileInput}
      on:change={handleFileSelect}
      accept=".csv,.ofx,.qfx"
      disabled={loading}
      hidden
    />
    <button
      type="button"
      on:click={() => fileInput?.click()}
      disabled={loading}
      class="upload-btn"
    >
      <span class="icon">📁</span>
      <span class="text">
        {selectedFile
          ? `Selected: ${selectedFile.name}`
          : 'Click to select or drag & drop'}
      </span>
      <span class="hint">CSV, OFX, or QFX</span>
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if preview.length > 0}
    <div class="preview-section">
      <h3>File Preview</h3>
      <div class="preview">
        {#each preview as line}
          <code>{line}</code>
        {/each}
      </div>
    </div>
  {/if}

  <div class="form-info">
    <h3>Format CSV attendu :</h3>
    <code class="code-block">date,amount,merchant_name,description,category_id</code>
    <code class="code-block">2026-04-10,-50.00,Carrefour,Courses,groceries</code>
  </div>

  <div class="form-actions">
    <button
      type="submit"
      class="btn-primary"
      disabled={!selectedFile || loading}
    >
      {loading ? 'Importation…' : 'Importer le fichier'}
    </button>
  </div>
</form>

<style>
  .form {
    background: var(--card);
    border: 1px solid var(--border-hi);
    padding: 2rem;
    border-radius: 10px;
  }

  .file-upload {
    margin-bottom: 1.5rem;
  }

  .upload-btn {
    width: 100%;
    padding: 2.5rem 2rem;
    border: 1px dashed rgba(255,255,255,0.15);
    border-radius: 10px;
    background: transparent;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .upload-btn:hover:not(:disabled) {
    border-color: rgba(0,217,126,0.4);
    background: rgba(0,217,126,0.04);
  }

  .upload-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .icon {
    font-size: 2rem;
  }

  .text {
    font-weight: 500;
    color: var(--text);
    font-size: 0.9rem;
  }

  .hint {
    font-size: 0.8rem;
    color: var(--text-dim);
  }

  .error {
    background: rgba(255,51,88,0.1);
    border: 1px solid rgba(255,51,88,0.25);
    color: var(--cri);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    font-size: 0.85rem;
  }

  .preview-section {
    background: var(--card);
    border: 1px solid var(--border-hi);
    padding: 1.25rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
  }

  .preview-section h3 {
    margin: 0 0 0.75rem 0;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-dim);
  }

  .preview {
    max-height: 160px;
    overflow-y: auto;
  }

  code {
    display: block;
    padding: 0.4rem 0;
    color: var(--text-dim);
    font-family: var(--mono);
    font-size: 0.8rem;
    border-bottom: 1px solid rgba(255,255,255,0.04);
  }

  code:last-child { border: none; }

  .form-info {
    background: rgba(255,255,255,0.02);
    border: 1px solid var(--border);
    padding: 1.25rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
  }

  .form-info h3 {
    margin: 0 0 0.75rem 0;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-dim);
  }

  .code-block {
    display: block;
    padding: 0.5rem 0.75rem;
    background: rgba(255,255,255,0.07);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: 6px;
    margin: 0.4rem 0;
    overflow-x: auto;
    color: var(--em);
    font-family: var(--mono);
    font-size: 0.8rem;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
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

  .btn-primary:hover:not(:disabled) {
    background: var(--em-dk);
  }

  .btn-primary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
