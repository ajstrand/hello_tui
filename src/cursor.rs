use crate::keys::Direction;

/// Cursor position and movement management for the text editor
#[derive(Debug, Clone)]
pub struct CursorPosition {
    pub row: usize,
    pub col: usize,
}

impl CursorPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn zero() -> Self {
        Self { row: 0, col: 0 }
    }
}

/// Cursor controller for managing cursor movement and positioning
pub struct CursorController {
    position: CursorPosition,
}

impl CursorController {
    pub fn new() -> Self {
        Self {
            position: CursorPosition::zero(),
        }
    }

    pub fn with_position(row: usize, col: usize) -> Self {
        Self {
            position: CursorPosition::new(row, col),
        }
    }

    /// Get current cursor position
    pub fn position(&self) -> &CursorPosition {
        &self.position
    }

    /// Get current cursor row
    pub fn row(&self) -> usize {
        self.position.row
    }

    /// Get current cursor column
    pub fn col(&self) -> usize {
        self.position.col
    }

    /// Set cursor position directly
    pub fn set_position(&mut self, row: usize, col: usize) {
        self.position.row = row;
        self.position.col = col;
    }

    /// Move cursor in a specific direction
    pub fn move_cursor(&mut self, direction: Direction, lines: &[String]) -> bool {
        let old_position = self.position.clone();
        
        match direction {
            Direction::Up => {
                if self.position.row > 0 {
                    self.position.row -= 1;
                    let line_len = lines[self.position.row].len();
                    self.position.col = self.position.col.min(line_len);
                }
            }
            Direction::Down => {
                if self.position.row < lines.len() - 1 {
                    self.position.row += 1;
                    let line_len = lines[self.position.row].len();
                    self.position.col = self.position.col.min(line_len);
                }
            }
            Direction::Left => {
                if self.position.col > 0 {
                    self.position.col -= 1;
                } else if self.position.row > 0 {
                    self.position.row -= 1;
                    self.position.col = lines[self.position.row].len();
                }
            }
            Direction::Right => {
                let line_len = lines[self.position.row].len();
                if self.position.col < line_len {
                    self.position.col += 1;
                } else if self.position.row < lines.len() - 1 {
                    self.position.row += 1;
                    self.position.col = 0;
                }
            }
        }

        // Return true if position changed
        old_position.row != self.position.row || old_position.col != self.position.col
    }

    /// Move cursor to a specific position with scroll offset adjustment
    pub fn move_to_position(&mut self, row: usize, col: usize, scroll_offset: usize, lines: &[String]) -> bool {
        let old_position = self.position.clone();
        
        // Adjust for scroll offset
        let actual_row = row + scroll_offset;
        
        if actual_row < lines.len() {
            self.position.row = actual_row;
            let line_len = lines[self.position.row].len();
            self.position.col = col.min(line_len);
        }

        // Return true if position changed
        old_position.row != self.position.row || old_position.col != self.position.col
    }

    /// Move cursor to the start of the current line
    pub fn move_to_line_start(&mut self) -> bool {
        let old_col = self.position.col;
        self.position.col = 0;
        old_col != 0
    }

    /// Move cursor to the end of the current line
    pub fn move_to_line_end(&mut self, lines: &[String]) -> bool {
        let old_col = self.position.col;
        if self.position.row < lines.len() {
            self.position.col = lines[self.position.row].len();
        }
        old_col != self.position.col
    }

    /// Move cursor to the start of the document
    pub fn move_to_document_start(&mut self) -> bool {
        let old_position = self.position.clone();
        self.position.row = 0;
        self.position.col = 0;
        old_position.row != 0 || old_position.col != 0
    }

    /// Move cursor to the end of the document
    pub fn move_to_document_end(&mut self, lines: &[String]) -> bool {
        let old_position = self.position.clone();
        self.position.row = if lines.is_empty() { 0 } else { lines.len() - 1 };
        self.position.col = if lines.is_empty() { 0 } else { lines[self.position.row].len() };
        old_position.row != self.position.row || old_position.col != self.position.col
    }

