use std::io;
use std::path::Path;
use crate::file_io::FileIO;
use crate::iocraft_components::{loading_message, success_message, error_message, file_info_message, browser_header_message, recent_files_header_message};

/// IOCraft-powered file I/O handler with beautiful UI components
pub struct IOCraftFileHandler {
    current_file: Option<String>,
    last_operation: String,
    operation_success: bool,
}

impl Default for IOCraftFileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl IOCraftFileHandler {
    /// Create a new IOCraft file handler
    pub fn new() -> Self {
        Self {
            current_file: None,
            last_operation: "Ready".to_string(),
            operation_success: true,
        }
    }

    /// Load a file with IOCraft UI feedback
    pub fn load_file(&mut self, filename: &str) -> io::Result<Vec<String>> {
        self.display_loading_message("Loading file", filename);
        
        match FileIO::load_file(filename) {
            Ok(lines) => {
                self.current_file = Some(filename.to_string());
                self.last_operation = format!("Loaded: {}", filename);
                self.operation_success = true;
                self.display_success_message("File loaded successfully", filename, lines.len());
                Ok(lines)
            }
            Err(e) => {
                self.last_operation = format!("Error loading {}: {}", filename, e);
                self.operation_success = false;
                self.display_error_message("Failed to load file", filename, &e.to_string());
                Err(e)
            }
        }
    }

    /// Save a file with IOCraft UI feedback
    pub fn save_file(&mut self, filename: &str, lines: &[String]) -> io::Result<()> {
        self.display_loading_message("Saving file", filename);
        
        match FileIO::save_file(filename, lines) {
            Ok(()) => {
                self.current_file = Some(filename.to_string());
                self.last_operation = format!("Saved: {}", filename);
                self.operation_success = true;
                self.display_success_message("File saved successfully", filename, lines.len());
                Ok(())
            }
            Err(e) => {
                self.last_operation = format!("Error saving {}: {}", filename, e);
                self.operation_success = false;
                self.display_error_message("Failed to save file", filename, &e.to_string());
                Err(e)
            }
        }
    }

    /// Create a new file with IOCraft UI
    pub fn create_new_file(&mut self, filename: &str) -> io::Result<()> {
        self.display_loading_message("Creating new file", filename);
        
        let empty_content = vec!["".to_string()];
        match self.save_file(filename, &empty_content) {
            Ok(()) => {
                self.display_success_message("New file created", filename, 0);
                Ok(())
            }
            Err(e) => {
                self.display_error_message("Failed to create file", filename, &e.to_string());
                Err(e)
            }
        }
    }

    /// Display a beautiful file info panel
    pub fn display_file_info(&self, filename: &str, lines: &[String]) {
        let path = Path::new(filename);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("txt");
        let status = if self.operation_success { "✅ OK" } else { "❌ Error" };
        
        println!("{}", file_info_message(filename, extension, lines.len(), status));
    }

    /// Display a loading message with IOCraft styling
    fn display_loading_message(&self, operation: &str, filename: &str) {
        println!("{}", loading_message(operation, filename));
    }

    /// Display a success message with IOCraft styling
    fn display_success_message(&self, message: &str, filename: &str, line_count: usize) {
        println!("{}", success_message(message, filename, line_count));
    }

    /// Display an error message with IOCraft styling
    fn display_error_message(&self, message: &str, filename: &str, error: &str) {
        println!("{}", error_message(message, filename, error));
    }

    /// Get the current file path
    pub fn get_current_file(&self) -> Option<&String> {
        self.current_file.as_ref()
    }

    /// Get the last operation status
    pub fn get_last_operation(&self) -> &String {
        &self.last_operation
    }

    /// Check if the last operation was successful
    pub fn is_last_operation_successful(&self) -> bool {
        self.operation_success
    }

