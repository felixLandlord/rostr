use iced::widget::{button, column, container, pick_list, row, text, text_input, scrollable, Space};
use iced::{Alignment, Color, Element, Length, Padding, Theme};
use lucide_icons::iced::{
    icon_calendar, icon_calendar_check_2, icon_chart_no_axes_column,
    icon_settings, icon_trash_2, icon_user_plus, icon_user_round_pen, icon_x,
};
use crate::ui::theme;
use crate::ui::attendance_table::{AttendanceStatus, Employee};
use crate::core::models::types::Weekday;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReportData {
    pub date: String,
    pub total_employees: usize,
    pub total_males: usize,
    pub total_females: usize,
    pub males_per_day: [usize; 5],
    pub females_per_day: [usize; 5],
    pub roles_per_day: [HashMap<String, usize>; 5],
    pub day_most_attendance: String,
    pub day_least_attendance: String,
    pub daily_remote_percentage: [f32; 5],
    pub daily_office_percentage: [f32; 5],
    pub office_utilization_per_day: [f32; 5],
}

#[derive(Debug, Clone)]
pub enum Modal {
    None,
    AddEmployee(EmployeeForm),
    EditEmployee(usize, EmployeeForm),
    DeleteEmployee(usize),
    GeneralReport(ReportData),
    EmployeeReport(Employee, String),
    Settings,
    ConfirmResetApp,
    ConfirmResetSchedules,
    ConfirmResetEmployees,
}

#[derive(Debug, Clone)]
pub struct EmployeeForm {
    pub name: String,
    pub role: Option<String>,
    pub sex: Option<String>,
    pub days_per_week: Option<u8>,
    pub mentee: Vec<String>,
    pub mentor: Option<String>,
    pub attendance: [AttendanceStatus; 5],
    pub available_employees: Vec<String>,
}

impl EmployeeForm {
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
            && self.role.is_some()
            && self.sex.is_some()
            && self.days_per_week.is_some()
    }
}

impl Default for EmployeeForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            role: None,
            sex: None,
            days_per_week: None,
            mentee: Vec::new(),
            mentor: None,
            attendance: [AttendanceStatus::NA; 5],
            available_employees: Vec::new(),
        }
    }
}

impl From<&Employee> for EmployeeForm {
    fn from(e: &Employee) -> Self {
        let mut attendance = [AttendanceStatus::NA; 5];
        for day in &e.fixed_days {
             let idx = match day {
                 Weekday::Monday => 0,
                 Weekday::Tuesday => 1,
                 Weekday::Wednesday => 2,
                 Weekday::Thursday => 3,
                 Weekday::Friday => 4,
             };
             attendance[idx] = AttendanceStatus::Office;
        }

        Self {
            name: e.name.clone(),
            role: Some(e.role.clone()),
            sex: Some(e.sex.clone()),
            days_per_week: Some(e.days_per_week),
            mentee: e.mentee.clone(),
            mentor: e.mentor.clone(),
            attendance,
            available_employees: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Cancel,
    SubmitAdd,
    SubmitEdit(usize),
    ConfirmDelete(usize),
    NameChanged(String),
    RoleSelected(String),
    SexSelected(String),
    DaysChanged(u8),
    MenteeSelected(String),
    MenteeRemoved(String),
    MentorSelected(String),
    ToggleDay(usize),
    OverlayPressed,
    Close,
    DownloadPdf,
    ResetApp,
    ResetSchedules,
    ResetEmployees,
    ConfirmResetAppAction,
    ConfirmResetSchedulesAction,
    ConfirmResetEmployeesAction,
}

use crate::core::models::types::{Sex, Role, DAYS_OPTIONS};

pub fn view<'a>(modal: &'a Modal, is_dark: bool) -> Element<'a, Message> {
    match modal {
        Modal::None => text("").into(),
        Modal::AddEmployee(form) => form_view(form, "Add New Employee", "Add Employee", "person_add", false, is_dark),
        Modal::EditEmployee(idx, form) => form_view(form, "Edit Employee Details", "Save Changes", "person_edit", true, is_dark).map(move |msg| match msg {
             Message::SubmitAdd => Message::SubmitEdit(*idx),
             _ => msg
        }),
        Modal::DeleteEmployee(idx) => delete_confirmation_view(*idx, is_dark),
        Modal::GeneralReport(data) => general_report_view(data, is_dark),
        Modal::EmployeeReport(employee, date_str) => employee_report_view(employee, date_str, is_dark),
        Modal::Settings => settings_view(is_dark),
        Modal::ConfirmResetApp => confirm_reset_view("Reset Application", "This will delete ALL data including employees and schedules. This action cannot be undone.", Message::ConfirmResetAppAction, is_dark),
        Modal::ConfirmResetSchedules => confirm_reset_view("Reset Schedules", "This will delete ALL schedules. This action cannot be undone.", Message::ConfirmResetSchedulesAction, is_dark),
        Modal::ConfirmResetEmployees => confirm_reset_view("Reset Employees", "This will delete ALL employees and their associated schedules. This action cannot be undone.", Message::ConfirmResetEmployeesAction, is_dark),
    }
}