    /// Validate and adjust cursor position to ensure it's within bounds
    pub fn validate_position(&mut self, lines: &[String]) -> bool {
        let old_position = self.position.clone();
        
        // Ensure row is within bounds
        if self.position.row >= lines.len() {
            self.position.row = if lines.is_empty() { 0 } else { lines.len() - 1 };
        }

        // Ensure column is within bounds for the current line
        if self.position.row < lines.len() {
            let line_len = lines[self.position.row].len();
            if self.position.col > line_len {
                self.position.col = line_len;
            }
        }

        // Return true if position was adjusted
        old_position.row != self.position.row || old_position.col != self.position.col
    }

    /// Get cursor position relative to viewport (considering scroll offset)
    pub fn viewport_position(&self, scroll_offset: usize) -> CursorPosition {
        CursorPosition::new(
            self.position.row.saturating_sub(scroll_offset),
            self.position.col,
        )
    }

    /// Check if cursor is visible within the viewport
    pub fn is_visible_in_viewport(&self, scroll_offset: usize, viewport_height: usize) -> bool {
        self.position.row >= scroll_offset && 
        self.position.row < scroll_offset + viewport_height
    }

    /// Adjust scroll offset to keep cursor visible
    pub fn adjust_scroll_for_visibility(&self, scroll_offset: usize, viewport_height: usize, total_lines: usize) -> usize {
        let mut new_scroll_offset = scroll_offset;
        
        // If cursor is above viewport, scroll up
        if self.position.row < scroll_offset {
            new_scroll_offset = self.position.row;
        }
        // If cursor is below viewport, scroll down
        else if self.position.row >= scroll_offset + viewport_height {
            new_scroll_offset = self.position.row.saturating_sub(viewport_height - 1);
        }
        
        // Ensure scroll offset doesn't exceed document bounds
        let max_scroll = total_lines.saturating_sub(viewport_height);
        new_scroll_offset.min(max_scroll)
    }

    /// Get cursor position for mouse click (adjusted for scroll)
    pub fn position_for_mouse(&self, scroll_offset: usize) -> (usize, usize) {
        (self.position.row.saturating_sub(scroll_offset), self.position.col)
    }
}

impl Default for CursorController {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for cursor-related operations
pub mod cursor_utils {
    use super::*;

    /// Calculate the preferred column for vertical movement
    pub fn calculate_preferred_column(current_col: usize, target_line: &str) -> usize {
        current_col.min(target_line.len())
    }

    /// Find the next word boundary for cursor navigation
    pub fn find_next_word_boundary(line: &str, start_col: usize) -> usize {
        let chars: Vec<char> = line.chars().collect();
        let mut col = start_col;
        
        // Skip current word
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        
        // Skip whitespace
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }
        
        col.min(chars.len())
    }

    /// Find the previous word boundary for cursor navigation
    pub fn find_prev_word_boundary(line: &str, start_col: usize) -> usize {
        let chars: Vec<char> = line.chars().collect();
        
        if start_col == 0 || chars.is_empty() {
            return 0;
        }
        
        let mut col = start_col.min(chars.len()).saturating_sub(1);
        
        // Skip whitespace
        while col > 0 && chars[col].is_whitespace() {
            col = col.saturating_sub(1);
        }
        
        // Skip current word
        while col > 0 && !chars[col].is_whitespace() {
            col = col.saturating_sub(1);
        }
        
        // If we stopped on whitespace, move to the next character
        if col > 0 && chars[col].is_whitespace() {
            col += 1;
        }
        
        col
    }

    /// Check if cursor is at the start of a line
    pub fn is_at_line_start(cursor: &CursorPosition) -> bool {
        cursor.col == 0
    }

    /// Check if cursor is at the end of a line
    pub fn is_at_line_end(cursor: &CursorPosition, lines: &[String]) -> bool {
        cursor.row < lines.len() && cursor.col == lines[cursor.row].len()
    }

    /// Check if cursor is at the start of the document
    pub fn is_at_document_start(cursor: &CursorPosition) -> bool {
        cursor.row == 0 && cursor.col == 0
    }

    /// Check if cursor is at the end of the document
    pub fn is_at_document_end(cursor: &CursorPosition, lines: &[String]) -> bool {
        if lines.is_empty() {
            return cursor.row == 0 && cursor.col == 0;
        }
        cursor.row == lines.len() - 1 && cursor.col == lines[cursor.row].len()
    }
}
