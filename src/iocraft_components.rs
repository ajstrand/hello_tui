// Enhanced UI message formatting functions for beautiful console output

// Simple status message function - most commonly needed
pub fn status_message(message: &str) -> String {
    format!("📋 {}", message)
}

// Loading message function
pub fn loading_message(operation: &str, filename: &str) -> String {
    format!("⏳ {} \"{}\"...", operation, filename)
}

// Success message function
pub fn success_message(message: &str, filename: &str, line_count: usize) -> String {
    format!("✅ {}: \"{}\" ({} lines)", message, filename, line_count)
}

// Error message function
pub fn error_message(message: &str, filename: &str, error: &str) -> String {
    format!("❌ {}: \"{}\" - Error: {}", message, filename, error)
}

// File info message function
pub fn file_info_message(filename: &str, file_type: &str, line_count: usize, lint_status: &str) -> String {
    format!("📁 {} | 📄 {} | 📊 {} lines | 🔍 {}", filename, file_type, line_count, lint_status)
}

// Welcome message function
pub fn welcome_message() -> String {
    "🦀 Welcome to TUI Editor with Enhanced JavaScript Linting! 🦀".to_string()
}

// Browser header message
pub fn browser_header_message(directory: &str) -> String {
    format!("📁 File Browser: {}", directory)
}

// Recent files header message
pub fn recent_files_header_message() -> String {
    "🕐 Recent Files".to_string()
}
