use std::io;
use hello_tui::{file_io, syntax};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Show, Hide},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
};
use std::io::{stdout, Write};

#[derive(Default)]
struct QuickEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    filename: Option<String>,
    status_message: String,
    mode: EditorMode,
    quit: bool,
    syntax_highlighter: syntax::SyntaxHighlighter,
    syntax_enabled: bool,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum EditorMode {
    #[default]
    Normal,
    Insert,
}

impl QuickEditor {
    fn new(filename: Option<String>) -> io::Result<Self> {
        let mut editor = Self {
            lines: vec!["".to_string()],
            cursor_row: 0,
            cursor_col: 0,
            filename: filename.clone(),
            status_message: "Quick Edit Mode | 'i'=insert, 'q'=quit, 's'=save, 'h'=syntax toggle".to_string(),
            mode: EditorMode::Normal,
            quit: false,
            syntax_highlighter: syntax::SyntaxHighlighter::new(),
            syntax_enabled: true,
        };

        if let Some(filename) = filename {
            if let Err(_) = editor.load_file(&filename) {
                editor.lines = vec!["".to_string()];
                editor.status_message = format!("New file: {}", filename);
            }
        } else {
            editor.lines = vec![
                "🚀 Quick Text Editor".to_string(),
                "".to_string(),
                "Press 'i' to start editing".to_string(),
                "Press 'q' to quit, 's' to save".to_string(),
                "".to_string(),
            ];
        }

        Ok(editor)
    }

    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        self.lines = file_io::FileIO::load_file(filename)?;
        self.filename = Some(filename.to_string());
        self.status_message = format!("Loaded: {}", filename);
        Ok(())
    }

    fn save_file(&mut self) -> io::Result<()> {
        let filename = if let Some(ref name) = self.filename {
            name.clone()
        } else {
            "untitled.txt".to_string()
        };

        file_io::FileIO::save_file(&filename, &self.lines)?;
        self.filename = Some(filename.clone());
        self.status_message = format!("Saved: {}", filename);
        Ok(())
    }

    fn insert_char(&mut self, ch: char) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(self.cursor_col, ch);
            *line = chars.into_iter().collect();
            self.cursor_col += 1;
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
            }
        } else if self.cursor_row > 0 && self.cursor_col == 0 {
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (self.mode, key_event.code) {
            (EditorMode::Normal, KeyCode::Char('q')) => {
                self.quit = true;
            }
            (EditorMode::Normal, KeyCode::Char('i')) => {
                self.mode = EditorMode::Insert;
                self.status_message = "-- INSERT --".to_string();
            }
            (EditorMode::Normal, KeyCode::Char('s')) => {
                if let Err(e) = self.save_file() {
                    self.status_message = format!("Error: {}", e);
                }
            }
            (EditorMode::Normal, KeyCode::Char('h')) => {
                self.syntax_enabled = !self.syntax_enabled;
                let status = if self.syntax_enabled { "enabled" } else { "disabled" };
                self.status_message = format!("Syntax highlighting {}", status);
            }
            (EditorMode::Insert, KeyCode::Esc) => {
                self.mode = EditorMode::Normal;
                self.status_message = "-- NORMAL --".to_string();
            }
            (EditorMode::Insert, KeyCode::Char(ch)) => {
                self.insert_char(ch);
            }
            (EditorMode::Insert, KeyCode::Enter) => {
                self.insert_newline();
            }
            (EditorMode::Insert, KeyCode::Backspace) => {
                self.delete_char();
            }
            (_, KeyCode::Up) => self.move_cursor(Direction::Up),
            (_, KeyCode::Down) => self.move_cursor(Direction::Down),
            (_, KeyCode::Left) => self.move_cursor(Direction::Left),
            (_, KeyCode::Right) => self.move_cursor(Direction::Right),
            _ => {}
        }
    }

    fn render(&self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Hide)?;

        let syntax_name = if self.syntax_enabled {
            self.syntax_highlighter.get_syntax_name(self.filename.as_deref())
        } else {
            "Plain Text".to_string()
        };

        // Render content
        for (row, line) in self.lines.iter().enumerate() {
            if row == self.cursor_row {
                let (before_cursor, after_cursor) = if self.cursor_col <= line.len() {
                    (
                        &line[..self.cursor_col.min(line.len())],
                        &line[self.cursor_col.min(line.len())..]
                    )
                } else {
                    (line.as_str(), "")
                };

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
                
                match self.mode {
                    EditorMode::Insert => print!("\x1b[7m \x1b[0m"),
                    EditorMode::Normal => print!("\x1b[7m{}\x1b[0m", 
                        after_cursor.chars().next().unwrap_or(' ')),
                }

                if self.mode == EditorMode::Normal && !after_cursor.is_empty() {
                    print!("{}", &highlighted_after[1..]);
                } else if self.mode == EditorMode::Insert {
                    print!("{}", highlighted_after);
                }
            } else {
                if self.syntax_enabled {
                    let highlighted = self.syntax_highlighter.highlight_line(line, &syntax_name);
                    print!("{}", highlighted);
                } else {
                    print!("{}", line);
                }
            }
            println!();
        }

        let mode_str = match self.mode {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
        };

        let filename = self.filename.as_deref().unwrap_or("[No file]");
        let syntax_status = if self.syntax_enabled {
            format!("({})", syntax_name)
        } else {
            "(Plain)".to_string()
        };
        
        println!();
        println!("-- {} -- {} {} | Row: {}, Col: {} | {}", 
                 mode_str, filename, syntax_status, 
                 self.cursor_row + 1, self.cursor_col + 1, self.status_message);

        stdout().flush()?;
        Ok(())
    }

    fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;

        while !self.quit {
            self.render()?;

            if let Event::Key(key_event) = read()? {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                    break;
                }
                self.handle_key_event(key_event);
            }
        }

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

    println!("🚀 Quick Editor - Starting...");
    if let Some(ref name) = filename {
        println!("📂 File: {}", name);
    } else {
        println!("📝 New document");
    }
    
    let mut editor = QuickEditor::new(filename)?;
    editor.run()?;

    println!("👋 Quick Editor closed");
    Ok(())
}
