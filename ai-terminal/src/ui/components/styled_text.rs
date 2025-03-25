use iced::widget::{text, row, container};
use iced::{Element, Font, Length};

use crate::ui::theme::DraculaTheme;
use crate::app::Message;
use super::copy_button::copy_button;

pub fn styled_text<'a>(content: &str, is_command: bool, command_failed: bool, show_copy: bool) -> Element<'a, Message> {
    let text_element = if is_command {
        if command_failed {
            text(content)
                .font(Font::MONOSPACE)
                .size(13)  // Slightly larger for commands to make them stand out
                .style(DraculaTheme::error_command_text())
        } else {
            text(content)
                .font(Font::MONOSPACE)
                .size(13)  // Slightly larger for commands to make them stand out
                .style(DraculaTheme::command_text())
        }
    } else {
        text(content)
            .font(Font::MONOSPACE)
            .size(12)
            .style(DraculaTheme::output_text())
    };
    
    // Only add copy button if show_copy is true
    if show_copy {
        row![
            text_element,
            container(copy_button(content.to_string()))
                .width(Length::Shrink)
                .padding(4)
        ]
        .spacing(5)
        .align_items(iced::Alignment::Center)
        .into()
    } else {
        text_element.into()
    }
}

// Create styled text for git branch info
pub fn git_branch_text<'a>(branch_name: &str) -> Element<'a, crate::app::Message> {
    row![
        text("git")
            .font(Font::MONOSPACE)
            .size(12)
            .style(DraculaTheme::YELLOW),
        text("(")
            .font(Font::MONOSPACE)
            .size(12)
            .style(DraculaTheme::output_text()),
        text(branch_name)
            .font(Font::MONOSPACE)
            .size(12)
            .style(DraculaTheme::GREEN),
        text(")")
            .font(Font::MONOSPACE)
            .size(12)
            .style(DraculaTheme::output_text())
    ]
    .spacing(0)
    .into()
} 