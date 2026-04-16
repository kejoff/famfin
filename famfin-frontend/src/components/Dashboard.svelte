<script lang="ts">
  import { onMount } from 'svelte'
  import { apiCall } from '$lib/stores'
  import type { AuthState, MonthlySpending } from '$lib/types'

  export let auth: AuthState

  let spending: MonthlySpending | null = null
  let loading = true
  let error = ''

  // Trajectory state determines the canvas color
  // serein = on track, tension = slight overrun, alerte = critical
  let trajectoryState: 'serein' | 'tension' | 'alerte' = 'serein'

  const stateLabels = {
    serein:  'Serein',
    tension: 'Attention',
    alerte:  'Alerte',
  }

  // Format monetary amount with tabular numerics
  function fmt(amount: number): string {
    return Math.abs(amount).toLocaleString('fr-FR', { maximumFractionDigits: 0 })
  }

  function fmtDecimal(amount: number): string {
    return Math.abs(amount).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
  }

  function computeState(total: number) {
    // Heuristic until income tracking is added:
    // serein < 1800€, tension 1800–2500€, alerte > 2500€
    if (total < 1800) return 'serein'
    if (total < 2500) return 'tension'
    return 'alerte'
  }

  const monthNames = ['janvier','février','mars','avril','mai','juin',
                      'juillet','août','septembre','octobre','novembre','décembre']

  onMount(async () => {
    try {
      const today = new Date()
      const year = today.getFullYear()
      const month = today.getMonth() + 1
      spending = await apiCall(
        'GET',
        `/households/${auth.household_id}/dashboard?year=${year}&month=${month}`,
        auth
      )
      if (spending) {
        trajectoryState = computeState(Math.abs(spending.total_spending))
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Chargement échoué'
    } finally {
      loading = false
    }
  })

  $: currentMonth = (() => {
    const d = new Date()
    return `${monthNames[d.getMonth()]} ${d.getFullYear()}`
  })()
</script>

<div class="page" data-state={trajectoryState}>

  <!-- Painted canvas background -->
  <div class="canvas">
    {#if trajectoryState === 'serein'}
      <svg viewBox="0 0 375 812" xmlns="http://www.w3.org/2000/svg"
           style="position:absolute;inset:0;width:100%;height:100%;">
        <path d="M -40 80 C 40 60, 130 90, 200 160 C 250 210, 280 270, 260 320 C 240 370, 180 360, 140 320 C 100 280, 80 220, 60 180 C 40 140, -20 120, -40 80Z"
              fill="#00D97E" opacity="0.28"/>
        <path d="M 240 -20 C 300 20, 420 80, 430 180 C 440 280, 360 320, 320 270 C 280 220, 300 160, 260 110 C 220 60, 210 20, 240 -20Z"
              fill="#00FF9F" opacity="0.15"/>
        <path d="M 0 500 C 80 460, 200 450, 280 490 C 360 530, 420 600, 400 680 C 380 760, 260 790, 150 780 C 40 770, -30 700, 0 640 C 20 600, 0 540, 0 500Z"
              fill="#005C2E" opacity="0.5"/>
        <ellipse cx="290" cy="400" rx="70" ry="45" fill="#00E87A" opacity="0.12" transform="rotate(-25 290 400)"/>
        <path d="M 90 290 Q 87 340 93 380 Q 96 400 90 420"
              stroke="#00D97E" stroke-width="5" fill="none" opacity="0.2" stroke-linecap="round"/>
        <radialGradient id="vg" cx="50%" cy="100%" r="70%">
          <stop offset="0%" stop-color="#0B0B12" stop-opacity="0.65"/>
          <stop offset="100%" stop-color="#0B0B12" stop-opacity="0"/>
        </radialGradient>
        <rect width="375" height="812" fill="url(#vg)"/>
      </svg>
    {:else if trajectoryState === 'tension'}
      <svg viewBox="0 0 375 812" xmlns="http://www.w3.org/2000/svg"
           style="position:absolute;inset:0;width:100%;height:100%;">
        <path d="M -40 60 C 60 20, 180 80, 260 180 C 320 260, 340 360, 300 420 C 260 480, 180 460, 130 400 C 80 340, 70 260, 40 200 C 20 160, -40 100, -40 60Z"
              fill="#FF8500" opacity="0.25"/>
        <path d="M 260 -30 C 360 30, 460 140, 440 260 C 420 380, 340 420, 290 360 C 240 300, 280 220, 240 150 C 210 90, 200 30, 260 -30Z"
              fill="#FFA040" opacity="0.12"/>
        <path d="M 0 520 C 90 480, 220 470, 300 510 C 380 550, 440 620, 420 700 C 400 780, 270 812, 140 800 C 10 788, -30 720, 0 660 Z"
              fill="#4A2000" opacity="0.6"/>
        <radialGradient id="vgT" cx="50%" cy="100%" r="70%">
          <stop offset="0%" stop-color="#1F0800" stop-opacity="0.7"/>
          <stop offset="100%" stop-color="#1F0800" stop-opacity="0"/>
        </radialGradient>
        <rect width="375" height="812" fill="url(#vgT)"/>
      </svg>
    {:else}
      <svg viewBox="0 0 375 812" xmlns="http://www.w3.org/2000/svg"
           style="position:absolute;inset:0;width:100%;height:100%;">
        <path d="M -40 50 C 60 10, 200 70, 280 190 C 340 280, 350 390, 300 450 C 250 510, 170 490, 120 420 C 70 350, 60 270, 30 200 C 10 150, -40 90, -40 50Z"
              fill="#FF3358" opacity="0.22"/>
        <path d="M 260 -40 C 370 20, 470 150, 450 280 C 430 410, 340 450, 290 390 C 240 330, 280 240, 240 160 C 210 90, 200 20, 260 -40Z"
              fill="#FF5577" opacity="0.10"/>
        <path d="M 0 530 C 90 490, 230 480, 310 520 C 390 560, 450 630, 430 710 C 410 790, 270 812, 140 800 Z"
              fill="#3D0010" opacity="0.7"/>
        <radialGradient id="vgA" cx="50%" cy="100%" r="70%">
          <stop offset="0%" stop-color="#1F0008" stop-opacity="0.7"/>
          <stop offset="100%" stop-color="#1F0008" stop-opacity="0"/>
        </radialGradient>
        <rect width="375" height="812" fill="url(#vgA)"/>
      </svg>
    {/if}
    <div class="grain"></div>
  </div>

  <!-- Hero zone -->
  <div class="hero">
    <div class="eyebrow">Dépenses du mois</div>
    {#if loading}
      <div class="hero-num loading-num">—</div>
    {:else if spending}
      <div class="hero-num">
        <span class="hero-amount">{fmt(spending.total_spending)}</span><span class="hero-unit">€</span>
      </div>
    {:else}
      <div class="hero-num">—</div>
    {/if}
    <div class="hero-caption">{currentMonth}</div>
  </div>

  <!-- Signal pill -->
  <div class="signal">
    <div class="sig-dot"></div>
    <span class="sig-text">{stateLabels[trajectoryState]}</span>
  </div>

  <!-- Brushstroke separator -->
  <div class="brush-sep">
    <svg viewBox="0 0 375 18" xmlns="http://www.w3.org/2000/svg" style="width:100%;display:block;">
      <path d="M 0 14 C 40 6, 100 16, 175 10 C 250 4, 320 14, 375 8 L 375 18 L 0 18Z"
            fill="#F5EFE4"/>
      <path d="M -5 10 C 35 4, 90 12, 165 7 C 240 2, 315 11, 380 5"
            stroke="rgba(0,0,0,0.04)" stroke-width="2" fill="none"/>
    </svg>
  </div>

  <!-- Bottom card -->
  <div class="bot-card">
    <div class="card-grip"></div>

    {#if error}
      <div class="card-error">{error}</div>
    {:else if loading}
      <div class="card-loading">Chargement…</div>
    {:else if spending}
      <div class="month-row">
        <strong>{currentMonth}</strong>
        <span>{spending.category_breakdown.length} catégories</span>
      </div>

      {#if spending.category_breakdown.length > 0}
        <div class="categories">
          {#each spending.category_breakdown.slice(0, 4) as cat}
            <div class="cat-row">
              <div class="cat-info">
                <span class="cat-name">{cat.category_name}</span>
                <span class="cat-amount">{fmtDecimal(cat.amount)} €</span>
              </div>
              <div class="cat-bar">
                <div class="cat-fill" style="width:{Math.min(cat.percentage, 100)}%"></div>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <p class="empty-hint">Aucune transaction ce mois-ci</p>
      {/if}

      <div class="metrics">
        <div class="metric-cell">
          <div class="m-lbl">Total</div>
          <div class="m-val">{fmtDecimal(spending.total_spending)} €</div>
        </div>
        <div class="metric-cell">
          <div class="m-lbl">Catégories</div>
          <div class="m-val">{spending.category_breakdown.length}</div>
        </div>
        <div class="metric-cell">
          <div class="m-lbl">État</div>
          <div class="m-val" data-state={trajectoryState}>{stateLabels[trajectoryState]}</div>
        </div>
      </div>
    {:else}
      <p class="empty-hint">Importez des transactions pour commencer</p>
    {/if}
  </div>
</div>

<style>
  .page {
    position: relative;
    min-height: calc(100vh - 50px);
    margin-top: 50px;
    overflow: hidden;
  }

  /* Canvas background colors by state */
  .page[data-state="serein"]  { background: #001F12; }
  .page[data-state="tension"] { background: #1F0800; }
  .page[data-state="alerte"]  { background: #1F0008; }

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
    z-index: 50;
  }

  /* ── Hero ── */
  .hero {
    position: relative;
    z-index: 20;
    padding: 40px 28px 0;
  }

  .eyebrow {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 2.5px;
    text-transform: uppercase;
    color: rgba(255,255,255,0.45);
    margin-bottom: 4px;
  }

  .hero-num {
    font-family: var(--serif);
    font-style: italic;
    font-size: clamp(72px, 22vw, 100px);
    line-height: 0.9;
    letter-spacing: -3px;
    color: #fff;
    white-space: nowrap;
    display: flex;
    align-items: flex-start;
    gap: 4px;
  }

  .hero-amount {
    font-variant-numeric: tabular-nums;
  }

  .hero-unit {
    font-family: var(--serif);
    font-style: italic;
    font-size: clamp(32px, 9vw, 40px);
    color: rgba(255,255,255,0.4);
    align-self: flex-end;
    margin-bottom: 6px;
  }

  .loading-num {
    color: rgba(255,255,255,0.3);
  }

  .hero-caption {
    font-size: 11px;
    color: rgba(255,255,255,0.35);
    margin-top: 8px;
    letter-spacing: 0.2px;
  }

  /* ── Signal pill ── */
  .signal {
    position: absolute;
    top: 66px;
    right: 22px;
    z-index: 30;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 11px;
    border-radius: 20px;
    backdrop-filter: blur(10px);
  }

  .page[data-state="serein"]  .signal { background: rgba(0,217,126,0.15); border: 1px solid rgba(0,217,126,0.35); }
  .page[data-state="tension"] .signal { background: rgba(255,133,0,0.15);  border: 1px solid rgba(255,133,0,0.35); }
  .page[data-state="alerte"]  .signal { background: rgba(255,51,88,0.15);  border: 1px solid rgba(255,51,88,0.35); }

  .sig-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .page[data-state="serein"]  .sig-dot { background: var(--em);  box-shadow: 0 0 8px var(--em); }
  .page[data-state="tension"] .sig-dot { background: var(--amb); box-shadow: 0 0 8px var(--amb); }
  .page[data-state="alerte"]  .sig-dot { background: var(--cri); box-shadow: 0 0 8px var(--cri); }

  .sig-text {
    font-size: 10px;
    font-weight: 600;
    color: rgba(255,255,255,0.75);
  }

  /* ── Brushstroke separator ── */
  .brush-sep {
    position: absolute;
    bottom: 319px;
    left: 0; right: 0;
    z-index: 28;
    pointer-events: none;
  }

  /* ── Bottom card ── */
  .bot-card {
    position: absolute;
    bottom: 0; left: 0; right: 0;
    height: 324px;
    z-index: 22;
    background: var(--cream);
    padding: 16px 22px 24px;
    overflow: hidden;
  }

  .card-grip {
    width: 34px;
    height: 4px;
    border-radius: 2px;
    background: rgba(0,0,0,0.12);
    margin: 0 auto 14px;
  }

  .month-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
    font-size: 11px;
    color: #8E8E93;
  }

  .month-row strong {
    color: #1C1C1E;
    font-weight: 600;
    font-size: 12px;
  }

  /* ── Categories ── */
  .categories {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 14px;
  }

  .cat-row {}

  .cat-info {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 4px;
  }

  .cat-name {
    font-size: 11px;
    font-weight: 500;
    color: #1C1C1E;
  }

  .cat-amount {
    font-family: var(--mono);
    font-size: 11px;
    font-weight: 500;
    color: #3C3C43;
    font-variant-numeric: tabular-nums;
  }

  .cat-bar {
    height: 3px;
    background: rgba(0,0,0,0.08);
    border-radius: 2px;
    overflow: hidden;
  }

  .cat-fill {
    height: 100%;
    background: #1C1C1E;
    border-radius: 2px;
    opacity: 0.25;
  }

  /* ── Bottom metrics ── */
  .metrics {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 7px;
  }

  .metric-cell {
    background: rgba(0,0,0,0.04);
    border-radius: 10px;
    padding: 9px 10px;
  }

  .m-lbl {
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: #9E9E9E;
    margin-bottom: 3px;
  }

  .m-val {
    font-family: var(--mono);
    font-size: 13px;
    font-weight: 500;
    color: #1C1C1E;
    font-variant-numeric: tabular-nums;
  }

  .m-val[data-state="serein"]  { color: #009B54; }
  .m-val[data-state="tension"] { color: #B34B00; }
  .m-val[data-state="alerte"]  { color: #CC0027; }

  .card-error {
    background: rgba(204,0,39,0.08);
    border: 1px solid rgba(204,0,39,0.2);
    color: #CC0027;
    padding: 0.6rem 0.8rem;
    border-radius: 8px;
    font-size: 0.85rem;
  }

  .card-loading {
    color: #8E8E93;
    font-size: 0.85rem;
    text-align: center;
    padding: 1rem 0;
  }

  .empty-hint {
    color: #AEAEB2;
    font-size: 0.85rem;
    text-align: center;
    padding: 1.5rem 0;
  }
</style>
