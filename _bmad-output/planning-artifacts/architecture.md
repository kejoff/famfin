---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/product-brief-famfin-2026-03-24.md
  - _bmad-output/planning-artifacts/product-brief-famfin-distillate.md
  - _bmad-output/planning-artifacts/research/domain-finances-familiales-partage-budget-research-2026-03-22.md
workflowType: 'architecture'
project_name: 'famfin'
user_name: 'kj'
date: '2026-04-10'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Starter Template Evaluation

### Primary Technology Domain

Full-stack local web application + ML pipeline — Rust backend serving API and static assets, SvelteKit SPA frontend, SQLite/SQLCipher database layer. Cross-compiled for ARM64 deployment on Raspberry Pi 3B.

### Hardware Target (Critical Constraint)

- **Device:** Raspberry Pi 3B — ARM Cortex-A53 quad-core @ 1.2GHz, **1 GB RAM shared** with co-hosted services (Vaultwarden, Oxicloud, AdGuard, Radicale)
- **OS:** DietPi ARMv8 64-bit (Debian Trixie) — image `DietPi_RPi234-ARMv8-Trixie.img.xz`
- **Rust target:** `aarch64-unknown-linux-gnu`
- **Development machine:** Lenovo T460 (x86_64) — training and cross-compilation host

### Starter Options Considered

- **Loco.rs** (Rails-style Rust): trop opinionated, ORM SeaORM superflu pour SQLite local, overhead non justifié pour 2 utilisateurs
- **Actix-web**: 5-10% plus rapide en throughput brut, DX inférieure, aucun avantage réel sous charge domestique
- **Python/FastAPI**: 150-250 MB RAM pour le backend + modèle ML — incompatible avec le budget mémoire Pi 3B partagé
- **SolidJS**: runtime 7 KB vs Svelte 1.6 KB, écosystème moins mature, support PWA moins intégré
- **HTMX**: trop limité pour le dashboard interactif (graphiques, projections)

### Selected Stack: Axum 0.8 + SvelteKit (Svelte 5)

**Rationale:** Rust est nécessaire (pas seulement préféré) sur Pi 3B : empreinte mémoire ~20-40 MB contre 150-250 MB pour Python. Axum offre le meilleur ratio performance/maintenabilité. SvelteKit produit les bundles les plus légers (1.6 KB runtime) avec le meilleur support PWA natif.

**Initialization Commands:**

```bash
# Backend Rust
cargo new famfin-backend --bin
# Dependencies: axum 0.8, tokio (full), rusqlite (bundled-sqlcipher),
#   tower, tower-http, serde, serde_json, ort (ONNX Runtime)

# Frontend SvelteKit
npx sv create famfin-frontend
# Options: SvelteKit, TypeScript, Vite, vite-plugin-pwa
```

**Cross-Compilation Pipeline:**

```bash
# Install cross-compilation toolchain (on T460)
cargo install cross

# Build ARM64 binary
cross build --release --target aarch64-unknown-linux-gnu

# Deploy to Pi
scp target/aarch64-unknown-linux-gnu/release/famfin dietpi@pi:~/famfin/
scp models/model.onnx dietpi@pi:~/famfin/models/
ssh dietpi@pi "systemctl restart famfin"
```

**ML Pipeline (Train on T460, Infer on Pi):**

```bash
# Training workflow (T460, Python/scikit-learn)
python scripts/train.py --export models/model.onnx

# Inference (Pi 3B, Rust via ort crate)
# Model loaded at Axum startup, inference in-process
```

**justfile targets (project root):**

```makefile
build-arm:
    cross build --release --target aarch64-unknown-linux-gnu

train:
    python scripts/train.py --export models/model.onnx

deploy:
    scp target/aarch64-unknown-linux-gnu/release/famfin dietpi@pi:~/famfin/
    scp models/model.onnx dietpi@pi:~/famfin/models/
    ssh dietpi@pi "systemctl restart famfin"
```

**Architectural Decisions Provided by Stack:**

