use iced::widget::{text, row, container};
use iced::{Element, Font, Length};

use crate::ui::theme::DraculaTheme;
use crate::ui::messages::Message;
use super::copy_button::copy_button;

pub fn styled_text<'a>(content: &str, is_command: bool, command_failed: bool, show_copy: bool, search_term: Option<&str>) -> Element<'a, Message> {
    let text_element = if let Some(term) = search_term {
        if term.is_empty() {
            text(content)
                .font(Font::MONOSPACE)
                .size(if is_command { 13 } else { 12 })
                .style(if is_command {
                    if command_failed {
                        DraculaTheme::error_command_text()
                    } else {
                        DraculaTheme::command_text()
                    }
                } else {
                    DraculaTheme::output_text()
                })
                .into()
        } else {
            let mut elements = Vec::new();
            let mut current_pos = 0;
            let content_lower = content.to_lowercase();
            let term_lower = term.to_lowercase();

            while let Some(pos) = content_lower[current_pos..].find(&term_lower) {
                let actual_pos = current_pos + pos;
                if actual_pos > current_pos {
                    elements.push(
                        text(&content[current_pos..actual_pos])
                            .font(Font::MONOSPACE)
                            .size(if is_command { 13 } else { 12 })
                            .style(if is_command {
                                if command_failed {
                                    DraculaTheme::error_command_text()
                                } else {
                                    DraculaTheme::command_text()
                                }
                            } else {
                                DraculaTheme::output_text()
                            })
                            .into()
                    );
                }
                elements.push(
                    text(&content[actual_pos..actual_pos + term.len()])
                        .font(Font::MONOSPACE)
                        .size(if is_command { 13 } else { 12 })
                        .style(DraculaTheme::search_highlight())
                        .into()
                );
                current_pos = actual_pos + term.len();
            }
            if current_pos < content.len() {
                elements.push(
                    text(&content[current_pos..])
                        .font(Font::MONOSPACE)
                        .size(if is_command { 13 } else { 12 })
                        .style(if is_command {
                            if command_failed {
                                DraculaTheme::error_command_text()
                            } else {
                                DraculaTheme::command_text()
                            }
                        } else {
                            DraculaTheme::output_text()
                        })
                        .into()
                );
            }
            row(elements).spacing(0).into()
        }
    } else {
        text(content)
            .font(Font::MONOSPACE)
            .size(if is_command { 13 } else { 12 })
            .style(if is_command {
                if command_failed {
                    DraculaTheme::error_command_text()
                } else {
                    DraculaTheme::command_text()
                }
            } else {
                DraculaTheme::output_text()
            })
            .into()
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
        text_element
    }
}

// Create styled text for git branch info
pub fn git_branch_text<'a>(branch_name: &str) -> Element<'a, crate::ui::messages::Message> {
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