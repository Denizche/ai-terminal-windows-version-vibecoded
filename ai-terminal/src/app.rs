use iced::widget::{container, row, text_input, scrollable};
use iced::{Application, Command, Element, Length, Theme};
use iced::keyboard::{self, Event as KeyEvent};
use iced::event::Event;
use iced::subscription;
use std::time::Duration;

use crate::model::{App as AppState, Panel};
use crate::ollama::{api, commands};
use crate::ui::components::{drag_handle, TerminalPanel, AiPanel, ShortcutsModal};
use crate::ui::theme::DraculaTheme;
use crate::terminal::utils;
use crate::config::keyboard::{FocusTarget, handle_keyboard_shortcuts};
use crate::ui::components;

// Add these constants at the top of the file
const TERMINAL_INPUT_ID: &str = "terminal_input";
const AI_INPUT_ID: &str = "ai_input";

#[derive(Debug, Clone)]
pub enum Message {
    TerminalInput(String),
    AIInput(String),
    ExecuteCommand,
    ProcessAIQuery,
    OllamaResponse(Result<String, String>),
    SwitchPanel,
    ResizeLeft,
    ResizeRight,
    HistoryUp,
    HistoryDown,
    TildePressed,
    TerminalScroll(scrollable::Viewport),
    ToggleFocus,
    ScrollToBottom,
    UpdateTerminalOutput(String),
    SendInput(String),
    PollCommandOutput,
    TabPressed,
    NoOp,
    PasswordInput(String),
    SubmitPassword,
    TerminateCommand,
    ToggleShortcutsModal,
    CopyToClipboard(String, bool),
    HandleCtrlC,
    ToggleSearch,
    SearchInput(String),
    SearchNext,
    SearchPrev,
    ClearSearch,
    ToggleTerminalSearchFocus,
}

pub struct TerminalApp {
    state: AppState,
    terminal_input: String,
    ai_input: String,
    focus: FocusTarget,
    current_suggestions: Vec<String>,
    suggestion_index: usize,
    password_buffer: String,
    password_mode: bool,
    show_shortcuts_modal: bool,
    search_mode: bool,
    search_input: String,
    search_index: usize,
    search_matches: Vec<usize>,
    terminal_panel: TerminalPanel,
    ai_panel: AiPanel,
    terminal_focus: bool, // Track if terminal input has focus vs search input
}

// Add this struct at the top of the file, after the imports
struct PanelViews<'a> {
    terminal: Element<'a, Message>,
    ai: Element<'a, Message>,
}

