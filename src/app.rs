use crate::model::{App as AppState, Panel};
use crate::ollama::{api, commands};
use crate::ui::components::{drag_handle, TerminalPanel, AiPanel, ShortcutsModal};
use crate::ui::theme::DraculaTheme;
use crate::terminal::utils;
use crate::config::keyboard::{FocusTarget, handle_keyboard_shortcuts, handle_keyboard_event, ShortcutAction};
use crate::ui::components;

// Add these constants at the top of the file
const TERMINAL_INPUT_ID: &str = "terminal_input";
const AI_INPUT_ID: &str = "ai_input";

// Add a helper method to simplify terminal panel updates
impl TerminalApp {
    // Helper method to update terminal panel and handle focus consistently
    fn update_terminal_panel(&mut self, focus_terminal: bool) -> Command<Message> {
        // Update focus state consistently
        self.terminal_focus = focus_terminal;
        
        // Instead of recreating the whole panel, just update the relevant parts
        self.terminal_panel.update_input(self.terminal_input.clone());
        self.terminal_panel.update_state(&self.state);
        self.terminal_panel.set_terminal_focus(focus_terminal);
        
        // Return appropriate focus command
        if focus_terminal {
            Command::batch(vec![
                text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
            ])
        } else if self.search_mode {
            text_input::focus(text_input::Id::new("search_input"))
        } else {
            Command::none()
        }
    }
    
    // Add a method to handle command output updates consistently
    fn handle_command_output_update(&mut self) -> Command<Message> {
        // Update terminal panel to refresh UI
        self.update_terminal_panel(self.terminal_focus);
        
        // Return scroll command to ensure output is visible
        components::scrollable_container::scroll_to_bottom()
    }

    // ... existing methods ...
}

// ... existing code ... 

Message::PollCommandOutput => {
    if let Some(cmd) = self.state.poll_command_output() {
        // Just update the state instead of recreating the panel
        self.terminal_panel.update_state(&self.state);
        self.terminal_panel.set_terminal_focus(self.terminal_focus);
        
        cmd
    } else {
        Command::none()
    }
}
Message::CheckCommandOutput => {
    // Force an immediate check for command output and ensure UI updates
    if let Some(cmd) = self.state.poll_command_output() {
        // Just update the state instead of recreating the panel
        self.terminal_panel.update_state(&self.state);
        self.terminal_panel.set_terminal_focus(self.terminal_focus);
        
        cmd
    } else {
        // Update the panel state even if no new output
        self.terminal_panel.update_state(&self.state);
        self.terminal_panel.set_terminal_focus(self.terminal_focus);
        
        // Always return a command to force UI refresh for streaming commands
        components::scrollable_container::scroll_to_bottom()
    }
}

Message::TerminalInput(value) => {
    self.terminal_input = value;
    
    // When typing in terminal, ensure focus is correct
    self.focus = FocusTarget::Terminal;
    
    // Reset suggestions when input changes
    self.current_suggestions.clear();
    self.suggestion_index = 0;
    
    // Update input in panel instead of recreating it
    self.terminal_panel.update_input(self.terminal_input.clone());
    self.terminal_panel.set_terminal_focus(true);
    self.terminal_focus = true;
    
    // Ensure focus remains on terminal input
    Command::batch(vec![
        text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
        text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
    ])
}

Message::TabPressed => {
    // Tab should now only handle autocomplete, not context switching
    if self.focus == FocusTarget::Terminal {
        // If search mode is not active, handle autocomplete suggestions for terminal input
        
        // If we don't have any suggestions yet, get them
        if self.current_suggestions.is_empty() {
            self.state.input = self.terminal_input.clone();
            self.current_suggestions = self.state.get_autocomplete_suggestions();
        } 
        
        // Apply suggestions if available
        if !self.current_suggestions.is_empty() {
            // We have suggestions, move to the next one if there are multiple
            if self.current_suggestions.len() > 1 {
                self.suggestion_index = (self.suggestion_index + 1) % self.current_suggestions.len();
            }

            // Apply the current suggestion
            let suggestion = self.current_suggestions[self.suggestion_index].clone();
            self.terminal_input = suggestion;
            
            // Update only the input field, not recreate the whole panel
            self.terminal_panel.update_input(self.terminal_input.clone());
            self.terminal_panel.set_terminal_focus(true);
            self.terminal_focus = true;
            
            // Move cursor to end after applying suggestion
            return Command::batch(vec![
                text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
            ]);
        }
        
        // Even if no suggestions, ensure focus is on terminal input
        return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
    }
    
    // If not on terminal, do nothing for Tab
    Command::none()
}

Message::ToggleSearch => {
    // Toggle search mode
    self.search_mode = !self.search_mode;
    
    if self.search_mode {
        // When turning on search mode:
        // Clear search state
        self.search_input.clear();
        self.search_matches.clear();
        self.search_index = 0;
        
        // Update panel with search mode enabled
        self.terminal_panel.update_search_state(true, Some(self.search_input.clone()));
        self.terminal_panel.set_terminal_focus(false);
        self.terminal_focus = false;
        
        // Focus the search input when toggling search on
        return text_input::focus(text_input::Id::new("search_input"));
    } else {
        // When turning off search mode:
        // Update panel with search mode disabled
        self.terminal_panel.update_search_state(false, None);
        self.terminal_panel.set_terminal_focus(true);
        self.terminal_focus = true;
        
        // Focus back on terminal input when search is closed
        return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
    }
}

