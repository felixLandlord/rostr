use crate::ui::theme::*;
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Border, Color, Element, Theme};
use lucide_icons::iced::{
    icon_chart_no_axes_column, icon_download, icon_pencil, icon_undo_2, icon_save,
    icon_sparkles, icon_trash_2, icon_upload, icon_user_plus,
};

pub struct ActionBar;

#[derive(Debug, Clone)]
pub enum Message {
    Generate,
    Save,
    Undo,
    AddEmployee,
    EditEmployee,
    DeleteEmployee,
    Report,
    Import,
    Export,
}

impl ActionBar {
    pub fn view<'a>(is_dark: bool, has_selection: bool, is_read_only: bool) -> Element<'a, Message> {
        let border_color = if is_dark { BORDER_DARK } else { BORDER_LIGHT };
        let _text_color = if is_dark { TEXT_DARK } else { TEXT_LIGHT };
        let muted_color = if is_dark {
            TEXT_MUTED_DARK
        } else {
            TEXT_MUTED_LIGHT
        };

        // Generate Button (Primary)
        let generate_btn = if is_read_only {
            button(
                row![
                    icon_sparkles().size(18).color(muted_color),
                    text("Generate").size(14).font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(move |_| text::Style { color: Some(muted_color) })
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 16])
            .style(move |_, _| {
                button::Style {
                    background: Some(if is_dark { SURFACE_DARK } else { SURFACE_LIGHT }.into()),
                    border: Border {
                        radius: 8.0.into(),
                        color: if is_dark { BORDER_DARK } else { BORDER_LIGHT },
                        width: 1.0,
                    },
                    ..button::Style::default()
                }
            })
        } else {
            button(
                row![
                    icon_sparkles().size(18).color(Color::WHITE),
                    text("Generate").size(14).font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::Generate)
            .padding([8, 16])
            .style(move |_, status| {
                let bg = if status == button::Status::Hovered {
                    PRIMARY_HOVER
                } else {
                    PRIMARY
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        radius: 8.0.into(),
                        ..Border::default()
                    },
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            })
        };

        // Save Button (Outline)
        let save_btn: Element<'a, Message> = if is_read_only {
             ghost_button(
                icon_save(),
                "Save",
                None, // Disabled
                is_dark,
                muted_color,
                false,
            ).into()
        } else {
            button(
                row![
                    icon_save().size(18).color(muted_color),
                    text("Save").size(14).font(iced::font::Font {
                        weight: iced::font::Weight::Semibold,
                        ..Default::default()
                    })
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::Save)
            .padding([8, 16])
            .style(move |_, status| {
                let (bg, border_col) = if status == button::Status::Hovered {
                    (
                        if is_dark { GRAY_800 } else { Color::WHITE },
                        PRIMARY,
                    )
                } else {
                    (
                        if is_dark { GRAY_800 } else { Color::WHITE },
                        if is_dark { GRAY_800 } else { BORDER_LIGHT },
                    )
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        radius: 8.0.into(),
                        color: border_col,
                        width: 1.0,
                    },
                    text_color: if is_dark { TEXT_DARK } else { TEXT_LIGHT },
                    ..button::Style::default()
                }
            }).into()
        };

        // Undo Button (Ghost)
        let undo_btn = ghost_button(
            icon_undo_2(),
            "Undo Changes",
            if is_read_only { None } else { Some(Message::Undo) },
            is_dark,
            muted_color,
            false,
        );

        // Add Employee (Ghost)
        let add_btn = ghost_button(
            icon_user_plus(),
            "Add Employee",
            Some(Message::AddEmployee),
            is_dark,
            muted_color,
            false,
        );

        // Edit Employee (Ghost)
        let edit_btn = ghost_button(
            icon_pencil(),
            "Edit Employee",
            if has_selection { Some(Message::EditEmployee) } else { None },
            is_dark,
            muted_color,
            false,
        );

        // Delete Employee (Red Ghost)
        let delete_btn = ghost_button(
            icon_trash_2(),
            "Delete Employee",
            if has_selection { Some(Message::DeleteEmployee) } else { None },
            is_dark,
            muted_color,
            true,
        );

        // Report (Ghost)
        let report_btn = ghost_button(
            icon_chart_no_axes_column(),
            "Generation Report",
            Some(Message::Report),
            is_dark,
            muted_color,
            false,
        );

        // Import (Ghost)
        let import_btn = ghost_button(
            icon_upload(),
            "Import",
            Some(Message::Import),
            is_dark,
            muted_color,
            false,
        );

        // Export (Ghost)
        let export_btn = ghost_button(
            icon_download(),
            "Export",
            Some(Message::Export),
            is_dark,
            muted_color,
            false,
        );

        let separator = || {
            container(Space::new().width(1.0).height(24.0)).style(move |_| container::Style {
                background: Some(
                    if is_dark {
                        BORDER_DARK
                    } else {
                        BORDER_LIGHT
                    }
                    .into(),
                ),
                ..container::Style::default()
            })
        };

        container(
            row![
                generate_btn,
                Space::new().width(iced::Length::Fill),
                save_btn,
                Space::new().width(iced::Length::Fill),
                undo_btn,
                Space::new().width(iced::Length::Fill),
                separator(),
                Space::new().width(iced::Length::Fill),
                add_btn,
                Space::new().width(iced::Length::Fill),
                edit_btn,
                Space::new().width(iced::Length::Fill),
                delete_btn,
                Space::new().width(iced::Length::Fill),
                separator(),
                Space::new().width(iced::Length::Fill),
                report_btn,
                Space::new().width(iced::Length::Fill),
                import_btn,
                Space::new().width(iced::Length::Fill),
                export_btn,
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding(8),
        ).width(iced::Length::Fill)
        .style(move |theme: &Theme| {
            let _palette = theme.palette();
            container::Style {
                background: Some(if is_dark { SURFACE_DARK } else { SURFACE_LIGHT }.into()),
                border: Border {
                    width: 1.0,
                    color: border_color,
                    radius: 12.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.03),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            }
        })
        .into()
    }
}

fn ghost_button<'a, Message: Clone + 'a>(
    icon: iced::widget::Text<'a>,
    label: &'a str,
    msg: Option<Message>,
    is_dark: bool,
    muted_color: Color,
    is_danger: bool,
) -> Element<'a, Message> {
    let is_disabled = msg.is_none();
    let btn = button(
        row![
            icon.size(18).style(move |_| text::Style {
                color: Some(if is_disabled {
                     if is_dark { Color::from_rgb(0.3, 0.3, 0.3) } else { Color::from_rgb(0.8, 0.8, 0.8) }
                } else if is_danger {
                    Color::from_rgb(0.9, 0.2, 0.2)
                } else {
                    muted_color
                })
            }),
            text(label).size(14).font(iced::font::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .style(move |_| text::Style {
                color: Some(if is_disabled {
                     if is_dark { Color::from_rgb(0.3, 0.3, 0.3) } else { Color::from_rgb(0.8, 0.8, 0.8) }
                } else {
                     if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT } // Default text color
                })
            })
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([8, 12]);

    let btn = if let Some(m) = msg {
        btn.on_press(m)
    } else {
        btn
    };

    btn.style(move |_, status| {
        if is_disabled {
            return button::Style {
                background: None,
                text_color: if is_dark { Color::from_rgb(0.3, 0.3, 0.3) } else { Color::from_rgb(0.8, 0.8, 0.8) },
                ..button::Style::default()
            };
        }

        let bg = if status == button::Status::Hovered {
            if is_danger {
                if is_dark {
                    Color::from_rgba(0.5, 0.0, 0.0, 0.2)
                } else {
                    Color::from_rgb(1.0, 0.95, 0.95)
                }
            } else {
                if is_dark { GRAY_800 } else { GRAY_50 }
            }
        } else {
            Color::TRANSPARENT
        };
        
        let text_col = if status == button::Status::Hovered && is_danger {
             Color::from_rgb(0.8, 0.0, 0.0)
        } else if is_danger {
             Color::from_rgb(0.6, 0.2, 0.2)
        } else {
            if is_dark { TEXT_MUTED_DARK } else { TEXT_MUTED_LIGHT }
        };

        button::Style {
            background: Some(bg.into()),
            text_color: text_col,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        }
    })
    .into()
}
