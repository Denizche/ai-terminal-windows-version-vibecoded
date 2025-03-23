use iced::{Color, Theme, Background, BorderRadius};
use iced::widget::{container, text_input};
use iced::widget::container::Appearance;

// Official Dracula Theme Colors
pub struct DraculaTheme;

struct TextInputStyle;

impl text_input::StyleSheet for TextInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: DraculaTheme::CURRENT_LINE.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: DraculaTheme::PURPLE,
            icon_color: DraculaTheme::FOREGROUND,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::COMMENT
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::FOREGROUND
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::SELECTION
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::COMMENT
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: DraculaTheme::BACKGROUND.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: DraculaTheme::COMMENT,
            icon_color: DraculaTheme::COMMENT,
        }
    }
}

struct FocusedTextInputStyle;

impl text_input::StyleSheet for FocusedTextInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: DraculaTheme::CURRENT_LINE.into(),
            border_radius: 4.0.into(),
            border_width: 2.0,
            border_color: DraculaTheme::PURPLE,
            icon_color: DraculaTheme::FOREGROUND,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::COMMENT
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::FOREGROUND
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::SELECTION
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        DraculaTheme::COMMENT
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: DraculaTheme::BACKGROUND.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: DraculaTheme::COMMENT,
            icon_color: DraculaTheme::COMMENT,
        }
    }
}

impl DraculaTheme {
    pub const BACKGROUND: Color = Color::from_rgb(
        0x28 as f32 / 255.0,
        0x2A as f32 / 255.0,
        0x36 as f32 / 255.0,
    );
    
    pub const CURRENT_LINE: Color = Color::from_rgb(
        0x44 as f32 / 255.0,
        0x47 as f32 / 255.0,
        0x5A as f32 / 255.0,
    );
    
    pub const SELECTION: Color = Color::from_rgb(
        0x44 as f32 / 255.0,
        0x47 as f32 / 255.0,
        0x5A as f32 / 255.0,
    );
    
    pub const FOREGROUND: Color = Color::from_rgb(
        0xF8 as f32 / 255.0,
        0xF8 as f32 / 255.0,
        0xF2 as f32 / 255.0,
    );
    
    pub const COMMENT: Color = Color::from_rgb(
        0x6272A4 as f32 / 255.0,
        0x62 as f32 / 255.0,
        0xA4 as f32 / 255.0,
    );
    
    pub const CYAN: Color = Color::from_rgb(
        0x8B as f32 / 255.0,
        0xE9 as f32 / 255.0,
        0xFD as f32 / 255.0,
    );
    
    pub const GREEN: Color = Color::from_rgb(
        0x50 as f32 / 255.0,
        0xFA as f32 / 255.0,
        0x7B as f32 / 255.0,
    );
    
    pub const ORANGE: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0xB8 as f32 / 255.0,
        0x6C as f32 / 255.0,
    );
    
    pub const PINK: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0x79 as f32 / 255.0,
        0xC6 as f32 / 255.0,
    );
    
    pub const PURPLE: Color = Color::from_rgb(
        0xBD as f32 / 255.0,
        0x93 as f32 / 255.0,
        0xF9 as f32 / 255.0,
    );
    
    pub const RED: Color = Color::from_rgb(
        0xFF as f32 / 255.0,
        0x55 as f32 / 255.0,
        0x55 as f32 / 255.0,
    );
    
    pub const YELLOW: Color = Color::from_rgb(
        0xF1 as f32 / 255.0,
        0xFA as f32 / 255.0,
        0x8C as f32 / 255.0,
    );

    pub fn text_input_style() -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(TextInputStyle))
    }

    pub fn container_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb(0.156, 0.164, 0.211))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn drag_handle_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: Some(Self::COMMENT),
            background: Some(Background::Color(Self::CURRENT_LINE)),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn command_text() -> Color {
        Self::GREEN
    }

    pub fn error_command_text() -> Color {
        Self::RED
    }

    pub fn output_text() -> Color {
        Self::FOREGROUND
    }

    pub fn command_block_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb(0.117, 0.125, 0.172))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn success_command_block_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: Some(Self::FOREGROUND),
            background: Some(Background::Color(Color::from_rgba8(40, 100, 40, 0.15))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn failure_command_block_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba8(100, 40, 40, 0.15))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn current_dir_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb(0.117, 0.125, 0.172))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn focused_text_input_style() -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(FocusedTextInputStyle))
    }

    pub fn yellow_text_style() -> iced::theme::Text {
        iced::theme::Text::Color(Color::from_rgb(0.945, 0.776, 0.459))
    }

    pub fn button_style() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ButtonStyle))
    }

    pub fn close_button_style() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(CloseButtonStyle))
    }

    pub fn modal_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba(0.16, 0.17, 0.21, 0.90))),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: Self::PURPLE,
        })
    }

    pub fn modal_overlay_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.7))),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn shortcut_key_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: Some(Background::Color(Self::CURRENT_LINE)),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }

    pub fn transparent_container_style() -> Box<dyn Fn(&Theme) -> container::Appearance> {
        Box::new(|_| container::Appearance {
            text_color: None,
            background: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        })
    }
}

struct ButtonStyle;

impl iced::widget::button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(Background::Color(DraculaTheme::PURPLE)),
            text_color: DraculaTheme::FOREGROUND,
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }
    
    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);
        iced::widget::button::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                0xCE as f32 / 255.0,
                0xA4 as f32 / 255.0,
                0xFF as f32 / 255.0,
            ))),
            ..active
        }
    }
}

struct CloseButtonStyle;
impl iced::widget::button::StyleSheet for CloseButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(Background::Color(DraculaTheme::RED)),
            text_color: DraculaTheme::FOREGROUND,
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);
        iced::widget::button::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                0xFF as f32 / 255.0,
                0x35 as f32 / 255.0,
                0x35 as f32 / 255.0,
            ))),
            ..active
        }
    }
} 