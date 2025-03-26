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

    pub fn view(&self) -> Element<Message> {
                
        let output_elements = self.view_output_elements();
        let terminal_output = scrollable_container::scrollable_container(output_elements);

        let shortcuts_button = button(text("Shortcuts").size(14))
            .on_press(Message::ToggleShortcutsModal)
            .padding([4, 8])
            .style(DraculaTheme::button_style());

        let search_bar = if self.search_mode {
            self.search_bar.view()
        } else {
            container(row![]).into()
        };

        let top_bar = row![
            shortcuts_button,
            iced::widget::horizontal_space(Length::Fill),
            search_bar,
        ]
        .spacing(8)
        .padding([5, 10])
        .align_items(iced::alignment::Alignment::Center);

        let button_container = container(top_bar)
            .width(Length::Fill)
            .style(DraculaTheme::transparent_container_style());

        let current_dir = self.view_current_dir();
        let input = self.view_input();

        column![
            button_container,
            terminal_output,
            current_dir,
            input,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn view_output_elements(&self) -> Element<Message> {
        let mut blocks = Vec::new();
        let mut current_block = Vec::new();

        let visible_output = if self.state.output.len() > 2000 {
            self.state.output.iter().skip(self.state.output.len() - 2000).cloned().collect()
        } else {
            self.state.output.clone()
        };

        // Special handling for large output blocks (like directory listings)
        // Check if visible_output contains 'ls' command followed by many short lines (directory listing)
        let is_large_dir_listing = visible_output.iter()
            .any(|line| line.starts_with("> ls") || line.starts_with("> ls ")) &&
            visible_output.iter().filter(|line| line.len() < 60 && !line.starts_with(">")).count() > 50;
            
        // If this looks like a directory listing, use special formatting
        if is_large_dir_listing {
            // Find the command line index
            for (i, line) in visible_output.iter().enumerate() {
                if line.starts_with("> ls") {
                    // Get command line and all output after it
                    let mut command_block = vec![line.clone()];
                    if i+1 < visible_output.len() && visible_output[i+1] == "Running command: ls" {
                        command_block.push(visible_output[i+1].clone());
                    }
                    
                    // Add directory listing as a single block
                    let listing_start = if visible_output.get(i+1).map_or(false, |l| l == "Running command: ls") { i+2 } else { i+1 };
                    let directory_listing: Vec<String> = visible_output.iter()
                        .skip(listing_start)
                        .take(visible_output.len() - listing_start)
                        .cloned()
                        .collect();
                    
                    blocks.push(command_block);
                    blocks.push(directory_listing);
                    break;
                }
            }
        } else {
            // Standard output block processing
            for line in &visible_output {
                if line.starts_with("> ") && !current_block.is_empty() {
                    blocks.push(current_block);
                    current_block = Vec::new();
                }
                current_block.push(line.clone());
            }
            
            if !current_block.is_empty() {
                blocks.push(current_block);
            }
        }

        let mut block_status = self.state.command_status.clone();
        if blocks.len() > block_status.len() {
            block_status.insert(0, CommandStatus::Success);
        }

        column(
            blocks.iter().enumerate().map(|(i, block)| {
                let has_failed = i < block_status.len() && block_status[i] == CommandStatus::Failure;
                let style = if has_failed {
                    DraculaTheme::failure_command_block_style()
                } else {
                    DraculaTheme::command_block_style()
                };

                let show_copy = i >= self.state.initial_output_count || 
                    (block.iter().any(|line| line.starts_with("> ")) && 
                     !block.iter().any(|line| line.contains("Welcome") || line.contains("Operating System")) && 
                     !self.state.command_history.is_empty());
                
                if show_copy {
                    container(
                        column![
                            container(
                                column(
                                    block.iter().map(|line| {
                                        styled_text(
                                            line,
                                            line.starts_with("> "),
                                            line.starts_with("> ") && has_failed,
                                            false,
                                            if self.search_mode { Some(&self.search_bar.get_input()) } else { None }
                                        )
                                    }).collect()
                                ).spacing(2)
                                .width(Length::Fill)
                            )
                            .padding(10)
                            .width(Length::Fill),
                            container(
                                row![
                                    iced::widget::horizontal_space(Length::Fill),
                                    copy_button(block.join("\n\n"))
                                ]
                            )
                            .padding([0, 10, 10, 10])
                        ]
                    )
                    .width(Length::Fill)
                    .style(style)
                    .into()
                } else {
                    container(
                        column(
                            block.iter().map(|line| {
                                styled_text(
                                    line,
                                    line.starts_with("> "),
                                    line.starts_with("> ") && has_failed,
                                    false,
                                    if self.search_mode { Some(&self.search_bar.get_input()) } else { None }
                                )
                            }).collect()
                        ).spacing(2)
                        .width(Length::Fill)
                    )
                    .padding(10)
                    .width(Length::Fill)
                    .style(style)
                    .into()
                }
            }).collect()
        )
        .spacing(10)
        .width(Length::Fill)
        .into()
    }

    fn view_current_dir(&self) -> Element<Message> {
        let dir_path = if let Some(home) = dirs_next::home_dir() {
            if let Ok(path) = self.state.current_dir.strip_prefix(&home) {
                format!("~/{}", path.display())
            } else {
                self.state.current_dir.display().to_string()
            }
        } else {
            self.state.current_dir.display().to_string()
        };

        let current_dir_content = if self.state.is_git_repo {
            if let Some(branch) = &self.state.git_branch {
                row![
                    styled_text(&dir_path, false, false, false, if self.search_mode { Some(&self.search_bar.get_input()) } else { None }),
                    styled_text(" ", false, false, false, if self.search_mode { Some(&self.search_bar.get_input()) } else { None }),
                    git_branch_text(branch)
                ]
            } else {
                row![styled_text(&dir_path, false, false, false, if self.search_mode { Some(&self.search_bar.get_input()) } else { None })]
            }
        } else {
            row![styled_text(&dir_path, false, false, false, if self.search_mode { Some(&self.search_bar.get_input()) } else { None })]
        };

        container(current_dir_content)
            .padding(5)
            .width(Length::Fill)
            .style(DraculaTheme::current_dir_style())
            .into()
    }

    fn view_input(&self) -> Element<Message> {     
        if self.state.password_mode {
            text_input("Enter password...", &self.terminal_input)
                .on_input(Message::TerminalInput)
                .on_submit(Message::SendInput(self.terminal_input.clone()))
                .password()
                .padding(5)
                .font(Font::MONOSPACE)
                .size(12)
                .id(text_input::Id::new(TERMINAL_INPUT_ID))
                .style(if self.focus == FocusTarget::Terminal && (!self.search_mode || self.terminal_focus) {
                    DraculaTheme::focused_text_input_style()
                } else {
                    DraculaTheme::text_input_style()
                })
                .into()
        } else {
            let input = text_input("Enter command...", &self.terminal_input)
                .on_input(Message::TerminalInput)
                .on_submit(Message::ExecuteCommand)
                .padding(5)
                .font(Font::MONOSPACE)
                .size(12)
                .id(text_input::Id::new(TERMINAL_INPUT_ID));

            // Determine if this input should appear focused
            let is_focused = self.focus == FocusTarget::Terminal && (!self.search_mode || self.terminal_focus);            
            let styled_input = if is_focused {
                input.style(DraculaTheme::focused_text_input_style())
            } else {
                input.style(DraculaTheme::text_input_style())
            };

            styled_input.into()
        }
    }

    pub fn update_search_input(&mut self, input: String) {
        self.search_bar.update_input(input);
    }

    pub fn update_search_count(&mut self, current: usize, total: usize) {
        self.search_bar.update_count(current, total);
    }

    pub fn clear_search(&mut self) {
        self.search_bar.clear();
    }

    pub fn set_search_mode(&mut self, mode: bool) {
        self.search_mode = mode;
    }

    pub fn set_terminal_focus(&mut self, focus: bool) {
        self.terminal_focus = focus;
        self.search_bar.set_focused(!focus);
    }

    pub fn force_refresh(&mut self) {
        self.force_refresh = true;
        // Also increment view_update_id to ensure the view is seen as changed
        self.view_update_id = self.view_update_id.wrapping_add(1);
    }
} 