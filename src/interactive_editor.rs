use std::io::{self, stdout, Write};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Show, Hide},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, EnableMouseCapture, DisableMouseCapture},
    execute,
};
use hello_tui::{syntax, iocraft_file, mouse};

#[derive(Default)]
struct InteractiveTextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
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
}

impl InteractiveTextEditor {
    fn new(filename: Option<String>) -> io::Result<Self> {
        let mut editor = Self {
            lines: vec!["".to_string()],
            cursor_row: 0,
            cursor_col: 0,
            filename: filename.clone(),
            status_message: "Ctrl+S: Save | Ctrl+O: Open | Ctrl+N: New | Ctrl+Q: Quit | Ctrl+H: Toggle highlighting | Mouse: Click to move cursor".to_string(),
            quit: false,
            syntax_highlighter: syntax::SyntaxHighlighter::new(),
            syntax_enabled: true,
            iocraft_handler: iocraft_file::IOCraftFileHandler::new(),
            modified: false,
            mouse_controller: mouse::MouseController::new(),
            text_selection: None,
            scroll_offset: 0,
        };

        if let Some(filename) = filename {
            editor.load_file(&filename)?;
        } else {
            editor.lines = vec![
                "Welcome to IOCraft Text Editor (Sublime-style)!".to_string(),
                "".to_string(),
                "Keyboard Shortcuts:".to_string(),
                "  Ctrl+S - Save file".to_string(),
                "  Ctrl+O - Open file".to_string(),
                "  Ctrl+N - New file".to_string(),
                "  Ctrl+Q - Quit".to_string(),
                "  Ctrl+H - Toggle syntax highlighting".to_string(),
                "  Ctrl+Z - Undo (coming soon)".to_string(),
                "  Ctrl+Y - Redo (coming soon)".to_string(),
                "  Arrow keys - Move cursor".to_string(),
                "  Home/End - Start/End of line".to_string(),
                "  Ctrl+Home/End - Start/End of document".to_string(),
                "".to_string(),
                "Start typing to edit...".to_string(),
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
        match direction {
            Direction::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    let line_len = self.lines[self.cursor_row].len();
                    self.cursor_col = self.cursor_col.min(line_len);
                }
            }
            Direction::Down => {
                if self.cursor_row < self.lines.len() - 1 {
                    self.cursor_row += 1;
                    let line_len = self.lines[self.cursor_row].len();
                    self.cursor_col = self.cursor_col.min(line_len);
                }
            }
            Direction::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.lines[self.cursor_row].len();
                }
            }
            Direction::Right => {
                let line_len = self.lines[self.cursor_row].len();
                if self.cursor_col < line_len {
                    self.cursor_col += 1;
                } else if self.cursor_row < self.lines.len() - 1 {
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                }
            }
        }
    }

    fn insert_char(&mut self, ch: char) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(self.cursor_col, ch);
            *line = chars.into_iter().collect();
            self.cursor_col += 1;
            self.modified = true;
        }
    }

    fn delete_char(&mut self) {
        if self.cursor_col > 0 && self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            if self.cursor_col <= chars.len() && self.cursor_col > 0 {
                chars.remove(self.cursor_col - 1);
                *line = chars.into_iter().collect();
                self.cursor_col -= 1;
                self.modified = true;
            }
        } else if self.cursor_row > 0 && self.cursor_col == 0 {
            // Join with previous line
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
            self.modified = true;
        }
    }

    fn insert_newline(&mut self) {
        if self.cursor_row < self.lines.len() {
            let cursor_col = self.cursor_col;
            let line = self.lines[self.cursor_row].clone();
            let (left, right) = line.split_at(cursor_col);
            self.lines[self.cursor_row] = left.to_string();
            self.lines.insert(self.cursor_row + 1, right.to_string());
        } else {
            self.lines.push(String::new());
        }
        self.cursor_row += 1;
        self.cursor_col = 0;
        self.modified = true;
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
            }
            
            // Ctrl+H - Toggle syntax highlighting
            (true, KeyCode::Char('h')) => {
                self.syntax_enabled = !self.syntax_enabled;
                let status = if self.syntax_enabled { "enabled" } else { "disabled" };
                self.status_message = format!("Syntax highlighting {}", status);
            }
            
            // Ctrl+N - New file
            (true, KeyCode::Char('n')) => {
                if self.modified {
                    self.status_message = "Save current file before creating new one (Ctrl+S)".to_string();
                } else {
                    self.lines = vec!["".to_string()];
                    self.cursor_row = 0;
                    self.cursor_col = 0;
                    self.filename = None;
                    self.modified = false;
                    self.status_message = "New file created".to_string();
                }
            }
            
            // Ctrl+Home - Go to start of document
            (true, KeyCode::Home) => {
                self.cursor_row = 0;
                self.cursor_col = 0;
                self.status_message = "Start of document".to_string();
            }
            
            // Ctrl+End - Go to end of document
            (true, KeyCode::End) => {
                self.cursor_row = if self.lines.is_empty() { 0 } else { self.lines.len() - 1 };
                self.cursor_col = if self.lines.is_empty() { 0 } else { self.lines[self.cursor_row].len() };
                self.status_message = "End of document".to_string();
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
                    if self.cursor_row < self.lines.len() {
                        let line = &mut self.lines[self.cursor_row];
                        let mut chars: Vec<char> = line.chars().collect();
                        if self.cursor_col < chars.len() {
                            chars.remove(self.cursor_col);
                            *line = chars.into_iter().collect();
                            self.modified = true;
                        } else if self.cursor_row < self.lines.len() - 1 {
                            // Join with next line
                            let next_line = self.lines.remove(self.cursor_row + 1);
                            self.lines[self.cursor_row].push_str(&next_line);
                            self.modified = true;
                        }
                    }
                }
            }
            
            // Home - Go to start of line
            (false, KeyCode::Home) => {
                self.cursor_col = 0;
            }
            
            // End - Go to end of line
            (false, KeyCode::End) => {
                if self.cursor_row < self.lines.len() {
                    self.cursor_col = self.lines[self.cursor_row].len();
                }
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
            }
            
            mouse::MouseAction::DoubleClick { row, col } => {
                self.select_word_at_position(row, col);
                self.status_message = "Word selected (double-click)".to_string();
            }
            
            mouse::MouseAction::DragEnd { start_row, start_col, end_row, end_col } => {
                self.text_selection = Some(mouse::TextSelection::new(start_row, start_col, end_row, end_col));
                self.move_cursor_to_position(end_row, end_col);
                if let Some(ref selection) = self.text_selection {
                    let selected_text = selection.get_selected_text(&self.lines);
                    let char_count = selected_text.chars().count();
                    self.status_message = format!("Selected {} characters", char_count);
                }
            }
            
            mouse::MouseAction::RightClick { row, col } => {
                self.move_cursor_to_position(row, col);
                self.show_context_menu(row, col);
            }
            
            mouse::MouseAction::ScrollUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(3);
                }
                self.status_message = "Scrolled up".to_string();
            }
            
            mouse::MouseAction::ScrollDown => {
                let max_scroll = self.lines.len().saturating_sub(10);
                if self.scroll_offset < max_scroll {
                    self.scroll_offset = (self.scroll_offset + 3).min(max_scroll);
                }
                self.status_message = "Scrolled down".to_string();
            }
            
            mouse::MouseAction::Drag { start_row: _, start_col: _, current_row, current_col } => {
                self.move_cursor_to_position(current_row, current_col);
                self.status_message = "Selecting text...".to_string();
            }
            
            mouse::MouseAction::None => {}
        }
    }

    fn move_cursor_to_position(&mut self, row: usize, col: usize) {
        // Adjust for scroll offset
        let actual_row = row + self.scroll_offset;
        
        if actual_row < self.lines.len() {
            self.cursor_row = actual_row;
            let line_len = self.lines[self.cursor_row].len();
            self.cursor_col = col.min(line_len);
        }
    }

    fn select_word_at_position(&mut self, row: usize, col: usize) {
        let actual_row = row + self.scroll_offset;
        
        if actual_row < self.lines.len() {
            let line = &self.lines[actual_row];
            let (start_col, end_col) = mouse::find_word_boundaries(line, col.min(line.len()));
            
            self.text_selection = Some(mouse::TextSelection::new(
                actual_row, start_col, actual_row, end_col
            ));
            
            self.cursor_row = actual_row;
            self.cursor_col = end_col;
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

            self.cursor_row = start_row;
            self.cursor_col = start_col;
            self.text_selection = None;
            self.modified = true;
        }
    }

    fn render(&self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Hide)?;

        let syntax_name = if self.syntax_enabled {
            self.syntax_highlighter.get_syntax_name(self.filename.as_deref())
        } else {
            "Plain Text".to_string()
        };

        let terminal_height = crossterm::terminal::size()?.1 as usize;
        let visible_lines = terminal_height.saturating_sub(3); // Reserve space for status

        // Render content with scroll offset
        for (_, actual_row) in (self.scroll_offset..self.scroll_offset + visible_lines)
            .enumerate()
            .take_while(|(_, actual_row)| *actual_row < self.lines.len())
        {
            let line = &self.lines[actual_row];
            
            // Check if this line has selection
            let line_has_selection = self.text_selection.as_ref()
                .map(|sel| sel.start_row <= actual_row && actual_row <= sel.end_row)
                .unwrap_or(false);
            
            if actual_row == self.cursor_row && !line_has_selection {
                // Show cursor on current line (without selection)
                let (before_cursor, after_cursor) = if self.cursor_col <= line.len() {
                    (
                        &line[..self.cursor_col.min(line.len())],
                        &line[self.cursor_col.min(line.len())..]
                    )
                } else {
                    (line.as_str(), "")
                };

                // Apply syntax highlighting to the parts
                let highlighted_before = if self.syntax_enabled {
                    self.syntax_highlighter.highlight_line(before_cursor, &syntax_name)
                } else {
                    before_cursor.to_string()
                };

                let highlighted_after = if self.syntax_enabled && !after_cursor.is_empty() {
                    self.syntax_highlighter.highlight_line(after_cursor, &syntax_name)
                } else {
                    after_cursor.to_string()
                };

                print!("{}", highlighted_before);
                
                // Show cursor (always insert mode cursor for Sublime Text style)
                print!("\x1b[7m \x1b[0m"); // Inverted space for cursor
                print!("{}", highlighted_after);
            } else if line_has_selection {
                // Render line with selection highlighting
                self.render_line_with_selection(line, actual_row, &syntax_name)?;
            } else {
                // Regular line without cursor or selection
                if self.syntax_enabled {
                    let highlighted = self.syntax_highlighter.highlight_line(line, &syntax_name);
                    print!("{}", highlighted);
                } else {
                    print!("{}", line);
                }
            }
            println!();
        }

        // Render status line
        let filename = self.filename.as_deref().unwrap_or("[No file]");
        let syntax_status = if self.syntax_enabled {
            format!("({})", syntax_name)
        } else {
            "(No highlighting)".to_string()
        };
        
        let modified_indicator = if self.modified { "*" } else { "" };
        let selection_info = if let Some(ref sel) = self.text_selection {
            let selected_text = sel.get_selected_text(&self.lines);
            format!(" | {} chars selected", selected_text.chars().count())
        } else {
            String::new()
        };
        
        println!();
        println!("-- {} {}{} | Row: {}, Col: {} | {}{}", 
                 filename, modified_indicator, syntax_status, 
                 self.cursor_row + 1, self.cursor_col + 1, 
                 self.status_message, selection_info);

        stdout().flush()?;
        Ok(())
    }

    fn render_line_with_selection(&self, line: &str, row: usize, syntax_name: &str) -> io::Result<()> {
        if let Some(ref selection) = self.text_selection {
            let chars: Vec<char> = line.chars().collect();
            
            for (col, ch) in chars.iter().enumerate() {
                let is_selected = selection.contains(row, col);
                let is_cursor = row == self.cursor_row && col == self.cursor_col;
                
                if is_cursor && is_selected {
                    // Cursor within selection
                    print!("\x1b[7;4m{}\x1b[0m", ch); // Inverted and underlined
                } else if is_cursor {
                    // Cursor outside selection
                    print!("\x1b[7m{}\x1b[0m", ch); // Inverted
                } else if is_selected {
                    // Selected text
                    print!("\x1b[7m{}\x1b[0m", ch); // Inverted (highlighted)
                } else {
                    // Normal text
                    if self.syntax_enabled {
                        let char_str = ch.to_string();
                        let highlighted = self.syntax_highlighter.highlight_line(&char_str, syntax_name);
                        print!("{}", highlighted);
                    } else {
                        print!("{}", ch);
                    }
                }
            }
            
            // Handle cursor at end of line
            if row == self.cursor_row && self.cursor_col >= chars.len() {
                print!("\x1b[7m \x1b[0m"); // Cursor at end of line
            }
        } else {
            // Fallback to regular rendering
            if self.syntax_enabled {
                let highlighted = self.syntax_highlighter.highlight_line(line, syntax_name);
                print!("{}", highlighted);
            } else {
                print!("{}", line);
            }
        }
        
        Ok(())
    }

    fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnableMouseCapture)?;

        while !self.quit {
            self.render()?;

            match read()? {
                Event::Key(key_event) => {
                    // Handle Ctrl+C to quit
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                        break;
                    }
                    self.handle_key_event(key_event);
                }
                Event::Mouse(mouse_event) => {
                    self.handle_mouse_event(mouse_event);
                }
                _ => {}
            }
        }

        execute!(stdout(), DisableMouseCapture)?;
        disable_raw_mode()?;
        execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
