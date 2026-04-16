# famfin development tasks

set shell := ["bash", "-c"]
set dotenv-load := false

# Default target
default:
    @just --list

# Development — run both backend and frontend
dev:
    @echo "Starting famfin development servers..."
    @trap "kill %1" EXIT
    cd famfin-backend && cargo run &
    cd famfin-frontend && npm run dev
    wait

# Backend — check compilation
check-backend:
    cd famfin-backend && cargo check

# Backend — build x86_64 debug
build-backend:
    cd famfin-backend && cargo build

# Backend — build x86_64 release
build-backend-release:
    cd famfin-backend && cargo build --release

# Backend — build ARM64 (cross-compilation for Pi 3B)
build-arm:
    @echo "Building for ARM64 (aarch64-unknown-linux-gnu)..."
    @if ! command -v cross &> /dev/null; then \
        echo "Installing cross..."; \
        cargo install cross; \
    fi
    cd famfin-backend && cross build --release --target aarch64-unknown-linux-gnu
    @echo "Binary: famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend"

# Frontend — build static site for production (alias for build-frontend spec)
build-frontend:
    cd famfin-frontend && npm run build
    @echo "Frontend built to famfin-frontend/dist/"

# Frontend — install dependencies
fe-install:
    cd famfin-frontend && npm install

# Frontend — dev server
fe-dev:
    cd famfin-frontend && npm run dev

# Frontend — build static site
fe-build:
    cd famfin-frontend && npm run build

# Frontend — type check
fe-check:
    cd famfin-frontend && npm run check

# ML — train model (requires Python + scikit-learn)
train:
    @echo "Training ML model..."
    @if [ ! -f scripts/train.py ]; then \
        echo "Error: scripts/train.py not found"; \
        exit 1; \
    fi
    python scripts/train.py --export models/model.onnx
    @echo "Model saved to models/model.onnx"

# Deploy — copy binary, model, and frontend dist to Pi 3B
deploy HOST="pi.local" USER="dietpi":
    @echo "Deploying to {{HOST}} as {{USER}}..."
    @if [ ! -f famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend ]; then \
        echo "Error: ARM64 binary not found. Run 'just build-arm' first."; \
        exit 1; \
    fi
    @if [ ! -f models/model.onnx ]; then \
        echo "Warning: models/model.onnx not found. Model will not be deployed."; \
    fi
    scp famfin-backend/target/aarch64-unknown-linux-gnu/release/famfin-backend {{USER}}@{{HOST}}:~/famfin/
    @if [ -f models/model.onnx ]; then \
        scp models/model.onnx {{USER}}@{{HOST}}:~/famfin/models/; \
    fi
    @if [ -d famfin-frontend/dist ]; then \
        rsync -av --delete famfin-frontend/dist/ {{USER}}@{{HOST}}:~/famfin/dist/; \
    else \
        echo "Warning: famfin-frontend/dist not found. Run 'just build-frontend' first to include frontend."; \
    fi
    ssh {{USER}}@{{HOST}} "systemctl restart famfin"
    @echo "Waiting for service to become healthy..."
    @for i in {1..60}; do \
        status=$(ssh {{USER}}@{{HOST}} "curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3000/health 2>/dev/null" 2>/dev/null); \
        response=$(ssh {{USER}}@{{HOST}} "curl -s http://127.0.0.1:3000/health 2>/dev/null" 2>/dev/null); \
        if [ "$$status" = "200" ] && echo "$$response" | grep -q '\"status\".*\"healthy\"'; then \
            echo "✓ Health check passed (200 OK)"; \
            echo "Deployment complete."; \
            exit 0; \
        elif [ "$$status" = "503" ]; then \
            echo "Service initializing... (503)"; \
        fi; \
        sleep 1; \
    done
    @echo "⚠ Warning: Service did not respond to health check within 60s"
    @exit 1

# Assets — optimize SVG and images
optimize-assets:
    @echo "Optimizing SVG and images..."
    @mkdir -p famfin-frontend/static
    @if command -v svgo &> /dev/null; then \
        svgo --folder famfin-frontend/src/lib/assets/svg/ 2>/dev/null || true; \
    fi
    @echo "Assets optimized."

# Clean — remove build artifacts
clean:
    cd famfin-backend && cargo clean
    rm -rf famfin-frontend/build famfin-frontend/dist
    rm -rf models/

# Help — show all targets
help:
    @just --list --unsorted