fn stat_card_modern<'a>(label: &'a str, value: String, _icon_type: &'a str, is_dark: bool) -> Element<'a, Message> {
    container(
        column![
            text(label).size(12).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
            text(value).size(24).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        ].spacing(4)
    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
        border: iced::border::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
        },
        ..Default::default()
    })
    .into()
}

fn daily_utilization_card<'a>(day: &'a str, utilization_pct: f32, is_dark: bool) -> Element<'a, Message> {
    let bar_height = 6.0;
    
    let filled_bar = container(iced::widget::Space::new())
        .width(Length::FillPortion((utilization_pct as u16).max(1)))
        .height(Length::Fixed(bar_height))
        .style(move |_| container::Style {
            background: Some(theme::PRIMARY.into()),
            border: iced::border::Border { radius: 3.0.into(), ..Default::default() },
            ..Default::default()
        });

    let empty_bar = container(iced::widget::Space::new())
        .width(Length::FillPortion((100.0 - utilization_pct).max(0.0) as u16))
        .height(Length::Fixed(bar_height))
        .style(move |_| container::Style {
             background: Some(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }.into()),
             border: iced::border::Border { radius: 3.0.into(), ..Default::default() },
             ..Default::default()
        });

    container(
        column![
             row![
                text(day).size(12).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                text(format!("{:.0}%", utilization_pct))
                    .size(10)
                    .width(Length::Fill)
                    .align_x(Alignment::End)
                    .style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
            ].width(Length::Fill).align_y(Alignment::Center),
            
            row![filled_bar, empty_bar].width(Length::Fill).spacing(1),
            
            text("Capacity Used").size(10).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
        ].spacing(8)
    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
        border: iced::border::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
        },
        ..Default::default()
    })
    .into()
}

