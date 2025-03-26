use iced::keyboard::{KeyCode, Event as KeyEvent};

#[derive(Debug, Clone, PartialEq)]
pub enum FocusTarget {
    Terminal,
    AiChat,
}

#[derive(Debug, Clone)]
pub enum ShortcutAction {
    ToggleFocus,
    ResizeLeft,
    ResizeRight,
    HistoryUp,
    HistoryDown,
    TildeInsert,
    TerminateCommand,
    ToggleSearch,
    ToggleTerminalSearchFocus,
    TabAutocomplete,
    ExecuteCommand,
    None,
}

/// Handles all keyboard shortcuts for the application
pub fn handle_keyboard_shortcuts(key_event: KeyEvent, current_focus: &mut FocusTarget) -> bool {
    match handle_keyboard_event(key_event) {
        ShortcutAction::ToggleFocus => {
            *current_focus = match *current_focus {
                FocusTarget::Terminal => FocusTarget::AiChat,
                FocusTarget::AiChat => FocusTarget::Terminal,
            };
            true
        },
        ShortcutAction::None => false,
        _ => false, // Other actions are handled at the app level
    }
}

/// Processes keyboard events and returns the corresponding action
pub fn handle_keyboard_event(key_event: KeyEvent) -> ShortcutAction {
    match key_event {
        KeyEvent::KeyPressed { 
            key_code: KeyCode::E,
            modifiers,
            ..
        } if modifiers.control() => {
            ShortcutAction::ToggleFocus
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::C,
            modifiers,
            ..
        } if modifiers.control() => {
            ShortcutAction::TerminateCommand
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Tab,
            modifiers,
            ..
        } if modifiers.control() => {
            ShortcutAction::ToggleTerminalSearchFocus
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Tab,
            modifiers,
            ..
        } if !modifiers.alt() && !modifiers.shift() => {
            ShortcutAction::TabAutocomplete
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Enter,
            modifiers,
            ..
        } if !modifiers.alt() && !modifiers.control() && !modifiers.shift() => {
            ShortcutAction::ExecuteCommand
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Up,
            ..
        } => {
            ShortcutAction::HistoryUp
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Down,
            ..
        } => {
            ShortcutAction::HistoryDown
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Left,
            modifiers,
            ..
        } if modifiers.alt() => {
            ShortcutAction::ResizeLeft
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Right,
            modifiers,
            ..
        } if modifiers.alt() => {
            ShortcutAction::ResizeRight
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Grave,
            modifiers,
            ..
        } if modifiers.shift() => {
            ShortcutAction::TildeInsert
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::F,
            modifiers,
            ..
        } if modifiers.control() => {
            ShortcutAction::ToggleSearch
        },
        KeyEvent::KeyPressed { 
            key_code: KeyCode::Escape,
            ..
        } => {
            ShortcutAction::ToggleTerminalSearchFocus
        },
        _ => ShortcutAction::None,
    }
}

/// Converts a ShortcutAction to a string representation for debugging
pub fn shortcut_action_to_string(action: &ShortcutAction) -> &'static str {
    match action {
        ShortcutAction::ToggleFocus => "Toggle Focus",
        ShortcutAction::ResizeLeft => "Resize Left",
        ShortcutAction::ResizeRight => "Resize Right",
        ShortcutAction::HistoryUp => "History Up",
        ShortcutAction::HistoryDown => "History Down",
        ShortcutAction::TildeInsert => "Insert Tilde",
        ShortcutAction::TerminateCommand => "Terminate Command",
        ShortcutAction::ToggleSearch => "Toggle Search",
        ShortcutAction::ToggleTerminalSearchFocus => "Toggle Terminal/Search Focus",
        ShortcutAction::TabAutocomplete => "Tab Autocomplete",
        ShortcutAction::ExecuteCommand => "Execute Command",
        ShortcutAction::None => "None",
    }
}

/// Gets a list of all available keyboard shortcuts with descriptions
pub fn get_all_shortcuts() -> Vec<(String, String)> {
    vec![
        // Navigation
        ("Ctrl+E".to_string(), "Toggle focus between terminal and AI chat".to_string()),
        ("Alt+Left".to_string(), "Decrease terminal panel width".to_string()),
        ("Alt+Right".to_string(), "Increase terminal panel width".to_string()),
        
        // History
        ("Up".to_string(), "Previous command in history".to_string()),
        ("Down".to_string(), "Next command in history".to_string()),
        
        // Commands
        ("Tab".to_string(), "Autocomplete command".to_string()),
        ("Ctrl+C".to_string(), "Terminate running command".to_string()),
        ("Shift+`".to_string(), "Insert tilde character".to_string()),
        ("Ctrl+F".to_string(), "Toggle search in terminal".to_string()),
        ("Escape".to_string(), "Close search or modal".to_string()),
        ("Ctrl+Tab".to_string(), "Toggle between terminal and search".to_string()),
    ]
} 