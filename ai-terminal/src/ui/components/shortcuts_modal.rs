use iced::widget::{container, row, text, button, column};
use iced::{Element, Length};
use crate::ui::theme::DraculaTheme;
use crate::app::Message;

pub struct ShortcutsModal;

impl ShortcutsModal {
    pub fn view() -> Element<'static, Message> {
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
                    Self::shortcut_row("Ctrl+E", "Toggle focus between terminal and AI chat"),
                    Self::shortcut_row("Alt+Left", "Decrease terminal panel width"),
                    Self::shortcut_row("Alt+Right", "Increase terminal panel width"),
                ]
                .spacing(8)
            )
            .width(Length::Fill)
            .padding(10),
            
            // Section: History
            container(
                column![
                    text("History").size(16).style(DraculaTheme::PINK),
                    Self::shortcut_row("Up", "Previous command in history"),
                    Self::shortcut_row("Down", "Next command in history"),
                ]
                .spacing(8)
            )
            .width(Length::Fill)
            .padding(10),
            
            // Section: Commands
            container(
                column![
                    text("Commands").size(16).style(DraculaTheme::PINK),
                    Self::shortcut_row("Tab", "Autocomplete command"),
                    Self::shortcut_row("Ctrl+C", "Terminate running command"),
                    Self::shortcut_row("Shift+`", "Insert tilde character"),
                    Self::shortcut_row("Ctrl+F", "Toggle search in terminal"),
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

    fn shortcut_row<'a>(shortcut: &'a str, description: &'a str) -> Element<'a, Message> {
        row![
            container(
                text(shortcut)
                    .size(14)
                    .style(DraculaTheme::CYAN)
            )
            .width(Length::Fixed(120.0))
            .padding(5)
            .style(DraculaTheme::shortcut_key_style()),
            text(description)
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