fn table_view<'a>(title: &'a str, headers: Vec<String>, rows: Vec<Vec<String>>, is_dark: bool) -> Element<'a, Message> {
    let mut header_row = row![];
    for h in headers {
        header_row = header_row.push(
            container(text(h).size(11).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() })
                .style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }))
                .width(Length::Fill)
                .padding(Padding::from([8, 12]))
        );
    }
    header_row = header_row.spacing(1);

    let mut content_col = column![];
    for (i, r) in rows.into_iter().enumerate() {
        let mut row_widget = row![];
        for c in r {
            row_widget = row_widget.push(
                container(text(c).size(12))
                    .width(Length::Fill)
                    .padding(Padding::from([8, 12]))
            );
        }
        
        let bg_color = if i % 2 == 0 {
            None 
        } else {
            Some(if is_dark { Color::from_rgba(1.0, 1.0, 1.0, 0.02) } else { theme::GRAY_50 })
        };
        
        content_col = content_col.push(
            container(row_widget.spacing(1))
                .style(move |_| container::Style {
                    background: bg_color.map(|c| c.into()),
                    border: iced::border::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
        );
    }
    content_col = content_col.spacing(2);

    column![
        text(title).size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
        container(
            column![
                header_row,
                container(iced::widget::Space::new().width(Length::Fill).height(1)).style(move |_| container::Style { background: Some(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }.into()), ..Default::default() }),
                content_col
            ]
        )
        .style(move |_t: &Theme| container::Style {
            background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
            border: iced::border::Border {
                width: 1.0,
                color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
    ].spacing(12).into()
}

fn format_days(attendance: &[AttendanceStatus; 5]) -> String {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let present_days: Vec<&str> = attendance.iter().zip(days.iter())
        .filter(|(status, _)| **status == AttendanceStatus::Office)
        .map(|(_, day)| *day)
        .collect();
    
    if present_days.is_empty() {
        "Remote Week".to_string()
    } else {
        present_days.join(" ")
    }
}

fn daily_split_card<'a>(day: &'a str, office_pct: f32, remote_pct: f32, is_dark: bool) -> Element<'a, Message> {
    let bar_height = 6.0;
    
    let office_bar = container(iced::widget::Space::new())
        .width(Length::FillPortion((office_pct as u16).max(1)))
        .height(Length::Fixed(bar_height))
        .style(move |_| container::Style {
            background: Some(theme::PRIMARY.into()),
            border: iced::border::Border {
                radius: iced::border::Radius {
                    top_left: 3.0,
                    bottom_left: 3.0,
                    top_right: 0.0,
                    bottom_right: 0.0,
                },
                ..Default::default()
            },
            ..Default::default()
        });

    let remote_bar = container(iced::widget::Space::new())
        .width(Length::FillPortion((remote_pct as u16).max(1)))
        .height(Length::Fixed(bar_height))
        .style(move |_| container::Style {
            background: Some(Color::from_rgb(0.8, 0.8, 0.8).into()),
            border: iced::border::Border {
                radius: iced::border::Radius {
                    top_left: 0.0,
                    bottom_left: 0.0,
                    top_right: 3.0,
                    bottom_right: 3.0,
                },
                ..Default::default()
            },
            ..Default::default()
        });

    container(
        column![
            row![
                text(day).size(12).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                text(format!("{:.0}% / {:.0}%", office_pct, remote_pct))
                    .size(10)
                    .width(Length::Fill)
                    .align_x(Alignment::End)
                    .style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
            ].width(Length::Fill).align_y(Alignment::Center),
            
            row![office_bar, remote_bar].width(Length::Fill).spacing(1),
            
            text("Office / Remote").size(10).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
        ].spacing(8)
    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
        border: iced::border::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
        },
        ..Default::default()
    })
    .into()
}

fn utilization_card<'a>(utilization: [f32; 5], is_dark: bool) -> Element<'a, Message> {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    
    let bars: Vec<Element<'a, Message>> = days.iter().zip(utilization.iter()).map(|(day, &pct)| {
        let height = (pct / 100.0 * 60.0).max(4.0);
        column![
            container(iced::widget::Space::new())
                .width(Length::Fixed(8.0))
                .height(Length::Fixed(height))
                .style(move |_| container::Style {
                    background: Some(theme::PRIMARY.into()),
                    border: iced::border::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            text(*day).size(10).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
        ].spacing(4).align_x(Alignment::Center).into()
    }).collect();

    container(
        column![
            text("Office Utilization").size(12).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
            row(bars).spacing(8).align_y(Alignment::End).height(Length::Fixed(80.0))
        ].spacing(12)
    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
        border: iced::border::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
        },
        ..Default::default()
    })
    .into()
}

