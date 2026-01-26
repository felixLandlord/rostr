use iced::widget::{button, column, container, row, scrollable, text};
use iced::task::Task;
use iced::{Alignment, Color, Element, Length, Theme};
use std::time::Duration;

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
    selected_employee: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleStatus(usize, usize), // employee_index, day_index
    SelectEmployee(usize),
    AutoDeselect(usize),
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
            // Added Dummy Data
            Employee {
                name: "Alice Cooper".to_string(),
                role: "Marketing Lead".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Emmanuel Felix Nunoo".to_string(),
                role: "Sales Manager".to_string(),
                attendance: [
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "Charlie Brown".to_string(),
                role: "Intern".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Diana Prince".to_string(),
                role: "Security Analyst".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "Evan Wright".to_string(),
                role: "Data Scientist".to_string(),
                attendance: [
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Fiona Gallagher".to_string(),
                role: "HR Assistant".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "George Martin".to_string(),
                role: "Copywriter".to_string(),
                attendance: [
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Hannah Lee".to_string(),
                role: "QA Engineer".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Remote,
                ],
            },
            Employee {
                name: "Ian Somerhalder".to_string(),
                role: "System Admin".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                    AttendanceStatus::Remote,
                    AttendanceStatus::Office,
                ],
            },
            Employee {
                name: "Julia Roberts".to_string(),
                role: "Receptionist".to_string(),
                attendance: [
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                    AttendanceStatus::Office,
                ],
            },
        ];

        Self {
            employees,
            selected_employee: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleStatus(emp_idx, day_idx) => {
                if let Some(employee) = self.employees.get_mut(emp_idx) {
                    if day_idx < 5 {
                        employee.attendance[day_idx] = employee.attendance[day_idx].toggle();
                    }
                }
                Task::none()
            }
            Message::SelectEmployee(idx) => {
                if self.selected_employee == Some(idx) {
                    // Deselect if already selected
                    self.selected_employee = None;
                    Task::none()
                } else {
                    self.selected_employee = Some(idx);
                    // Auto-deselect after 15 seconds
                    Task::perform(
                        async move {
                            tokio::time::sleep(Duration::from_secs(15)).await;
                            idx
                        },
                        Message::AutoDeselect,
                    )
                }
            }
            Message::AutoDeselect(idx) => {
                if self.selected_employee == Some(idx) {
                    self.selected_employee = None;
                }
                Task::none()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.employees.len()
    }

    pub fn view(&self, is_dark: bool) -> Element<Message> {
        let days = ["MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY"];

        // Header
        let header = row![
            // Checkbox column spacer
            container(text(" ").size(14))
                .width(Length::Fixed(60.0))
                .padding([16, 8])
                .style(move |theme: &Theme| container::Style {
                    border: iced::border::Border {
                        color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                        width: 1.0,
                        radius: iced::border::Radius {
                            top_left: 12.0,
                            top_right: 0.0,
                            bottom_right: 0.0,
                            bottom_left: 0.0,
                        },
                    },
                    ..Default::default()
                }),
            container(
                row![
                    text("EMPLOYEE").size(12.5).font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .width(Length::FillPortion(2))
            .padding([17, 32])
            .style(move |theme: &Theme| container::Style {
                border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
        ]
        .push(row(days.iter().enumerate().map(|(i, day)| {
            let is_last = i == days.len() - 1;
            container(
                text(*day)
                    .size(14)
                    .font(iced::font::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([16, 16])
            .style(move |_t: &Theme| container::Style {
                 border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }, // Approximation
                    width: 1.0,
                    radius: if is_last {
                        iced::border::Radius {
                            top_left: 0.0,
                            top_right: 12.0,
                            bottom_right: 0.0,
                            bottom_left: 0.0,
                        }
                    } else {
                        0.0.into()
                    },
                },
                ..Default::default()
            })
            .into()
        }))
        .width(Length::FillPortion(5)))
        .spacing(0);

        // Rows
        let rows = column(
            self.employees
                .iter()
                .enumerate()
                .map(|(emp_idx, employee)| {
                    let is_selected = self.selected_employee == Some(emp_idx);

                    let checkbox_cell = container(
                        button(
                            container(
                                if is_selected {
                                    container("")
                                        .width(10)
                                        .height(10)
                                        .style(|_t: &Theme| container::Style {
                                            background: Some(theme::PRIMARY.into()), // Green dot
                                            border: iced::border::Border {
                                                radius: 5.0.into(),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                } else {
                                    container("").width(0).height(0)
                                }
                            )
                            .width(20)
                            .height(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(move |theme: &Theme| container::Style {
                                border: iced::border::Border {
                                    color: if is_selected {
                                        theme::PRIMARY
                                    } else {
                                        if theme == &Theme::Dark {
                                            theme::BORDER_DARK
                                        } else {
                                            theme::BORDER_LIGHT
                                        }
                                    },
                                    width: 1.5,
                                    radius: 4.0.into(),
                                },
                                ..Default::default()
                            })
                        )
                        .on_press(Message::SelectEmployee(emp_idx))
                        .padding(0)
                        .style(|_, _| button::Style::default()) // No default button bg
                    )
                    .width(Length::Fixed(60.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .padding([16, 8])
                    .style(move |theme: &Theme| container::Style {
                        background: Some(theme.palette().background.into()),
                        border: iced::border::Border {
                            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    });

                    let name_cell = container(
                        row![
                            text(&employee.name).size(16).font(iced::font::Font {
                                weight: iced::font::Weight::Semibold,
                                ..Default::default()
                            }),
                            text(&employee.role)
                                .size(12)
                                .style(|_t: &Theme| text::Style {
                                    color: Some(Color::from_rgb(0.6, 0.6, 0.6)), // Gray 400
                                })
                        ]
                        .spacing(12)
                        .align_y(Alignment::Center)
                    )
                    .width(Length::FillPortion(2))
                    .padding([16, 32])
                    .style(move |theme: &Theme| container::Style {
                         background: Some(theme.palette().background.into()),
                         border: iced::border::Border {
                            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                            width: 1.0,
                            radius: 0.0.into(),
                        },
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
                            .width(Length::Fill)
                            .align_x(Alignment::Center)
                            .padding([12, 16])
                            .style(move |_t: &Theme| container::Style {
                                border: iced::border::Border {
                                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                                    width: 1.0,
                                    radius: 0.0.into(),
                                },
                                ..Default::default()
                            })
                            .into()
                        })
                    )
                    .width(Length::FillPortion(5)).spacing(0);

                    row![checkbox_cell, name_cell, day_cells].into()
                })
        )
        .spacing(-1.0);

        // Footer
        let totals = (0..5).map(|day_idx| {
            self.employees.iter().filter(|e| e.attendance[day_idx] == AttendanceStatus::Office).count()
        }).collect::<Vec<_>>();

        let footer = row![
            // Checkbox column spacer
            container(text(" ").size(12))
                .width(Length::Fixed(60.0))
                .padding([16, 8])
                .style(move |theme: &Theme| container::Style {
                    border: iced::border::Border {
                        color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                        width: 1.0,
                        radius: iced::border::Radius {
                            top_left: 0.0,
                            top_right: 0.0,
                            bottom_right: 0.0,
                            bottom_left: 12.0,
                        },
                    },
                    ..Default::default()
                }),
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
            .width(Length::FillPortion(2))
            .padding([16, 32])
            .style(move |theme: &Theme| container::Style {
                border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
        ]
        .push(row(totals.iter().enumerate().map(|(i, count)| {
             let is_last = i == totals.len() - 1;
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
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([12, 16])
            .style(move |_t: &Theme| container::Style {
                border: iced::border::Border {
                    color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                    width: 1.0,
                    radius: if is_last {
                        iced::border::Radius {
                            top_left: 0.0,
                            top_right: 0.0,
                            bottom_right: 12.0,
                            bottom_left: 0.0,
                        }
                    } else {
                        0.0.into()
                    },
                },
                ..Default::default()
            })
            .into()
        }))
        .width(Length::FillPortion(5)))
        .spacing(0);

        // Main container
        let content = column![
            header,
            scrollable(rows)
                .direction(scrollable::Direction::Vertical(scrollable::Scrollbar::default()))
                .height(Length::Fill),
            footer
        ]
        .width(Length::Fill); // Was fixed

        container(
            content
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
