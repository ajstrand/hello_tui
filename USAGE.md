# IOCraft Text Editor - Quick Setup Guide

## 🚀 Getting Started

### 1. **Temporary Aliases** (current session only)
```bash
cd /path/to/hello_tui
source setup_aliases.sh
```

### 2. **Permanent Aliases** (persistent across terminal sessions)
```bash
cd /path/to/hello_tui
source setup_aliases.sh
setup_permanent_aliases
```

Then restart your terminal or run:
```bash
source ~/.bashrc   # for bash
source ~/.zshrc    # for zsh
```

## ⌨️ Available Commands

### **edit** - Quick file editor with browser
- `edit filename` - Open specific file in quick editor
- `edit` - Browse files and get overview

### **text-editor** - Full interactive editor
- `text-editor filename` - Open file in full editor with mouse support
- `text-editor` - Start empty interactive editor

### **quick-edit** - Quick launcher
- `quick-edit` - Quick launcher with file browser

### **editor-demo** - Feature showcase
- `editor-demo` - Demonstration of editor capabilities

### **file-demo** - File operations demo
- `file-demo` - Demo of file I/O operations

## 🎯 Examples

```bash
# Edit a Rust file
edit main.rs

# Open interactive editor with mouse support
text-editor config.json

# Browse available files
quick-edit

# See all features in action
editor-demo
```

## 🔧 Features Available

### **Quick Editor** (`edit`)
- ✅ File browser with icons
- ✅ Syntax detection and highlighting
- ✅ File preview
- ✅ File information display
- ✅ Beautiful IOCraft UI

### **Interactive Editor** (`text-editor`)
- ✅ Full text editing capabilities
- ✅ Sublime Text-style keyboard shortcuts
- ✅ Mouse support (click, drag, scroll, select)
- ✅ Real-time syntax highlighting
- ✅ File operations (save, open, new)
- ✅ Text selection and manipulation

### **Keyboard Shortcuts** (Interactive Editor)
- `Ctrl+S` - Save file
- `Ctrl+Q` - Quit
- `Ctrl+N` - New file
- `Ctrl+H` - Toggle syntax highlighting
- `Arrow keys` - Navigate
- `Home/End` - Line start/end
- `Ctrl+Home/End` - Document start/end

### **Mouse Controls** (Interactive Editor)
- **Left click** - Move cursor
- **Double-click** - Select word
- **Right-click** - Context menu
- **Click + drag** - Select text
- **Mouse wheel** - Scroll document

## 🐛 Troubleshooting

### **Aliases not working?**
1. Make sure you ran `source setup_aliases.sh`
2. For permanent aliases, run `setup_permanent_aliases` after sourcing
3. Restart your terminal or source your shell config

### **Binary not found?**
The aliases use these exact binary names:
- `quick_editor`
- `interactive_editor`
- `editor_demo`
- `file_demo`

If you get errors, make sure you're in the project directory and run `cargo build` first.

### **Mouse not working in terminal?**
Some terminals don't support mouse events. Try:
- Modern terminals: kitty, iTerm2, Windows Terminal
- Enable mouse support in your terminal settings
- The keyboard shortcuts work in all terminals

## 📁 Project Structure

- `src/interactive_editor.rs` - Full editor with mouse support
- `src/quick_editor.rs` - Quick launcher and file browser
- `src/editor_demo.rs` - Feature demonstration
- `src/file_demo.rs` - File operations demo
- `src/mouse.rs` - Mouse control module
- `src/syntax.rs` - Syntax highlighting
- `src/iocraft_file.rs` - Beautiful file UI components
- `src/file_io.rs` - Core file operations

Enjoy editing! 🎉