fn general_report_view<'a>(data: &'a ReportData, is_dark: bool) -> Element<'a, Message> {
    let header = row![
        row![
            container(
                icon_chart_no_axes_column()
                    .size(24)
                    .style(move |_| text::Style { color: Some(theme::PRIMARY) })
            )
            .padding(12)
            .style(move |_| container::Style {
                background: Some(Color::from_rgba(0.196, 0.505, 0.498, 0.1).into()),
                border: iced::border::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            column![
                text(format!("Attendance Report - {}", data.date))
                    .size(20)
                    .font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                text("Consolidated analytics and employee office utilization")
                    .size(14)
                    .style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
            ].spacing(4)
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        button(icon_x().size(20).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }))
            .on_press(Message::Close)
            .padding(4)
            .style(move |_, _| button::Style {
                background: None,
                ..Default::default()
            }),
    ]
    .align_y(Alignment::Start)
    .width(Length::Fill);

    // Row 1: Counts
    let counts_row = row![
        stat_card_modern("Total Employees", data.total_employees.to_string(), "active", is_dark),
        stat_card_modern("Total Males", data.total_males.to_string(), "demographics", is_dark),
        stat_card_modern("Total Females", data.total_females.to_string(), "demographics", is_dark),
    ].spacing(12).width(Length::Fill);

    // Row 2: Peaks
    let peaks_row = row![
        stat_card_modern("Peak Day", data.day_most_attendance.clone(), "busy", is_dark),
        stat_card_modern("Quiet Day", data.day_least_attendance.clone(), "quiet", is_dark),
    ].spacing(12).width(Length::Fill);

    // Row 3: Remote vs Office
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let mut remote_office_row = row![];
    for i in 0..5 {
        remote_office_row = remote_office_row.push(
            daily_split_card(
                days[i],
                data.daily_office_percentage[i],
                data.daily_remote_percentage[i],
                is_dark
            )
        );
    }
    remote_office_row = remote_office_row.spacing(8).width(Length::Fill);

    // Row 4: Utilization
    let mut utilization_row = row![];
    for i in 0..5 {
        utilization_row = utilization_row.push(
            daily_utilization_card(
                days[i],
                data.office_utilization_per_day[i],
                is_dark
            )
        );
    }
    utilization_row = utilization_row.spacing(8).width(Length::Fill);

    // Row 5: Gender Distribution Table
    let gender_headers = vec!["Sex".to_string(), "Mon".to_string(), "Tue".to_string(), "Wed".to_string(), "Thu".to_string(), "Fri".to_string()];
    let gender_rows = vec![
        vec!["Male".to_string(), data.males_per_day[0].to_string(), data.males_per_day[1].to_string(), data.males_per_day[2].to_string(), data.males_per_day[3].to_string(), data.males_per_day[4].to_string()],
        vec!["Female".to_string(), data.females_per_day[0].to_string(), data.females_per_day[1].to_string(), data.females_per_day[2].to_string(), data.females_per_day[3].to_string(), data.females_per_day[4].to_string()],
    ];
    let gender_table = table_view("Gender Distribution", gender_headers, gender_rows, is_dark);

    // Row 6: Role Distribution Table
    let role_headers = vec!["Role".to_string(), "Mon".to_string(), "Tue".to_string(), "Wed".to_string(), "Thu".to_string(), "Fri".to_string()];
    let mut all_roles: Vec<String> = data.roles_per_day.iter().flat_map(|map| map.keys().cloned()).collect();
    all_roles.sort();
    all_roles.dedup();
    
    let role_rows: Vec<Vec<String>> = all_roles.iter().map(|role| {
        let mut r = vec![role.clone()];
        for i in 0..5 {
            r.push(data.roles_per_day[i].get(role).unwrap_or(&0).to_string());
        }
        r
    }).collect();
    
    let role_table = table_view("Role Distribution", role_headers, role_rows, is_dark);

    let content = column![
        container(header).padding(Padding { top: 24.0, right: 32.0, bottom: 0.0, left: 32.0 }),
        container(iced::widget::Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
            .style(move |_t: &Theme| container::Style {
                background: Some(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }.into()),
                ..Default::default()
            })
            .padding(Padding { top: 0.0, right: 32.0, bottom: 0.0, left: 32.0 }),
        scrollable(
            column![
                counts_row,
                peaks_row,
                text("Remote vs Office %").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                remote_office_row,
                text("Office Space Utilization").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                utilization_row,
                gender_table,
                role_table
            ]
            .spacing(16)
            .padding(Padding { top: 24.0, right: 32.0, bottom: 32.0, left: 32.0 })
        ).height(Length::Fill)
    ]
    .spacing(16);

    let card = container(
        content
    )
    .width(Length::Fixed(800.0))
    .height(Length::Fixed(700.0))
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::BACKGROUND_DARK } else { theme::BACKGROUND_LIGHT }.into()),
        border: iced::border::Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        shadow: iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 10.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    });

    backdrop(card)
}

