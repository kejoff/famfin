<script lang="ts">
  import { onMount } from 'svelte'
  import type { AuthState } from '$lib/types'
  import { authStore } from '$lib/stores'
  import Login from '$components/Login.svelte'
  import Dashboard from '$components/Dashboard.svelte'
  import Transactions from '$components/Transactions.svelte'
  import Goals from '$components/Goals.svelte'
  import Categories from '$components/Categories.svelte'
  import Import from '$components/Import.svelte'
  import Navigation from '$components/Navigation.svelte'

  let auth: AuthState | null = null
  let currentView: string = 'login'
  let authChecked = false

  onMount(async () => {
    // Check if session is valid by calling protected /api/me endpoint
    try {
      const response = await fetch(`/api/me`, {
        credentials: 'include'
      })
      if (response.ok) {
        const data = await response.json()
        auth = {
          session_id: 'cookie',
          expires_at: '',
          household_id: data.household_id,
        }
        authStore.set(auth)
        currentView = 'dashboard'
      }
    } catch (e) {
      // No session, stay on login
    } finally {
      authChecked = true
    }
  })

  function handleLogin(event: CustomEvent<AuthState>) {
    auth = event.detail
    authStore.set(auth)
    currentView = 'dashboard'
  }

  function handleLogout() {
    auth = null
    authStore.set(null)
    currentView = 'login'
  }

  function navigateTo(view: string) {
    currentView = view
  }
</script>

<main>
  {#if !authChecked}
    <!-- Hide UI until auth check completes to prevent login flash -->
  {:else if auth}
    <Navigation {auth} on:logout={handleLogout} {currentView} on:navigate={(e) => navigateTo(e.detail)} />
    {#if currentView === 'dashboard'}
      <Dashboard {auth} />
    {:else if currentView === 'transactions'}
      <Transactions {auth} on:navigate={(e) => navigateTo(e.detail)} />
    {:else if currentView === 'goals'}
      <Goals {auth} />
    {:else if currentView === 'categories'}
      <Categories {auth} />
    {:else if currentView === 'import'}
      <Import {auth} />
    {/if}
  {:else}
    <Login on:login={handleLogin} />
  {/if}
</main>

<style>
  main {
    min-height: 100vh;
    background: var(--surface);
  }
</style>
