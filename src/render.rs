use std::io::{self, stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
};
use crate::{syntax, linter, mouse};

/// Rendering module for the text editor
pub struct EditorRenderer;

impl EditorRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Main render function for the editor
    pub fn render_editor(
        &self,
        lines: &[String],
        cursor_row: usize,
        cursor_col: usize,
        filename: Option<&str>,
        modified: bool,
        status_message: &str,
        syntax_highlighter: &syntax::SyntaxHighlighter,
        syntax_enabled: bool,
        scroll_offset: usize,
        lint_issues: &[linter::LintIssue],
        linter: &linter::Linter,
        text_selection: Option<&mouse::TextSelection>,
    ) -> io::Result<()> {
        // Move to top and hide cursor, but don't clear entire screen to reduce flicker
        execute!(stdout(), MoveTo(0, 0), Hide)?;

        let syntax_name = if syntax_enabled {
            syntax_highlighter.get_syntax_name(filename)
        } else {
            "Plain Text".to_string()
        };

        let terminal_size = crossterm::terminal::size()?;
        let terminal_width = terminal_size.0 as usize;
        let terminal_height = terminal_size.1 as usize;
        let visible_lines = terminal_height.saturating_sub(3); // Reserve space for header and status
        let line_number_width = 4; // Fixed width for cleaner alignment
        let content_width = terminal_width.saturating_sub(line_number_width + 2); // Account for line numbers and space

        // Render header
        self.render_header(filename, modified, &syntax_name, syntax_enabled, terminal_width)?;

        // Render content lines
        self.render_content_lines(
            lines,
            cursor_row,
            cursor_col,
            scroll_offset,
            visible_lines,
            content_width,
            syntax_highlighter,
            syntax_enabled,
            &syntax_name,
            lint_issues,
            linter,
            text_selection,
        )?;

        // Render status line
        self.render_status_line(
            lines,
            cursor_row,
            cursor_col,
            status_message,
            lines.len(),
            &syntax_name,
            lint_issues,
            linter,
            text_selection,
            terminal_width,
        )?;

        stdout().flush()?;
        Ok(())
    }

    /// Render the header with file info
    fn render_header(
        &self,
        filename: Option<&str>,
        modified: bool,
        syntax_name: &str,
        syntax_enabled: bool,
        terminal_width: usize,
    ) -> io::Result<()> {
        let filename = filename.unwrap_or("[No file]");
        let modified_indicator = if modified { " ●" } else { "" };
        let syntax_indicator = if syntax_enabled { 
            format!(" [{}]", syntax_name) 
        } else { 
            " [Plain Text]".to_string() 
        };
        
        let header = format!("📝 {}{}{}", filename, modified_indicator, syntax_indicator);
        let padding = terminal_width.saturating_sub(header.chars().count()).min(terminal_width);
        print!("{}{}{}\x1b[0m\r\n",
            "\x1b[44;37m", // Blue background, white text
            header,
            " ".repeat(padding)
        );
        
        Ok(())
    }

    /// Render the main content lines
    fn render_content_lines(
        &self,
        lines: &[String],
        cursor_row: usize,
        cursor_col: usize,
        scroll_offset: usize,
        visible_lines: usize,
        content_width: usize,
        syntax_highlighter: &syntax::SyntaxHighlighter,
        syntax_enabled: bool,
        syntax_name: &str,
        lint_issues: &[linter::LintIssue],
        linter: &linter::Linter,
        text_selection: Option<&mouse::TextSelection>,
    ) -> io::Result<()> {
        // Clear each line as we render to prevent artifacts
        for (_, actual_row) in (scroll_offset..scroll_offset + visible_lines)
            .enumerate()
            .take_while(|(_, actual_row)| *actual_row < lines.len())
        {
            // Clear current line
            print!("\x1b[2K");
            
            let line = &lines[actual_row];
            
            // Render line number with lint indicator
            self.render_line_number(actual_row, cursor_row, lint_issues, linter)?;
            
            // Content area - handle text properly
            let display_line = if line.len() > content_width {
                format!("{}…", &line[..content_width.saturating_sub(1)])
            } else {
                line.clone()
            };
            
            // Check if this line has selection
            let line_has_selection = text_selection
                .map(|sel| sel.start_row <= actual_row && actual_row <= sel.end_row)
                .unwrap_or(false);
            
            if actual_row == cursor_row && !line_has_selection {
                // Current line with cursor
                self.render_current_line(
                    &display_line,
                    cursor_col,
                    content_width,
                    syntax_highlighter,
                    syntax_enabled,
                    syntax_name,
                )?;
            } else if line_has_selection {
                // Render line with selection highlighting
                self.render_line_with_selection(
                    &display_line,
                    actual_row,
                    cursor_row,
                    cursor_col,
                    syntax_name,
                    content_width,
                    text_selection.unwrap(),
                    syntax_highlighter,
                    syntax_enabled,
                )?;
            } else {
                // Regular line - simple and clean
                if syntax_enabled {
                    let highlighted = syntax_highlighter.highlight_line(&display_line, syntax_name);
                    print!("{}", highlighted);
                } else {
                    print!("{}", display_line);
                }
            }
            
            // Move to next line
            print!("\r\n");
        }

        // Render empty lines
        self.render_empty_lines(scroll_offset, visible_lines, lines.len())?;

        Ok(())
    }

    /// Render line number with lint indicator
    fn render_line_number(
        &self,
        actual_row: usize,
        cursor_row: usize,
        lint_issues: &[linter::LintIssue],
        linter: &linter::Linter,
    ) -> io::Result<()> {
        let is_current_line = actual_row == cursor_row;
        let lint_indicator = self.get_lint_indicator_for_line(actual_row, lint_issues, linter);
        
        if is_current_line {
            print!("\x1b[43;30m{:>3}{}\x1b[0m ", actual_row + 1, if lint_indicator.is_empty() { " " } else { &lint_indicator });
        } else {
            print!("\x1b[100;37m{:>3}{}\x1b[0m ", actual_row + 1, if lint_indicator.is_empty() { " " } else { &lint_indicator });
        }
        
        Ok(())
    }

    /// Render current line with cursor
    fn render_current_line(
        &self,
        display_line: &str,
        cursor_col: usize,
        content_width: usize,
        syntax_highlighter: &syntax::SyntaxHighlighter,
        syntax_enabled: bool,
        syntax_name: &str,
    ) -> io::Result<()> {
        // Current line with cursor - add subtle background
        print!("\x1b[48;5;235m"); // Dark gray background
        
        let cursor_pos = cursor_col.min(display_line.len());
        let (before_cursor, after_cursor) = display_line.split_at(cursor_pos);

        // Apply syntax highlighting
        if syntax_enabled {
            let highlighted_before = syntax_highlighter.highlight_line(before_cursor, syntax_name);
            print!("{}", highlighted_before);
        } else {
            print!("{}", before_cursor);
        }
        
        // Modern cursor - vertical line
        print!("\x1b[48;5;220;30m│\x1b[48;5;235m");
        
        if !after_cursor.is_empty() {
            if syntax_enabled {
                let highlighted_after = syntax_highlighter.highlight_line(after_cursor, syntax_name);
                print!("{}", highlighted_after);
            } else {
                print!("{}", after_cursor);
            }
        }
        
        // Fill rest of line with background
        let used_width = before_cursor.chars().count() + 1 + after_cursor.chars().count();
        if used_width < content_width {
            print!("{}", " ".repeat(content_width - used_width));
        }
        print!("\x1b[0m"); // Reset background
        
        Ok(())
    }

    /// Render line with selection highlighting
    fn render_line_with_selection(
        &self,
        line: &str,
        row: usize,
        cursor_row: usize,
        cursor_col: usize,
        syntax_name: &str,
        content_width: usize,
        selection: &mouse::TextSelection,
        syntax_highlighter: &syntax::SyntaxHighlighter,
        syntax_enabled: bool,
    ) -> io::Result<()> {
        let chars: Vec<char> = line.chars().collect();
        let max_chars = chars.len().min(content_width);
        
        for (col, ch) in chars.iter().take(max_chars).enumerate() {
            let is_selected = selection.contains(row, col);
            let is_cursor = row == cursor_row && col == cursor_col;
            
            if is_cursor && is_selected {
                // Cursor within selection - bright highlight
                print!("\x1b[48;5;220;30m{}\x1b[0m", ch); // Yellow background, black text
            } else if is_cursor {
                // Cursor outside selection - vertical line cursor
                print!("\x1b[48;5;220;30m│\x1b[0m"); // Yellow cursor line
                print!("\x1b[48;5;220;30m{}\x1b[0m", ch); // Character with cursor background
            } else if is_selected {
                // Selected text - blue highlight
                print!("\x1b[48;5;68;37m{}\x1b[0m", ch); // Blue background, white text
            } else {
                // Normal text
                if syntax_enabled {
                    let char_str = ch.to_string();
                    let highlighted = syntax_highlighter.highlight_line(&char_str, syntax_name);
                    print!("{}", highlighted);
                } else {
                    print!("{}", ch);
                }
            }
        }
        
        // Handle cursor at end of line
        if row == cursor_row && cursor_col >= max_chars {
            print!("\x1b[48;5;220;30m│\x1b[0m"); // Cursor at end of line
        }
        
        // Show truncation indicator if line was cut off
        if chars.len() > content_width {
            print!("\x1b[2m…\x1b[0m"); // Dim ellipsis
        }
        
        Ok(())
    }

    /// Render empty lines (tilde indicators)
    fn render_empty_lines(
        &self,
        scroll_offset: usize,
        visible_lines: usize,
        total_lines: usize,
    ) -> io::Result<()> {
        let lines_rendered = (scroll_offset..scroll_offset + visible_lines)
            .take_while(|actual_row| *actual_row < total_lines)
            .count();
        
        let terminal_height = crossterm::terminal::size()?.1 as usize;
        for _ in lines_rendered..visible_lines.min(terminal_height.saturating_sub(3)) {
            print!("\x1b[2K");
            print!("\x1b[100;37m   ~ \x1b[0m");
            print!("\r\n");
        }
        
        Ok(())
    }

    /// Render status line
    fn render_status_line(
        &self,
        lines: &[String],
        cursor_row: usize,
        cursor_col: usize,
        status_message: &str,
        line_count: usize,
        syntax_name: &str,
        lint_issues: &[linter::LintIssue],
        linter: &linter::Linter,
        text_selection: Option<&mouse::TextSelection>,
        terminal_width: usize,
    ) -> io::Result<()> {
        // Enhanced status line with better formatting
        print!("\x1b[2K"); // Clear status line
        let cursor_info = format!("Ln {}, Col {}", cursor_row + 1, cursor_col + 1);
        let selection_info = if let Some(sel) = text_selection {
            let selected_text = sel.get_selected_text(lines);
            format!(" | {} chars selected", selected_text.chars().count())
        } else {
            String::new()
        };
        
        let left_status = format!("{}{}", cursor_info, selection_info);
        
        // Add lint information to right status
        let lint_info = if linter.is_enabled() && !lint_issues.is_empty() {
            let (errors, warnings, info, hints) = linter.get_issue_counts(lint_issues);
            let mut parts = Vec::new();
            if errors > 0 { parts.push(format!("{}❌{}", errors, if errors > 1 { "" } else { "" })); }
            if warnings > 0 { parts.push(format!("{}⚠️{}", warnings, if warnings > 1 { "" } else { "" })); }
            if info > 0 { parts.push(format!("{}ℹ️{}", info, if info > 1 { "" } else { "" })); }
            if hints > 0 { parts.push(format!("{}💡{}", hints, if hints > 1 { "" } else { "" })); }
            if parts.is_empty() { String::new() } else { format!(" | {}", parts.join(" ")) }
        } else if linter.is_enabled() {
            " | ✅".to_string()
        } else {
            String::new()
        };
        
        let right_status = format!("{} | {} lines{}", syntax_name, line_count, lint_info);
        let center_message = status_message;
        
        // Status line with dark background
        print!("\x1b[48;5;235;37m{}", left_status);
        
        // Center the message
        let left_width = left_status.chars().count();
        let right_width = right_status.chars().count();
        let center_width = center_message.chars().count();
        let available_width = terminal_width.saturating_sub(left_width + right_width);
        
        if center_width < available_width {
            let center_padding = (available_width - center_width) / 2;
            print!("{}", " ".repeat(center_padding));
            print!("{}", center_message);
            print!("{}", " ".repeat(center_padding));
        } else {
            print!(" {} ", center_message);
        }
        
        print!("{}", right_status);
        
        // Fill remaining space
        let used = left_width + center_width + right_width + if center_width < available_width { available_width - center_width } else { 2 };
        if used < terminal_width {
            print!("{}", " ".repeat(terminal_width - used));
        }
        print!("\x1b[0m\r"); // Reset formatting and return to start of line

        Ok(())
    }

    /// Get lint indicator for a specific line
    fn get_lint_indicator_for_line(
        &self,
        line: usize,
        lint_issues: &[linter::LintIssue],
        linter: &linter::Linter,
    ) -> String {
        if !linter.is_enabled() {
            return String::new();
        }
        
        // Find issues for this line (1-based indexing)
        let line_number = line + 1;
        let issues_for_line: Vec<&linter::LintIssue> = lint_issues
            .iter()
            .filter(|issue| issue.line == line_number)
            .collect();
            
        if issues_for_line.is_empty() {
            return String::new();
        }
        
        // Find the most severe issue for this line
        let mut most_severe = &linter::LintSeverity::Hint;
        for issue in &issues_for_line {
            most_severe = match (&most_severe, &issue.severity) {
                (linter::LintSeverity::Error, _) => most_severe, // Error is already most severe
                (_, linter::LintSeverity::Error) => &issue.severity,
                (linter::LintSeverity::Warning, linter::LintSeverity::Warning) => most_severe,
                (_, linter::LintSeverity::Warning) => &issue.severity,
                (linter::LintSeverity::Info, linter::LintSeverity::Info) => most_severe,
                (linter::LintSeverity::Hint, linter::LintSeverity::Info) => &issue.severity,
                _ => most_severe,
            };
        }
        
        match most_severe {
            linter::LintSeverity::Error => "🔴".to_string(),
            linter::LintSeverity::Warning => "🟡".to_string(),
            linter::LintSeverity::Info => "ℹ️".to_string(),
            linter::LintSeverity::Hint => "💡".to_string(),
        }
    }
}

impl Default for EditorRenderer {
    fn default() -> Self {
        Self::new()
    }
}
