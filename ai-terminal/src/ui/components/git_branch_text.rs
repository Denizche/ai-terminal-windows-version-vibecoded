use iced::widget::{text, row};
use iced::{Element, Font};
use crate::ui::theme::DraculaTheme;
use crate::ui::messages::Message;

pub fn git_branch_text(branch: &str) -> Element<Message> {
    text(format!("[{}]", branch))
        .size(12)
        .style(DraculaTheme::CYAN)
        .into()
} 