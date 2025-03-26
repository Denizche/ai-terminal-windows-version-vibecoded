use iced::widget::{container, text_input, column, row};
use iced::{Element, Length, Font};
use crate::ui::theme::DraculaTheme;
use crate::ui::messages::Message;
use crate::config::keyboard::FocusTarget;
use crate::ui::components::{styled_text, copy_button};
use crate::ui::components::scrollable_container;
use crate::model::App as AppState;

const AI_INPUT_ID: &str = "ai_input";

pub struct AiPanel {
    state: AppState,
    ai_input: String,
    focus: FocusTarget,
}

impl AiPanel {
    pub fn new(state: AppState, ai_input: String, focus: FocusTarget) -> Self {
        Self {
            state,
            ai_input,
            focus,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let output_elements = self.view_output_elements();
        let ai_output = scrollable_container::scrollable_container(output_elements);

        let input = text_input("Ask AI...", &self.ai_input)
            .on_input(Message::AIInput)
            .on_submit(Message::ProcessAIQuery)
            .padding(5)
            .font(Font::MONOSPACE)
            .size(12)
            .id(text_input::Id::new(AI_INPUT_ID))
            .style(if self.focus == FocusTarget::AiChat {
                DraculaTheme::focused_text_input_style()
            } else {
                DraculaTheme::text_input_style()
            });

        column![
            ai_output,
            input,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn view_output_elements(&self) -> Element<Message> {
        let mut blocks = Vec::new();
        let mut current_block = Vec::new();

        let visible_output = if self.state.ai_output.len() > 50 {
            self.state.ai_output.iter().skip(self.state.ai_output.len() - 50).cloned().collect()
        } else {
            self.state.ai_output.clone()
        };

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

        column(
            blocks.iter().enumerate().map(|(i, block)| {
                let show_copy = i >= self.state.initial_ai_output_count || 
                    !block.iter().any(|line| line.contains("instruction") || line.contains("welcome"));
                
                if show_copy {
                    container(
                        column![
                            container(
                                column(
                                    block.iter().map(|line| {
                                        styled_text(
                                            line,
                                            line.starts_with("> "),
                                            false,
                                            false,
                                            None
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
                    .style(DraculaTheme::command_block_style())
                    .into()
                } else {
                    container(
                        column(
                            block.iter().map(|line| {
                                styled_text(
                                    line,
                                    line.starts_with("> "),
                                    false,
                                    false,
                                    None
                                )
                            }).collect()
                        ).spacing(2)
                        .width(Length::Fill)
                    )
                    .padding(10)
                    .width(Length::Fill)
                    .style(DraculaTheme::command_block_style())
                    .into()
                }
            }).collect()
        )
        .spacing(10)
        .width(Length::Fill)
        .into()
    }

    /// Update the input value directly
    pub fn update_input(&mut self, input: String) {
        self.ai_input = input;
    }
} 