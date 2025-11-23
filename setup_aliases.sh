#!/bin/bash

# Setup script to create convenient aliases for the text editor
# Run with: source setup_aliases.sh

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up text editor aliases..."

# Create aliases for the current session
alias edit="cd '$PROJECT_DIR' && cargo run --bin quick_editor --quiet --"
alias quick-edit="cd '$PROJECT_DIR' && cargo run --bin quick_editor --quiet"
alias text-editor="cd '$PROJECT_DIR' && cargo run --bin interactive_editor --quiet --"
alias editor-demo="cd '$PROJECT_DIR' && cargo run --bin editor_demo --quiet"
alias file-demo="cd '$PROJECT_DIR' && cargo run --bin file_demo --quiet"

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
    echo "# Text Editor Aliases (added by hello_tui)" >> "$shell_profile"
    echo "alias edit=\"cd '$PROJECT_DIR' && cargo run --bin quick_editor --quiet --\"" >> "$shell_profile"
    echo "alias quick-edit=\"cd '$PROJECT_DIR' && cargo run --bin quick_editor --quiet\"" >> "$shell_profile"
    echo "alias text-editor=\"cd '$PROJECT_DIR' && cargo run --bin interactive_editor --quiet --\"" >> "$shell_profile"
    echo "alias editor-demo=\"cd '$PROJECT_DIR' && cargo run --bin editor_demo --quiet\"" >> "$shell_profile"
    echo "alias file-demo=\"cd '$PROJECT_DIR' && cargo run --bin file_demo --quiet\"" >> "$shell_profile"
    
    echo "Aliases added to $shell_profile"
    echo "Run 'source $shell_profile' or restart your terminal to use them permanently"
}

echo "✅ Temporary aliases set for current session:"
echo "  edit [filename]       - Quick editor with file browser"
echo "  quick-edit           - Quick editor (no file)"  
echo "  text-editor [file]   - Full interactive editor with mouse support"
echo "  editor-demo          - Demo mode for showcasing features"
echo "  file-demo           - File operations demo"
echo ""
echo "To make these permanent, run: setup_permanent_aliases"
