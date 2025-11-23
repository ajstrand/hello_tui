use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use std::path::Path;

/// Syntax highlighting utilities for the text editor
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    current_theme: String,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default settings
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        
        Self {
            syntax_set,
            theme_set,
            current_theme: "base16-ocean.dark".to_string(),
        }
    }

    /// Set the theme for syntax highlighting
    pub fn set_theme(&mut self, theme_name: &str) {
        if self.theme_set.themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
        }
    }

    /// Get available themes
    pub fn get_available_themes(&self) -> Vec<&String> {
        self.theme_set.themes.keys().collect()
    }

    /// Detect syntax from file extension
    pub fn detect_syntax_from_filename(&self, filename: &str) -> Option<&str> {
        let path = Path::new(filename);
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension {
                "rs" => Some("Rust"),
                "py" => Some("Python"),
                "js" | "jsx" => Some("JavaScript"),
                "ts" | "tsx" => Some("TypeScript"),
                "html" => Some("HTML"),
                "css" => Some("CSS"),
                "json" => Some("JSON"),
                "xml" => Some("XML"),
                "yaml" | "yml" => Some("YAML"),
                "toml" => Some("TOML"),
                "md" => Some("Markdown"),
                "txt" => Some("Plain Text"),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Highlight a single line of text
    pub fn highlight_line(&self, line: &str, syntax_name: &str) -> String {
        let theme = &self.theme_set.themes[&self.current_theme];
        
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(syntax_name) {
            let mut highlight_lines = HighlightLines::new(syntax, theme);
            let ranges: Vec<(Style, &str)> = highlight_lines.highlight_line(line, &self.syntax_set).unwrap_or_default();
            as_24_bit_terminal_escaped(&ranges[..], false)
        } else {
            line.to_string()
        }
    }

    /// Highlight multiple lines of text
    pub fn highlight_text(&self, text: &str, syntax_name: &str) -> Vec<String> {
        let theme = &self.theme_set.themes[&self.current_theme];
        
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(syntax_name) {
            let mut highlight_lines = HighlightLines::new(syntax, theme);
            let mut highlighted_lines = Vec::new();
            
            for line in LinesWithEndings::from(text) {
                let ranges: Vec<(Style, &str)> = highlight_lines.highlight_line(line, &self.syntax_set).unwrap_or_default();
                highlighted_lines.push(as_24_bit_terminal_escaped(&ranges[..], false));
            }
            
            highlighted_lines
        } else {
            text.lines().map(|s| s.to_string()).collect()
        }
    }

    /// Get syntax name for highlighting
    pub fn get_syntax_name(&self, filename: Option<&str>) -> String {
        if let Some(filename) = filename {
            if let Some(detected) = self.detect_syntax_from_filename(filename) {
                detected.to_string()
            } else {
                "Plain Text".to_string()
            }
        } else {
            "Plain Text".to_string()
        }
    }

    /// Check if syntax highlighting is available for a language
    pub fn is_syntax_supported(&self, syntax_name: &str) -> bool {
        self.syntax_set.find_syntax_by_name(syntax_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_detection() {
        let highlighter = SyntaxHighlighter::new();
        
        assert_eq!(highlighter.detect_syntax_from_filename("test.rs"), Some("Rust"));
        assert_eq!(highlighter.detect_syntax_from_filename("test.py"), Some("Python"));
        assert_eq!(highlighter.detect_syntax_from_filename("test.js"), Some("JavaScript"));
        assert_eq!(highlighter.detect_syntax_from_filename("test.unknown"), None);
    }

    #[test]
    fn test_syntax_support() {
        let highlighter = SyntaxHighlighter::new();
        
        assert!(highlighter.is_syntax_supported("Rust"));
        assert!(highlighter.is_syntax_supported("Python"));
        assert!(highlighter.is_syntax_supported("JavaScript"));
        assert!(!highlighter.is_syntax_supported("NonExistentLanguage"));
    }

    #[test]
    fn test_theme_setting() {
        let mut highlighter = SyntaxHighlighter::new();
        let available_themes = highlighter.get_available_themes();
        
        assert!(!available_themes.is_empty());
        
        // Test setting a valid theme
        if let Some(theme) = available_themes.first() {
            let theme_name = theme.to_string(); // Clone the theme name
            highlighter.set_theme(&theme_name);
            assert_eq!(highlighter.current_theme, theme_name);
        }
    }
}
