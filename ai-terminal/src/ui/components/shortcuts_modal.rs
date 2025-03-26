use iced::widget::{container, row, text, button, column};
use iced::{Element, Length};
use crate::ui::theme::DraculaTheme;
use crate::ui::messages::Message;
use crate::config::keyboard::get_all_shortcuts;

pub struct ShortcutsModal;

impl ShortcutsModal {
    pub fn view() -> Element<'static, Message> {
        // Get all the shortcuts from the central keyboard definitions
        let all_shortcuts = get_all_shortcuts();
        
        // Create completely separate pre-processed lists for each category
        let mut nav_elements = Vec::new();
        let mut history_elements = Vec::new();
        let mut command_elements = Vec::new();
        
        // Process all shortcuts and categorize them
        for (shortcut, description) in all_shortcuts {
            let element = Self::shortcut_row(&shortcut, &description);
            
            if description.contains("focus") || description.contains("width") || description.contains("panel") {
                nav_elements.push(element);
            } else if description.contains("history") {
                history_elements.push(element);
            } else {
                command_elements.push(element);
            }
        }

        column![
            // Modal title
            container(
                row![
                    text("Keyboard Shortcuts")
                        .size(20)
                        .style(DraculaTheme::PURPLE),
                    iced::widget::horizontal_space(Length::Fill),
                    button(text("Close").size(14))
                        .on_press(Message::ToggleShortcutsModal)
                        .padding(5)
                        .style(DraculaTheme::close_button_style())
                ]
                .spacing(10)
                .width(Length::Fill)
            )
            .width(Length::Fill)
            .padding(5),
            
            // Section: Navigation
            container(
                column![
                    text("Navigation").size(16).style(DraculaTheme::PINK),
                    column(nav_elements).spacing(8)
                ]
                .spacing(8)
            )
            .width(Length::Fill)
            .padding(10),
            
            // Section: History
            container(
                column![
                    text("History").size(16).style(DraculaTheme::PINK),
                    column(history_elements).spacing(8)
                ]
                .spacing(8)
            )
            .width(Length::Fill)
            .padding(10),
            
            // Section: Commands
            container(
                column![
                    text("Commands").size(16).style(DraculaTheme::PINK),
                    column(command_elements).spacing(8)
                ]
                .spacing(8)
            )
            .width(Length::Fill)
            .padding(10),
        ]
        .spacing(10)
        .width(Length::Fill)
        .into()
    }

    fn shortcut_row(shortcut: &str, description: &str) -> Element<'static, Message> {
        // Create the row with owned String data to avoid lifetimes issues
        row![
            container(
                text(shortcut.to_string())
                    .size(14)
                    .style(DraculaTheme::CYAN)
            )
            .width(Length::Fixed(120.0))
            .padding(5)
            .style(DraculaTheme::shortcut_key_style()),
            text(description.to_string())
                .size(14)
                .style(DraculaTheme::FOREGROUND)
                .width(Length::Fill)
        ]
        .align_items(iced::alignment::Alignment::Center)
        .spacing(10)
        .width(Length::Fill)
        .into()
    }
} 