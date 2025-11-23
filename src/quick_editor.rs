use hello_tui::{iocraft_file, syntax};
use std::io;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file_handler = iocraft_file::IOCraftFileHandler::new();

    // Display welcome screen
    file_handler.display_welcome_screen();
    println!();

    // Handle command line arguments
    if args.len() > 1 {
        let filename = &args[1];
        
        println!("🚀 Quick Editor Mode: Opening file \"{}\"", filename);
        println!();

        // Try to load the file
        match file_handler.load_file(filename) {
            Ok(lines) => {
                println!();
                file_handler.display_file_info(filename, &lines);
                println!();
                
                // Display syntax highlighting info
                let highlighter = syntax::SyntaxHighlighter::new();
                let syntax_name = highlighter.get_syntax_name(Some(filename));
                println!("🎨 Syntax: {} (highlighting {})", 
                        syntax_name,
                        if highlighter.is_syntax_supported(&syntax_name) { "✅ supported" } else { "❌ not supported" });
                
                // Display file content preview
                println!();
                display_file_preview(&lines, filename, &highlighter, &syntax_name);
                
                println!();
                println!("💡 To edit this file interactively, run:");
                println!("   cargo run --bin interactive_editor {}", filename);
            }
            Err(_) => {
                println!();
                println!("🤔 File doesn't exist. Would you like to create it?");
                println!("   Creating new file: {}", filename);
                
                if let Err(e) = file_handler.create_new_file(filename) {
                    eprintln!("❌ Failed to create file: {}", e);
                    return Err(e);
                }
                
                println!();
                println!("💡 File created! You can now edit it with:");
                println!("   cargo run --bin interactive_editor {}", filename);
            }
        }
    } else {
        // No file specified - show file browser and recent files
        println!("🔍 No file specified. Here's what's in the current directory:");
        println!();
        
        if let Ok(_files) = file_handler.display_file_browser(".") {
            println!();
        }
        
        // Show recent files (mock data for demo)
        let recent_files = vec![
            "sample.rs".to_string(),
            "sample.js".to_string(),
            "sample.txt".to_string(),
        ];
        file_handler.display_recent_files(&recent_files);
        
        println!();
        println!("💡 To open a specific file, run:");
        println!("   cargo run --bin quick_editor <filename>");
        println!();
        println!("💡 To start interactive editing mode:");
        println!("   cargo run --bin interactive_editor [filename]");
    }

    Ok(())
}

fn display_file_preview(lines: &[String], filename: &str, highlighter: &syntax::SyntaxHighlighter, syntax_name: &str) {
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ 👀 File Preview: {:<45} │", filename);
    println!("├─────────────────────────────────────────────────────────────┤");
    
    let preview_lines = lines.iter().take(15); // Show first 15 lines
    let mut line_number = 1;
    
    for line in preview_lines {
        let display_line = if line.len() > 55 {
            format!("{}...", &line[0..52])
        } else {
            line.clone()
        };
        
        // Apply syntax highlighting for supported languages
        let highlighted_line = if highlighter.is_syntax_supported(syntax_name) {
            highlighter.highlight_line(&display_line, syntax_name)
        } else {
            display_line
        };
        
        println!("│ {:>2} │ {:<55} │", line_number, highlighted_line);
        line_number += 1;
    }
    
    if lines.len() > 15 {
        println!("│    │ ... and {} more lines ...                       │", lines.len() - 15);
    }
    
    if lines.is_empty() {
        println!("│    │ (empty file)                                     │");
    }
    
    println!("└─────────────────────────────────────────────────────────────┘");
}