impl Application for TerminalApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        println!("[app.rs] Creating new TerminalApp");
        let app_state = AppState::new();
        
        // Create the initial terminal panel
        let terminal_panel = TerminalPanel::new(
            app_state.clone(), 
            String::new(), 
            FocusTarget::Terminal,
            false
        );
        
        let ai_panel = AiPanel::new(
            app_state.clone(),
            String::new(),
            FocusTarget::Terminal
        );
        
        // Create a batch of commands to initialize the app
        let init_commands = Command::batch(vec![
            // Force focus on terminal input at startup
            text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
            // Move cursor to end to ensure visibility
            text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
        ]);
        
        println!("[app.rs] Initializing with focus on terminal input");
        
        (
            Self {
                state: app_state,
                terminal_input: String::new(),
                ai_input: String::new(),
                focus: FocusTarget::Terminal,
                current_suggestions: Vec::new(),
                suggestion_index: 0,
                password_buffer: String::new(),
                password_mode: false,
                show_shortcuts_modal: false,
                search_mode: false,
                search_input: String::new(),
                search_index: 0,
                search_matches: Vec::new(),
                terminal_panel,
                ai_panel,
                terminal_focus: true,
            },
            // Initialize focus at startup
            init_commands
        )
    }

    fn title(&self) -> String {
        String::from("AI Terminal")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        println!("[app.rs] Updating with message: {:?}", message);

        // First, check if we have a streaming command that needs polling
        if let Some(command) = self.state.poll_command_output() {
            return command;
        }

        let command = match message {
            Message::SearchInput(input) => {
                println!("[app.rs] SearchInput message received with value: '{}'", input);
                self.search_input = input.clone();
                self.search_index = 0;
                self.search_matches = Vec::new();
                
                // When typing in search, we're focused on search
                self.terminal_focus = false;
                println!("[app.rs] Setting terminal_focus to false (search has focus)");
                self.terminal_panel.set_terminal_focus(false);
                
                // Note: We're not changing self.focus here because
                // the global focus remains on the terminal panel
                
                if !input.is_empty() {
                    // Find all matches in the terminal output
                    let visible_output = if self.state.output.len() > 100 {
                        self.state.output.iter().skip(self.state.output.len() - 100).cloned().collect()
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
                    println!("[app.rs] Found {} matches for search query", self.search_matches.len());
                }
                
                // Create a new terminal panel with updated search input
                self.terminal_panel = TerminalPanel::new(
                    self.state.clone(),
                    self.terminal_input.clone(), 
                    self.focus.clone(),
                    self.search_mode
                );
                
                // Update search count in terminal panel
                self.terminal_panel.update_search_input(input);
                self.terminal_panel.update_search_count(self.search_index, self.search_matches.len());
                
                // Make sure terminal_focus is false since we're in search
                self.terminal_panel.set_terminal_focus(false);
                
                // Make sure search input keeps focus
                println!("[app.rs] Focusing search input after SearchInput message");
                text_input::focus(text_input::Id::new("search_input"))
            }
            Message::ToggleSearch => {
                // Toggle search mode
                self.search_mode = !self.search_mode;
                
                if self.search_mode {
                    // When turning on search mode:
                    // 1. Focus should go to search bar
                    self.terminal_focus = false;
                    
                    // 2. Create a new terminal panel with search mode enabled
                    self.terminal_panel = TerminalPanel::new(
                        self.state.clone(),
                        self.terminal_input.clone(), 
                        self.focus.clone(),
                        true
                    );
                    
                    // Make sure terminal panel has correct focus state
                    self.terminal_panel.set_terminal_focus(false);
                    
                    // Clear search state
                    self.search_input.clear();
                    self.search_matches.clear();
                    self.search_index = 0;
                    
                    // Focus the search input when toggling search on
                    println!("[app.rs] Toggling search ON, focusing search input");
                    return text_input::focus(text_input::Id::new("search_input"));
                } else {
                    // When turning off search mode:
                    self.terminal_focus = true;
                    
                    // Create a new terminal panel with search mode disabled
                    self.terminal_panel = TerminalPanel::new(
                        self.state.clone(),
                        self.terminal_input.clone(), 
                        self.focus.clone(),
                        false
                    );
                    
                    // Make sure terminal panel has correct focus state
                    self.terminal_panel.set_terminal_focus(true);
                    
                    // Focus back on terminal input when search is closed
                    println!("[app.rs] Toggling search OFF, focusing terminal input");
                    return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
                }
            }
            Message::PollCommandOutput => {
                if let Some(cmd) = self.state.poll_command_output() {
                    cmd
                } else {
                    Command::none()
                }
            }
            Message::TerminalInput(value) => {
                println!("[app.rs] Received TerminalInput message with value: '{}'", value);
                println!("[app.rs] Current terminal_input before update: '{}'", self.terminal_input);
                self.terminal_input = value;
                println!("[app.rs] Current terminal_input after update: '{}'", self.terminal_input);
                
                // When typing in terminal, ensure we're focused on terminal
                self.terminal_focus = true;
                println!("[app.rs] Setting terminal_focus to true");
                self.terminal_panel.set_terminal_focus(true);
                
                // Also ensure overall focus is correct
                self.focus = FocusTarget::Terminal;
                println!("[app.rs] Focus set to Terminal");
                
                // Reset suggestions when input changes
                self.current_suggestions.clear();
                self.suggestion_index = 0;
                
                // Update the terminal panel with the new input
                self.terminal_panel = TerminalPanel::new(
                    self.state.clone(),
                    self.terminal_input.clone(),
                    self.focus.clone(),
                    self.search_mode
                );
                
                // Make sure the panel focus is consistent with app state
                self.terminal_panel.set_terminal_focus(true);
                
                // Ensure focus remains on terminal input
                Command::batch(vec![
                    text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                    text_input::move_cursor_to_end(text_input::Id::new(TERMINAL_INPUT_ID))
                ])
            }
            Message::AIInput(value) => {
                self.ai_input = value;
                Command::none()
            }
            Message::ExecuteCommand => {
                println!("[app.rs] Execute command message received: '{}'", self.terminal_input);
                
                if !self.terminal_input.is_empty() {
                    println!("[app.rs] Executing command: '{}'", self.terminal_input);
                    self.state.input = self.terminal_input.clone();

                    // Start command execution
                    self.state.execute_command();
                    self.terminal_input.clear();
                    
                    // Reset suggestion state
                    self.current_suggestions.clear();
                    self.suggestion_index = 0;
                    
                    // Update the terminal panel with the cleared input
                    self.terminal_panel = TerminalPanel::new(
                        self.state.clone(), 
                        self.terminal_input.clone(),
                        self.focus.clone(),
                        self.search_mode
                    );

                    // Add slight delay before scrolling to improve smoothness
                    let scroll_cmd = components::scrollable_container::scroll_to_bottom();
                    
                    // Keep focus on the terminal input field after execution
                    let focus_cmd = text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
                    
                    Command::batch(vec![
                        Command::perform(async {}, |_| Message::NoOp),
                        scroll_cmd,
                        focus_cmd,
                    ])
                } else {
                    // Even if no command, ensure focus remains on terminal input
                    text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID))
                }
            }
            Message::ProcessAIQuery => {
                if !self.ai_input.is_empty() {
                    let query = self.ai_input.clone();
                    self.ai_input.clear();

                    // Add query to output
                    let formatted_query = format!("> {}", query);
                    self.state.ai_output.push(formatted_query.clone());

                    // Check if the input is a command
                    if query.starts_with('/') {
                        let parts: Vec<&str> = query.split_whitespace().collect();
                        let cmd = parts[0];

                        match cmd {
                            "/models" => {
                                println!("Processing /models command");
                                self.state.ai_output.push("🔍 Fetching models...".to_string());
                                Command::perform(
                                    async move {
                                        println!("Fetching models from Ollama...");
                                        match api::list_models().await {
                                            Ok(models) => {
                                                println!("Successfully fetched models: {:?}", models);
                                                Ok(models)
                                            },
                                            Err(e) => {
                                                println!("Error fetching models: {}", e);
                                                Err(format!("Error listing models: {}", e))
                                            }
                                        }
                                    },
                                    |result| {
                                        println!("Processing models result: {:?}", result);
                                        match result {
                                            Ok(models) => {
                                                let response = format!(
                                                    "Available models:\n{}",
                                                    models.iter()
                                                        .map(|model| format!("- {}", model))
                                                        .collect::<Vec<_>>()
                                                        .join("\n")
                                                );
                                                println!("Formatted response: {}", response);
                                                Message::OllamaResponse(Ok(response))
                                            },
                                            Err(e) => {
                                                println!("Error response: {}", e);
                                                Message::OllamaResponse(Err(e))
                                            }
                                        }
                                    }
                                )
                            }
                            _ => {
                                // Handle other commands synchronously
                                commands::process_ai_command(&mut self.state, &query);
                                Command::none()
                            }
                        }
                    } else {
                        self.state.ai_output.push("Thinking...".to_string());

                        // Create the context for Ollama
                        let message_with_context = self.create_ollama_context(&query);
                        let model = self.state.ollama_model.clone();

                        println!("Sending chat query to Ollama with model: {}", model);
                        // First check if Ollama is running
                        Command::perform(
                            async move {
                                println!("Checking if Ollama is running...");
                                match api::list_models().await {
                                    Ok(_) => {
                                        println!("Ollama is running, sending prompt...");
                                        match api::send_prompt(&model, &message_with_context).await {
                                            Ok(response) => {
                                                println!("Got response from Ollama");
                                                Ok(response)
                                            },
                                            Err(e) => {
                                                println!("Error from Ollama: {}", e);
                                                Err(e)
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        println!("Ollama is not running");
                                        Err("Error: Ollama is not running. Please start Ollama and try again.".to_string())
                                    }
                                }
                            },
                            Message::OllamaResponse
                        )
                    }
                } else {
                    Command::none()
                }
            }
            Message::OllamaResponse(result) => {
                println!("Handling OllamaResponse message");
                match result {
                    Ok(response) => {
                        println!("Processing successful response");
                        // Remove thinking message
                        if let Some(last) = self.state.ai_output.last() {
                            if last.contains("Thinking") || last.contains("🔍 Fetching") {
                                println!("Removing thinking/fetching message");
                                self.state.ai_output.pop();
                            }
                        }

                        // Extract commands from the response
                        let extracted_command = utils::extract_commands(&response);
                        if !extracted_command.is_empty() {
                            println!("Extracted command: {}", extracted_command);
                            self.state.last_ai_command = Some(extracted_command.clone());
                            self.terminal_input = extracted_command;
                        }

                        // Add the AI response to output
                        println!("Adding response to output: {}", response);
                        self.state.ai_output.push(response.clone());

                        // Add slight delay before scrolling to improve smoothness
                        let scroll_cmd = components::scrollable_container::scroll_to_bottom();
                        Command::batch(vec![
                            Command::perform(async {}, |_| Message::NoOp),
                            scroll_cmd,
                        ])
                    }
                    Err(error) => {
                        println!("Processing error response: {}", error);
                        // Remove thinking message
                        if let Some(last) = self.state.ai_output.last() {
                            if last.contains("Thinking") || last.contains("🔍 Fetching") {
                                println!("Removing thinking/fetching message");
                                self.state.ai_output.pop();
                            }
                        }
                        self.state.ai_output.push(format!("Error: {}", error));

                        // Add slight delay before scrolling to improve smoothness
                        let scroll_cmd = components::scrollable_container::scroll_to_bottom();
                        Command::batch(vec![
                            Command::perform(async {}, |_| Message::NoOp),
                            scroll_cmd,
                        ])
                    }
                }
            }
            Message::SwitchPanel => {
                self.state.active_panel = match self.state.active_panel {
                    Panel::Terminal => Panel::Assistant,
                    Panel::Assistant => Panel::Terminal,
                };
                Command::none()
            }
            Message::ResizeLeft => {
                let new_ratio = (self.state.panel_ratio - 5).max(20);
                self.state.panel_ratio = new_ratio;
                Command::none()
            }
            Message::ResizeRight => {
                let new_ratio = (self.state.panel_ratio + 5).min(80);
                self.state.panel_ratio = new_ratio;
                Command::none()
            }
            Message::HistoryUp => {
                if self.focus == FocusTarget::Terminal {
                    if let Some(current_index) = self.state.command_history_index {
                        // Already navigating history, try to go to older command
                        if current_index > 0 {
                            self.state.command_history_index = Some(current_index - 1);
                            if let Some(command) = self.state.command_history.get(current_index - 1) {
                                self.terminal_input = command.clone();
                            }
                        }
                    } else if !self.state.command_history.is_empty() {
                        // Start from the newest command (last in the vector)
                        let last_idx = self.state.command_history.len() - 1;
                        self.state.command_history_index = Some(last_idx);
                        if let Some(command) = self.state.command_history.last() {
                            self.terminal_input = command.clone();
                        }
                    }
                }
                Command::none()
            }
            Message::HistoryDown => {
                if self.focus == FocusTarget::Terminal {
                    if let Some(current_index) = self.state.command_history_index {
                        // Move to newer command
                        if current_index < self.state.command_history.len() - 1 {
                            self.state.command_history_index = Some(current_index + 1);
                            if let Some(command) = self.state.command_history.get(current_index + 1) {
                                self.terminal_input = command.clone();
                            }
                        } else {
                            // At newest command, clear input
                            self.state.command_history_index = None;
                            self.terminal_input.clear();
                        }
                    }
                }
                Command::none()
            }
            Message::TildePressed => {
                if self.state.active_panel == Panel::Terminal {
                    self.terminal_input.push('~');
                } else {
                    self.ai_input.push('~');
                }
                Command::none()
            }
            Message::TerminalScroll(viewport) => {
                // Only update the scroll position if we're not actively
                // trying to scroll to the bottom
                self.state.terminal_scroll = viewport.relative_offset().y as usize;
                Command::none()
            }
            Message::ToggleFocus => {
                self.focus = match self.focus {
                    FocusTarget::Terminal => FocusTarget::AiChat,
                    FocusTarget::AiChat => FocusTarget::Terminal,
                };
                // Return a command to focus the correct input
                match self.focus {
                    FocusTarget::Terminal => text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID)),
                    FocusTarget::AiChat => text_input::focus(text_input::Id::new(AI_INPUT_ID)),
                }
            }
            Message::ScrollToBottom => {
                // Only scroll to bottom when explicitly requested, not on every scroll event
                // This prevents scroll stuttering when user is manually scrolling
                components::scrollable_container::scroll_to_bottom()
            }
            Message::UpdateTerminalOutput(line) => {
                self.state.output.push(line);
                components::scrollable_container::scroll_to_bottom()
            }
            Message::SendInput(input) => {
                if self.state.password_mode {
                    self.state.send_input(input);
                    self.terminal_input.clear();  // Clear the input after sending password
                }
                Command::none()
            }
            Message::TabPressed => {
                println!("[app.rs] Tab pressed message received for autocomplete");
                
                // Tab should now only handle autocomplete, not context switching
                if self.focus == FocusTarget::Terminal {
                    // If search mode is not active, handle autocomplete suggestions for terminal input
                    println!("[app.rs] Getting autocomplete suggestions");

                    // If we don't have any suggestions yet, get them
                    if self.current_suggestions.is_empty() {
                        println!("[app.rs] Getting new suggestions");
                        self.state.input = self.terminal_input.clone();
                        self.current_suggestions = self.state.get_autocomplete_suggestions();
                        println!("[app.rs] Got suggestions: {:?}", self.current_suggestions);
                    } 
                    
                    // Apply suggestions if available
                    if !self.current_suggestions.is_empty() {
                        // We have suggestions, move to the next one if there are multiple
                        if self.current_suggestions.len() > 1 {
                            self.suggestion_index = (self.suggestion_index + 1) % self.current_suggestions.len();
                            println!("[app.rs] Moving to suggestion {}/{}", 
                                self.suggestion_index + 1, self.current_suggestions.len());
                        }

                        // Apply the current suggestion
                        let suggestion = self.current_suggestions[self.suggestion_index].clone();
                        println!("[app.rs] Using suggestion: {}", suggestion);
                        self.terminal_input = suggestion;
                        
                        // Update the terminal panel with the new input
                        self.terminal_panel = TerminalPanel::new(
                            self.state.clone(),
                            self.terminal_input.clone(),
                            self.focus.clone(),
                            self.search_mode
                        );

                        // Make sure the panel focus is consistent with app state
                        self.terminal_panel.set_terminal_focus(true);

                        // Move cursor to end after applying suggestion and make sure terminal is focused
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
            Message::NoOp => {
                Command::none()
            }
            Message::PasswordInput(password) => {
                // Store password temporarily (don't display it)
                self.password_buffer = password;
                Command::none()
            }
            Message::SubmitPassword => {
                // Send the password to the running command
                let password = std::mem::take(&mut self.password_buffer);
                self.state.send_input(password);
                Command::none()
            }
            Message::TerminateCommand => {
                if let Some(cmd) = self.state.terminate_running_command() {
                    cmd
                } else {
                    Command::none()
                }
            }
            Message::ToggleShortcutsModal => {
                self.show_shortcuts_modal = !self.show_shortcuts_modal;
                Command::none()
            }
            Message::CopyToClipboard(content, _show_feedback) => {
                // Just copy to clipboard without feedback mechanism
                iced::clipboard::write(content)
            }
            Message::HandleCtrlC => {
                // Check if there's a running command first
                if self.state.command_receiver.is_some() {
                    // There's a running command, terminate it
                    if let Some(cmd) = self.state.terminate_running_command() {
                        cmd
                    } else {
                        Command::none()
                    }
                } else {
                    // No running command, try to get selected text from OS clipboard
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if let Ok(text) = clipboard.get_text() {
                            // Text was copied via OS selection mechanisms, we don't need to do anything
                            // The OS clipboard already has the text
                            println!("Text copied: {}", if text.len() > 20 { 
                                format!("{}...", &text[..20]) 
                            } else { 
                                text.clone() 
                            });
                        }
                    }
                    Command::none()
                }
            }
            Message::SearchNext => {
                if let Some(index) = self.search_matches.get(self.search_index) {
                    let visible_output = if self.state.output.len() > 100 {
                        self.state.output.iter().skip(self.state.output.len() - 100).cloned().collect()
                    } else {
                        self.state.output.clone()
                    };
                    self.terminal_input = visible_output[*index].clone();
                    self.search_index = (self.search_index + 1) % self.search_matches.len();
                    // Update search count in terminal panel
                    self.terminal_panel.update_search_count(self.search_index, self.search_matches.len());
                }
                Command::none()
            }
            Message::SearchPrev => {
                if let Some(index) = self.search_matches.get(self.search_index) {
                    let visible_output = if self.state.output.len() > 100 {
                        self.state.output.iter().skip(self.state.output.len() - 100).cloned().collect()
                    } else {
                        self.state.output.clone()
                    };
                    self.terminal_input = visible_output[*index].clone();
                    self.search_index = if self.search_index == 0 { self.search_matches.len() - 1 } else { self.search_index - 1 };
                    // Update search count in terminal panel
                    self.terminal_panel.update_search_count(self.search_index, self.search_matches.len());
                }
                Command::none()
            }
            Message::ClearSearch => {
                self.search_input.clear();
                self.search_matches.clear();
                self.search_index = 0;
                
                // Recreate terminal panel with cleared search
                self.terminal_panel = TerminalPanel::new(
                    self.state.clone(),
                    self.terminal_input.clone(), 
                    self.focus.clone(),
                    self.search_mode
                );
                
                // Update search count in terminal panel
                self.terminal_panel.update_search_count(0, 0);
                
                // Focus remains on search but terminal_focus should be false
                self.terminal_focus = false;
                self.terminal_panel.set_terminal_focus(false);
                // Focus back on the search input
                return text_input::focus(text_input::Id::new("search_input"));
            }
            Message::ToggleTerminalSearchFocus => {
                // This is now triggered by Ctrl+Tab or Escape
                println!("[app.rs] ToggleTerminalSearchFocus triggered (Ctrl+Tab)");
                
                // Only toggle focus between terminal and search input when search is active
                if self.search_mode {
                    // Toggle terminal focus - if currently on search, switch to terminal and vice versa
                    self.terminal_focus = !self.terminal_focus;
                    // Sync the focus state to the panel
                    self.terminal_panel.set_terminal_focus(self.terminal_focus);
                    
                    if self.terminal_focus {
                        // Focus the terminal input and ensure global focus is terminal
                        self.focus = FocusTarget::Terminal;
                        println!("[app.rs] Context switch: Focusing terminal input");
                        return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
                    } else {
                        // Focus the search input
                        println!("[app.rs] Context switch: Focusing search input");
                        return text_input::focus(text_input::Id::new("search_input"));
                    }
                } else {
                    // If search is not active, focus the terminal input
                    self.terminal_focus = true;
                    self.terminal_panel.set_terminal_focus(true);
                    self.focus = FocusTarget::Terminal;
                    println!("[app.rs] Context switch: Focusing terminal input (search inactive)");
                    return text_input::focus(text_input::Id::new(TERMINAL_INPUT_ID));
                }
            }
        };

        // Update panels with current state
        // We already updated the terminal panel in individual handlers
        self.ai_panel = AiPanel::new(
            self.state.clone(),
            self.ai_input.clone(),
            self.focus.clone(),
        );

        command
    }

    fn view(&self) -> Element<Message> {
        let views = self.create_panel_views();

        // Build the main content using the stored views
        let content = row![
            container(views.terminal)
                .width(Length::FillPortion(self.state.panel_ratio as u16))
                .height(Length::Fill)
                .style(DraculaTheme::container_style()),
            drag_handle(),
            container(views.ai)
                .width(Length::FillPortion((100 - self.state.panel_ratio) as u16))
                .height(Length::Fill)
                .style(DraculaTheme::container_style()),
        ]
        .height(Length::Fill);

        // If modal is visible, show it centered without a backdrop
        if self.show_shortcuts_modal {
            // Create a floating container for the modal
            container(
                container(ShortcutsModal::view())
                    .width(Length::Fixed(450.0))
                    .padding(20)
                    .style(DraculaTheme::modal_style())
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(DraculaTheme::transparent_container_style())
            .into()
        } else {
            // Just show the normal content
            content.into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        struct EventHandler;
        impl EventHandler {
            fn handle(event: Event, status: iced::event::Status) -> Option<Message> {
                // Log status when we receive keyboard events
                if let Event::Keyboard(key_event) = &event {
                    println!("[app.rs:subscription] Keyboard event: {:?}, status: {:?}", key_event, status);
                    
                    // Special handling for character events
                    if let KeyEvent::CharacterReceived(ch) = key_event {
                        println!("[app.rs:subscription] Character received: '{}'", ch);
                    }
                }
                
                if let Event::Keyboard(key_event) = event {
                    match key_event {
                        KeyEvent::KeyPressed {
                            key_code: keyboard::KeyCode::C,
                            modifiers,
                            ..
                        } if modifiers.control() => {
                            // If text is selected in an input field, copy it
                            // Otherwise, terminate running commands
                            println!("[app.rs:subscription] Ctrl+C pressed");
                            return Some(Message::HandleCtrlC);
                        }
                        KeyEvent::KeyPressed {
                            key_code: keyboard::KeyCode::Tab,
                            modifiers,
                            ..
                        } if modifiers.control() => {
                            // Ctrl+Tab is now used for switching context between search and terminal
                            println!("[app.rs:subscription] Ctrl+Tab pressed, switching context");
                            Some(Message::ToggleTerminalSearchFocus)
                        }
                        KeyEvent::KeyPressed {
                            key_code: keyboard::KeyCode::Tab,
                            modifiers,
                            ..
                        } if !modifiers.alt() && !modifiers.shift() => {
                            // Regular Tab (without Ctrl) is used for autocomplete
                            println!("[app.rs:subscription] Tab key pressed for autocomplete");
                            Some(Message::TabPressed)
                        }
                        KeyEvent::KeyPressed {
                            key_code: keyboard::KeyCode::Enter,
                            modifiers,
                            ..
                        } if !modifiers.alt() && !modifiers.control() && !modifiers.shift() => {
                            // Handle Enter key explicitly for command execution
                            println!("[app.rs:subscription] Enter key pressed, sending ExecuteCommand");
                            Some(Message::ExecuteCommand)
                        }
                        KeyEvent::KeyPressed {
                            key_code,
                            modifiers,
                            ..
                        } => {
                            // For debugging, log all key presses
                            println!("[app.rs:subscription] Key pressed: {:?}, modifiers: {:?}", key_code, modifiers);
                            
                            match key_code {
                                keyboard::KeyCode::Up => Some(Message::HistoryUp),
                                keyboard::KeyCode::Down => Some(Message::HistoryDown),
                                keyboard::KeyCode::Left if modifiers.alt() => Some(Message::ResizeLeft),
                                keyboard::KeyCode::Right if modifiers.alt() => Some(Message::ResizeRight),
                                keyboard::KeyCode::Grave if modifiers.shift() => Some(Message::TildePressed),
                                keyboard::KeyCode::E if modifiers.control() => Some(Message::ToggleFocus),
                                keyboard::KeyCode::F if modifiers.control() => Some(Message::ToggleSearch),
                                keyboard::KeyCode::Escape => {
                                    // Explicitly handle Escape key for toggling focus in search mode
                                    println!("[app.rs:subscription] Escape key pressed");
                                    Some(Message::ToggleTerminalSearchFocus)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }

        let keyboard_events = iced::subscription::events_with(EventHandler::handle);

        // Only create the terminal poll subscription if we have a command running
        let terminal_poll = if self.state.command_receiver.is_some() {
            subscription::unfold(
                "terminal_poll",
                State::Ready,
                move |state| async move {
                    match state {
                        State::Ready => {
                            tokio::time::sleep(Duration::from_millis(1)).await;
                            (Message::PollCommandOutput, State::Waiting)
                        }
                        State::Waiting => {
                            tokio::time::sleep(Duration::from_millis(1)).await;
                            (Message::PollCommandOutput, State::Waiting)
                        }
                    }
                },
            )
        } else {
            subscription::unfold("inactive_poll", (), |_| async {
                tokio::time::sleep(Duration::from_secs(3600)).await;
                (Message::PollCommandOutput, ())
            })
        };

        iced::Subscription::batch(vec![
            keyboard_events,
            terminal_poll,
        ])
    }
}

impl TerminalApp {
    fn create_ollama_context(&self, query: &str) -> String {
        format!(
            "System Info: {}\n\nRecent Terminal Output:\n{}\n\nRecent Chat History:\n{}\n\nUser query: {}\n\nCurrent directory: {}",
            self.state.os_info,
            self.state.output.iter().rev().take(20).map(String::as_str).collect::<Vec<_>>().join("\n"),
            self.state.ai_output.iter().rev().take(10).map(String::as_str).collect::<Vec<_>>().join("\n"),
            query,
            self.state.current_dir.display()
        )
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent::KeyPressed { 
                key_code: keyboard::KeyCode::Tab,
                modifiers,
                ..
            } if !modifiers.alt() && !modifiers.control() && !modifiers.shift() => {
                println!("[app.rs] Tab key pressed in handle_input");
                if self.focus == FocusTarget::Terminal {
                    println!("[app.rs] Focus is on terminal, getting suggestions");
                    self.state.input = self.terminal_input.clone();
                    println!("[app.rs] Current input: {}", self.terminal_input);
                    let suggestions = self.state.get_autocomplete_suggestions();
                    println!("[app.rs] Got suggestions: {:?}", suggestions);
                    if !suggestions.is_empty() {
                        println!("[app.rs] Using first suggestion: {}", suggestions[0]);
                        self.terminal_input = suggestions[0].clone();
                    } else {
                        println!("[app.rs] No suggestions found");
                    }
                } else {
                    println!("[app.rs] Focus is not on terminal");
                }
                return;
            }
            _ => {
                if handle_keyboard_shortcuts(key_event, &mut self.focus) {
                    return;
                }
            }
        }
        
        // Handle other input based on focus
        match self.focus {
            FocusTarget::Terminal => {
                // ... existing terminal input handling ...
            }
            FocusTarget::AiChat => {
                // ... existing AI chat input handling ...
            }
        }
    }

    fn create_panel_views(&self) -> PanelViews<'_> {
        PanelViews {
            terminal: self.terminal_panel.view(),
            ai: self.ai_panel.view(),
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Ready,
    Waiting,
}
