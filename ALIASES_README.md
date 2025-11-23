# Text Editor Aliases Setup

This project provides multiple ways to run the text editors with convenient commands.

## Quick Start Options

### 1. Direct Script Execution (Recommended)
No setup required - just run the scripts directly:

```bash
./text-editor [filename]    # Full interactive editor with IOCraft UI
./edit [filename]           # Quick editor with file browser  
./quick-edit               # Quick editor (no file)
./editor-demo              # Demo mode
```

### 2. Shell Functions (Source Required)
Set up temporary shell functions:

```bash
source setup_aliases.sh    # Load functions into current session
text-editor [filename]     # Now you can use the commands
edit [filename]
quick-edit
editor-demo
```

### 3. Permanent Installation Options

#### A. Add to Your Shell Profile
```bash
source setup_aliases.sh    # Load functions
setup_permanent_aliases    # Add to ~/.zshrc or ~/.bashrc
```

#### B. System-wide Installation
```bash
sudo ./install_system_wide.sh    # Install to /usr/local/bin
```

## Available Commands

| Command | Description | Binary |
|---------|-------------|---------|
| `text-editor [file]` | Full interactive editor with IOCraft file dialog, mouse support | `interactive_editor` |
| `edit [filename]` | Quick editor with file browser | `quick_editor` |
| `quick-edit` | Quick editor without file | `quick_editor` |
| `editor-demo` | Demo mode showcasing features | `editor_demo` |

## Features

- **IOCraft File Dialog** - Beautiful terminal UI for file operations
- **Mouse Support** - Click to move cursor, select text
- **Syntax Highlighting** - Multi-language support via Syntect
- **Keyboard Shortcuts**:
  - `Ctrl+O` - Open file (with IOCraft dialog)
  - `Ctrl+S` - Save file
  - `Ctrl+N` - New file
  - `Ctrl+Q` - Quit
  - `Ctrl+H` - Toggle syntax highlighting

## Troubleshooting

If aliases don't work:
1. Try the direct script approach: `./text-editor`
2. Make sure you sourced the setup: `source setup_aliases.sh`
3. Check if scripts are executable: `ls -la text-editor edit quick-edit editor-demo`
4. For permanent setup, restart your terminal after running `setup_permanent_aliases`

## Direct Binary Execution

You can also run the compiled binaries directly:
```bash
cargo run --bin interactive_editor -- [filename]
cargo run --bin quick_editor -- [filename]
cargo run --bin editor_demo
./target/debug/interactive_editor [filename]
./target/debug/quick_editor [filename]
./target/debug/editor_demo
```
