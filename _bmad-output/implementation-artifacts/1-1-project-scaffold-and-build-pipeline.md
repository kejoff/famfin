# Story 1.1: Project Scaffold & Build Pipeline

**Status:** review  
**Epic:** 1 — Secure Local Service & Foundation  
**Story ID:** 1.1  
**Created:** 2026-04-12  
**Implemented:** 2026-04-12  

---

## User Story

As a developer,  
I want a working Rust/SvelteKit project scaffold with a cross-compilation pipeline to aarch64,  
So that I can build and deploy a famfin binary to the Raspberry Pi 3B from the T460 development machine.

---

## Current State — What Already Exists

**The scaffold is ~85% complete. DO NOT re-implement what exists.**

| Component | Status | Notes |
|-----------|--------|-------|
| `famfin-backend/` Rust crate | ✅ EXISTS | Axum 0.8, all deps in Cargo.toml |
| `#[tokio::main(worker_threads = 2)]` | ✅ EXISTS | In `famfin-backend/src/main.rs:27` |
| `GET /health` endpoint | ✅ EXISTS | Returns `"OK"` in `main.rs:82` |
| `just build-arm` target | ✅ EXISTS | Runs `cross build --release --target aarch64-unknown-linux-gnu` |
| `just deploy` target | ✅ EXISTS | scp + systemctl restart, parameterized HOST/USER |
| `just train` target | ✅ EXISTS | `python scripts/train.py --export models/model.onnx` |
| `just` default (lists targets) | ✅ EXISTS | `default: @just --list` |
| refinery migrations | ✅ EXISTS | `embed_migrations!("migrations")` in `db/connection.rs` |
| `famfin-frontend/` Svelte SPA | ✅ EXISTS | Plain Svelte 5 + Vite (see note below) |

**⚠️ EXISTING DEVIATIONS FROM ARCHITECTURE SPEC — DO NOT CHANGE:**
- The frontend is a **plain Svelte 5 SPA** (not SvelteKit). Architecture doc says SvelteKit, but significant frontend work already exists as a Svelte SPA. **Keep Svelte SPA — do not migrate to SvelteKit.** The UX overhaul and authentication UI are already built.
- Binary is named `famfin-backend` (Cargo package name), not `famfin`. The `deploy` target already accounts for this. **Keep as-is.**

---

## Acceptance Criteria

### AC1 — `just build-arm` produces ARM64 binary
```
Given: T460 with `cross` and `just` installed
When: `just build-arm` is run
Then: binary at `famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend`
And: binary serves `GET /health` → `200 OK` on Pi 3B
```
**✅ Already satisfied.** No action required.

### AC2 — `just build-frontend` produces compiled dist served by Axum
```
Given: SvelteKit frontend project initialized
When: `just build-frontend` is run
Then: compiled dist/ directory produced
And: Axum serves static files from dist/ in production mode
```
**🔴 GAP — Two changes required:**
1. `just build-frontend` target missing from justfile (only `fe-build` exists)
2. Axum does not serve static files (no `tower_http::services::ServeDir` in `main.rs`)

### AC3 — `just` lists all targets with descriptions
```
Given: justfile at project root
When: `just` is run without arguments
Then: build-arm, build-frontend, deploy, train targets listed with descriptions
```
**🟡 PARTIAL** — `default: @just --list` is present. Once `build-frontend` is added (AC2), this is satisfied.

### AC4 — `worker_threads = 2` in Rust code
```
Given: Tokio runtime configuration in main.rs
When: binary starts
Then: worker_threads = 2 via #[tokio::main(worker_threads = 2)]
```
**✅ Already satisfied** at `main.rs:27`.

---

## Implementation Tasks

### Task 1 — Add `build-frontend` justfile target

Add to `justfile` (after `build-arm`):

```justfile
# Frontend — build static site for production
build-frontend:
    cd famfin-frontend && npm run build
    @echo "Frontend built to famfin-frontend/dist/"
```

This is an alias to `fe-build`. Keep both targets — they serve different naming conventions.

**File:** `/home/kj/projects/famfin/justfile`

### Task 2 — Add static file serving to Axum

Axum must serve the SvelteKit `dist/` at runtime. Add `tower-http` `ServeDir` to `main.rs`.

**Required changes to `famfin-backend/src/main.rs`:**

```rust
// Add these imports
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

// In main(), after building the protected router, add static file fallback:
let static_dir = std::env::var("STATIC_DIR")
    .unwrap_or_else(|_| "../famfin-frontend/dist".to_string());

let app = Router::new()
    .route("/health", get(health_check))
    .route("/api/households", post(create_household))
    .route("/api/households/{household_id}/login", post(login))
    .merge(protected)
    // Serve static files — SPA fallback to index.html for client-side routing
    .nest_service(
        "/",
        ServeDir::new(&static_dir)
            .not_found_service(ServeFile::new(
                PathBuf::from(&static_dir).join("index.html")
            )),
    )
    .with_state(state)
    .layer(CorsLayer::permissive());
```

**`tower-http` is already in `Cargo.toml`** — `tower-http = { version = "0.5", features = ["trace", "cors"] }`. Add `fs` feature:

```toml
tower-http = { version = "0.5", features = ["trace", "cors", "fs"] }
```

**IMPORTANT:** API routes must be registered BEFORE the static file fallback, otherwise `ServeDir` will intercept API calls. The router ordering above ensures this.

**Environment variable:** `STATIC_DIR` defaults to `../famfin-frontend/dist` (relative to where binary runs). For production on Pi, the binary runs from `~/famfin/` and static files should be in `~/famfin/dist/`. Update `just deploy` to copy the dist folder:

