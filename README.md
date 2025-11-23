# IOCraft Text Editor

A modern terminal-based text editor built in Rust with **Sublime Text-style controls** and beautiful UI components powered by IOCraft.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)

## ✨ Features

### 🎯 **Sublime Text-Style Editing**
- **Direct typing** - No modal editing, start typing immediately
- **Familiar keyboard shortcuts** - Ctrl+S (save), Ctrl+O (open), Ctrl+Q (quit)
- **Real-time feedback** - Instant status updates and modification indicators

### 🎨 **Advanced Syntax Highlighting**
- **Multi-language support** - Rust, JavaScript, Python, and more
- **Theme support** - Beautiful syntax highlighting with color themes
- **Automatic detection** - File type detection based on extension

### 🖥️ **Beautiful Terminal UI**
- **IOCraft integration** - Stunning file browser and status displays
- **Clean interface** - Modern, intuitive terminal experience
- **Real-time cursor** - Visual feedback during editing
- **Mouse support** - Full mouse navigation and text selection

### 🖱️ **Advanced Mouse Controls**
- **Click navigation** - Click anywhere to position cursor
- **Text selection** - Drag to select with visual highlighting
- **Word selection** - Double-click to select entire words
- **Scroll navigation** - Mouse wheel scrolling through documents
- **Context menus** - Right-click for available actions

### 🏗️ **Modular Architecture**
- **Multiple editor variants** - Interactive, quick launcher, demo modes
- **Separated concerns** - File I/O, syntax highlighting, and UI in separate modules
- **Extensible design** - Easy to add new features and functionality

## 🚀 Quick Start

### Prerequisites
- Rust (2024 edition or later)
- Terminal with Unicode support

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/yourusername/hello_tui.git
cd hello_tui
```

2. **Build the project:**
```bash
cargo build --release
```

3. **Run the editor:**
```bash
# Interactive editor
cargo run --bin interactive_editor

# With a specific file
cargo run --bin interactive_editor myfile.rs

# Quick launcher with file browser
cargo run --bin quick_editor

# Demo mode
cargo run --bin editor_demo
```

## ⌨️ Controls

### File Operations
- `Ctrl+S` - Save current file
- `Ctrl+O` - Open file (coming soon)
- `Ctrl+N` - Create new file
- `Ctrl+Q` - Quit editor
- `Ctrl+C` - Force quit

### Text Editing
- **Direct typing** - All characters insert immediately
- `Enter` - Insert new line
- `Backspace` - Delete character before cursor
- `Delete` - Delete character after cursor or selected text

### Navigation
- `Arrow keys` - Move cursor
- `Home` - Go to start of line
- `End` - Go to end of line
- `Ctrl+Home` - Go to start of document
- `Ctrl+End` - Go to end of document

### Mouse Controls 🖱️
- **Left click** - Move cursor to clicked position
- **Double-click** - Select word at position
- **Right-click** - Show context menu with available actions
- **Click and drag** - Select text with visual highlighting
- **Scroll wheel** - Scroll up/down through document

### Features
- `Ctrl+H` - Toggle syntax highlighting

## 🏗️ Architecture

### Core Components

#### **Interactive Editor** (`src/interactive_editor.rs`)
The main editor with full Sublime Text-style editing capabilities:
- Real-time character insertion
- Syntax highlighting integration
- File loading and saving
- Cursor management and navigation

#### **Syntax Highlighting** (`src/syntax.rs`)
Powered by the `syntect` library:
- Multi-language syntax detection
- Theme support and management
- Efficient highlighting for large files

#### **IOCraft File Handler** (`src/iocraft_file.rs`)
Beautiful file operations with IOCraft UI:
- Elegant file browser interface
- Rich status displays
- Error handling with user feedback

#### **File I/O Module** (`src/file_io.rs`)
Core file operations:
- Efficient file reading and writing
- Error handling and validation
- Cross-platform file system support

#### **Mouse Module** (`src/mouse.rs`)
Full mouse interaction support:
- Click and drag text selection
- Double-click word selection
- Mouse wheel scrolling
- Right-click context menus
- Visual selection highlighting

### Binary Targets

| Binary | Purpose | Use Case |
|--------|---------|----------|
| `interactive_editor` | Full-featured editor | Daily text editing |
| `quick_editor` | Quick file launcher | Fast file operations |
| `editor_demo` | Feature demonstration | Testing and showcasing |
| `hello_tui` | Basic editor | Simple editing tasks |

## 🧪 Testing

Run the test suite to verify all functionality:

```bash
cargo test
```

All modules include comprehensive tests covering:
- File operations and I/O
- Syntax highlighting accuracy
- IOCraft UI components
- Error handling scenarios

## 🛠️ Dependencies

### Core Libraries
- **IOCraft** `0.3` - Modern terminal UI framework
- **Crossterm** `0.28` - Cross-platform terminal manipulation
- **Syntect** `5.1` - Syntax highlighting engine

### Development
- **Rust 2024 Edition** - Latest language features
- **Cargo** - Build system and package manager

## 🎯 Design Philosophy

### **Sublime Text Experience**
This editor brings the familiar, intuitive editing experience of Sublime Text to the terminal:
- **No modal editing** - Unlike vim, start typing immediately
- **Consistent shortcuts** - Familiar Ctrl+ combinations
- **Real-time feedback** - Instant visual responses

### **Modern Terminal UI**
Leveraging IOCraft for beautiful, responsive interfaces:
- **Rich components** - File browsers, status bars, dialogs
- **Consistent styling** - Professional appearance
- **User-friendly** - Clear feedback and intuitive navigation

### **Modular Design**
Clean separation of concerns for maintainability:
- **Single responsibility** - Each module has one clear purpose
- **Easy testing** - Isolated components enable thorough testing
- **Extensible** - Simple to add new features and capabilities

## 🚧 Roadmap

### Upcoming Features
- [ ] **Ctrl+O** - File open dialog
- [ ] **Ctrl+Z/Y** - Undo/Redo functionality
- [ ] **Find and Replace** - Search capabilities
- [ ] **Multiple tabs** - Edit multiple files simultaneously
- [ ] **Plugin system** - Extensible functionality
- [ ] **Custom themes** - Personalized syntax highlighting

### Performance Enhancements
- [ ] **Large file support** - Efficient handling of big files
- [ ] **Incremental highlighting** - Faster syntax updates
- [ ] **Memory optimization** - Reduced resource usage

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- **IOCraft** - For the beautiful terminal UI framework
- **Syntect** - For robust syntax highlighting capabilities
- **Crossterm** - For cross-platform terminal support
- **Rust Community** - For excellent documentation and libraries

---

**Built with ❤️ in Rust**
