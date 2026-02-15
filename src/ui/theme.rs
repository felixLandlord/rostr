use iced::widget::{pick_list, text_input, button};
use iced::{Color, Border};

pub const PRIMARY: Color = Color::from_rgb(0.196, 0.505, 0.498); // #32817f
pub const PRIMARY_HOVER: Color = Color::from_rgb(0.157, 0.400, 0.392); // #286664

pub const BACKGROUND_LIGHT: Color = Color::from_rgb(0.988, 0.988, 0.988); // #fcfcfc
pub const BACKGROUND_DARK: Color = Color::from_rgb(0.106, 0.122, 0.137); // #1b1f23

pub const SURFACE_LIGHT: Color = Color::from_rgb(1.0, 1.0, 1.0); // #ffffff
pub const SURFACE_DARK: Color = Color::from_rgb(0.141, 0.161, 0.180); // #24292e

pub const BORDER_LIGHT: Color = Color::from_rgb(0.898, 0.906, 0.922); // #e5e7eb
pub const BORDER_DARK: Color = Color::from_rgb(0.216, 0.255, 0.318); // #374151

pub const TEXT_LIGHT: Color = Color::from_rgb(0.075, 0.086, 0.086); // #131616
pub const TEXT_DARK: Color = Color::from_rgb(0.945, 0.945, 0.945); // #f1f1f1

pub const TEXT_MUTED_LIGHT: Color = Color::from_rgb(0.420, 0.455, 0.502); // #6b7280
pub const TEXT_MUTED_DARK: Color = Color::from_rgb(0.612, 0.639, 0.686); // #9ca3af

pub const GRAY_50: Color = Color::from_rgb(0.976, 0.980, 0.984); // #f9fafb
pub const GRAY_800: Color = Color::from_rgb(0.122, 0.149, 0.192); // #1f2937

pub fn text_input_style(_theme: &iced::Theme, status: text_input::Status, is_dark: bool) -> text_input::Style {
    let bg = if is_dark { SURFACE_DARK } else { SURFACE_LIGHT };
    let border_color = match status {
        text_input::Status::Focused { .. } => PRIMARY,
        text_input::Status::Hovered => if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT },
        _ => if is_dark { BORDER_DARK } else { BORDER_LIGHT },
    };
    let text_color = if is_dark { TEXT_DARK } else { TEXT_LIGHT };
    let placeholder_color = if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT };

    text_input::Style {
        background: bg.into(),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border_color,
        },
        icon: placeholder_color,
        placeholder: placeholder_color,
        value: text_color,
        selection: PRIMARY,
    }
}

pub fn pick_list_style(_theme: &iced::Theme, status: pick_list::Status, is_dark: bool) -> pick_list::Style {
    let bg = if is_dark { SURFACE_DARK } else { SURFACE_LIGHT };
    let border_color = match status {
        pick_list::Status::Opened { .. } => PRIMARY,
        pick_list::Status::Hovered => if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT },
        _ => if is_dark { BORDER_DARK } else { BORDER_LIGHT },
    };
    let text_color = if is_dark { TEXT_DARK } else { TEXT_LIGHT };
    let placeholder_color = if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT };

    pick_list::Style {
        text_color,
        placeholder_color,
        handle_color: text_color,
        background: bg.into(),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border_color,
        },
    }
}

pub fn primary_button_style(_theme: &iced::Theme) -> button::Style {
    button::Style {
        background: Some(PRIMARY.into()),
        text_color: Color::WHITE,
        border: Border { radius: 8.0.into(), ..Default::default() },
        ..Default::default()
    }
}

pub fn secondary_button_style(_theme: &iced::Theme, is_dark: bool) -> button::Style {
    let bg = if is_dark { SURFACE_DARK } else { SURFACE_LIGHT };
    let text_color = if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT };
    let border_color = if is_dark { BORDER_DARK } else { BORDER_LIGHT };
    
    button::Style {
        background: Some(bg.into()),
        text_color,
        border: Border { radius: 8.0.into(), width: 1.0, color: border_color },
        ..Default::default()
    }
}
