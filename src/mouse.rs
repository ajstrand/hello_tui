use std::io;
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};

/// Mouse control handler for the text editor
pub struct MouseController {
    last_click_row: usize,
    last_click_col: usize,
    drag_start_row: Option<usize>,
    drag_start_col: Option<usize>,
    selection_active: bool,
    double_click_threshold_ms: u64,
    last_click_time: std::time::Instant,
}

impl Default for MouseController {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseController {
    /// Create a new mouse controller
    pub fn new() -> Self {
        Self {
            last_click_row: 0,
            last_click_col: 0,
            drag_start_row: None,
            drag_start_col: None,
            selection_active: false,
            double_click_threshold_ms: 500,
            last_click_time: std::time::Instant::now(),
        }
    }

    /// Handle mouse events and return the appropriate action
    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> MouseAction {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let row = mouse_event.row as usize;
                let col = mouse_event.column as usize;
                
                let now = std::time::Instant::now();
                let is_double_click = now.duration_since(self.last_click_time).as_millis() 
                    < self.double_click_threshold_ms as u128
                    && self.last_click_row == row 
                    && (self.last_click_col as i32 - col as i32).abs() <= 2;

                self.last_click_time = now;
                self.last_click_row = row;
                self.last_click_col = col;

                if is_double_click {
                    MouseAction::DoubleClick { row, col }
                } else {
                    self.drag_start_row = Some(row);
                    self.drag_start_col = Some(col);
                    self.selection_active = false;
                    MouseAction::Click { row, col }
                }
            }
            
            MouseEventKind::Up(MouseButton::Left) => {
                if let (Some(start_row), Some(start_col)) = (self.drag_start_row, self.drag_start_col) {
                    let end_row = mouse_event.row as usize;
                    let end_col = mouse_event.column as usize;
                    
                    self.drag_start_row = None;
                    self.drag_start_col = None;
                    
                    if start_row != end_row || start_col != end_col {
                        self.selection_active = true;
                        MouseAction::DragEnd { 
                            start_row, 
                            start_col, 
                            end_row, 
                            end_col 
                        }
                    } else {
                        MouseAction::None
                    }
                } else {
                    MouseAction::None
                }
            }
            
            MouseEventKind::Drag(MouseButton::Left) => {
                if let (Some(start_row), Some(start_col)) = (self.drag_start_row, self.drag_start_col) {
                    let current_row = mouse_event.row as usize;
                    let current_col = mouse_event.column as usize;
                    
                    MouseAction::Drag { 
                        start_row, 
                        start_col, 
                        current_row, 
                        current_col 
                    }
                } else {
                    MouseAction::None
                }
            }
            
            MouseEventKind::Down(MouseButton::Right) => {
                MouseAction::RightClick { 
                    row: mouse_event.row as usize, 
                    col: mouse_event.column as usize 
                }
            }
            
            MouseEventKind::ScrollUp => {
                MouseAction::ScrollUp
            }
            
            MouseEventKind::ScrollDown => {
                MouseAction::ScrollDown
            }
            
            _ => MouseAction::None,
        }
    }

    /// Check if there's an active selection
    pub fn has_active_selection(&self) -> bool {
        self.selection_active
    }

    /// Clear the current selection
    pub fn clear_selection(&mut self) {
        self.selection_active = false;
        self.drag_start_row = None;
        self.drag_start_col = None;
    }

    /// Get the last click position
    pub fn get_last_click_position(&self) -> (usize, usize) {
        (self.last_click_row, self.last_click_col)
    }
}

/// Actions that can result from mouse events
#[derive(Debug, Clone, PartialEq)]
pub enum MouseAction {
    /// Single click at position
    Click { row: usize, col: usize },
    
    /// Double click at position (word selection)
    DoubleClick { row: usize, col: usize },
    
    /// Right click at position (context menu)
    RightClick { row: usize, col: usize },
    
    /// Mouse drag in progress
    Drag { 
        start_row: usize, 
        start_col: usize, 
        current_row: usize, 
        current_col: usize 
    },
    
    /// Mouse drag finished (text selection)
    DragEnd { 
        start_row: usize, 
        start_col: usize, 
        end_row: usize, 
        end_col: usize 
    },
    
    /// Scroll wheel up
    ScrollUp,
    
    /// Scroll wheel down
    ScrollDown,
    
