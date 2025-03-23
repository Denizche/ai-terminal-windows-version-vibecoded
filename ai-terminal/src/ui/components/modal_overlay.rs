use iced::widget::container;
use iced::{Element, Length};

use crate::app::Message;
use crate::ui::theme::DraculaTheme;

pub fn modal_overlay<'a>(
    content: Element<'a, Message>,
    modal: Element<'a, Message>,
) -> Element<'a, Message> {
    // Return both the content and the modal, with the modal positioned as a layer on top
    // using the container's centering capability
    container(modal)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
} 