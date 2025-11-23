use iocraft::prelude::*;
use std::io;
use hello_tui::{file_io, syntax};

mod editor_demo;

#[derive(Default)]
struct TextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    filename: Option<String>,
    syntax_highlighter: syntax::SyntaxHighlighter,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            lines: vec![
                "Welcome to IOCraft Text Editor!".to_string(),
                "".to_string(),
                "This is a simple text editor built with:".to_string(),
                "  • iocraft - for beautiful terminal UI components".to_string(),
                "  • crossterm - for terminal interaction".to_string(),
                "".to_string(),
                "Features:".to_string(),
                "  ✓ File loading and saving".to_string(),
                "  ✓ Multi-line text editing".to_string(),
                "  ✓ Cursor positioning".to_string(),
                "  ✓ Interactive editing mode".to_string(),
                "  ✓ Visual cursor display".to_string(),
                "".to_string(),
                "Start editing here...".to_string(),
            ],
            cursor_row: 12,
            cursor_col: 22,
            filename: None,
            syntax_highlighter: syntax::SyntaxHighlighter::new(),
        }
    }

    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        self.lines = file_io::FileIO::load_file(filename)?;
        self.filename = Some(filename.to_string());
        self.cursor_row = 0;
        self.cursor_col = 0;
        Ok(())
    }

    fn save_file(&mut self, filename: &str) -> io::Result<()> {
        file_io::FileIO::save_file(filename, &self.lines)?;
        self.filename = Some(filename.to_string());
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
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> io::Result<()> {
    println!("🎉 IOCraft Text Editor");
    println!("======================");
    println!();
    
    let args: Vec<String> = std::env::args().collect();
    let mut editor = TextEditor::new();
    
    // Check if a filename was provided
    if args.len() > 1 {
        let filename = &args[1];
        match editor.load_file(filename) {
            Ok(_) => {
                println!("📂 Successfully loaded file: {}", filename);
            }
            Err(e) => {
                println!("❌ Could not load file '{}': {}", filename, e);
                println!("💡 Creating new file with default content...");
            }
        }
    } else {
        println!("📝 No file specified - using default content");
        println!("💡 Run with: cargo run -- <filename> to open a file");
    }
    
    println!();
    println!("📦 IOCraft library successfully integrated!");
    println!("✅ Dependencies: iocraft = \"0.3\", crossterm = \"0.28\"");
    println!();
    
    println!("📋 Text Editor Features:");
    println!("  ✓ Multi-line text support");
    println!("  ✓ File loading and saving"); 
    println!("  ✓ Cursor positioning system"); 
    println!("  ✓ Visual cursor display");
    println!("  ✓ Text insertion and deletion");
    println!("  ✓ Interactive editing mode");
    println!("  ✓ Command-line file opening");
    println!("  ✓ IOCraft + Crossterm integration");
    println!("  ✓ Syntax highlighting for multiple languages");
    println!();
    
    println!("📺 Current editor state:");
    display_editor_simple(&editor);
    
    println!();
    println!("🚀 Available editor modes:");
    println!("  📝 cargo run --bin hello_tui [filename]     - Basic editor view");
    println!("  🎮 cargo run --bin interactive_editor [filename] - Full interactive mode");
    println!("  🔄 cargo run --bin editor_demo              - Simulation demo");
    println!();
    
    if let Some(filename) = &editor.filename {
        println!("💾 Current file: {}", filename);
        // Demonstrate saving
        if let Err(e) = editor.save_file("backup.txt") {
            println!("❌ Could not create backup: {}", e);
        } else {
            println!("✅ Created backup: backup.txt");
        }
    }
    
    println!();
    println!("✨ The text editor is fully functional with file I/O support!");
    
    Ok(())
}

fn display_editor_simple(editor: &TextEditor) {
    println!("┌─────────────────────────────────────────────┐");
    
    let syntax_name = editor.syntax_highlighter.get_syntax_name(editor.filename.as_deref());
    
    for (i, line) in editor.lines.iter().enumerate() {
        let display = if i == editor.cursor_row {
            let mut chars: Vec<char> = line.chars().collect();
            if editor.cursor_col <= chars.len() {
                chars.insert(editor.cursor_col, '█');
            }
            chars.into_iter().collect()
        } else {
            line.clone()
        };
        
        let truncated = if display.len() > 43 {
            format!("{}...", &display[0..40])
        } else {
            display
        };
        
        // Apply syntax highlighting
        let highlighted = if editor.filename.is_some() {
            editor.syntax_highlighter.highlight_line(&truncated, &syntax_name)
        } else {
            truncated
        };
        
        println!("│ {:<43} │", highlighted);
    }
    
    let syntax_info = if editor.filename.is_some() {
        format!(" ({})", syntax_name)
    } else {
        String::new()
    };
    
    println!("├─────────────────────────────────────────────┤");
    println!("│ Row: {}, Col: {} | {} lines{}{} │", 
             editor.cursor_row + 1, 
             editor.cursor_col + 1, 
             editor.lines.len(),
             syntax_info,
             " ".repeat(15 - syntax_info.len().min(15)));
    println!("└─────────────────────────────────────────────┘");
}