Message::ToggleTerminalSearchFocus => {
    // This is triggered by Ctrl+Tab or Escape
    
    // Only toggle focus between terminal and search input when search is active
    if self.search_mode {
        // Toggle terminal focus - if currently on search, switch to terminal and vice versa
        self.terminal_focus = !self.terminal_focus;
        self.terminal_panel.set_terminal_focus(self.terminal_focus);
        
        if self.terminal_focus {
            // Focus the terminal input and ensure global focus is terminal
            self.focus = FocusTarget::Terminal;
            return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
        } else {
            // Focus the search input
            return text_input::focus(text_input::Id::new("search_input"));
        }
    } else {
        // If search is not active, focus the terminal input
        self.terminal_focus = true;
        self.terminal_panel.set_terminal_focus(true);
        self.focus = FocusTarget::Terminal;
        return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
    }
}

Message::HistoryUp => {
    if self.focus == FocusTarget::Terminal {
        let need_update = if let Some(current_index) = self.state.command_history_index {
            // Already navigating history, try to go to older command
            if current_index > 0 {
                self.state.command_history_index = Some(current_index - 1);
                if let Some(command) = self.state.command_history.get(current_index - 1) {
                    self.terminal_input = command.clone();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else if !self.state.command_history.is_empty() {
            // Start from the newest command (last in the vector)
            let last_idx = self.state.command_history.len() - 1;
            self.state.command_history_index = Some(last_idx);
            if let Some(command) = self.state.command_history.last() {
                self.terminal_input = command.clone();
                true
            } else {
                false
            }
        } else {
            false
        };
        
        if need_update {
            // Update input in panel instead of recreating it
            self.terminal_panel.update_input(self.terminal_input.clone());
            self.terminal_panel.set_terminal_focus(true);
            self.terminal_focus = true;
            
            // Return a command to focus the terminal input and move cursor to end
            return Command::batch(vec![
                text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
            ]);
        }
    }
    Command::none()
}

Message::HistoryDown => {
    if self.focus == FocusTarget::Terminal {
        let mut need_update = false;
        
        if let Some(current_index) = self.state.command_history_index {
            // Move to newer command
            if current_index < self.state.command_history.len() - 1 {
                self.state.command_history_index = Some(current_index + 1);
                if let Some(command) = self.state.command_history.get(current_index + 1) {
                    self.terminal_input = command.clone();
                    need_update = true;
                }
            } else {
                // At newest command, clear input
                self.state.command_history_index = None;
                self.terminal_input.clear();
                need_update = true;
            }
        }
        
        if need_update {
            // Update input in panel instead of recreating it
            self.terminal_panel.update_input(self.terminal_input.clone());
            self.terminal_panel.set_terminal_focus(true);
            self.terminal_focus = true;
            
            // Return a command to focus the terminal input and move cursor to end
            return Command::batch(vec![
                text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
            ]);
        }
    }
    Command::none()
}

Message::ExecuteCommand => {
    if !self.terminal_input.is_empty() {
        self.state.input = self.terminal_input.clone();

        // Start command execution
        self.state.execute_command();
        self.terminal_input.clear();
        
        // Update panel with new state and empty input
        self.terminal_panel.update_state(&self.state);
        self.terminal_panel.update_input(self.terminal_input.clone());
        self.terminal_panel.set_terminal_focus(true);
        self.terminal_focus = true;
        
        // Reset suggestion state
        self.current_suggestions.clear();
        self.suggestion_index = 0;
        
        Command::batch(vec![
            Command::perform(async {}, |_| Message::NoOp),
            components::scrollable_container::scroll_to_bottom(),
            // Keep focus on the terminal input field after execution
            text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
            // Add an immediate check for command output to display results faster
            Command::perform(async {}, |_| Message::CheckCommandOutput),
            // Schedule additional checks shortly after
            Command::perform(async {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }, |_| Message::CheckCommandOutput),
            Command::perform(async {
                tokio::time::sleep(Duration::from_millis(30)).await;
            }, |_| Message::CheckCommandOutput),
            Command::perform(async {
                tokio::time::sleep(Duration::from_millis(60)).await;
            }, |_| Message::CheckCommandOutput),
        ])
    } else {
        // Even if no command, ensure focus remains on terminal input
        text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID))
    }
}

Message::SearchInput(input) => {
    self.search_input = input.clone();
    self.search_index = 0;
    self.search_matches = Vec::new();
    
    // When typing in search, we're focused on search
    self.terminal_focus = false;
    self.terminal_panel.set_terminal_focus(false);
    
    if !input.is_empty() {
        // Find all matches in the terminal output
        let visible_output = if self.state.output.len() > 2000 {
            self.state.output.iter().skip(self.state.output.len() - 2000).cloned().collect()
        } else {
            self.state.output.clone()
        };
        
        // Count all matches in each line
        for (i, line) in visible_output.iter().enumerate() {
            let mut pos = 0;
            while let Some(pos_found) = line[pos..].to_lowercase().find(&input.to_lowercase()) {
                self.search_matches.push(i);
                pos += pos_found + 1;
            }
        }
    }
    
    // Update search input and count in terminal panel
    self.terminal_panel.update_search_input(input);
    self.terminal_panel.update_search_count(self.search_index, self.search_matches.len());
    
    // Focus the search input
    text_input::focus(text_input::Id::new("search_input"))
} 

fn update(&mut self, message: Message) -> Command<Message> {
    // First, check if we have a streaming command that needs polling
    if let Some(command) = self.state.poll_command_output() {
        return command;
    }

    let command = match message {
        // ... rest of the match statements ...
        _ => Command::none(),
    };

    command
} 