use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

/// Direction for cursor movement
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Key action that should be performed
#[derive(Debug)]
pub enum KeyAction {
    // File operations
    Quit,
    Save,
    Open,
    NewFile,
    
    // Editing operations
    InsertChar(char),
    InsertNewline,
    DeleteBackward,
    DeleteForward,
    
    // Navigation
    MoveCursor(Direction),
    GoToLineStart,
    GoToLineEnd,
    GoToDocumentStart,
    GoToDocumentEnd,
    
    // Line operations
    DuplicateLine,
    DeleteLine,
    
    // View operations
    ToggleSyntaxHighlighting,
    ToggleLinting,
    GoToLine,
    
    // No action
    None,
}

/// Key handler for processing keyboard events
pub struct KeyHandler;

impl KeyHandler {
    pub fn new() -> Self {
        Self
    }
    
    /// Process a key event and return the corresponding action
    pub fn handle_key_event(&self, key_event: KeyEvent) -> KeyAction {
        match (key_event.modifiers.contains(KeyModifiers::CONTROL), key_event.code) {
            // Ctrl+Q - Quit
            (true, KeyCode::Char('q')) => KeyAction::Quit,
            
            // Ctrl+S - Save
            (true, KeyCode::Char('s')) => KeyAction::Save,
            
            // Ctrl+O - Open file
            (true, KeyCode::Char('o')) => KeyAction::Open,
            
            // Ctrl+H - Toggle syntax highlighting
            (true, KeyCode::Char('h')) => KeyAction::ToggleSyntaxHighlighting,
            
            // Ctrl+E - Toggle linting
            (true, KeyCode::Char('e')) => KeyAction::ToggleLinting,
            
            // Ctrl+N - New file
            (true, KeyCode::Char('n')) => KeyAction::NewFile,
            
            // Ctrl+Home - Go to start of document
            (true, KeyCode::Home) => KeyAction::GoToDocumentStart,
            
            // Ctrl+End - Go to end of document
            (true, KeyCode::End) => KeyAction::GoToDocumentEnd,
            
            // Ctrl+L - Go to line
            (true, KeyCode::Char('l')) => KeyAction::GoToLine,
            
            // Ctrl+D - Duplicate line
            (true, KeyCode::Char('d')) => KeyAction::DuplicateLine,
            
            // Ctrl+K - Delete line
            (true, KeyCode::Char('k')) => KeyAction::DeleteLine,

            // Regular character input
            (false, KeyCode::Char(ch)) => KeyAction::InsertChar(ch),
            
            // Enter key
            (false, KeyCode::Enter) => KeyAction::InsertNewline,
            
            // Backspace
            (false, KeyCode::Backspace) => KeyAction::DeleteBackward,
            
            // Delete key
            (false, KeyCode::Delete) => KeyAction::DeleteForward,
            
            // Home - Go to start of line
            (false, KeyCode::Home) => KeyAction::GoToLineStart,
            
            // End - Go to end of line
            (false, KeyCode::End) => KeyAction::GoToLineEnd,

            // Movement commands
            (false, KeyCode::Up) => KeyAction::MoveCursor(Direction::Up),
            (false, KeyCode::Down) => KeyAction::MoveCursor(Direction::Down),
            (false, KeyCode::Left) => KeyAction::MoveCursor(Direction::Left),
            (false, KeyCode::Right) => KeyAction::MoveCursor(Direction::Right),

            _ => KeyAction::None,
        }
    }
}

impl Default for KeyHandler {
    fn default() -> Self {
        Self::new()
    }
}