    /// No action needed
    None,
}

/// Helper struct for text selection
#[derive(Debug, Clone, PartialEq)]
pub struct TextSelection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl TextSelection {
    /// Create a new text selection
    pub fn new(start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Self {
        // Ensure start comes before end
        if start_row < end_row || (start_row == end_row && start_col <= end_col) {
            Self { start_row, start_col, end_row, end_col }
        } else {
            Self { 
                start_row: end_row, 
                start_col: end_col, 
                end_row: start_row, 
                end_col: start_col 
            }
        }
    }

    /// Check if a position is within this selection
    pub fn contains(&self, row: usize, col: usize) -> bool {
        if row < self.start_row || row > self.end_row {
            return false;
        }
        
        if row == self.start_row && row == self.end_row {
            col >= self.start_col && col < self.end_col
        } else if row == self.start_row {
            col >= self.start_col
        } else if row == self.end_row {
            col < self.end_col
        } else {
            true
        }
    }

    /// Get the selected text from the given lines
    pub fn get_selected_text(&self, lines: &[String]) -> String {
        if self.start_row >= lines.len() {
            return String::new();
        }

        if self.start_row == self.end_row {
            // Single line selection
            let line = &lines[self.start_row];
            let start = self.start_col.min(line.len());
            let end = self.end_col.min(line.len());
            if start < end {
                line[start..end].to_string()
            } else {
                String::new()
            }
        } else {
            // Multi-line selection
            let mut result = String::new();
            
            for row in self.start_row..=self.end_row.min(lines.len() - 1) {
                let line = &lines[row];
                
                if row == self.start_row {
                    let start = self.start_col.min(line.len());
                    result.push_str(&line[start..]);
                } else if row == self.end_row {
                    let end = self.end_col.min(line.len());
                    result.push_str(&line[..end]);
                } else {
                    result.push_str(line);
                }
                
                if row < self.end_row {
                    result.push('\n');
                }
            }
            
            result
        }
    }
}

/// Word boundary detection for double-click word selection
pub fn find_word_boundaries(line: &str, col: usize) -> (usize, usize) {
    if line.is_empty() || col >= line.len() {
        return (col, col);
    }

    let chars: Vec<char> = line.chars().collect();
    let mut start = col;
    let mut end = col;

    // Find start of word
    while start > 0 {
        let ch = chars[start - 1];
        if ch.is_alphanumeric() || ch == '_' {
            start -= 1;
        } else {
            break;
        }
    }

    // Find end of word
    while end < chars.len() {
        let ch = chars[end];
        if ch.is_alphanumeric() || ch == '_' {
            end += 1;
        } else {
            break;
        }
    }

    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_controller_creation() {
        let controller = MouseController::new();
        assert!(!controller.has_active_selection());
        assert_eq!(controller.get_last_click_position(), (0, 0));
    }

    #[test]
    fn test_text_selection() {
        let selection = TextSelection::new(1, 5, 3, 10);
        assert!(selection.contains(2, 0));
        assert!(selection.contains(1, 5));
        assert!(!selection.contains(1, 4));
        assert!(!selection.contains(3, 10));
    }

    #[test]
    fn test_word_boundaries() {
        let line = "hello world test";
        assert_eq!(find_word_boundaries(line, 7), (6, 11)); // "world"
        assert_eq!(find_word_boundaries(line, 0), (0, 5));   // "hello"
        assert_eq!(find_word_boundaries(line, 12), (12, 16)); // "test"
    }

    #[test]
    fn test_text_selection_content() {
        let lines = vec![
            "first line".to_string(),
            "second line".to_string(),
            "third line".to_string(),
        ];
        
        // Single line selection
        let selection = TextSelection::new(1, 2, 1, 8);
        assert_eq!(selection.get_selected_text(&lines), "cond l");
        
        // Multi-line selection
        let selection = TextSelection::new(0, 6, 2, 5);
        assert_eq!(selection.get_selected_text(&lines), "line\nsecond line\nthird");
    }

    #[test]
    fn test_selection_ordering() {
        // Test that selection automatically orders start before end
        let selection = TextSelection::new(3, 10, 1, 5);
        assert_eq!(selection.start_row, 1);
        assert_eq!(selection.start_col, 5);
        assert_eq!(selection.end_row, 3);
        assert_eq!(selection.end_col, 10);
    }
}
