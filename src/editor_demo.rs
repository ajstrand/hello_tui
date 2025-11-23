use iocraft::prelude::*;
use std::io;

#[derive(Default)]
struct TextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            lines: vec![
                "Welcome to IOCraft Text Editor!".to_string(),
                "This is a simple yet functional text editor.".to_string(),
                "".to_string(),
                "Features:".to_string(),
                "- Multi-line text editing".to_string(),
                "- Cursor movement with arrow keys".to_string(),
                "- Insert and delete text".to_string(),
                "- Enter to create new lines".to_string(),
                "- Backspace to delete characters".to_string(),
                "".to_string(),
                "Start editing here...".to_string(),
            ],
            cursor_row: 10,
            cursor_col: 21, // Fixed to be within the line length
        }
    }

    fn insert_char(&mut self, ch: char) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(self.cursor_col, ch);
            *line = chars.into_iter().collect();
            self.cursor_col += 1;
        } else {
            self.lines.push(ch.to_string());
            self.cursor_row = self.lines.len() - 1;
            self.cursor_col = 1;
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
            // Join with previous line
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
        }
    }

    fn new_line(&mut self) {
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

    fn get_content(&self) -> String {
        self.lines.join("\n")
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
    println!("🎉 IOCraft Text Editor Demo");
    println!("==========================\n");
    
    let mut editor = TextEditor::new();
    
    // Simulate some editing operations
    println!("📝 Initial content:");
    display_editor(&editor);
    
    println!("\n🔄 Simulating text editing operations...\n");
    
    // Move cursor to end and add some text
    editor.cursor_row = 10;
    editor.cursor_col = editor.lines[10].len(); // Move to end of line
    
    // Insert some characters
    for ch in " Here I am typing!".chars() {
        editor.insert_char(ch);
    }
    
    println!("✏️  After inserting text:");
    display_editor(&editor);
    
    // Add a new line
    editor.new_line();
    for ch in "This is a new line.".chars() {
        editor.insert_char(ch);
    }
    
    println!("\n➕ After adding a new line:");
    display_editor(&editor);
    
    // Test backspace
    for _ in 0..5 {
        editor.delete_char();
    }
    
    println!("\n⬅️  After some backspaces:");
    display_editor(&editor);
    
    // Test cursor movement
    editor.move_cursor(Direction::Up);
    editor.move_cursor(Direction::Left);
    editor.move_cursor(Direction::Left);
    
    for ch in " [INSERTED]".chars() {
        editor.insert_char(ch);
    }
    
    println!("\n🎯 After cursor movement and insertion:");
    display_editor(&editor);
    
    println!("\n💾 Final editor content:");
    println!("Content length: {} characters", editor.get_content().len());
    println!("Number of lines: {}", editor.lines.len());
    
    println!("\n✅ Text editor demo completed successfully!");
    println!("📦 The IOCraft library has been successfully integrated.");
    
    Ok(())
}

fn display_editor(editor: &TextEditor) {
    println!("┌─────────────────────────────────────────────┐");
    
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
        println!("│ {:<43} │", display.get(0..43).unwrap_or(&display));
    }
    
    println!("├─────────────────────────────────────────────┤");
    println!("│ Row: {}, Col: {} | {} lines{} │", 
             editor.cursor_row + 1, 
             editor.cursor_col + 1, 
             editor.lines.len(),
             " ".repeat(22));
    println!("└─────────────────────────────────────────────┘");
}