    /// Display a beautiful file browser-style interface
    pub fn display_file_browser(&self, directory: &str) -> io::Result<Vec<String>> {
        use std::fs;
        
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│ {:<59} │", browser_header_message(directory));
        println!("├─────────────────────────────────────────────────────────────┤");
        
        let mut files = Vec::new();
        
        match fs::read_dir(directory) {
            Ok(entries) => {
                for (index, entry) in entries.enumerate() {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown");
                        
                        let icon = if path.is_dir() {
                            "📁"
                        } else {
                            match path.extension().and_then(|ext| ext.to_str()) {
                                Some("rs") => "🦀",
                                Some("js") | Some("jsx") => "🟨",
                                Some("ts") | Some("tsx") => "🔷",
                                Some("py") => "🐍",
                                Some("html") => "🌐",
                                Some("css") => "🎨",
                                Some("json") => "📋",
                                Some("md") => "📝",
                                Some("txt") => "📄",
                                _ => "📄",
                            }
                        };
                        
                        println!("│ {:<2} {} {:<51} │", index + 1, icon, name);
                        files.push(name.to_string());
                        
                        if index >= 10 { // Limit display to first 10 items
                            println!("│    ... and more files ...                               │");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("│ ❌ Error reading directory: {:<32} │", e.to_string());
            }
        }
        
        println!("└─────────────────────────────────────────────────────────────┘");
        Ok(files)
    }

    /// Display recent files with IOCraft styling
    pub fn display_recent_files(&self, recent_files: &[String]) {
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│ {:<59} │", recent_files_header_message());
        println!("├─────────────────────────────────────────────────────────────┤");
        
        if recent_files.is_empty() {
            println!("│ No recent files                                             │");
        } else {
            for (index, file) in recent_files.iter().take(5).enumerate() {
                let path = Path::new(file);
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(file);
                println!("│ {:<2} 📄 {:<52} │", index + 1, name);
            }
        }
        
        println!("└─────────────────────────────────────────────────────────────┘");
    }

    /// Display a welcome screen with IOCraft styling
    pub fn display_welcome_screen(&self) {
        println!("┌─────────────────────────────────────────────────────────────┐");
        println!("│                                                             │");
        println!("│        🎉 Welcome to IOCraft Text Editor! 🎉                │");
        println!("│                                                             │");
        println!("│  A beautiful terminal text editor with syntax highlighting  │");
        println!("│                                                             │");
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ Quick Actions:                                              │");
        println!("│                                                             │");
        println!("│  📝 Create new file    → cargo run --bin quick_editor      │");
        println!("│  📂 Open existing file → cargo run --bin quick_editor <file>│");
        println!("│  🎮 Interactive mode   → cargo run --bin interactive_editor│");
        println!("│  🔄 Demo mode          → cargo run --bin editor_demo       │");
        println!("│                                                             │");
        println!("├─────────────────────────────────────────────────────────────┤");
        println!("│ Features:                                                   │");
        println!("│                                                             │");
        println!("│  ✅ Syntax highlighting for multiple languages             │");
        println!("│  ✅ File I/O operations with beautiful UI                  │");
        println!("│  ✅ Interactive editing with Sublime Text-style controls  │");
        println!("│  ✅ Full mouse support (click, drag, scroll, select)      │");
        println!("│  ✅ Crossterm integration for smooth terminal experience   │");
        println!("│                                                             │");
        println!("└─────────────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_iocraft_file_handler_creation() {
        let handler = IOCraftFileHandler::new();
        assert!(handler.get_current_file().is_none());
        assert_eq!(handler.get_last_operation(), "Ready");
        assert!(handler.is_last_operation_successful());
    }

    #[test]
    fn test_file_operations() {
        let mut handler = IOCraftFileHandler::new();
        let test_file = "test_iocraft.txt";
        let test_content = vec!["Hello".to_string(), "World".to_string()];

        // Test save
        assert!(handler.save_file(test_file, &test_content).is_ok());
        assert_eq!(handler.get_current_file().unwrap(), test_file);
        assert!(handler.is_last_operation_successful());

        // Test load
        let loaded_content = handler.load_file(test_file).unwrap();
        assert_eq!(loaded_content, test_content);

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_error_handling() {
        let mut handler = IOCraftFileHandler::new();
        
        // Try to load a non-existent file
        let result = handler.load_file("/non/existent/file.txt");
        assert!(result.is_err());
        assert!(!handler.is_last_operation_successful());
        assert!(handler.get_last_operation().contains("Error"));
    }
}
