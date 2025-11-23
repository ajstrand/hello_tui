#!/bin/bash

# Setup script to create convenient aliases for the text editor
# Run with: source setup_aliases.sh

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up text editor aliases..."

# Create aliases for the current session
alias edit="cd '$PROJECT_DIR' && cargo run --bin edit --quiet --"
alias quick-edit="cd '$PROJECT_DIR' && cargo run --bin edit --quiet"
alias text-editor="cd '$PROJECT_DIR' && cargo run --bin interactive_editor --quiet --"

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
    echo "alias edit=\"cd '$PROJECT_DIR' && cargo run --bin edit --quiet --\"" >> "$shell_profile"
    echo "alias quick-edit=\"cd '$PROJECT_DIR' && cargo run --bin edit --quiet\"" >> "$shell_profile"
    echo "alias text-editor=\"cd '$PROJECT_DIR' && cargo run --bin interactive_editor --quiet --\"" >> "$shell_profile"
    
    echo "Aliases added to $shell_profile"
    echo "Run 'source $shell_profile' or restart your terminal to use them permanently"
}

echo "✅ Temporary aliases set for current session:"
echo "  edit [filename]       - Quick editor"
echo "  quick-edit           - Quick editor (no file)"  
echo "  text-editor [file]   - Full interactive editor"
echo ""
echo "To make these permanent, run: setup_permanent_aliases"
