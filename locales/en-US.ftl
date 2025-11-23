# Application Startup
app-title = IOCraft Interactive Text Editor
app-startup = 🎉 Starting IOCraft Interactive Text Editor...
app-thanks = 👋 Thanks for using IOCraft Text Editor!

# File Operations
file-new = 📝 Creating new file...
file-opening = 📂 Opening file: { $filename }
file-loaded = Loaded: { $filename }
file-saved = Saved: { $filename }
file-error-loading = Error loading { $filename }: { $error }
file-error-saving = Error saving { $filename }: { $error }
file-not-found = ⚠️ File '{ $filename }' not found.
file-create-new = 🆕 Would you like to create a new file? (y/n)
file-creation-canceled = File creation canceled
file-new-ready = New file '{ $filename }' ready for editing
file-no-input = No input provided
file-no-file-specified = No file specified
file-opened-success = ✅ Opened: { $filename }
file-new-created = 🆕 New file '{ $filename }' created - ready for editing!
file-unsaved-changes = File has unsaved changes! Press Ctrl+Q again to quit without saving.
file-save-before-new = Save current file before creating new one (Ctrl+S)
file-save-before-open = Save current file before opening new one (Ctrl+S)

# Welcome Content
welcome-title = Welcome to IOCraft Enhanced Text Editor!
welcome-features = Features:
welcome-feature-highlighting = ✨ Beautiful line numbers and syntax highlighting
welcome-feature-cursor = 🎯 Modern cursor and visual indicators
welcome-feature-mouse = 🖱️ Full mouse support (click, drag, select)
welcome-feature-dialogs = 📁 IOCraft file dialogs and browser
welcome-feature-shortcuts = ⌨️ Sublime Text-style keyboard shortcuts
welcome-feature-linting = 🔍 Real-time code linting and issue detection
welcome-shortcuts = Keyboard Shortcuts:
welcome-shortcuts-file = 📄 File: Ctrl+O (open), Ctrl+S (save), Ctrl+N (new)
welcome-shortcuts-edit = ✂️ Edit: Ctrl+D (duplicate line), Ctrl+K (delete line)
welcome-shortcuts-navigate = 🔍 Navigate: Ctrl+Home/End (document), Home/End (line)
welcome-shortcuts-view = 🎨 View: Ctrl+H (toggle highlighting), Ctrl+E (toggle linting)
welcome-shortcuts-quit = 🚪 Quit: Ctrl+Q
welcome-start-editing = Start editing here...

# Status Messages
status-file-saved = File saved successfully!
status-syntax-enabled = Syntax highlighting enabled
status-syntax-disabled = Syntax highlighting disabled
status-linting-enabled = Code linting enabled
status-linting-disabled = Code linting disabled
status-new-file-created = New file created
status-document-start = Start of document
status-document-end = End of document
status-goto-line-soon = Go to line: (feature coming soon)
status-line-duplicated = Line duplicated
status-line-deleted = Line deleted
status-line-cleared = Line cleared
status-cursor-moved = Cursor moved to row { $row }, col { $col }
status-word-selected = Word selected (double-click)
status-text-selected = Selected { $count } characters
status-selecting = Selecting text...
status-scrolled-up = Scrolled up
status-scrolled-down = Scrolled down

# Context Menu
context-menu-copy-cut-paste = Context menu: Copy/Cut/Paste available at row { $row }, col { $col }
context-menu-paste = Context menu: Paste available at row { $row }, col { $col }

# Dialog System
dialog-open-file = 📂 Open File - IOCraft File Browser
dialog-navigate-select = 🎯 Navigate and select a file to open:
dialog-quick-actions = 🚀 Quick Actions:
dialog-action-type-filename = 📝 [1] Type filename below
dialog-action-browse-recent = 📁 [2] Browse recent files
dialog-action-create-new = 🆕 [3] Create new file
dialog-action-cancel = ❌ [ESC] Cancel and return to editor
dialog-enter-filename = 📝 Enter filename or action [1-3]:
dialog-filename-prompt = 📝 Filename:
dialog-new-filename = 🆕 New filename:
dialog-recent-files = 📁 Recent Files:
dialog-select-file = 📝 Select file (or type name):
dialog-create-choice = 📝 Your choice:
dialog-open-canceled = Open canceled
dialog-error-reading-directory = Error reading directory

# Help System
help-status-message = Ctrl+S: Save | Ctrl+O: Open | Ctrl+N: New | Ctrl+Q: Quit | Ctrl+H: Toggle highlighting | Ctrl+E: Toggle linting | Mouse: Click to move cursor

# UI Elements
ui-no-file = [No file]
ui-plain-text = Plain Text
ui-line-count = { $count ->
    [one] { $count } line
   *[other] { $count } lines
}
ui-modified-indicator = ✅
ui-line-prefix = Ln
ui-column-prefix = Col
ui-chars-selected = { $count } chars selected

# Language Settings
lang-switch-success = Language switched to { $language }
lang-switch-error = Failed to switch language: { $error }
lang-not-supported = Language '{ $language }' is not supported
lang-current = Current language: { $language }
lang-available = Available languages: { $languages }

# Error Messages
error-invalid-locale = Invalid locale: { $locale }
error-locale-not-supported = Locale '{ $locale }' not supported
error-file-operation = File operation error: { $error }
error-general = Error: { $message }

# Exit Prompt
exit-prompt = Press Ctrl+C to exit if needed
