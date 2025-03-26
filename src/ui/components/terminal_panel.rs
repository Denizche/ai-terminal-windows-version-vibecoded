use iced::widget::{container, row, text, text_input, button, column};
use iced::{Element, Length, Font};
use crate::ui::theme::DraculaTheme;
use crate::app::Message;
use crate::config::keyboard::FocusTarget;
use crate::ui::components::{styled_text, copy_button};
use crate::ui::components::scrollable_container;
use crate::model::{CommandStatus, App as AppState};
use crate::ui::components::git_branch_text;

const TERMINAL_INPUT_ID: &str = "terminal_input";

#[derive(Debug, Clone)]
pub struct TerminalPanel {
    state: AppState,
    terminal_input: String,
    focus: FocusTarget,
    search_mode: bool,
    terminal_focus: bool,
    view_update_id: u64,
    search_bar: super::search::SearchBar,
    force_refresh: bool,
}

impl TerminalPanel {
    pub fn new(state: AppState, terminal_input: String, focus: FocusTarget, search_mode: bool) -> Self {
        // Use the current time to ensure panel views are never identical when created
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
            
        TerminalPanel {
            state,
            terminal_input,
            focus,
            search_mode,
            terminal_focus: true,
            view_update_id: now,
            search_bar: super::search::SearchBar::new(),
            force_refresh: true,
        }
    }
    
    // New method to update just the input text
    pub fn update_input(&mut self, input: String) {
        self.terminal_input = input;
        self.force_refresh = true;
        self.update_view_id();
    }
    
    // New method to update the application state
    pub fn update_state(&mut self, state: &AppState) {
        self.state = state.clone();
        self.force_refresh = true;
        self.update_view_id();
    }
    
    // Method to update search-related state
    pub fn update_search_state(&mut self, search_mode: bool, search_input: Option<String>) {
        self.search_mode = search_mode;
        if let Some(input) = search_input {
            self.search_bar.update_input(input);
        }
        self.update_view_id();
    }
    
    // Set terminal focus (vs search bar)
    pub fn set_terminal_focus(&mut self, focus: bool) {
        self.terminal_focus = focus;
        self.update_view_id();
    }
    
    // Update view ID to force refresh
    fn update_view_id(&mut self) {
        self.view_update_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }
    
    // Existing method to update search input
    pub fn update_search_input(&mut self, input: String) {
        self.search_bar.update_input(input);
        self.update_view_id();
    }
    
    // Existing method to update search count
    pub fn update_search_count(&mut self, index: usize, total: usize) {
        self.search_bar.update_count(index, total);
        self.update_view_id();
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Check if we should render the search bar
        let search_bar = if self.search_mode {
            self.search_bar.view()
        } else {
            row![].into()
        };
        
        // Render the main view
        let scrollable_content = self.view_terminal_output();
        let input_area = self.view_input();
        
        column![
            scrollable_content,
            search_bar,
            input_area,
        ].into()
    }
    
    fn view_input(&self) -> Element<'_, Message> {
        // Determine whether to use password mode
        let password_mode = self.state.password_mode;
        
        // Determine if terminal input should be focused
        let input_should_be_focused = self.focus == FocusTarget::Terminal && self.terminal_focus;
        
        // Create the command prompt with appropriate styling
        let prompt = self.create_prompt();
        
        // Build the input area with prompt and input field
        row![
            prompt,
            text_input(
                TERMINAL_INPUT_ID,
                &self.terminal_input,
                Message::TerminalInput
            )
            .padding(5)
            .password(password_mode)
            .width(Length::Fill)
            .style(DraculaTheme::terminal_input_style(input_should_be_focused))
        ]
        .spacing(5)
        .width(Length::Fill)
        .into()
    }
    
    // Helper method to create the command prompt
    fn create_prompt(&self) -> Element<'_, Message> {
        // Implementation of create_prompt (needs to be added based on actual implementation)
        let prompt_text = if self.state.is_git_repo {
            // Show git branch in prompt if in a git repo
            if let Some(branch) = &self.state.git_branch {
                format!("{}@{} $ ", self.state.current_dir.display(), branch)
            } else {
                format!("{} $ ", self.state.current_dir.display())
            }
        } else {
            format!("{} $ ", self.state.current_dir.display())
        };
        
        text(&prompt_text)
            .font(Font::MONOSPACE)
            .style(DraculaTheme::prompt_style())
            .into()
    }
    
    // Helper method to render terminal output
    fn view_terminal_output(&self) -> Element<'_, Message> {
        // Create a scrollable container with the terminal output
        scrollable_container::terminal_output(
            &self.state.output,
            &self.state.command_status,
            self.state.command_receiver.is_some()
        )
    }
} 