fn employee_report_view<'a>(employee: &'a Employee, date_str: &'a str, is_dark: bool) -> Element<'a, Message> {
    let header = row![
        row![
            container(
                // icon_user_round_pen()
                icon_calendar_check_2()
                    .size(24)
                    .style(move |_| text::Style { color: Some(theme::PRIMARY) })
            )
            .padding(12)
            .style(move |_| container::Style {
                background: Some(Color::from_rgba(0.196, 0.505, 0.498, 0.1).into()),
                border: iced::border::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            column![
                text(&employee.name)
                    .size(20)
                    .font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                text(&employee.role)
                    .size(14)
                    .style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
            ].spacing(4)
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        button(icon_x().size(20).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }))
            .on_press(Message::Close)
            .padding(4)
            .style(move |_, _| button::Style {
                background: None,
                ..Default::default()
            }),
    ]
    .align_y(Alignment::Start)
    .width(Length::Fill);

    let mut schedule_items = column![];

    // Current Schedule (Highlighted)
    let current_days = format_days(&employee.attendance);
    schedule_items = schedule_items.push(
        container(
            row![
                column![
                    text("Current Schedule")
                        .size(12)
                        .style(move |_| text::Style { color: Some(theme::PRIMARY) }),
                    text(date_str)
                        .size(14)
                        .font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                ].width(Length::FillPortion(1)),
                text(current_days)
                    .size(14)
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::End),
            ]
            .align_y(Alignment::Center)
        )
        .padding(16)
        .style(move |_| container::Style {
            background: Some(Color::from_rgba(0.196, 0.505, 0.498, 0.1).into()),
            border: iced::border::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: theme::PRIMARY,
            },
            ..Default::default()
        })
    );

    // Past Schedules
    if !employee.past_attendance.is_empty() {
        schedule_items = schedule_items.push(iced::widget::Space::new().height(8));
        schedule_items = schedule_items.push(
             text("Past Schedules").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() })
        );
        
        for (label, attendance) in &employee.past_attendance {
             let days_str = format_days(attendance);
             schedule_items = schedule_items.push(
                container(
                    row![
                        row![
                            icon_calendar().size(16).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
                            text(label).size(14),
                        ].spacing(8).width(Length::FillPortion(1)).align_y(Alignment::Center),
                        
                        text(days_str)
                            .size(14)
                            .width(Length::FillPortion(2))
                            .align_x(Alignment::End)
                            .style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(16)
                .style(move |_| container::Style {
                    background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
                    border: iced::border::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
                    },
                    ..Default::default()
                })
            );
        }
    }

    let scrollable_content = scrollable(
        column![
            header,
            container(iced::widget::Space::new().width(Length::Fill).height(Length::Fixed(1.0))).style(move |_t: &Theme| container::Style {
                background: Some(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }.into()),
                ..Default::default()
            }),
            schedule_items.spacing(12)
        ]
        .spacing(24)
        .padding([24, 32])
    )
    .height(Length::Fill);

    let card = container(
        column![
            scrollable_content,
        ]
    )
    .width(Length::Fixed(450.0))
    .height(Length::Fixed(600.0))
    .style(move |_t: &Theme| container::Style {
        background: Some(if is_dark { theme::BACKGROUND_DARK } else { theme::BACKGROUND_LIGHT }.into()),
        border: iced::border::Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        shadow: iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 10.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    });

    backdrop(card)
}

