use std::io::{self, stdout, stdin, Write};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, EnableMouseCapture, DisableMouseCapture},
    execute,
};
use hello_tui::{syntax, iocraft_file, mouse, linter, keys::{KeyHandler, Direction}, render, cursor};

struct InteractiveTextEditor {
    lines: Vec<String>,
    cursor: cursor::CursorController,
    filename: Option<String>,
    status_message: String,
    quit: bool,
    syntax_highlighter: syntax::SyntaxHighlighter,
    syntax_enabled: bool,
    iocraft_handler: iocraft_file::IOCraftFileHandler,
    modified: bool,
    mouse_controller: mouse::MouseController,
    text_selection: Option<mouse::TextSelection>,
    scroll_offset: usize,
    linter: linter::Linter,
    lint_issues: Vec<linter::LintIssue>,
    content: String,
    // Anti-flicker optimization fields
    last_render_time: std::time::Instant,
    render_throttle_ms: u64,
    needs_full_render: bool,
    // Key handling
    key_handler: KeyHandler,
    // Rendering
    renderer: render::EditorRenderer,
}

impl InteractiveTextEditor {
    fn new(filename: Option<String>) -> io::Result<Self> {
        let mut editor = Self {
            lines: vec!["".to_string()],
            cursor: cursor::CursorController::new(),
            filename: filename.clone(),
            status_message: "Ctrl+S: Save | Ctrl+O: Open | Ctrl+N: New | Ctrl+Q: Quit | Ctrl+H: Toggle highlighting | Ctrl+E: Toggle linting | Mouse: Click to move cursor".to_string(),
            quit: false,
            syntax_highlighter: syntax::SyntaxHighlighter::new(),
            syntax_enabled: true,
            iocraft_handler: iocraft_file::IOCraftFileHandler::new(),
            modified: false,
            mouse_controller: mouse::MouseController::new(),
            text_selection: None,
            scroll_offset: 0,
            linter: linter::Linter::new(),
            lint_issues: Vec::new(),
            content: String::new(),
            // Anti-flicker optimization initialization
            last_render_time: std::time::Instant::now(),
            render_throttle_ms: 16, // ~60 FPS throttling
            needs_full_render: true,
            // Key handling initialization
            key_handler: KeyHandler::new(),
            // Rendering initialization
            renderer: render::EditorRenderer::new(),
        };

        if let Some(filename) = filename {
            editor.load_file(&filename)?;
        } else {
            editor.lines = vec![
                "Welcome to IOCraft Enhanced Text Editor!".to_string(),
                "".to_string(),
                "Features:".to_string(),
                "  ✨ Beautiful line numbers and syntax highlighting".to_string(),
                "  🎯 Modern cursor and visual indicators".to_string(),
                "  🖱️ Full mouse support (click, drag, select)".to_string(),
                "  📁 IOCraft file dialogs and browser".to_string(),
                "  ⌨️ Sublime Text-style keyboard shortcuts".to_string(),
                "  🔍 Real-time code linting and issue detection".to_string(),
                "".to_string(),
                "Keyboard Shortcuts:".to_string(),
                "  📄 File: Ctrl+O (open), Ctrl+S (save), Ctrl+N (new)".to_string(),
                "  ✂️ Edit: Ctrl+D (duplicate line), Ctrl+K (delete line)".to_string(),
                "  🔍 Navigate: Ctrl+Home/End (document), Home/End (line)".to_string(),
                "  🎨 View: Ctrl+H (toggle highlighting), Ctrl+E (toggle linting)".to_string(),
                "  🚪 Quit: Ctrl+Q".to_string(),
                "".to_string(),
                "Start editing here...".to_string(),
            ];
        }

        Ok(editor)
    }

    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        match self.iocraft_handler.load_file(filename) {
            Ok(lines) => {
                self.lines = lines;
                self.filename = Some(filename.to_string());
                self.status_message = format!("Loaded: {}", filename);
                if self.linter.is_enabled() {
                    self.run_linting();
                }
                Ok(())
            }
            Err(e) => {
                self.status_message = format!("Error loading {}: {}", filename, e);
                Err(e)
            }
        }
    }

    fn save_file(&mut self) -> io::Result<()> {
        let filename = if let Some(ref name) = self.filename {
            name.clone()
        } else {
            "untitled.txt".to_string()
        };

        match self.iocraft_handler.save_file(&filename, &self.lines) {
            Ok(()) => {
                self.filename = Some(filename.clone());
                self.status_message = format!("Saved: {}", filename);
                Ok(())
            }
            Err(e) => {
                self.status_message = format!("Error saving {}: {}", filename, e);
                Err(e)
            }
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        self.cursor.move_cursor(direction, &self.lines);
    }

    fn insert_char(&mut self, ch: char) {
        let cursor_row = self.cursor.row();
        let cursor_col = self.cursor.col();
        
        if cursor_row < self.lines.len() {
            let line = &mut self.lines[cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(cursor_col, ch);
            *line = chars.into_iter().collect();
            
            // Move cursor right after inserting character
            self.cursor.move_cursor(Direction::Right, &self.lines);
            self.modified = true;
            if self.linter.is_enabled() {
                self.run_linting();
            }
        }
    }

    fn delete_char(&mut self) {
        let cursor_row = self.cursor.row();
        let cursor_col = self.cursor.col();
        
        if cursor_col > 0 && cursor_row < self.lines.len() {
            let line = &mut self.lines[cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            if cursor_col <= chars.len() && cursor_col > 0 {
                chars.remove(cursor_col - 1);
                *line = chars.into_iter().collect();
                self.cursor.move_cursor(Direction::Left, &self.lines);
                self.modified = true;
                if self.linter.is_enabled() {
                    self.run_linting();
                }
            }
        } else if cursor_row > 0 && cursor_col == 0 {
            // Join with previous line
            let current_line = self.lines.remove(cursor_row);
            let new_cursor_col = self.lines[cursor_row - 1].len();
            self.lines[cursor_row - 1].push_str(&current_line);
            self.cursor.set_position(cursor_row - 1, new_cursor_col);
            self.modified = true;
            if self.linter.is_enabled() {
                self.run_linting();
            }
        }
    }

    fn insert_newline(&mut self) {
        let cursor_row = self.cursor.row();
        let cursor_col = self.cursor.col();
        
        if cursor_row < self.lines.len() {
            let line = self.lines[cursor_row].clone();
            let (left, right) = line.split_at(cursor_col);
            self.lines[cursor_row] = left.to_string();
            self.lines.insert(cursor_row + 1, right.to_string());
        } else {
            self.lines.push(String::new());
        }
        
        self.cursor.set_position(cursor_row + 1, 0);
        self.modified = true;
        if self.linter.is_enabled() {
            self.run_linting();
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // Clear text selection when typing (except for selection-related keys)
        if !key_event.modifiers.contains(KeyModifiers::SHIFT) && 
           !key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {},
                _ => self.text_selection = None,
            }
        }

        match (key_event.modifiers.contains(KeyModifiers::CONTROL), key_event.code) {
            // Ctrl+Q - Quit
            (true, KeyCode::Char('q')) => {
                if self.modified {
                    self.status_message = "File has unsaved changes! Press Ctrl+Q again to quit without saving.".to_string();
                    self.modified = false; // Allow quit on second Ctrl+Q
                } else {
                    self.quit = true;
                }
            }
            
            // Ctrl+S - Save
            (true, KeyCode::Char('s')) => {
                if let Err(e) = self.save_file() {
                    self.status_message = format!("Error saving: {}", e);
                } else {
                    self.status_message = "File saved successfully!".to_string();
                    self.modified = false;
                }
                self.mark_for_full_render(); // Save affects status bar
            }
            
            // Ctrl+O - Open file
            (true, KeyCode::Char('o')) => {
                self.open_file_dialog();
                self.mark_for_full_render(); // File open changes everything
            }
            
            // Ctrl+H - Toggle syntax highlighting
            (true, KeyCode::Char('h')) => {
                self.syntax_enabled = !self.syntax_enabled;
                let status = if self.syntax_enabled { "enabled" } else { "disabled" };
                self.status_message = format!("Syntax highlighting {}", status);
                if self.linter.is_enabled() {
                    self.run_linting();
                }
                self.mark_for_full_render(); // Highlighting affects all content
            }
            
            // Ctrl+E - Toggle linting
            (true, KeyCode::Char('e')) => {
                self.linter.toggle();
                let status = if self.linter.is_enabled() { "enabled" } else { "disabled" };
                self.status_message = format!("Code linting {}", status);
                if self.linter.is_enabled() {
                    self.run_linting();
                } else {
                    self.lint_issues.clear();
                }
                self.mark_for_full_render(); // Linting affects line indicators and status
            }
            
            // Ctrl+N - New file
            (true, KeyCode::Char('n')) => {
                if self.modified {
                    self.status_message = "Save current file before creating new one (Ctrl+S)".to_string();
                } else {
                    self.lines = vec!["".to_string()];
                    self.cursor.set_position(0, 0);
                    self.filename = None;
                    self.modified = false;
                    self.status_message = "New file created".to_string();
                }
            }
            
            // Ctrl+Home - Go to start of document
            (true, KeyCode::Home) => {
                self.cursor.move_to_document_start();
                self.status_message = "Start of document".to_string();
            }
            
            // Ctrl+End - Go to end of document
            (true, KeyCode::End) => {
                self.cursor.move_to_document_end(&self.lines);
                self.status_message = "End of document".to_string();
            }
            
            // Ctrl+L - Go to line
            (true, KeyCode::Char('l')) => {
                self.status_message = "Go to line: (feature coming soon)".to_string();
            }
            
            // Ctrl+D - Duplicate line
            (true, KeyCode::Char('d')) => {
                let cursor_row = self.cursor.row();
                if cursor_row < self.lines.len() {
                    let line_to_duplicate = self.lines[cursor_row].clone();
                    self.lines.insert(cursor_row + 1, line_to_duplicate);
                    self.cursor.set_position(cursor_row + 1, self.cursor.col());
                    self.modified = true;
                    self.status_message = "Line duplicated".to_string();
                }
            }
            
            // Ctrl+K - Delete line
            (true, KeyCode::Char('k')) => {
                let cursor_row = self.cursor.row();
                if cursor_row < self.lines.len() && self.lines.len() > 1 {
                    self.lines.remove(cursor_row);
                    let new_row = if cursor_row >= self.lines.len() {
                        self.lines.len() - 1
                    } else {
                        cursor_row
                    };
                    self.cursor.set_position(new_row, 0);
                    self.modified = true;
                    self.status_message = "Line deleted".to_string();
                } else if self.lines.len() == 1 {
                    self.lines[0].clear();
                    self.cursor.set_position(cursor_row, 0);
                    self.modified = true;
                    self.status_message = "Line cleared".to_string();
                }
            }

            // Regular character input
            (false, KeyCode::Char(ch)) => {
                if self.text_selection.is_some() {
                    self.delete_selected_text();
                }
                self.insert_char(ch);
            }
            
            // Enter key
            (false, KeyCode::Enter) => {
                if self.text_selection.is_some() {
                    self.delete_selected_text();
                }
                self.insert_newline();
            }
            
            // Backspace
            (false, KeyCode::Backspace) => {
                if self.text_selection.is_some() {
                    self.delete_selected_text();
                } else {
                    self.delete_char();
                }
            }
            
            // Delete key
            (false, KeyCode::Delete) => {
                if self.text_selection.is_some() {
                    self.delete_selected_text();
                } else {
                    // Delete character forward
                    let cursor_row = self.cursor.row();
                    let cursor_col = self.cursor.col();
                    
                    if cursor_row < self.lines.len() {
                        let line = &mut self.lines[cursor_row];
                        let mut chars: Vec<char> = line.chars().collect();
                        if cursor_col < chars.len() {
                            chars.remove(cursor_col);
                            *line = chars.into_iter().collect();
                            self.modified = true;
                        } else if cursor_row < self.lines.len() - 1 {
                            // Join with next line
                            let next_line = self.lines.remove(cursor_row + 1);
                            self.lines[cursor_row].push_str(&next_line);
                            self.modified = true;
                        }
                    }
                }
            }
            
            // Home - Go to start of line
            (false, KeyCode::Home) => {
                self.cursor.move_to_line_start();
            }
            
            // End - Go to end of line
            (false, KeyCode::End) => {
                self.cursor.move_to_line_end(&self.lines);
            }

            // Movement commands
            (false, KeyCode::Up) => self.move_cursor(Direction::Up),
            (false, KeyCode::Down) => self.move_cursor(Direction::Down),
            (false, KeyCode::Left) => self.move_cursor(Direction::Left),
            (false, KeyCode::Right) => self.move_cursor(Direction::Right),

            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let action = self.mouse_controller.handle_mouse_event(mouse_event);
        
        match action {
            mouse::MouseAction::Click { row, col } => {
                self.move_cursor_to_position(row, col);
                self.text_selection = None;
                self.status_message = format!("Cursor moved to row {}, col {}", row + 1, col + 1);
                self.mark_for_full_render(); // Click needs full render for cursor position
            }
            
            mouse::MouseAction::DoubleClick { row, col } => {
                self.select_word_at_position(row, col);
                self.status_message = "Word selected (double-click)".to_string();
                self.mark_for_full_render(); // Selection needs full render
            }
            
            mouse::MouseAction::DragEnd { start_row, start_col, end_row, end_col } => {
                self.text_selection = Some(mouse::TextSelection::new(start_row, start_col, end_row, end_col));
                self.move_cursor_to_position(end_row, end_col);
                if let Some(ref selection) = self.text_selection {
                    let selected_text = selection.get_selected_text(&self.lines);
                    let char_count = selected_text.chars().count();
                    self.status_message = format!("Selected {} characters", char_count);
                }
                self.mark_for_full_render(); // Selection end needs full render
            }
            
            mouse::MouseAction::RightClick { row, col } => {
                self.move_cursor_to_position(row, col);
                self.show_context_menu(row, col);
                self.mark_for_full_render(); // Context menu needs full render
            }
            
            mouse::MouseAction::ScrollUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(3);
                }
                self.status_message = "Scrolled up".to_string();
                self.mark_for_full_render(); // Scroll needs full render for content change
            }
            
            mouse::MouseAction::ScrollDown => {
                let max_scroll = self.lines.len().saturating_sub(10);
                if self.scroll_offset < max_scroll {
                    self.scroll_offset = (self.scroll_offset + 3).min(max_scroll);
                }
                self.status_message = "Scrolled down".to_string();
                self.mark_for_full_render(); // Scroll needs full render for content change
            }
            
            mouse::MouseAction::Drag { start_row: _, start_col: _, current_row, current_col } => {
                self.move_cursor_to_position(current_row, current_col);
                self.status_message = "Selecting text...".to_string();
                // Don't mark for full render during drag - use throttled rendering instead
            }
            
            mouse::MouseAction::None => {}
        }
    }

    fn move_cursor_to_position(&mut self, row: usize, col: usize) {
        self.cursor.move_to_position(row, col, self.scroll_offset, &self.lines);
    }

    fn select_word_at_position(&mut self, row: usize, col: usize) {
        let actual_row = row + self.scroll_offset;
        
        if actual_row < self.lines.len() {
            let line = &self.lines[actual_row];
            let (start_col, end_col) = mouse::find_word_boundaries(line, col.min(line.len()));
            
            self.text_selection = Some(mouse::TextSelection::new(
                actual_row, start_col, actual_row, end_col
            ));
            
            self.cursor.set_position(actual_row, end_col);
        }
    }

    fn show_context_menu(&mut self, row: usize, col: usize) {
        let actual_row = row + self.scroll_offset;
        
        if let Some(ref selection) = self.text_selection {
            let selected_text = selection.get_selected_text(&self.lines);
            if !selected_text.is_empty() {
                self.status_message = format!("Context menu: Copy/Cut/Paste available at row {}, col {}", actual_row + 1, col + 1);
            } else {
                self.status_message = format!("Context menu: Paste available at row {}, col {}", actual_row + 1, col + 1);
            }
        } else {
            self.status_message = format!("Context menu: Paste available at row {}, col {}", actual_row + 1, col + 1);
        }
    }

    fn delete_selected_text(&mut self) {
        if let Some(selection) = &self.text_selection {
            let start_row = selection.start_row;
            let start_col = selection.start_col;
            let end_row = selection.end_row;
            let end_col = selection.end_col;

            if start_row == end_row {
                // Single line selection
                let line = &mut self.lines[start_row];
                let mut chars: Vec<char> = line.chars().collect();
                for _ in start_col..end_col.min(chars.len()) {
                    if start_col < chars.len() {
                        chars.remove(start_col);
                    }
                }
                *line = chars.into_iter().collect();
            } else {
                // Multi-line selection
                let start_line_part = if start_col < self.lines[start_row].len() {
                    self.lines[start_row][..start_col].to_string()
                } else {
                    self.lines[start_row].clone()
                };
                
                let end_line_part = if end_col < self.lines[end_row].len() {
                    self.lines[end_row][end_col..].to_string()
                } else {
                    String::new()
                };
                
                // Remove the lines in between
                for _ in start_row..end_row {
                    if start_row + 1 < self.lines.len() {
                        self.lines.remove(start_row + 1);
                    }
                }
                
                // Combine the remaining parts
                self.lines[start_row] = start_line_part + &end_line_part;
            }

            self.cursor.set_position(start_row, start_col);
            self.text_selection = None;
            self.modified = true;
        }
    }

    fn run_linting(&mut self) {
        // Update content from lines
        self.content = self.lines.join("\n");
        
        // Run linting with new API
        self.lint_issues = self.linter.lint(&self.content, self.filename.as_deref());
    }

    /// Throttled render that prevents excessive screen updates during mouse events
    fn render_throttled(&mut self) -> io::Result<()> {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_render_time).as_millis() as u64;
        
        // Only render if enough time has passed or if we need a full render
        if elapsed >= self.render_throttle_ms || self.needs_full_render {
            self.render()?;
            self.last_render_time = now;
            self.needs_full_render = false;
        }
        
        Ok(())
    }

    /// Force a full render on the next update
    fn mark_for_full_render(&mut self) {
        self.needs_full_render = true;
    }

    fn render(&self) -> io::Result<()> {
        self.renderer.render_editor(
            &self.lines,
            self.cursor.row(),
            self.cursor.col(),
            self.filename.as_deref(),
            self.modified,
            &self.status_message,
            &self.syntax_highlighter,
            self.syntax_enabled,
            self.scroll_offset,
            &self.lint_issues,
            &self.linter,
            self.text_selection.as_ref(),
        )
    }

    fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnableMouseCapture, Clear(ClearType::All))?;

        // Initial render
        self.render()?;

        while !self.quit {
            match read()? {
                Event::Key(key_event) => {
                    // Handle Ctrl+C to quit
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                        break;
                    }
                    self.handle_key_event(key_event);
                    // Use optimized rendering for better performance
                    if self.needs_full_render {
                        self.render()?;
                        self.last_render_time = std::time::Instant::now();
                        self.needs_full_render = false;
                    } else {
                        self.render_throttled()?;
                    }
                }
                Event::Mouse(mouse_event) => {
                    self.handle_mouse_event(mouse_event);
                    // Use throttled rendering for mouse events to prevent flickering
                    self.render_throttled()?;
                }
                _ => {}
            }
        }

        execute!(stdout(), DisableMouseCapture)?;
        disable_raw_mode()?;
        execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }

    fn open_file_dialog(&mut self) {
        if self.modified {
            self.status_message = "Save current file before opening new one (Ctrl+S)".to_string();
            return;
        }

        // Clear the screen for the dialog
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).ok();
        
        // Show IOCraft file browser dialog with enhanced UI
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│ 📂 Open File - IOCraft File Browser                        │");
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│                                                             │");
        println!("│ 🎯 Navigate and select a file to open:                     │");
        println!("│                                                             │");
        
        // Get current directory files
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        if let Ok(_files) = self.iocraft_handler.display_file_browser(current_dir.to_str().unwrap_or(".")) {
            println!("│                                                             │");
            println!("├─────────────────────────────────────────────────────────────┤");
            println!("│ 🚀 Quick Actions:                                          │");
            println!("│                                                             │");
            println!("│  📝 [1] Type filename below                                │");
            println!("│  📁 [2] Browse recent files                                │");
            println!("│  🆕 [3] Create new file                                    │");
            println!("│  ❌ [ESC] Cancel and return to editor                     │");
            println!("│                                                             │");
            println!("├─────────────────────────────────────────────────────────────┤");
            print!("│ 📝 Enter filename or action [1-3]: ");
            stdout().flush().ok();

            // Read user input for filename or action
            if let Some(input) = self.read_filename_input() {
                let input = input.trim();
                
                if input.is_empty() {
                    self.status_message = "No input provided".to_string();
                    return;
                }
                
                match input {
                    "1" => {
                        println!("│                                                             │");
                        print!("│ 📝 Filename: ");
                        stdout().flush().ok();
                        
                        if let Some(filename) = self.read_filename_input() {
                            self.process_file_open(&filename.trim());
                        }
                    }
                    "2" => {
                        self.show_recent_files_dialog();
                    }
                    "3" => {
                        println!("│                                                             │");
                        print!("│ 🆕 New filename: ");
                        stdout().flush().ok();
                        
                        if let Some(filename) = self.read_filename_input() {
                            let filename = filename.trim();
                            if !filename.is_empty() {
                                self.create_new_file_interactive(&filename);
                            }
                        }
                    }
                    _ => {
                        // Try to open as filename directly
                        self.process_file_open(input);
                    }
                }
            } else {
                self.status_message = "Open canceled".to_string();
            }
        } else {
            self.status_message = "Error reading directory".to_string();
        }
        
        println!("│                                                             │");
        println!("└─────────────────────────────────────────────────────────────┘");
        
        // Brief pause to show result
        std::thread::sleep(std::time::Duration::from_millis(800));
    }

    fn process_file_open(&mut self, filename: &str) {
        if filename.is_empty() {
            self.status_message = "No file specified".to_string();
            return;
        }
        
        // Attempt to load the file
        match self.load_file(filename) {
            Ok(()) => {
                self.status_message = format!("✅ Opened: {}", filename);
            }
            Err(_) => {
                // Offer to create new file if it doesn't exist
                self.offer_create_new_file(filename);
            }
        }
    }

    fn show_recent_files_dialog(&mut self) {
        println!("│                                                             │");
        println!("│ 📁 Recent Files:                                           │");
        
        // Show recent files (for now, we'll show some common file extensions in the directory)
        let recent_files: Vec<String> = vec![
            "sample.rs".to_string(), 
            "sample.js".to_string(), 
            "sample.txt".to_string(), 
            "README.md".to_string(), 
            "Cargo.toml".to_string()
        ];
        
        self.iocraft_handler.display_recent_files(&recent_files);
        
        println!("│                                                             │");
        print!("│ 📝 Select file (or type name): ");
        stdout().flush().ok();
        
        if let Some(filename) = self.read_filename_input() {
            self.process_file_open(&filename.trim());
        }
    }

    fn create_new_file_interactive(&mut self, filename: &str) {
        self.lines = vec!["".to_string()];
        self.cursor.set_position(0, 0);
        self.filename = Some(filename.to_string());
        self.modified = true; // Mark as modified since it's new
        self.status_message = format!("🆕 New file '{}' created - ready for editing!", filename);
    }

    fn read_filename_input(&mut self) -> Option<String> {
        // Temporarily disable raw mode for text input
        disable_raw_mode().ok();
        
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                // Re-enable raw mode
                enable_raw_mode().ok();
                Some(input.trim().to_string())
            }
            Err(_) => {
                enable_raw_mode().ok();
                None
            }
        }
    }

    fn offer_create_new_file(&mut self, filename: &str) {
        println!("│                                                             │");
        println!("│ ⚠️  File '{}' not found.                     │", filename);
        println!("│                                                             │");
        println!("│ 🆕 Would you like to create a new file? (y/n)              │");
        print!("│ 📝 Your choice: ");
        stdout().flush().ok();

        if let Some(response) = self.read_filename_input() {
            if response.to_lowercase().starts_with('y') {
                // Create new file
                self.lines = vec!["".to_string()];
                self.cursor.set_position(0, 0);
                self.filename = Some(filename.to_string());
                self.modified = false;
                self.status_message = format!("New file '{}' ready for editing", filename);
            } else {
                self.status_message = "File creation canceled".to_string();
            }
        } else {
            self.status_message = "File creation canceled".to_string();
        }
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    println!("🎉 Starting IOCraft Interactive Text Editor...");
    if let Some(ref name) = filename {
        println!("📂 Opening file: {}", name);
    } else {
        println!("📝 Creating new file...");
    }
    println!("Press Ctrl+C to exit if needed\n");

    let mut editor = InteractiveTextEditor::new(filename)?;
    editor.run()?;

    println!("👋 Thanks for using IOCraft Text Editor!");
    Ok(())
}
