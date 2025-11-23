use std::io;
use hello_tui::file_io;

#[derive(Default)]
struct SimpleTextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    filename: Option<String>,
}

impl SimpleTextEditor {
    fn new() -> Self {
        Self::default()
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

    fn display(&self) {
        println!("┌─────────────────────────────────────────────────────────────┐");
        
        for (i, line) in self.lines.iter().enumerate() {
            let display = if i == self.cursor_row {
                let mut chars: Vec<char> = line.chars().collect();
                if self.cursor_col <= chars.len() {
                    chars.insert(self.cursor_col, '█');
                }
                chars.into_iter().collect()
            } else {
                line.clone()
            };
            
            let truncated = if display.len() > 59 {
                format!("{}...", &display[0..56])
            } else {
                display
            };
            
            println!("│ {:<59} │", truncated);
        }
        
        let filename_text = self.filename.as_deref().unwrap_or("[No file]");
        
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ File: {} | Row: {}, Col: {} | {} lines{} │", 
                 filename_text,
                 self.cursor_row + 1, 
                 self.cursor_col + 1,
                 self.lines.len(),
                 " ".repeat(25 - filename_text.len().min(20)));
        println!("└─────────────────────────────────────────────────────────────┘");
    }

    fn simulate_editing(&mut self) {
        println!("🔄 Simulating text editing operations...");
        
        // Move to end of first line and add text
        self.cursor_row = 0;
        self.cursor_col = self.lines[0].len();
        
        let additional_text = " [EDITED WITH IOCRAFT]";
        for ch in additional_text.chars() {
            self.insert_char(ch);
        }
        
        // Add a new line
        self.insert_newline();
        let new_content = "This line was added by the IOCraft text editor!";
        for ch in new_content.chars() {
            self.insert_char(ch);
        }
        
        println!("✅ Text editing operations completed");
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
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    println!("🎉 IOCraft Simple Text Editor Demo");
    println!("===================================");
    println!();
    
    let mut editor = SimpleTextEditor::new();
    
    if args.len() > 1 {
        let filename = &args[1];
        println!("📂 Loading file: {}", filename);
        
        match editor.load_file(filename) {
            Ok(_) => {
                println!("✅ File loaded successfully!");
                println!();
                
                println!("📄 Original content:");
                editor.display();
                
                // Simulate editing
                println!();
                editor.simulate_editing();
                
                println!();
                println!("📄 After editing:");
                editor.display();
                
                // Save the edited version
                let output_filename = format!("{}.edited", filename);
                match editor.save_file(&output_filename) {
                    Ok(_) => {
                        println!();
                        println!("💾 Saved edited version to: {}", output_filename);
                    }
                    Err(e) => {
                        println!("❌ Error saving: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error loading file: {}", e);
                return Err(e);
            }
        }
    } else {
        println!("📝 No file provided - creating demo content");
        
        editor.lines = vec![
            "Welcome to IOCraft Text Editor!".to_string(),
            "This is a demonstration of file I/O capabilities.".to_string(),
            "".to_string(),
            "Features demonstrated:".to_string(),
            "- Loading text files".to_string(),
            "- Editing text content".to_string(),
            "- Saving modified files".to_string(),
            "- Cursor positioning".to_string(),
        ];
        editor.cursor_row = 0;
        editor.cursor_col = 0;
        
        println!("📄 Demo content:");
        editor.display();
        
        // Save demo content
        match editor.save_file("demo_output.txt") {
            Ok(_) => {
                println!();
                println!("💾 Demo content saved to: demo_output.txt");
            }
            Err(e) => {
                println!("❌ Error saving demo: {}", e);
            }
        }
    }
    
    println!();
    println!("✨ File I/O demonstration completed!");
    println!("🚀 For interactive editing, run: cargo run --bin interactive_editor [filename]");
    
    Ok(())
}
