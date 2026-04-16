<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState } from '$lib/types'

  export let auth: AuthState
  export let currentView: string

  const dispatch = createEventDispatcher()

  let loading = false
  let showUserMenu = false

  async function handleLogout() {
    loading = true
    showUserMenu = false
    try {
      await apiCall('POST', `/households/${auth.household_id}/logout`, auth)
      dispatch('logout')
    } catch (e) {
      console.error('Logout failed:', e)
      dispatch('logout')
    } finally {
      loading = false
    }
  }

  function handleNavigate(view: string) {
    dispatch('navigate', view)
  }

  function toggleUserMenu(e: MouseEvent) {
    e.stopPropagation()
    showUserMenu = !showUserMenu
  }

  function handleWindowClick() {
    showUserMenu = false
  }

  const navItems = [
    { id: 'dashboard',    label: 'Accueil' },
    { id: 'transactions', label: 'Transactions' },
    { id: 'goals',        label: 'Projets' },
    { id: 'categories',   label: 'Catégories' },
    { id: 'import',       label: 'Importer' },
  ]
</script>

<svelte:window on:click={handleWindowClick} />

<nav class="nav">
  <span class="nav-logo">famfin</span>
  {#each navItems as item}
    <button
      class="nav-btn"
      class:active={currentView === item.id}
      on:click={() => handleNavigate(item.id)}
    >
      {item.label}
    </button>
  {/each}
  <div class="nav-spacer"></div>
  <div class="user-menu-wrap">
    <button class="user-avatar" on:click={toggleUserMenu} aria-label="Menu utilisateur" aria-expanded={showUserMenu}>
      <svg width="15" height="15" viewBox="0 0 15 15" fill="none">
        <circle cx="7.5" cy="4.5" r="2.5" stroke="currentColor" stroke-width="1.4"/>
        <path d="M2 13.5c0-3.038 2.462-5.5 5.5-5.5s5.5 2.462 5.5 5.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
      </svg>
    </button>
    {#if showUserMenu}
      <div class="user-dropdown">
        <button class="dropdown-logout" on:click={handleLogout} disabled={loading}>
          {loading ? '…' : 'Se déconnecter'}
        </button>
      </div>
    {/if}
  </div>
</nav>

<style>
  .nav {
    position: fixed;
    top: 0; left: 0; right: 0;
    z-index: 200;
    background: rgba(9,9,15,0.97);
    backdrop-filter: blur(16px);
    border-bottom: 1px solid var(--border);
    padding: 0 20px;
    display: flex;
    align-items: center;
    gap: 4px;
    height: 50px;
  }

  .nav-logo {
    font-family: var(--serif);
    font-style: italic;
    font-size: 17px;
    color: var(--text);
    margin-right: 14px;
    flex-shrink: 0;
  }

  .nav-btn {
    padding: 5px 13px;
    border-radius: 20px;
    border: none;
    cursor: pointer;
    font-family: var(--font);
    font-size: 12px;
    font-weight: 500;
    background: transparent;
    color: rgba(242,242,247,0.35);
    transition: all 0.2s;
  }

  .nav-btn:hover {
    background: rgba(255,255,255,0.07);
    color: var(--text);
  }

  .nav-btn.active {
    background: rgba(255,255,255,0.12);
    color: var(--text);
  }

  .nav-spacer {
    flex: 1;
  }

  .user-menu-wrap {
    position: relative;
  }

  .user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: 1px solid var(--border-hi);
    background: rgba(255,255,255,0.05);
    color: rgba(242,242,247,0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    padding: 0;
  }

  .user-avatar:hover,
  .user-avatar[aria-expanded="true"] {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.2);
    color: var(--text);
  }

  .user-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    background: var(--card);
    border: 1px solid var(--border-hi);
    border-radius: 8px;
    padding: 4px;
    min-width: 148px;
    box-shadow: 0 8px 24px rgba(0,0,0,0.4);
    z-index: 300;
  }

  .dropdown-logout {
    width: 100%;
    padding: 7px 12px;
    border: none;
    border-radius: 5px;
    background: transparent;
    color: var(--cri);
    font-family: var(--font);
    font-size: 12px;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: background 0.15s;
  }

  .dropdown-logout:hover:not(:disabled) {
    background: rgba(255,51,88,0.1);
  }

  .dropdown-logout:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
