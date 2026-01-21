use crate::theme::*;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Padding};
use lucide_icons::iced::{
    icon_bell, icon_calendar, icon_chevron_left, icon_chevron_right, icon_search,
};

pub struct TopBar;

#[derive(Debug, Clone)]
pub enum Message {
    PreviousDate,
    NextDate,
    SearchChanged(String),
    NotificationPressed,
    ProfilePressed,
}

impl TopBar {
    pub fn view<'a>(
        current_date: &'a str,
        sub_text: &'a str,
        search_value: &'a str,
        is_dark: bool,
    ) -> Element<'a, Message> {
        let background_color = if is_dark { SURFACE_DARK } else { SURFACE_LIGHT };
        let border_color = if is_dark { BORDER_DARK } else { BORDER_LIGHT };
        let text_color = if is_dark { TEXT_DARK } else { TEXT_LIGHT };
        let muted_color = if is_dark {
            TEXT_MUTED_DARK
        } else {
            TEXT_MUTED_LIGHT
        };
        let icon_bg = Color { a: 0.1, ..PRIMARY };

        // Left Section: Logo and Date
        let logo = container(icon_calendar().size(24).color(PRIMARY))
            .width(40)
            .height(40)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_theme| container::Style {
                background: Some(icon_bg.into()),
                border: Border {
                    radius: 8.0.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            });

        let date_nav = row![
            text(current_date).size(24).style(move |_| text::Style {
                color: Some(text_color)
            }),
            row![
                button(icon_chevron_left().size(18).color(muted_color))
                    .on_press(Message::PreviousDate)
                    .padding(4)
                    .style(move |_, _| button::Style {
                        background: Some(if is_dark { GRAY_800 } else { GRAY_50 }.into()),
                        border: Border {
                            radius: 6.0.into(),
                            color: border_color,
                            width: 1.0,
                        },
                        ..button::Style::default()
                    }),
                button(icon_chevron_right().size(18).color(muted_color))
                    .on_press(Message::NextDate)
                    .padding(4)
                    .style(move |_, _| button::Style {
                        background: Some(if is_dark { GRAY_800 } else { GRAY_50 }.into()),
                        border: Border {
                            radius: 6.0.into(),
                            color: border_color,
                            width: 1.0,
                        },
                        ..button::Style::default()
                    }),
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
                text(sub_text).size(14).style(move |_| text::Style {
                    color: Some(muted_color)
                }),
            ]
            .spacing(2)
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        // Right Section: Search, separator, notifications, profile
        let search_bar = container(
            row![
                icon_search().size(16).color(muted_color),
                text_input("Search employees...", search_value)
                    .on_input(Message::SearchChanged)
                    .size(14)
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding(Padding {
                top: 0.0,
                right: 12.0,
                bottom: 0.0,
                left: 12.0,
            }),
        )
        .width(256)
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

        let separator = container(iced::widget::Space::with_width(Length::Fixed(1.0)))
            .height(24)
            .style(move |_| container::Style {
                background: Some(border_color.into()),
                ..container::Style::default()
            });

        let right_section = row![
            search_bar,
            row![
                separator,
                button(container(icon_bell().size(20).color(muted_color)).padding(8))
                    .on_press(Message::NotificationPressed)
                    .style(button::text),
                button(
                    container(iced::widget::Space::with_width(Length::Fixed(40.0)))
                        .height(40)
                        .style(move |_| container::Style {
                            background: Some(if is_dark { GRAY_800 } else { GRAY_50 }.into()),
                            border: Border {
                                radius: 20.0.into(),
                                color: border_color,
                                width: 1.0,
                            },
                            ..container::Style::default()
                        })
                )
                .on_press(Message::ProfilePressed)
                .style(button::text),
            ]
            .spacing(16)
            .align_y(Alignment::Center)
            .padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 24.0
            })
        ]
        .spacing(24)
        .align_y(Alignment::Center);

        container(
            row![
                left_section,
                iced::widget::horizontal_space(),
                right_section,
            ]
            .align_y(Alignment::Center)
            .padding(Padding {
                top: 0.0,
                right: 32.0,
                bottom: 0.0,
                left: 32.0,
            }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(72.0))
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(background_color.into()),
            border: Border {
                width: 1.0,
                color: border_color,
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
    }
}