fn form_view<'a>(
    form: &'a EmployeeForm,
    title: &'a str,
    submit_label: &'a str,
    _icon_name: &'a str,
    is_edit: bool,
    is_dark: bool,
) -> Element<'a, Message> {
    let subtitle = if is_edit {
        "Update personal info of existing employees"
    } else {
        "Enter personal details and recurring preferences."
    };

    let content = column![
        // Header
        row![
            row![
                container(
                    (if is_edit { icon_user_round_pen() } else { icon_user_plus() })
                        .size(20)
                        .style(move |_| text::Style { color: Some(theme::PRIMARY) })
                )
                .padding(10)
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(0.196, 0.505, 0.498, 0.1).into()),
                    border: iced::border::Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                column![
                    text(title).size(18).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                    text(subtitle).size(12).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
                ].spacing(2)
            ].spacing(12).align_y(Alignment::Center),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center),

        // Form Fields
        row![
            input_group("Full Name", 
                text_input("e.g. Sarah Jenkins", &form.name)
                    .on_input(Message::NameChanged)
                    .padding(12)
                    .style(move |t, s| theme::text_input_style(t, s, is_dark)),
                is_dark
            ),
            input_group("Role", 
                pick_list(
                    Role::all().iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    form.role.clone(),
                    Message::RoleSelected
                )
                    .padding(12)
                    .width(Length::Fill)
                    .style(move |t, s| theme::pick_list_style(t, s, is_dark)),
                is_dark
            )
        ].spacing(20),

        row![
            input_group("Sex", 
                pick_list(
                    Sex::all().iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    form.sex.clone(),
                    Message::SexSelected
                )
                    .padding(12)
                    .width(Length::Fill)
                    .style(move |t, s| theme::pick_list_style(t, s, is_dark)),
                is_dark
            ),
            input_group("Required Office Days", 
                pick_list(DAYS_OPTIONS, form.days_per_week, Message::DaysChanged)
                    .padding(12)
                    .width(Length::Fill)
                    .placeholder("Select days")
                    .style(move |t, s| theme::pick_list_style(t, s, is_dark)),
                is_dark
            )
        ].spacing(20),

        // Relationships
        row![
             // Mentor Selection
             input_group("Assign Mentor", 
                pick_list(
                    form.available_employees.clone(),
                    form.mentor.clone(), 
                    Message::MentorSelected
                )
                .padding(12)
                .width(Length::Fill)
                .style(move |t, s| theme::pick_list_style(t, s, is_dark)),
                is_dark
            ),
            // Mentee Selection (Multi-select simulation via chips?)
            // For now, simple picklist to add one by one or a custom widget.
            // Let's use a column of added mentees and a picklist to add more.
            column![
                text("Assign Mentees").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                row![
                     pick_list(
                        form.available_employees.clone(),
                        None::<String>, 
                        Message::MenteeSelected
                     )
                     .padding(12)
                     .width(Length::Fill)
                     .placeholder("Add mentee...")
                     .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                ],
                // Chips for selected mentees
                row(
                    form.mentee.iter().map(|m| {
                        container(
                            row![
                                text(m).size(12),
                                button(icon_x().size(12))
                                    .on_press(Message::MenteeRemoved(m.clone()))
                                    .style(button::text)
                                    .padding(0)
                            ].spacing(4).align_y(Alignment::Center)
                        )
                        .padding([4, 8])
                        .style(move |_| container::Style {
                            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()),
                            border: iced::border::Border { radius: 12.0.into(), ..Default::default() },
                            ..Default::default()
                        })
                        .into()
                    })
                ).spacing(4).wrap()
            ].spacing(8).width(Length::Fill)
        ].spacing(20),

        // Fixed Days Selection
        column![
            text("Fixed Office Days").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
            row![
                day_toggle("Mon", form.attendance[0], 0, is_dark),
                day_toggle("Tue", form.attendance[1], 1, is_dark),
                day_toggle("Wed", form.attendance[2], 2, is_dark),
                day_toggle("Thu", form.attendance[3], 3, is_dark),
                day_toggle("Fri", form.attendance[4], 4, is_dark),
            ].spacing(12)
        ].spacing(8),

        // Action Buttons
        row![
            Space::new().width(Length::Fill),
            button(text("Cancel").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                .on_press(Message::Cancel)
                .padding([12, 24])
                .style(move |_t, _s| {
                    button::Style {
                        text_color: if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT },
                        background: None,
                        ..Default::default()
                    }
                }),
            button(text(submit_label).size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                .on_press(if is_edit { Message::SubmitEdit(0) } else { Message::SubmitAdd }) // Index ignored for SubmitAdd
                .padding([12, 24])
                .style(move |_t, _s| {
                     button::Style {
                        background: Some(theme::PRIMARY.into()),
                        text_color: Color::WHITE,
                        border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                        ..Default::default()
                    }
                })
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill)
    ]
    .spacing(24)
    .padding([24, 32]);

    let card = modal_card(content, 600.0, is_dark);
    backdrop(card)
}