```justfile
deploy HOST="pi.local" USER="dietpi":
    @echo "Deploying to {{HOST}} as {{USER}}..."
    # ... existing checks ...
    scp famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend {{USER}}@{{HOST}}:~/famfin/
    @if [ -f models/model.onnx ]; then \
        scp models/model.onnx {{USER}}@{{HOST}}:~/famfin/models/; \
    fi
    # Deploy frontend dist if built
    @if [ -d famfin-frontend/dist ]; then \
        rsync -av --delete famfin-frontend/dist/ {{USER}}@{{HOST}}:~/famfin/dist/; \
    fi
    ssh {{USER}}@{{HOST}} "systemctl restart famfin"
    @echo "Deployment complete."
```

And the systemd unit or Pi environment should set: `Environment=STATIC_DIR=/home/dietpi/famfin/dist`

---

## Architecture Constraints (DO NOT VIOLATE)

| Constraint | Requirement | Where |
|-----------|-------------|-------|
| Tokio threads | `worker_threads = 2` in Rust code only — NOT env var or justfile | `main.rs:27` ✅ |
| Static file serving | `ServeDir` from `tower-http` | Add to `main.rs` |
| Cross-compilation | `cross` Docker-based, NOT cargo directly | `justfile` ✅ |
| Frontend build output | `famfin-frontend/dist/` | Vite default ✅ |
| CORS in dev | `CorsLayer::permissive()` acceptable in dev; tighten in V1.x | `main.rs` ✅ |
| Health check | `GET /health` → 200 OK, no auth required, public route | `main.rs` ✅ |
| Logging | `tracing_subscriber::fmt::init()` — stdout → journald | `main.rs` ✅ |

---

## Key File Locations

```
famfin/
├── justfile                          ← ADD build-frontend target here
├── famfin-backend/
│   ├── Cargo.toml                    ← ADD "fs" to tower-http features
│   └── src/
│       ├── main.rs                   ← ADD ServeDir static file serving
│       └── db/
│           └── connection.rs         ← init_db, init_test_db (Story 1.2 owns SQLCipher key)
├── famfin-frontend/
│   ├── package.json                  ← npm run build → dist/
│   ├── vite.config.ts               ← builds to dist/ (Vite default)
│   └── dist/                         ← generated by `just build-frontend`
└── models/
    └── model.onnx                    ← deployed by `just deploy`
```

---

## Out of Scope for This Story

- **SQLCipher encryption key setup** → Story 1.2 (Encrypted Database)
- **Migration timeout** → Story 1.2
- **Session auth middleware** → Story 1.3
- **systemd service file** → Story 1.4
- **ONNX model loading** → Story 2.1
- **install.sh script** → Story 1.4

---

## Testing Verification

After implementation, verify manually:

```bash
# 1. Build frontend
just build-frontend
ls famfin-frontend/dist/index.html  # must exist

# 2. Start backend (dev mode, serves dist locally)
cd famfin-backend && cargo run

# 3. Verify health check
curl http://localhost:3000/health  # must return "OK"

# 4. Verify static file serving
curl http://localhost:3000/  # must return HTML from dist/index.html

# 5. Verify just lists targets
just  # must list build-arm, build-frontend, deploy, train with descriptions

# 6. Verify ARM build (requires cross installed)
just build-arm
file famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend
# must show: ELF 64-bit LSB executable, ARM aarch64
```

No automated tests added for this story — the scaffold setup is verified by the build/run checks above.

---

## Implementation Record

### Tasks Completed

- [x] Task 1 — Add `build-frontend` justfile target
  - Added `build-frontend` target to justfile after `build-arm`
  - Runs `cd famfin-frontend && npm run build`
  - Kept existing `fe-build` target for backward compatibility

- [x] Task 2 — Add static file serving to Axum
  - Updated `Cargo.toml`: added `"fs"` feature to `tower-http`
  - Updated `main.rs`:
    - Added imports: `PathBuf`, `ServeDir`, `ServeFile`
    - Added static files directory env var (`STATIC_DIR`, default `../famfin-frontend/dist`)
    - Nested `ServeDir` at root path `/` with SPA fallback to `index.html`
    - API routes properly ordered BEFORE static file handler to prevent interception
  - Updated `deploy` justfile target:
    - Added `rsync` deployment of frontend `dist/` folder
    - Added warning if `dist/` not found (user should run `build-frontend` first)

### Verification

- ✅ Frontend builds successfully: `npm run build` produces `famfin-frontend/dist/`
- ✅ Backend compiles without errors: `cargo check` passes
- ✅ Health endpoint responds: `GET /health` returns `200 OK`
- ✅ Static file serving code integrated (ServeDir configured)

### Files Modified

1. `/home/kj/projects/famfin/justfile`
   - Added `build-frontend` target
   - Updated `deploy` target with dist folder copying

2. `/home/kj/projects/famfin/famfin-backend/Cargo.toml`
   - Updated `tower-http` features: added `"fs"`

3. `/home/kj/projects/famfin/famfin-backend/src/main.rs`
   - Added imports for `PathBuf`, `ServeDir`, `ServeFile`
   - Configured static directory from env var
   - Nested static file service at root with SPA fallback

---

## Dev Notes

- The `famfin-frontend/` is a **Svelte 5 SPA** (not SvelteKit). SvelteKit was specified in architecture but not what was built. All existing frontend work (auth UI, dashboard, UX overhaul) is on this Svelte SPA. Do not change the frontend framework.
- `init_test_db()` in `db/connection.rs` uses in-memory SQLite without SQLCipher — correct and intended for tests.
- The `famfin.db` file in `famfin-backend/` is the dev database. It is gitignored.
- Svelte 5 runes are disabled (`runes: false` in `svelte.config.js`), using Svelte 4 reactivity syntax.
