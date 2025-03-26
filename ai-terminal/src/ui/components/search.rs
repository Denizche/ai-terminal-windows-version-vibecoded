use iced::widget::{container, row, text, text_input};
use iced::{Element, Length, Font};
use crate::ui::theme::DraculaTheme;
use crate::ui::messages::Message;

#[derive(Debug, Clone)]
pub struct SearchBar {
    input: String,
    current_index: usize,
    total_matches: usize,
    is_focused: bool,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            current_index: 0,
            total_matches: 0,
            is_focused: true, // Search bar starts focused when opened
        }
    }

    pub fn view(&self) -> Element<Message> {
        let count_text = if self.total_matches > 0 {
            format!("{}/{}", self.current_index + 1, self.total_matches)
        } else {
            String::new()
        };

        container(
            row![
                text_input("Search in terminal...", &self.input)
                    .on_input(Message::SearchInput)
                    .padding(8)
                    .font(Font::MONOSPACE)
                    .size(12)
                    .id(text_input::Id::new("search_input"))
                    .style(if self.is_focused {
                        DraculaTheme::focused_text_input_style()
                    } else {
                        DraculaTheme::text_input_style()
                    }),
                text(count_text)
                    .size(12)
                    .style(DraculaTheme::COMMENT)
                    .width(Length::Fixed(50.0)),
                iced::widget::button(text("Clear").size(12))
                    .on_press(Message::ClearSearch)
                    .padding(8)
                    .style(DraculaTheme::button_style()),
            ]
            .spacing(8)
            .padding(8)
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .style(DraculaTheme::current_dir_style())
        .into()
    }

    pub fn update_input(&mut self, input: String) {
        self.input = input;
        self.current_index = 0;
        self.total_matches = 0;
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.current_index = 0;
        self.total_matches = 0;
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn update_count(&mut self, current: usize, total: usize) {
        self.current_index = current;
        self.total_matches = total;
    }
    
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }
} 