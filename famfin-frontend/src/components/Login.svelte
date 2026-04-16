<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { API_BASE } from '$lib/stores'
  import type { AuthState } from '$lib/types'

  const dispatch = createEventDispatcher<{ login: AuthState }>()

  let householdId = ''
  let password = ''
  let isCreateMode = false
  let name = ''
  let error = ''
  let loading = false

  async function handleLogin() {
    if (!householdId || !password) {
      error = 'Identifiant et mot de passe requis'
      return
    }

    loading = true
    error = ''

    try {
      const id = householdId.trim()
      const response = await fetch(
        `${API_BASE}/households/${encodeURIComponent(id)}/login`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ password }),
          credentials: 'include',
        }
      )

      if (!response.ok) {
        throw new Error('Identifiants invalides')
      }

      const data = await response.json()
      // Session in httpOnly cookie; use backend-confirmed id
      dispatch('login', {
        session_id: 'cookie',
        expires_at: data.expires_at || '',
        household_id: data.household_id || id,
      })
    } catch (e) {
      error = e instanceof Error ? e.message : 'Connexion échouée'
    } finally {
      loading = false
    }
  }

  async function handleCreateHousehold() {
    if (!householdId || !password || !name) {
      error = 'Tous les champs sont requis'
      return
    }

    // Validate password strength
    if (password.length < 12) {
      error = 'Mot de passe: min 12 caractères'
      return
    }
    const hasUpper = /[A-Z]/.test(password)
    const hasLower = /[a-z]/.test(password)
    const hasDigit = /[0-9]/.test(password)
    if (!hasUpper || !hasLower || !hasDigit) {
      error = 'Mot de passe: majuscule, minuscule, chiffre requis'
      return
    }

    loading = true
    error = ''

    try {
      const id = householdId.trim()
      const response = await fetch(`${API_BASE}/households`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id, name: name.trim(), password }),
        credentials: 'include',
      })

      if (!response.ok) {
        const msg = await response.text()
        throw new Error(msg || 'Création du foyer échouée')
      }

      const data = await response.json()
      // Session in httpOnly cookie; use backend-confirmed id
      dispatch('login', {
        session_id: 'cookie',
        expires_at: data.expires_at || '',
        household_id: data.id || id,
      })
    } catch (e) {
      error = e instanceof Error ? e.message : 'Création échouée'
    } finally {
      loading = false
    }
  }
</script>

<div class="login-wrap">
  <!-- Painted canvas background -->
  <div class="canvas">
    <svg viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg"
         style="position:absolute;inset:0;width:100%;height:100%;">
      <path d="M -80 100 C 80 40, 280 120, 420 220 C 540 300, 600 400, 560 480 C 520 560, 380 540, 280 480 C 180 420, 80 320, 40 240 C 0 160, -60 140, -80 100Z"
            fill="#00D97E" opacity="0.15"/>
      <path d="M 600 -40 C 720 40, 840 160, 820 300 C 800 440, 700 500, 640 440 C 580 380, 620 280, 580 200 C 540 120, 520 40, 600 -40Z"
            fill="#00FF9F" opacity="0.09"/>
      <path d="M 100 400 C 200 360, 380 350, 500 400 C 620 450, 700 540, 660 600 L 0 600 Z"
            fill="#005C2E" opacity="0.4"/>
      <radialGradient id="vgL" cx="50%" cy="100%" r="70%">
        <stop offset="0%" stop-color="#0B0B12" stop-opacity="0.7"/>
        <stop offset="100%" stop-color="#0B0B12" stop-opacity="0"/>
      </radialGradient>
      <rect width="800" height="600" fill="url(#vgL)"/>
    </svg>
    <!-- Grain overlay -->
    <div class="grain"></div>
  </div>

  <div class="card">
    <h1 class="title">famfin</h1>
    <p class="subtitle">Finances du foyer</p>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    {#if isCreateMode}
      <label class="field-label">Nom du foyer</label>
      <input
        type="text"
        placeholder="ex: Famille Dupont"
        bind:value={name}
        disabled={loading}
      />
    {/if}

    <label class="field-label">Identifiant du foyer</label>
    <input
      type="text"
      placeholder="ex: dupont"
      bind:value={householdId}
      disabled={loading}
    />

    <label class="field-label">Mot de passe</label>
    <input
      type="password"
      placeholder="••••••••"
      bind:value={password}
      disabled={loading}
      on:keydown={(e) => e.key === 'Enter' && (isCreateMode ? handleCreateHousehold() : handleLogin())}
    />

    <button
      class="btn-primary"
      on:click={isCreateMode ? handleCreateHousehold : handleLogin}
      disabled={loading}
    >
      {loading ? '…' : isCreateMode ? 'Créer le foyer' : 'Se connecter'}
    </button>

    <button
      class="btn-secondary"
      on:click={() => { isCreateMode = !isCreateMode; error = '' }}
      disabled={loading}
    >
      {isCreateMode ? '← Retour à la connexion' : 'Créer un nouveau foyer'}
    </button>
  </div>
</div>

<style>
  .login-wrap {
    position: relative;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    background: var(--em-bg);
  }

  .canvas {
    position: absolute;
    inset: 0;
    z-index: 0;
  }

  .grain {
    position: absolute;
    inset: 0;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='300' height='300'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.75' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='300' height='300' filter='url(%23n)' opacity='0.07'/%3E%3C/svg%3E");
    mix-blend-mode: overlay;
    pointer-events: none;
  }

  .card {
    position: relative;
    z-index: 10;
    background: rgba(9,9,15,0.75);
    backdrop-filter: blur(24px);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 2.5rem;
    width: 100%;
    max-width: 380px;
    box-shadow: 0 32px 80px rgba(0,0,0,0.5);
  }

  .title {
    font-family: var(--serif);
    font-style: italic;
    font-size: 2.25rem;
    color: var(--text);
    text-align: center;
    margin-bottom: 4px;
    font-weight: 400;
  }

  .subtitle {
    text-align: center;
    color: var(--text-dim);
    font-size: 0.8rem;
    letter-spacing: 0.5px;
    margin-bottom: 2rem;
  }

  .field-label {
    display: block;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: rgba(242,242,247,0.35);
    margin-bottom: 6px;
  }

  input {
    width: 100%;
    padding: 0.65rem 0.85rem;
    margin-bottom: 1.25rem;
    background: rgba(255,255,255,0.08);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.9rem;
    color: var(--text);
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  input::placeholder {
    color: rgba(242,242,247,0.2);
  }

  input:focus {
    outline: none;
    border-color: rgba(0,217,126,0.4);
    box-shadow: 0 0 0 3px rgba(0,217,126,0.1);
  }

  input:disabled {
    opacity: 0.5;
  }

  .btn-primary {
    width: 100%;
    padding: 0.7rem;
    background: var(--em);
    color: #002010;
    border: none;
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    margin-bottom: 0.5rem;
    transition: background 0.2s;
    letter-spacing: 0.2px;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--em-dk);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    width: 100%;
    padding: 0.7rem;
    background: transparent;
    color: var(--text-dim);
    border: 1px solid var(--border);
    border-radius: 8px;
    font-family: var(--font);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover:not(:disabled) {
    background: rgba(255,255,255,0.08);
    color: var(--text);
  }

  .btn-secondary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .error {
    background: rgba(255,51,88,0.12);
    border: 1px solid rgba(255,51,88,0.3);
    color: #FF3358;
    padding: 0.65rem 0.85rem;
    border-radius: 8px;
    margin-bottom: 1.25rem;
    font-size: 0.85rem;
  }
</style>
