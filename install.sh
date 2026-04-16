#!/usr/bin/env bash
# famfin installation script for Debian/Ubuntu systems
# Usage: sudo ./install.sh

set -e

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "✗ This script must be run with sudo"
    exit 1
fi

echo "🚀 famfin Installation Script"
echo "=============================="
echo "Target: Debian/Ubuntu system"
echo ""

# Detect OS
if ! [ -f /etc/os-release ]; then
    echo "✗ Cannot detect OS. This script requires /etc/os-release"
    exit 1
fi

source /etc/os-release

if [[ ! "$ID" =~ ^(debian|ubuntu)$ ]]; then
    echo "✗ Unsupported OS: $ID (this script requires Debian/Ubuntu)"
    exit 1
fi

echo "✓ Detected: $ID $VERSION_ID"
echo ""

# Install system dependencies
echo "📦 Installing system dependencies..."
apt-get update -qq
apt-get install -y -qq curl openssl sqlite3 ca-certificates > /dev/null 2>&1
echo "✓ System dependencies installed"
echo ""

# Create non-root user for famfin if not exists
if ! id -u famfin > /dev/null 2>&1; then
    echo "👤 Creating famfin user..."
    useradd -r -s /bin/false -d /opt/famfin -m famfin
    echo "✓ User 'famfin' created"
else
    echo "✓ User 'famfin' already exists"
fi
echo ""

# Create directories
echo "📁 Setting up directories..."
mkdir -p /opt/famfin/data
mkdir -p /opt/famfin/models
mkdir -p /opt/famfin/famfin-frontend/dist
mkdir -p /etc/famfin
chown -R famfin:famfin /opt/famfin
chmod 700 /opt/famfin/data
echo "✓ Directories created and permissions set"
echo ""

# Install famfin.service if binary provided
if [ -f "famfin-backend" ]; then
    echo "📦 Installing binary..."
    install -m 755 famfin-backend /opt/famfin/
    chown famfin:famfin /opt/famfin/famfin-backend
    echo "✓ Binary installed to /opt/famfin/"
else
    echo "⚠ Binary not provided in current directory"
    echo "  After obtaining the binary, copy it to /opt/famfin/famfin-backend"
fi
echo ""

# Install ONNX model if provided
if [ -f "model.onnx" ]; then
    echo "🧠 Installing ML model..."
    install -m 644 model.onnx /opt/famfin/models/
    chown famfin:famfin /opt/famfin/models/model.onnx
    echo "✓ ONNX model installed to /opt/famfin/models/"
else
    echo "ℹ ONNX model not provided (optional, required for auto-categorization)"
fi
echo ""

# Install frontend dist if provided
if [ -d "dist" ]; then
    echo "🎨 Installing frontend..."
    cp -r dist/* /opt/famfin/famfin-frontend/dist/
    chown -R famfin:famfin /opt/famfin/famfin-frontend
    echo "✓ Frontend installed to /opt/famfin/famfin-frontend/dist/"
else
    echo "ℹ Frontend dist not provided (use 'just build-frontend' on dev machine)"
fi
echo ""

# Install systemd service file
if [ -f "famfin.service" ]; then
    echo "⚙️ Installing systemd service..."
    install -m 644 famfin.service /etc/systemd/system/
    echo "✓ Service file installed to /etc/systemd/system/"
else
    echo "✗ famfin.service not found in current directory"
    exit 1
fi
echo ""

# Reload systemd and enable service
echo "⚙️ Configuring systemd..."
systemctl daemon-reload
systemctl enable famfin.service
echo "✓ Service enabled for auto-start at boot"
echo ""

# Configuration instructions
echo "📋 Initial Configuration"
echo "========================"
echo ""
echo "Before starting the service, configure environment variables:"
echo ""
echo "1. Generate random keys:"
echo ""
echo "   CIPHER_KEY=\$(openssl rand -hex 32)"
echo "   SESSION_HMAC_KEY=\$(openssl rand -hex 32)"
echo ""
echo "2. Create /etc/famfin/famfin.env with:"
echo ""
echo "   CIPHER_KEY=\$CIPHER_KEY"
echo "   SESSION_HMAC_KEY=\$SESSION_HMAC_KEY"
echo ""
echo "3. Set restrictive permissions:"
echo "   chmod 600 /etc/famfin/famfin.env"
echo "   chown famfin:famfin /etc/famfin/famfin.env"
echo ""
echo "3. Start the service:"
echo "   systemctl start famfin"
echo ""
echo "4. Verify it's running:"
echo "   systemctl status famfin"
echo "   curl http://127.0.0.1:3000/health"
echo ""
echo "5. View logs:"
echo "   journalctl -u famfin -f"
echo ""

# Check if environment file exists
if [ ! -f /etc/famfin/famfin.env ]; then
    echo "⚠ /etc/famfin/famfin.env not configured"
    echo "  Service will not start until this is set up"
    echo ""
fi

echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "  1. Configure /etc/famfin/famfin.env"
echo "  2. Run: systemctl start famfin"
echo "  3. Monitor: journalctl -u famfin -f"
echo ""
