use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Color, Element, Length, Padding, Theme};
use lucide_icons::iced::{icon_user_plus, icon_user_round_pen};
use crate::theme;
use crate::attendance_table::{AttendanceStatus, Employee};

#[derive(Debug, Clone)]
pub enum Modal {
    None,
    AddEmployee(EmployeeForm),
    EditEmployee(usize, EmployeeForm),
    DeleteEmployee(usize),
}

#[derive(Debug, Clone)]
pub struct EmployeeForm {
    pub name: String,
    pub role: Option<String>,
    pub sex: Option<String>,
    pub days_per_week: Option<u8>,
    pub mentee: Option<String>,
    pub mentor: Option<String>,
    pub attendance: [AttendanceStatus; 5],
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
            mentee: Some("None".to_string()),
            mentor: Some("None".to_string()),
            attendance: [AttendanceStatus::Remote; 5], // Default to Remote (gray) to match UI "inactive" look initially? Or match logic.
            // The UI shows "Fixed Office Days" where active=Office, inactive=Remote.
        }
    }
}

impl From<&Employee> for EmployeeForm {
    fn from(e: &Employee) -> Self {
        Self {
            name: e.name.clone(),
            role: Some(e.role.clone()),
            sex: Some(e.sex.clone()),
            days_per_week: Some(e.days_per_week),
            mentee: e.mentee.clone(),
            mentor: e.mentor.clone(),
            attendance: e.attendance,
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
    MentorSelected(String),
    ToggleDay(usize),
    OverlayPressed, // To close on click outside? Optional, maybe strictly modal.
}

// Lists for dropdowns
const ROLES: &[&str] = &[
    "UX Designer",
    "Product Manager",
    "Senior Developer",
    "QA Engineer",
    "Data Analyst",
    "Frontend Dev",
    "Backend Dev",
];

const SEX_OPTIONS: &[&str] = &["Male", "Female"];

const DAYS_OPTIONS: &[u8] = &[1, 2, 3, 4, 5];

const MENTEES: &[&str] = &["None", "Alex Rivera", "Sam Chen", "Jordan Smith"];
const MENTORS: &[&str] = &["None", "Sarah Jenkins", "Michael Ross", "Elena Rodriguez"];

pub fn view<'a>(modal: &'a Modal, is_dark: bool) -> Element<'a, Message> {
    match modal {
        Modal::None => text("").into(),
        Modal::AddEmployee(form) => form_view(form, "Add New Employee", "Add Employee", "person_add", false, is_dark),
        Modal::EditEmployee(idx, form) => form_view(form, "Edit Employee Details", "Save Changes", "person_edit", true, is_dark).map(move |msg| match msg {
             Message::SubmitAdd => Message::SubmitEdit(*idx), // Remap submit
             _ => msg
        }),
        Modal::DeleteEmployee(idx) => delete_confirmation_view(*idx, is_dark),
    }
}