fn day_toggle<'a>(label: &'a str, status: AttendanceStatus, idx: usize, is_dark: bool) -> Element<'a, Message> {
    let (bg, text_color) = match status {
        AttendanceStatus::Office => (theme::PRIMARY, Color::WHITE),
        AttendanceStatus::Remote => (if is_dark { theme::GRAY_800 } else { theme::GRAY_100 }, if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }), // Should be NA ideally for unselected
        AttendanceStatus::NA => (if is_dark { theme::GRAY_800 } else { theme::GRAY_100 }, if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }),
    };

    // If Remote, we treat it as unselected in this form context (since we are selecting fixed OFFICE days).
    // Actually, form logic: toggle between Office and NA (or Remote).
    // The message handler logic: ToggleDay toggles status.

    let is_selected = status == AttendanceStatus::Office;
    let (bg, text_color) = if is_selected {
        (theme::PRIMARY, Color::WHITE)
    } else {
        (if is_dark { theme::GRAY_800 } else { theme::GRAY_100 }, if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT })
    };

    button(
        container(text(label).size(14))
            .width(Length::Fill)
            .align_x(Alignment::Center)
    )
    .on_press(Message::ToggleDay(idx))
    .width(60)
    .padding(10)
    .style(move |_t, _s| {
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
            ..Default::default()
        }
    })
    .into()
}

fn delete_confirmation_view<'a>(idx: usize, is_dark: bool) -> Element<'a, Message> {
    let content = column![
            row![
                container(
                    icon_trash_2()
                        .size(24)
                        .style(move |_| text::Style { color: Some(Color::from_rgb(0.8, 0.2, 0.2)) })
                )
                .padding(12)
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(0.8, 0.2, 0.2, 0.1).into()),
                    border: iced::border::Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                column![
                    text("Remove Employee").size(18).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                    text("Are you sure you want to remove this employee? This action cannot be undone.").size(12).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
                ].spacing(2)
            ].spacing(12).align_y(Alignment::Center).padding([24, 32]),
        
        container(
            row![
                button(text("Cancel").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                    .on_press(Message::Cancel)
                    .style(move |_t, _s| {
                        button::Style {
                            text_color: if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT },
                            background: None,
                            ..Default::default()
                        }
                    }),
                button(text("Remove").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                    .on_press(Message::ConfirmDelete(idx))
                    .padding([10, 24])
                    .style(move |_t, _s| {
                         button::Style {
                            background: Some(Color::from_rgb(0.8, 0.2, 0.2).into()),
                            text_color: Color::WHITE,
                            border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                            ..Default::default()
                        }
                    })
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding([24, 32])
        .align_x(Alignment::End)
        .style(move |_t: &Theme| container::Style {
            background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
            border: iced::border::Border {
                radius: iced::border::Radius { bottom_left: 16.0, bottom_right: 16.0, ..0.0.into() },
                ..Default::default()
            },
            ..Default::default()
        })
    ];

    let card = modal_card(content, 500.0, is_dark);
    backdrop(card)
}

fn modal_card<'a>(content: impl Into<Element<'a, Message>>, max_width: f32, is_dark: bool) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .max_width(max_width)
        .style(move |_t: &Theme| container::Style {
            background: Some(if is_dark { theme::SURFACE_DARK } else { theme::SURFACE_LIGHT }.into()),
            border: iced::border::Border {
                radius: 16.0.into(),
                ..Default::default()
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                offset: iced::Vector::new(0.0, 10.0),
                blur_radius: 30.0,
            },
            ..Default::default()
        })
        .into()
}

