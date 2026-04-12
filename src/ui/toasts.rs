use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Color, Element, Length, Theme, Border, Shadow, Vector};
use crate::ui::theme;
use lucide_icons::iced::{
    icon_check, icon_info, icon_triangle_alert, icon_x, 
    icon_upload, icon_download, icon_pencil, icon_trash_2, icon_undo_2, icon_sparkles
};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Success,
    Error,
    Info,
    Warning,
    Import,
    Export,
    Add,
    Edit,
    Delete,
    Undo,
    Generate,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub status: Status,
    pub created_at: Instant,
}

pub fn view<'a, Message>(toasts: &'a [Toast], on_close: impl Fn(u64) -> Message + 'a + Clone) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = column(
        toasts.iter().map(|toast| {
            toast_view(toast, on_close.clone())
        })
    )
    .spacing(10)
    .align_x(Alignment::End)
    .height(Length::Shrink);

    container(content)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill) // Ensure it takes full height to position at top
        .align_x(Alignment::End) // Align horizontally to the right
        .align_y(Alignment::Start) // Align vertically to the top
        .into()
}

fn toast_view<'a, Message>(toast: &'a Toast, on_close: impl Fn(u64) -> Message + 'a) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let (icon, base_color) = match toast.status {
        Status::Success => (icon_check(), theme::TOAST_SUCCESS),
        Status::Error => (icon_triangle_alert(), theme::TOAST_ERROR),
        Status::Info => (icon_info(), theme::TOAST_INFO),
        Status::Warning => (icon_triangle_alert(), theme::TOAST_WARNING),
        Status::Import => (icon_upload(), theme::TOAST_SUCCESS),
        Status::Export => (icon_download(), theme::TOAST_INFO),
        Status::Add => (icon_check(), theme::TOAST_SUCCESS),
        Status::Edit => (icon_pencil(), theme::TOAST_INFO),
        Status::Delete => (icon_trash_2(), theme::TOAST_ERROR),
        Status::Undo => (icon_undo_2(), theme::TOAST_WARNING),
        Status::Generate => (icon_sparkles(), theme::TOAST_PRIMARY),
    };

    let elapsed = toast.created_at.elapsed().as_millis() as f32;
    let duration = 300.0;
    let opacity_value = if elapsed < duration {
        elapsed / duration
    } else {
        1.0
    };

    let apply_opacity = move |mut c: Color| {
        c.a *= opacity_value;
        c
    };

    let content = container(
        row![
            // Left colored strip
            container(Space::new())
                .width(Length::Fixed(4.0))
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(apply_opacity(base_color).into()),
                    ..Default::default()
                }),

            // Main content
            row![
                // Icon container
                container(
                    icon
                        .size(20)
                        .style(move |_| text::Style { color: Some(apply_opacity(base_color)) })
                )
                .padding(8)
                .style(move |_| container::Style {
                    background: Some(apply_opacity(Color { a: 0.1, ..base_color }).into()),
                    border: Border {
                        radius: 20.0.into(), // Full circle
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                
                // Text content
                column![
                    text(&toast.title)
                        .size(14)
                        .font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() })
                        .style(move |t: &Theme| text::Style { 
                            color: Some(apply_opacity(if t.palette().background.r < 0.5 { theme::TEXT_DARK } else { theme::TEXT_LIGHT })) 
                        }),
                    text(&toast.body)
                        .size(12)
                        .style(move |t: &Theme| text::Style { 
                            color: Some(apply_opacity(if t.palette().background.r < 0.5 { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT })) 
                        })
                ].spacing(2).width(Length::Fill),

                // Close button
                button(
                    icon_x().size(16).style(move |t: &Theme| text::Style { 
                         color: Some(apply_opacity(if t.palette().background.r < 0.5 { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT })) 
                    })
                )
                .on_press(on_close(toast.id))
                .style(move |_t, status| {
                    let text_c: Color = if status == button::Status::Hovered {
                        apply_opacity(Color::from_rgb(0.4, 0.4, 0.4))
                    } else {
                        apply_opacity(Color::from_rgb(0.6, 0.6, 0.6))
                    };
                    button::Style {
                        background: None,
                        text_color: text_c,
                        ..Default::default()
                    }
                })
                .padding(4)
            ]
            .spacing(12)
            .padding(12)
            .align_y(Alignment::Center)
            .width(Length::Fill)
        ]
    )
    .width(340)
    .height(Length::Shrink)
    .clip(true)
    .style(move |t: &Theme| {
        let is_dark = t.palette().background.r < 0.5;
        container::Style {
            background: Some(apply_opacity(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }).into()),
            border: Border {
                radius: 12.0.into(),
                width: 1.0,
                color: apply_opacity(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }),
            },
            shadow: Shadow {
                color: apply_opacity(Color::from_rgba(0.0, 0.0, 0.0, 0.1)),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    });

    content.into()
}