| Concern | Decision |
|---|---|
| Language (backend) | Rust — type safety, zero-cost abstractions, minimal runtime |
| Language (frontend) | TypeScript + Svelte 5 |
| Runtime (backend) | Tokio async runtime |
| Database | SQLite via rusqlite with bundled-sqlcipher (AES-256) — satisfies NFR-S1 |
| ML inference | ONNX Runtime via `ort` crate — model trained on T460, served on Pi |
| Frontend build | Vite 6 (bundled with SvelteKit) |
| PWA | vite-plugin-pwa with stale-while-revalidate |
| API protocol | REST/JSON over HTTP (Axum → SvelteKit) |
| Testing (backend) | cargo test + axum::test for integration |
| Testing (frontend) | Vitest (unit) + Playwright (E2E) |
| Static file serving | Axum serves compiled SvelteKit `dist/` in production |
| Cross-compilation | `cross` (Docker-based) from T460 x86_64 → aarch64 |

**NFR-P2 Revalidation Required:**
Original budget (import 12 months < 30s) was calibrated for Pi 4 (Cortex-A72, 4 GB).
Must revalidate on Pi 3B (Cortex-A53, 1 GB shared) — Rust native parsing expected to remain within budget but requires benchmarking during implementation.

**Note:** Project initialization and cross-compilation setup should be the first implementation story (Epic 0 / Story 0.1 — Project Scaffold & Build Pipeline).

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- Migrations DB : refinery (SQL versionné, synchrone)
- Session auth : cookie httpOnly signé HMAC, stocké SQLite
- LLM abstraction : trait `LlmProvider` + type `SanitizedPayload`
- Cross-compilation : build manuel via `just build-arm`

**Important Decisions (Shape Architecture):**
- LLM API keys : stockées dans SQLCipher (pas en fichier config)
- Cache frontend : localStorage pour métriques dérivées non-sensibles uniquement
- Logging : crate `tracing` → journald via systemd
- State management : SvelteKit `load` + Svelte stores

**Deferred Decisions (Post-MVP):**
- CI/CD automatisé : non requis en V1, build manuel suffisant
- Backup automatisé : export manuel via app (NFR-R5)

### Data Architecture

