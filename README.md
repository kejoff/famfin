# famfin

Local-first household finance management for couples.

**Stack:** Rust (Axum) + SvelteKit (Svelte 5) + SQLite/SQLCipher + ONNX ML

**Target:** Raspberry Pi 3B (ARM64, 1 GB RAM shared) — cross-compiled from T460 (x86_64)

## Project Structure

```
famfin/
  famfin-backend/     Rust backend (Axum, rusqlite, ONNX Runtime)
  famfin-frontend/    SvelteKit frontend (Svelte 5, PWA, Tailwind)
  docs/               Project documentation
  models/             ML models (ONNX format)
  scripts/            Training scripts
  justfile            Development tasks
```

## Setup

### Prerequisites

- **Rust** (1.70+) — `rustup`
- **Node.js** (18+) — npm
- **Cross** (for ARM64 builds) — installed via justfile
- **Python 3** (for ML training) — scikit-learn, numpy

### Installation

```bash
# Backend dependencies
cargo check --manifest-path famfin-backend/Cargo.toml

# Frontend dependencies
cd famfin-frontend && npm install
```

## Development

### Quick Start

```bash
just dev
```

Starts both backend (port 3000) and frontend (port 5173).

### Individual Servers

**Backend only:**
```bash
cd famfin-backend && cargo run
```

**Frontend only:**
```bash
cd famfin-frontend && npm run dev
```

## Building

### For x86_64 (development machine)

```bash
just build-backend-release
just fe-build
```

### For ARM64 (Raspberry Pi)

```bash
just build-arm
```

Produces `famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend`

## Deployment

### Prerequisites on Pi 3B

1. **DietPi** ARMv8 64-bit (Debian Trixie)
2. **SQLCipher** library (usually included)
3. **systemd** service file at `/etc/systemd/system/famfin.service`

### Deploy

```bash
just deploy pi.local dietpi
```

Copies binary and ONNX model, restarts the systemd service.

## ML Training

```bash
just train
```

Trains the categorization model on household transaction history, exports to `models/model.onnx`.

## Architecture

### Backend (Rust/Axum)

- REST API on `127.0.0.1:3000`
- SQLite/SQLCipher database (encrypted at rest)
- ONNX Runtime for ML inference (category prediction)
- LLM provider abstraction (Ollama or Claude API fallback)
- Migrations via Refinery

### Frontend (SvelteKit)

- PWA installable on home screen
- Responsive: mobile (< 768px) + desktop (≥ 1024px)
- Stale-while-revalidate caching strategy
- Tailwind CSS + Svelte 5 runes
- Type-safe with TypeScript

## Configuration

### Backend

- Environment: `famfin-backend/.env`
- Database path, LLM endpoints, ONNX model path

### Frontend

- API base: `http://localhost:3000` (development)
- PWA manifest: `vite.config.ts`

## Documentation

- `docs/HELO/` — Visual identity reference (HELO BD universe)
- `_bmad-output/planning-artifacts/ux-design-specification.md` — Complete UX spec
- `_bmad-output/planning-artifacts/architecture.md` — Technical architecture

## Testing

**Backend:**
```bash
cd famfin-backend && cargo test
```

**Frontend:**
```bash
cd famfin-frontend && npm run check
```

## License

Private project for household use.
