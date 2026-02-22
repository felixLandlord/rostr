use crate::ui::theme::*;
use iced::widget::{Space, button, column, container, row, text, text_input, tooltip};
use iced::{Alignment, Border, Color, Element, Length, Padding, Theme};
use lucide_icons::iced::{
    icon_bell, icon_calendar, icon_chevron_left, icon_chevron_right, icon_search, icon_settings,
};

pub struct TopBar;

#[derive(Debug, Clone)]
pub enum Message {
    PreviousDate,
    NextDate,
    SearchChanged(String),
    NotificationPressed,
    SettingsPressed,
    ToggleTheme,
}

impl TopBar {
    pub fn view<'a>(
        current_date: String,
        sub_text: String,
        search_value: &'a str,
        is_dark: bool,
    ) -> Element<'a, Message> {
        let border_color = if is_dark { BORDER_DARK } else { BORDER_LIGHT };
        let text_color = if is_dark { TEXT_DARK } else { TEXT_LIGHT };
        let muted_color = if is_dark {
            TEXT_MUTED_DARK
        } else {
            TEXT_MUTED_LIGHT
        };
        let icon_bg = Color { a: 0.1, ..PRIMARY };

        // Left Section: Logo and Date
        let logo = button(
            container(icon_calendar().size(22).color(PRIMARY))
                .width(40)
                .height(40)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .on_press(Message::ToggleTheme)
        .style(move |_, _status| button::Style {
            background: Some(icon_bg.into()),
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        });

        let date_nav = row![
            container(text(current_date).size(22).style(move |_| text::Style {
                color: Some(text_color)
            }))
            .width(165),
            row![
                tooltip(
                    button(icon_chevron_left().size(18).color(muted_color))
                        .on_press(Message::PreviousDate)
                        .padding(4)
                        .style(move |_, status| {
                            let bg = if is_dark { GRAY_800 } else { GRAY_50 };
                            let b_color = if status == button::Status::Hovered {
                                PRIMARY
                            } else {
                                border_color
                            };
                            button::Style {
                                background: Some(bg.into()),
                                border: Border {
                                    radius: 6.0.into(),
                                    color: b_color,
                                    width: 1.0,
                                },
                                ..button::Style::default()
                            }
                        }),
                    "Previous Month",
                    tooltip::Position::Bottom
                )
                .gap(8)
                .style(container::rounded_box),
                tooltip(
                    button(icon_chevron_right().size(18).color(muted_color))
                        .on_press(Message::NextDate)
                        .padding(4)
                        .style(move |_, status| {
                            let bg = if is_dark { GRAY_800 } else { GRAY_50 };
                            let b_color = if status == button::Status::Hovered {
                                PRIMARY
                            } else {
                                border_color
                            };
                            button::Style {
                                background: Some(bg.into()),
                                border: Border {
                                    radius: 6.0.into(),
                                    color: b_color,
                                    width: 1.0,
                                },
                                ..button::Style::default()
                            }
                        }),
                    "Next Month",
                    tooltip::Position::Bottom
                )
                .gap(8)
                .style(container::rounded_box),
            ]
            .spacing(4)
            .padding(4)
            .align_y(Alignment::Center)
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let left_section = row![
            logo,
            column![
                date_nav,
                text(sub_text).size(13).style(move |_| text::Style {
                    color: Some(muted_color)
                }),
            ]
            .spacing(4)
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        // Right Section: Search Bar
        let search_bar = container(
            row![
                icon_search().size(16).color(muted_color),
                text_input("Search employees...", search_value)
                    .on_input(Message::SearchChanged)
                    .size(14)
                    .style(move |_, _| {
                        text_input::Style {
                            background: Color::TRANSPARENT.into(),
                            border: Border::default(),
                            icon: muted_color,
                            placeholder: muted_color,
                            value: text_color,
                            selection: PRIMARY,
                        }
                    })
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding(Padding::from([0, 12])),
        )
        .width(240)
        .height(36)
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(if is_dark { SURFACE_DARK } else { GRAY_50 }.into()),
            border: Border {
                radius: 18.0.into(),
                color: border_color,
                width: 1.0,
            },
            ..container::Style::default()
        });

        let separator =
            container(Space::new().width(1.0).height(24.0)).style(move |_| container::Style {
                background: Some(border_color.into()),
                ..container::Style::default()
            });

        let right_section = row![
            search_bar,
            row![
                separator,
                tooltip(
                    button(container(icon_bell().size(20).color(muted_color)).padding(8))
                        .on_press(Message::NotificationPressed)
                        .style(move |_, status| {
                            let bg = if status == button::Status::Hovered {
                                if is_dark { GRAY_800 } else { GRAY_50 }
                            } else {
                                Color::TRANSPARENT
                            };
                            button::Style {
                                background: Some(bg.into()),
                                border: Border {
                                    radius: 20.0.into(),
                                    ..Border::default()
                                },
                                ..button::Style::default()
                            }
                        }),
                    "Notifications",
                    tooltip::Position::Bottom
                )
                .gap(8)
                .style(container::rounded_box),
                tooltip(
                    button(
                        container(icon_settings().size(20).color(muted_color))
                            .width(40)
                            .height(40)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                    )
                    .on_press(Message::SettingsPressed)
                    .style(move |_, status| {
                        let bg = if status == button::Status::Hovered {
                            if is_dark { GRAY_800 } else { GRAY_50 }
                        } else {
                            Color::TRANSPARENT
                        };
                        button::Style {
                            background: Some(bg.into()),
                            border: Border {
                                radius: 20.0.into(),
                                ..Border::default()
                            },
                            ..button::Style::default()
                        }
                    }),
                    "Settings",
                    tooltip::Position::Bottom
                )
                .gap(8)
                .style(container::rounded_box),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 20.0
            })
        ]
        .spacing(20)
        .align_y(Alignment::Center);

        container(
            row![
                left_section,
                Space::new().width(Length::Fill),
                right_section,
            ]
            .align_y(Alignment::Center)
            .padding(Padding::from([0, 32])),
        )
        .width(Length::Fill)
        .height(Length::Fixed(85.0))
        .align_y(Alignment::Center)
        .style(move |theme: &Theme| {
            let _palette = theme.palette();
            container::Style {
                background: Some(if is_dark { SURFACE_DARK } else { SURFACE_LIGHT }.into()),
                border: Border {
                    width: 1.0,
                    color: border_color,
                    ..Border::default()
                },
                ..container::Style::default()
            }
        })
        .into()
    }
}