fn form_view<'a>(
    form: &'a EmployeeForm,
    title: &'a str,
    submit_label: &'a str,
    _icon_name: &'a str, // We'll just use text for icon or look up lucide if available in this module
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
        .align_y(Alignment::Center)
        .padding([24, 32])
        .spacing(20), // Space between title group and close button if needed

        container(column![].spacing(0).width(Length::Fill).height(1)).style(move |_t: &Theme| container::Style {
            background: Some(if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT }.into()),
            ..Default::default()
        }),

        // Body
        column![
            // Row 1
            row![
                input_group("Full Name", 
                    text_input("e.g. Jane Doe", &form.name)
                        .on_input(Message::NameChanged)
                        .padding(10)
                        .style(move |t, s| theme::text_input_style(t, s, is_dark))
                ),
                input_group("Job Title/Role", 
                    pick_list(ROLES, form.role.as_deref(), |r| Message::RoleSelected(r.to_string()))
                        .width(Length::Fill)
                        .padding(10)
                        .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                )
            ].spacing(24),

            // Row 2
            row![
                input_group("Sex", 
                     pick_list(SEX_OPTIONS, form.sex.as_deref(), |s| Message::SexSelected(s.to_string()))
                        .width(Length::Fill)
                        .padding(10)
                        .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                ),
                input_group("Required Days Per Week", 
                     pick_list(DAYS_OPTIONS, form.days_per_week, Message::DaysChanged)
                        .width(Length::Fill)
                        .padding(10)
                        .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                )
            ].spacing(24),

            // Row 3
            row![
                input_group("Mentee", 
                     pick_list(MENTEES, form.mentee.as_deref(), |m| Message::MenteeSelected(m.to_string()))
                        .width(Length::Fill)
                        .padding(10)
                        .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                ),
                input_group("Mentor", 
                     pick_list(MENTORS, form.mentor.as_deref(), |m| Message::MentorSelected(m.to_string()))
                        .width(Length::Fill)
                        .padding(10)
                        .style(move |t, s| theme::pick_list_style(t, s, is_dark))
                )
            ].spacing(24),

            // Fixed Office Days
            column![
                row![
                    text("FIXED OFFICE DAYS").size(12).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                    text("Select recurring office days").size(11).style(move |t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
                ].width(Length::Fill).align_y(Alignment::Center).spacing(12).padding(Padding { bottom: 8.0, ..Default::default() }), // Justify between?

                row(
                    (0..5).map(|i| {
                        let day_name = match i { 0 => "MON", 1 => "TUE", 2 => "WED", 3 => "THU", 4 => "FRI", _ => "" };
                        let is_active = form.attendance[i] == AttendanceStatus::Office;
                        
                        button(text(day_name).size(11).font(iced::font::Font { weight: iced::font::Weight::Medium, ..Default::default() }).align_x(Alignment::Center))
                            .on_press(Message::ToggleDay(i))
                            .width(Length::Fill)
                            .padding([8, 0])
                            .style(move |_t, _s| {
                                let active_bg = theme::PRIMARY;
                                let inactive_bg = if is_dark { theme::SURFACE_DARK } else { Color::WHITE };
                                let inactive_text = if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT };
                                let border_col = if is_dark { theme::BORDER_DARK } else { theme::BORDER_LIGHT };

                                if is_active {
                                    button::Style {
                                        background: Some(active_bg.into()),
                                        text_color: Color::WHITE,
                                        border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                                        ..Default::default()
                                    }
                                } else {
                                     button::Style {
                                        background: Some(inactive_bg.into()),
                                        text_color: inactive_text,
                                        border: iced::border::Border { radius: 8.0.into(), width: 1.0, color: border_col },
                                        ..Default::default()
                                    }
                                }
                            })
                            .into()
                    })
                ).spacing(8)
            ].spacing(4)

        ]
        .padding([24, 32])
        .spacing(24),

        // Footer
        container(
            row![
                button(text("Cancel").size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }))
                    .on_press(Message::Cancel)
                    .style(move |_t, _s| {
                        button::Style {
                            text_color: if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT },
                            background: None,
                            ..Default::default()
                        }
                    }),
                {
                    let is_valid = form.is_valid();
                    let btn = button(text(submit_label).size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }));
                    let btn = if is_valid { btn.on_press(Message::SubmitAdd) } else { btn };
                    btn.padding([10, 24])
                    .style(move |_t, _s| {
                        if is_valid {
                             button::Style {
                                background: Some(theme::PRIMARY.into()),
                                text_color: Color::WHITE,
                                border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                                ..Default::default()
                            }
                        } else {
                             button::Style {
                                background: Some(if is_dark { Color::from_rgb(0.2, 0.2, 0.2) } else { Color::from_rgb(0.9, 0.9, 0.9) }.into()),
                                text_color: if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT },
                                border: iced::border::Border { radius: 8.0.into(), ..Default::default() },
                                ..Default::default()
                            }
                        }
                    })
                }
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding([24, 32])
        .align_x(Alignment::End)
        .style(move |_t: &Theme| container::Style {
            background: Some(if is_dark { Color::from_rgb(0.1, 0.1, 0.1) } else { Color::from_rgb(0.98, 0.98, 0.98) }.into()),
            border: iced::border::Border {
                radius: iced::border::Radius { bottom_left: 16.0, bottom_right: 16.0, ..0.0.into() },
                ..Default::default()
            },
            ..Default::default()
        })
    ]
    .width(600);

    overlay_container(content, is_dark)
}

fn delete_confirmation_view<'a>(idx: usize, is_dark: bool) -> Element<'a, Message> {
    let content = column![
         row![
                container(
                    text("⚠️") 
                        .size(20)
                )
                .padding(10)
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(0.9, 0.2, 0.2, 0.1).into()),
                    border: iced::border::Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                column![
                    text("Delete Employee").size(18).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                    text("Are you sure you want to delete this employee? This action cannot be undone.").size(12).style(move |_t: &Theme| text::Style { color: Some(if is_dark { theme::TEXT_MUTED_DARK } else { theme::TEXT_MUTED_LIGHT }) })
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
                button(text("Delete").size(14).font(iced::font::Font { weight: iced::font::Weight::Semibold, ..Default::default() }))
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
            background: Some(if is_dark { Color::from_rgb(0.1, 0.1, 0.1) } else { Color::from_rgb(0.98, 0.98, 0.98) }.into()),
            border: iced::border::Border {
                radius: iced::border::Radius { bottom_left: 16.0, bottom_right: 16.0, ..0.0.into() },
                ..Default::default()
            },
            ..Default::default()
        })
    ].width(500);

    overlay_container(content, is_dark)
}

fn overlay_container<'a>(content: impl Into<Element<'a, Message>>, is_dark: bool) -> Element<'a, Message> {
    container(
        container(content)
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
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_| container::Style {
        background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
        ..Default::default()
    })
    .into()
}

fn input_group<'a>(label: &'a str, input: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![
        text(label).size(14).font(iced::font::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
        input.into()
    ]
    .spacing(8)
    .width(Length::Fill)
    .into()
}