**Migrations de schéma :**
- Outil : `refinery` — fichiers `.sql` versionnés, compatible rusqlite synchrone
- Rationale : SQLx non requis (pas d'async indispensable au projet) ; refinery
  est léger et sans dépendance supplémentaire
- Pattern : migrations embarquées dans le binaire, appliquées au démarrage
- **Timeout obligatoire** sur les migrations au démarrage — prévient la boucle
  de restart systemd si une migration longue dépasse le health check timeout

**Async & Thread Pool :**
- rusqlite synchrone dans `spawn_blocking` Tokio — pattern standard Axum
- `worker_threads = 2` configuré dans le code Rust (`#[tokio::main(worker_threads = 2)]`)
  pour limiter la contention mémoire sur Pi 3B (1 Go RAM partagé)
- Configuration versionnée dans le code, pas dans le justfile

**Cache métriques primaires :**
- localStorage uniquement pour valeurs dérivées non-sensibles :
  absorption capacity (€), trajectory signal (green/orange/red),
  timestamp dernière mise à jour
- Jamais dans localStorage : transactions, montants, noms marchands,
  soldes, tokens de session, clés API
- Pattern : store Svelte `derived` lit localStorage au démarrage,
  écrit après chaque fetch API réussi

### Authentication & Security

**Session management :**
- Cookie HMAC signé, état serveur dans SQLite (table `sessions`)
- Flags obligatoires : `HttpOnly; Secure; SameSite=Strict; Max-Age=28800`
- `HttpOnly` : inaccessible à JavaScript — protection XSS fondamentale
- Réinitialisation mot de passe : accès direct serveur uniquement
- Erreurs auth : message générique sans distinction — prévient l'énumération

**Stockage clés API LLM :**
- Stockées dans la DB SQLCipher (AES-256) — périmètre de sécurité unique
- Rationale : fichier config = plaintext sur disque, capturé par sauvegardes ;
  SQLCipher = chiffré au même niveau que les données financières
- Accès uniquement via interface de configuration de l'app

**Sanitization LLM (NFR-S4) :**
- Implémentée dans le trait `LlmProvider` via type `SanitizedPayload`
- Le compilateur Rust interdit l'envoi de `RawTransactionLabel` directement
- Suppression : IBAN, numéros de carte, montants, identifiants de compte
- Garantie par construction, pas par discipline

### API & Communication Patterns

**Abstraction LLM :**
```rust
trait LlmProvider {
    async fn categorize(&self, payload: SanitizedPayload) -> Result<Category>;
}

struct OllamaProvider { /* ... */ }
struct ClaudeProvider  { /* ... */ }
```
- Hot-swap provider sans redémarrage via rechargement config
- Fallback automatique : LLM indisponible → modèle ML ONNX (NFR-R2)
- `MockLlmProvider` pour les tests d'intégration (retourne catégories déterministes)

**Gestion d'erreurs :**
- Client : `{ "error": { "code": "IMPORT_FAILED", "message": "..." } }`
  — jamais de stack trace, jamais de détail technique
- Serveur : logs structurés complets via `tracing` (contexte, fichier, ligne)
- Codes HTTP sémantiques : 400 (client), 401 (auth), 500 (serveur)

### Frontend Architecture

**State management :**
- SvelteKit `load` functions : fetch initial des données depuis l'API Axum
- Svelte stores (`writable`, `derived`) : état partagé entre composants
- Pas de bibliothèque state externe — stores natifs Svelte suffisants

**Sécurité frontend :**
- Session : cookie `httpOnly` — jamais accessible à JavaScript
- localStorage : métriques dérivées non-sensibles uniquement
- Séparation stricte : optimisation display ≠ stockage de données financières

### Infrastructure & Deployment

**Service systemd (DietPi) :**
- Unit file `famfin.service` — user dédié non-root, restart on failure
- Démarrage automatique au boot

**Build & Deploy (manuel) :**
```bash
just build-arm   # cross build --release --target aarch64-unknown-linux-gnu
just deploy      # scp binaire + model.onnx → Pi, systemctl restart famfin
just train       # python scripts/train.py --export models/model.onnx
```

**Logging :**
- Crate `tracing` → stdout → capturé par journald
- Consultation : `journalctl -u famfin -f`
- Niveau par défaut : INFO en production, DEBUG via variable d'env

**Backup :**
- Export manuel CSV/JSON via interface app (NFR-R5)
- Pas de backup automatisé en V1

### Testing Strategy

**Tests d'intégration :**
- Environnement : T460 (x86_64), SQLite in-memory (pas SQLCipher en test)
- Harness : `axum::test` + `MockLlmProvider`
- SQLCipher activé uniquement en production — simplifie le build de test

**Correction de performance (NFR-P2, NFR-R5) :**
- Facteur estimé T460 → Pi 3B : ×4 à ×6 pour opérations CPU-bound
- Assertions test T460 : import 1500 tx < 6s, export 7500 tx < 2s
- **Benchmark de référence Pi 3B** à tourner une fois au premier déploiement
  pour calibrer le multiplicateur réel et documenter les valeurs mesurées

### Decision Impact Analysis

**Séquence d'implémentation :**
1. Scaffold projet + justfile + cross-compilation (bloque tout)
2. DB schema + migrations refinery (bloque import et catégorisation)
3. Auth session (bloque accès à toutes les routes)
4. Import CSV + déduplication (valeur primaire V1)
5. Pipeline ML ONNX + trait LlmProvider (catégorisation)
6. API REST Axum (routes métier)
7. Frontend SvelteKit + stores + localStorage cache
8. CalDAV + PWA

**Dépendances cross-composants :**
- `SanitizedPayload` doit exister avant toute intégration LLM
- Migrations refinery doivent précéder toute écriture DB
- Cookie httpOnly configuré dans Axum avant tout déploiement
- ONNX model compilé sur T460 avant premier déploiement Pi
- Benchmark Pi 3B à exécuter lors du premier déploiement

## Project Context Analysis

### Requirements Overview

**Functional Requirements (V1 — 32 FRs):**
Organized into 8 capability groups:
- Data Import & Ingestion (FR1–FR5): CSV/OFX multi-bank import with per-bank guides, revenue auto-detection, 36-month historical support, portable export
- Transaction Categorization (FR6–FR11): ML model + dual LLM provider (Ollama/Claude API), learning engine with batch merchant correction, one-time expense tagging
- Financial Analysis & Projection (FR12–FR17): 3–6 month projection, absorption capacity metric, anomaly detection, subscription detection, month-over-month delta
- Dashboard & Visualization (FR18–FR22): Dual-mode interface (desktop ritual + mobile consultation), trajectory signal, contextual reassurance messaging
- Onboarding & Setup (FR23–FR25): Intention-first flow, per-device consent screen, guided per-bank import
- Security & Access Control (FR26–FR28): Session password (default on), SQLCipher encrypted DB, graceful error messaging
- Configuration & Integration (FR29–FR32): LLM provider config, CalDAV calendar event, PWA installable, OSS link in-app

**Non-Functional Requirements:**
- Performance: Mobile TTI ≤ 3s; import 12 months < 30s on Raspberry Pi 4 (ARM Cortex-A72, 4GB RAM); read operations < 500ms p95; projection recalculation < 3s; progressive loading indicator within 200ms
- Security: AES-256 SQLCipher encryption at rest; TLS 1.2+ in transit; session auth default 8h; LLM payload sanitization (no IBAN/amounts/account identifiers); per-device consent gate
- Reliability: Core functions operate without external services (LLM, CalDAV); automatic LLM→ML fallback with user notification; idempotent imports via composite fingerprint; full data export < 10s for 5yr history; cold start < 60s on ARM quad-core
- Integration: CalDAV RFC 4791 (Radicale); dual LLM config (Ollama + Claude API); hot-swap provider without restart

**Scale & Complexity:**
- Primary domain: Full-stack local web app + ML pipeline
- Complexity level: Medium
- Estimated architectural components: ~8 major modules
- Concurrency: 2 users maximum, local network only

### Technical Constraints & Dependencies

- **Hardware target:** Raspberry Pi 4 (ARM Cortex-A72 quad-core, 4GB RAM) — all performance budgets calibrated to this floor
- **Local-first:** No cloud services, no external accounts, no CDN
- **Tech stack:** Unspecified — free choice (constraint: must run efficiently on ARM)
- **VPN boundary:** Security perimeter is the VPN; app assumes trusted network post-VPN
- **ML bootstrapping:** Model trains on household's own multi-year history at setup — no pre-trained generic model for French bank labels; LLM fills gaps until ML confidence stabilizes
- **No mobile app:** Browser-based only; PWA for home screen installability
- **Solo developer:** Strict sequential scoping required; no parallel workstreams

### Cross-Cutting Concerns Identified

1. **Encryption layer** — SQLCipher wraps all data persistence; must not create performance bottleneck on Pi hardware
2. **Graceful degradation** — every external integration (Ollama, Claude API, CalDAV/Radicale) has a defined fallback; architecture must enforce isolation at service boundaries
3. **Idempotency** — import pipeline must be re-runnable safely; deduplication via composite fingerprint (date + amount + normalized label + ordinal index) across all ingestion paths
4. **LLM payload sanitization** — PII scrubbing must occur at the service boundary before any data leaves the server; applies to both Ollama (local) and Claude API
5. **Dual-interface state sync** — desktop and mobile share same data layer; mobile read-only view must reflect current server state without manual refresh
6. **Progressive loading** — initial paint within 200ms; stale-while-revalidate pattern for mobile consultation speed
7. **ML training pipeline isolation** — training/inference must not block UI operations on the same hardware thread pool; requires async job management with train/evaluate/version/swap lifecycle
8. **Optimistic display layer** — last known values of primary metrics (absorption capacity, trajectory signal) must be available immediately on load, even if stale; applies especially to mobile consultation mode; implemented via lightweight localStorage/IndexedDB cache (not full offline caching)
