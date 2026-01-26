use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Theme};
use crate::theme;
use lucide_icons::iced::{icon_check, icon_info, icon_triangle_alert, icon_x};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Success,
    Error,
    Info,
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
    .align_x(Alignment::End);

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
        Status::Success => (icon_check(), Color::from_rgb(0.13, 0.65, 0.35)), // Green
        Status::Error => (icon_triangle_alert(), Color::from_rgb(0.85, 0.25, 0.25)), // Red
        Status::Info => (icon_info(), theme::PRIMARY), // Primary
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
            // Icon container
            container(
                icon
                    .size(20)
                    .style(move |_| text::Style { color: Some(apply_opacity(Color::WHITE)) })
            )
            .padding(8)
            .style(move |_| container::Style {
                background: Some(apply_opacity(base_color).into()),
                border: iced::border::Border {
                    radius: 8.0.into(),
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
            .style(move |_t, _s| button::Style {
                background: None,
                ..Default::default()
            })
            .padding(4)
        ]
        .spacing(16)
        .align_y(Alignment::Center)
    )
    .padding(12)
    .width(300)
    .style(move |t: &Theme| {
        let is_dark = t.palette().background.r < 0.5;
        container::Style {
            background: Some(apply_opacity(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }).into()),
            border: iced::border::Border {
                radius: 12.0.into(),
                width: 1.0,
                color: apply_opacity(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }),
            },
            shadow: iced::Shadow {
                color: apply_opacity(Color::from_rgba(0.0, 0.0, 0.0, 0.1)),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    });

    content.into()
}
