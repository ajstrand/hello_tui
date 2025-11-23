#!/bin/bash

# Quick shortcut to run the interactive text editor
# Usage: ./edit.sh [filename]

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$PROJECT_DIR"

if [ $# -eq 0 ]; then
    echo "🚀 Starting Quick Editor..."
    cargo run --bin edit --quiet
else
    echo "🚀 Starting Quick Editor with file: $1"
    cargo run --bin edit --quiet -- "$1"
fi
