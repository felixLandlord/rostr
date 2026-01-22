use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Theme};

use crate::theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttendanceStatus {
    Office,
    Remote,
}

impl AttendanceStatus {
    fn toggle(&self) -> Self {
        match self {
            Self::Office => Self::Remote,
            Self::Remote => Self::Office,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub role: String,
    pub attendance: [AttendanceStatus; 5], // Mon-Fri
}

#[derive(Debug, Clone)]
pub struct AttendanceTable {
    employees: Vec<Employee>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleStatus(usize, usize), // employee_index, day_index
}

impl AttendanceTable {
    pub fn new() -> Self {
        // Dummy data
        let employees = vec![
            Employee {
                name: "Sarah Jenkins".to_string(),
                role: "UX Designer".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Michael Ross".to_string(),
                role: "Product Manager".to_string(),
                attendance: [
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Emily Chen".to_string(),
                role: "Frontend Dev".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "David Kim".to_string(),
                role: "Backend Dev".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Linda Martinez".to_string(),
                role: "HR Director".to_string(),
                attendance: [
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "Robert Fox".to_string(),
                role: "DevOps".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
        ];

        Self { employees }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleStatus(emp_idx, day_idx) => {
                if let Some(employee) = self.employees.get_mut(emp_idx) {
                    if day_idx < 5 {
                        employee.attendance[day_idx] = employee.attendance[day_idx].toggle();
                    }
                }
            }
        }
    }

    pub fn view(&self, is_dark: bool) -> Element<Message> {
        let days = ["MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY"];

        // Header
        let header = row![
            container(
                row![
                    text("EMPLOYEE").size(12).font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .width(Length::Fixed(300.0))
            .padding([16, 24]),
        ]
        .push(row(days.iter().map(|day| {
            container(
                text(*day)
                    .size(14)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
            )
            .width(Length::Fixed(150.0))
            .align_x(Alignment::Center)
            .padding([16, 16])
            .style(move |_t: &Theme| container::Style {
                 border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::GRAY_50 }, // Approximation
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
        })))
        .spacing(0);

        // Rows
        let rows = column(
            self.employees
                .iter()
                .enumerate()
                .map(|(emp_idx, employee)| {
                    let name_cell = container(
                        column![
                            text(&employee.name).size(14).font(iced::font::Font {
                                weight: iced::font::Weight::Semibold,
                                ..Default::default()
                            }),
                            text(&employee.role)
                                .size(12)
                                .style(|_t: &Theme| text::Style {
                                    color: Some(Color::from_rgb(0.6, 0.6, 0.6)), // Gray 400
                                })
                        ]
                    )
                    .width(Length::Fixed(300.0))
                    .padding([16, 24])
                    .style(move |theme: &Theme| container::Style {
                         background: Some(theme.palette().background.into()),
                         ..Default::default()
                    });

                    let day_cells = row(
                        employee.attendance.iter().enumerate().map(|(day_idx, status)| {
                            let (label, bg_color, text_color, border_color) = match status {
                                AttendanceStatus::Office => (
                                    "OFFICE",
                                    Color::from_rgba(0.196, 0.505, 0.498, 0.1), // Primary/10
                                    theme::PRIMARY,
                                    Color::from_rgba(0.196, 0.505, 0.498, 0.2), // Primary/20
                                ),
                                AttendanceStatus::Remote => (
                                    "REMOTE",
                                    if is_dark {
                                        Color::from_rgb(0.12, 0.12, 0.12)
                                    } else {
                                        Color::from_rgb(0.96, 0.96, 0.96) // Gray 100
                                    },
                                    if is_dark {
                                         Color::from_rgb(0.8, 0.8, 0.8)
                                    } else {
                                        Color::from_rgb(0.4, 0.45, 0.55) // Slate 600
                                    },
                                    if is_dark {
                                        Color::from_rgb(0.2, 0.2, 0.2)
                                    } else {
                                        Color::from_rgb(0.9, 0.9, 0.9) // Gray 200
                                    },
                                ),
                            };

                            container(
                                button(
                                    text(label)
                                        .size(11)
                                        .font(iced::font::Font {
                                            weight: iced::font::Weight::Bold,
                                            ..Default::default()
                                        })
                                        .align_x(Alignment::Center)
                                )
                                .on_press(Message::ToggleStatus(emp_idx, day_idx))
                                .padding([6, 16])
                                .style(move |_t: &Theme, status| {
                                    let base = button::Style {
                                        background: Some(bg_color.into()),
                                        text_color,
                                        border: iced::border::Border {
                                            color: border_color,
                                            width: 1.0,
                                            radius: 999.0.into(),
                                        },
                                        ..Default::default()
                                    };
                                    match status {
                                        button::Status::Hovered => button::Style {
                                            background: Some(Color { a: bg_color.a * 1.5, ..bg_color }.into()), // Slightly darker/more opaque
                                            ..base
                                        },
                                        _ => base,
                                    }
                                })
                            )
                            .width(Length::Fixed(150.0))
                            .align_x(Alignment::Center)
                            .padding([12, 16])
                            .style(move |_t: &Theme| container::Style {
                                border: iced::border::Border {
                                    color: if is_dark { theme::BORDER_DARK } else { theme::GRAY_50 },
                                    width: 1.0,
                                    radius: 0.0.into(),
                                },
                                ..Default::default()
                            })
                            .into()
                        })
                    ).spacing(0);

                    row![name_cell, day_cells].into()
                })
        );

        // Footer
        let totals = (0..5).map(|day_idx| {
            self.employees.iter().filter(|e| e.attendance[day_idx] == AttendanceStatus::Office).count()
        }).collect::<Vec<_>>();

        let footer = row![
            container(
                text("TOTAL EMPLOYEE COUNT")
                    .size(12)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(|_t: &Theme| text::Style {
                        color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                    })
            )
            .width(Length::Fixed(300.0))
            .padding([16, 24]),
        ]
        .push(row(totals.iter().map(|count| {
             container(
                container(
                    text(count.to_string())
                        .size(12)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                )
                .padding([4, 12])
                .style(move |theme: &Theme| {
                    let palette = theme.palette();
                    container::Style {
                        background: Some(if theme == &Theme::Dark {
                            Color::from_rgba(0.1, 0.3, 0.8, 0.3).into()
                        } else {
                             Color::from_rgb(0.85, 0.9, 1.0).into() // Blue 100
                        }),
                        text_color: Some(if theme == &Theme::Dark {
                             Color::from_rgb(0.6, 0.8, 1.0)
                        } else {
                             Color::from_rgb(0.1, 0.3, 0.6) // Blue 800
                        }),
                        border: iced::border::Border {
                            radius: 999.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
            )
            .width(Length::Fixed(150.0))
            .align_x(Alignment::Center)
            .padding([12, 16])
            .style(move |_t: &Theme| container::Style {
                border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::GRAY_50 },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
        })))
        .spacing(0);

        // Main container
        let content = column![
            header,
            scrollable(rows)
                .direction(scrollable::Direction::Vertical(scrollable::Scrollbar::default()))
                .height(Length::Fill),
            footer
        ]
        .width(Length::Fixed(1050.0));

        container(
            scrollable(content)
                .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::default()))
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .style(move |theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(palette.background.into()),
                border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