fn backdrop<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|_| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .into()
}

fn input_group<'a>(label: &'a str, input: impl Into<Element<'a, Message>>, is_dark: bool) -> Element<'a, Message> {
    column![
        text(label)
            .size(14)
            .font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() })
            .style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
        input.into()
    ]
    .spacing(8)
    .width(Length::Fill)
    .into()
}

fn settings_view<'a>(is_dark: bool) -> Element<'a, Message> {
    let header = row![
        row![
            container(
                icon_settings()
                    .size(20)
                    .style(move |_| text::Style { color: Some(theme::PRIMARY) })
            )
            .padding(10)
            .style(move |_| container::Style {
                background: Some(Color::from_rgba(0.196, 0.505, 0.498, 0.1).into()),
                border: iced::border::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            text("Settings")
                .size(18)
                .font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
        ].spacing(12).align_y(Alignment::Center),
        Space::new().width(Length::Fill),
        button(icon_x().size(20).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }))
            .on_press(Message::Close)
            .style(move |_t, _s| button::Style {
                background: None,
                ..Default::default()
            })
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let content = column![
        header,
        column![
            settings_button("Reset Application", "Clear all data and start fresh", Message::ResetApp, is_dark, true),
            settings_button("Reset Schedules", "Clear all generated schedules", Message::ResetSchedules, is_dark, true),
            settings_button("Reset Employees", "Remove all employees", Message::ResetEmployees, is_dark, true),
        ].spacing(12)
    ]
    .spacing(24)
    .padding([24, 32]);

    let card = modal_card(content, 600.0, is_dark);
    backdrop(card)
}

fn settings_button<'a>(
    title: &'a str,
    description: &'a str,
    message: Message,
    is_dark: bool,
    is_danger: bool
) -> Element<'a, Message> {
    button(
        row![
            column![
                text(title).size(16).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                text(description).size(12).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
            ].spacing(4),
            Space::new().width(Length::Fill),
            icon_trash_2().size(18).style(move |_| text::Style { color: Some(if is_danger { theme::ERROR } else { if is_dark { theme::TEXT_DARK } else { theme::TEXT_LIGHT } }) })
        ]
        .align_y(Alignment::Center)
        .padding(12)
    )
    .on_press(message)
    .width(Length::Fill)
    .style(move |_, status| {
        let bg = if status == button::Status::Hovered {
             if is_dark { theme::GRAY_800 } else { theme::GRAY_50 }
        } else {
             Color::TRANSPARENT
        };
        button::Style {
            background: Some(bg.into()),
            border: iced::border::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT },
            },
            ..button::Style::default()
        }
    })
    .into()
}

fn confirm_reset_view<'a>(
    title: &'a str,
    description: &'a str,
    confirm_msg: Message,
    is_dark: bool
) -> Element<'a, Message> {
    let content = column![
        text(title).size(20).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        text(description).size(14).style(move |_| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) }),
        row![
            button(text("Cancel").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                .on_press(Message::Close)
                .padding([10, 24])
                .style(move |_t, _s| {
                    button::Style {
                        text_color: if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT },
                        background: None,
                        ..Default::default()
                    }
                }),
            button(text("Confirm Reset").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
                .on_press(confirm_msg)
                .padding([10, 24])
                .style(move |_t, _s| {
                     button::Style {
                        background: Some(theme::ERROR.into()),
                        text_color: Color::WHITE,
                        border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                        ..Default::default()
                    }
                })
        ].spacing(12).align_y(Alignment::Center)
    ]
    .spacing(20)
    .padding(24);

    let card = modal_card(content, 400.0, is_dark);
    backdrop(card)
}
