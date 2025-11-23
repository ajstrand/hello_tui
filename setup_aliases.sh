#!/bin/bash

# Setup script to create convenient aliases for the text editor
# Run with: source setup_aliases.sh

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up text editor aliases and functions..."

# Unset any existing aliases to avoid conflicts
unalias edit 2>/dev/null || true
unalias quick-edit 2>/dev/null || true  
unalias text-editor 2>/dev/null || true
unalias editor-demo 2>/dev/null || true
unalias file-demo 2>/dev/null || true

# Create functions (work in both interactive and non-interactive shells)
edit() {
    (cd "$PROJECT_DIR" && cargo run --bin quick_editor --quiet -- "$@")
}

quick-edit() {
    (cd "$PROJECT_DIR" && cargo run --bin quick_editor --quiet)
}

text-editor() {
    (cd "$PROJECT_DIR" && cargo run --bin interactive_editor --quiet -- "$@")
}

editor-demo() {
    (cd "$PROJECT_DIR" && cargo run --bin editor_demo --quiet)
}

file-demo() {
    (cd "$PROJECT_DIR" && cargo run --bin file_demo --quiet)
}

# Function to add to shell profile
setup_permanent_aliases() {
    local shell_profile=""
    
    if [ -n "$ZSH_VERSION" ]; then
        shell_profile="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        shell_profile="$HOME/.bashrc"
    else
        echo "Unable to detect shell type"
        return 1
    fi

    echo "" >> "$shell_profile"
    echo "# Text Editor Functions (added by hello_tui)" >> "$shell_profile"
    echo "edit() { (cd \"$PROJECT_DIR\" && cargo run --bin quick_editor --quiet -- \"\$@\"); }" >> "$shell_profile"
    echo "quick-edit() { (cd \"$PROJECT_DIR\" && cargo run --bin quick_editor --quiet); }" >> "$shell_profile"
    echo "text-editor() { (cd \"$PROJECT_DIR\" && cargo run --bin interactive_editor --quiet -- \"\$@\"); }" >> "$shell_profile"
    echo "editor-demo() { (cd \"$PROJECT_DIR\" && cargo run --bin editor_demo --quiet); }" >> "$shell_profile"
    echo "file-demo() { (cd \"$PROJECT_DIR\" && cargo run --bin file_demo --quiet); }" >> "$shell_profile"
    
    echo "Aliases added to $shell_profile"
    echo "Run 'source $shell_profile' or restart your terminal to use them permanently"
}

echo "✅ Shell functions set for current session:"
echo "  edit [filename]       - Quick editor with file browser"
echo "  quick-edit           - Quick editor (no file)"  
echo "  text-editor [file]   - Full interactive editor with mouse support"
echo "  editor-demo          - Demo mode for showcasing features"
echo "  file-demo           - File operations demo"
echo ""
echo "🔧 Functions work in both interactive and non-interactive shells"
echo "🚀 Executable wrapper scripts also available:"
echo "  ./text-editor [file] - Direct script (no sourcing needed)"
echo "  ./edit [filename]    - Direct script for quick editor" 
echo "  ./quick-edit         - Direct script for quick editor (no file)"
echo "  ./editor-demo        - Direct script for demo"
echo ""
echo "To make functions permanent, run: setup_permanent_aliases"
echo "For system-wide installation, run: sudo ./install_system_wide.sh"
