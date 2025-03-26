use iced::widget::text;
use iced::Element;
use crate::ui::theme::DraculaTheme;
use crate::app::Message;

pub fn git_branch_text(branch: &str) -> Element<Message> {
    text(format!("[{}]", branch))
        .size(12)
        .style(DraculaTheme::CYAN)
        .into()
} 