#!/bin/bash

# Simple installation script to add wrapper scripts to system PATH
# Run with: sudo ./install_system_wide.sh

if [ "$EUID" -ne 0 ]; then
    echo "❌ Please run as root (with sudo) to install system-wide"
    echo "💡 Alternative: Run './text-editor' directly from this directory"
    echo "💡 Or source setup_aliases.sh for temporary aliases"
    exit 1
fi

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="/usr/local/bin"

echo "🔧 Installing text editor commands to $INSTALL_DIR..."

# Install wrapper scripts
cp "$PROJECT_DIR/text-editor" "$INSTALL_DIR/text-editor"
cp "$PROJECT_DIR/edit" "$INSTALL_DIR/edit"  
cp "$PROJECT_DIR/quick-edit" "$INSTALL_DIR/quick-edit"
cp "$PROJECT_DIR/editor-demo" "$INSTALL_DIR/editor-demo"

# Make sure they're executable
chmod +x "$INSTALL_DIR/text-editor"
chmod +x "$INSTALL_DIR/edit"
chmod +x "$INSTALL_DIR/quick-edit"
chmod +x "$INSTALL_DIR/editor-demo"

echo "✅ Commands installed successfully!"
echo ""
echo "📋 Available commands (system-wide):"
echo "  text-editor [file]   - Full interactive editor with IOCraft UI"
echo "  edit [filename]      - Quick editor with file browser"
echo "  quick-edit          - Quick editor (no file)"
echo "  editor-demo         - Demo mode"
echo ""
echo "🚀 You can now run these commands from anywhere!"
