mod theme;
mod top_bar;
mod action_bar;
mod attendance_table;
mod modals;

use attendance_table::{AttendanceTable, Message as AttendanceTableMessage, Employee};
use action_bar::{ActionBar, Message as ActionBarMessage};
use modals::{Modal, Message as ModalMessage, EmployeeForm};
use chrono::{Datelike, Local, NaiveDate};
use iced::task::Task;
use iced::widget::{column, container, stack};
use iced::{Element, Length, Padding, Theme};
use lucide_icons::LUCIDE_FONT_BYTES;
use top_bar::{Message as TopBarMessage, TopBar};

pub fn main() -> iced::Result {
    iced::application(RostrApp::default, RostrApp::update, RostrApp::view)
        .title("rostr")
        .theme(RostrApp::theme)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct RostrApp {
    is_dark: bool,
    search_query: String,
    current_date: NaiveDate,
    attendance_table: AttendanceTable,
    modal: Modal,
}

impl Default for RostrApp {
    fn default() -> Self {
        let now = Local::now();
        Self {
            is_dark: false,
            search_query: String::new(),
            current_date: NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
            attendance_table: AttendanceTable::new(),
            modal: Modal::None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TopBar(TopBarMessage),
    ActionBar(ActionBarMessage),
    AttendanceTable(AttendanceTableMessage),
    Modal(ModalMessage),
}

impl RostrApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionBar(msg) => match msg {
                ActionBarMessage::AddEmployee => {
                    self.modal = Modal::AddEmployee(EmployeeForm::default());
                }
                ActionBarMessage::EditEmployee => {
                    if let Some(idx) = self.attendance_table.selected_employee {
                        if let Some(employee) = self.attendance_table.employees.get(idx) {
                             self.modal = Modal::EditEmployee(idx, EmployeeForm::from(employee));
                        }
                    }
                }
                ActionBarMessage::DeleteEmployee => {
                     if let Some(idx) = self.attendance_table.selected_employee {
                        self.modal = Modal::DeleteEmployee(idx);
                     }
                }
                _ => {}
            },
            Message::AttendanceTable(msg) => {
                return self.attendance_table.update(msg).map(Message::AttendanceTable);
            }
            Message::TopBar(top_bar_msg) => match top_bar_msg {
                TopBarMessage::SearchChanged(query) => self.search_query = query,
                TopBarMessage::ToggleTheme => self.is_dark = !self.is_dark,
                TopBarMessage::PreviousDate => {
                    self.current_date = if self.current_date.month() == 1 {
                        NaiveDate::from_ymd_opt(self.current_date.year() - 1, 12, 1).unwrap()
                    } else {
                        NaiveDate::from_ymd_opt(
                            self.current_date.year(),
                            self.current_date.month() - 1,
                            1,
                        )
                        .unwrap()
                    };
                }
                TopBarMessage::NextDate => {
                    self.current_date = if self.current_date.month() == 12 {
                        NaiveDate::from_ymd_opt(self.current_date.year() + 1, 1, 1).unwrap()
                    } else {
                        NaiveDate::from_ymd_opt(
                            self.current_date.year(),
                            self.current_date.month() + 1,
                            1,
                        )
                        .unwrap()
                    };
                }
                _ => {}
            },
            Message::Modal(msg) => match msg {
                ModalMessage::Cancel => {
                    self.modal = Modal::None;
                }
                ModalMessage::SubmitAdd => {
                    if let Modal::AddEmployee(form) = &self.modal {
                        // Create new employee from form
                        let new_employee = Employee {
                            name: if form.name.is_empty() { "New Employee".to_string() } else { form.name.clone() },
                            role: form.role.clone().unwrap_or("Role".to_string()),
                            sex: form.sex.clone().unwrap_or("Other".to_string()),
                            days_per_week: form.days_per_week.unwrap_or(0),
                            mentee: form.mentee.clone().filter(|s| s != "None"),
                            mentor: form.mentor.clone().filter(|s| s != "None"),
                            attendance: form.attendance,
                        };
                        self.attendance_table.employees.push(new_employee);
                        self.modal = Modal::None;
                    }
                }
                ModalMessage::SubmitEdit(idx) => {
                     if let Modal::EditEmployee(_, form) = &self.modal {
                        if let Some(employee) = self.attendance_table.employees.get_mut(idx) {
                            employee.name = form.name.clone();
                            employee.role = form.role.clone().unwrap_or_default();
                            employee.sex = form.sex.clone().unwrap_or_default();
                            employee.days_per_week = form.days_per_week.unwrap_or(0);
                            employee.mentee = form.mentee.clone().filter(|s| s != "None");
                            employee.mentor = form.mentor.clone().filter(|s| s != "None");
                            employee.attendance = form.attendance;
                        }
                        self.modal = Modal::None;
                    }
                }
                ModalMessage::ConfirmDelete(idx) => {
                    if idx < self.attendance_table.employees.len() {
                        self.attendance_table.employees.remove(idx);
                        // Adjust selection if needed or clear it
                        self.attendance_table.selected_employee = None;
                    }
                    self.modal = Modal::None;
                }
                // Handle form updates
                ModalMessage::NameChanged(name) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.name = name;
                    }
                }
                ModalMessage::RoleSelected(role) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.role = Some(role);
                    }
                }
                ModalMessage::SexSelected(sex) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.sex = Some(sex);
                    }
                }
                ModalMessage::DaysChanged(days) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.days_per_week = Some(days);
                    }
                }
                ModalMessage::MenteeSelected(mentee) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.mentee = Some(mentee);
                    }
                }
                ModalMessage::MentorSelected(mentor) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        form.mentor = Some(mentor);
                    }
                }
                ModalMessage::ToggleDay(day_idx) => {
                    if let Modal::AddEmployee(form) | Modal::EditEmployee(_, form) = &mut self.modal {
                        if day_idx < 5 {
                             // Toggle between Office and Remote
                             form.attendance[day_idx] = match form.attendance[day_idx] {
                                 attendance_table::AttendanceStatus::Office => attendance_table::AttendanceStatus::Remote,
                                 attendance_table::AttendanceStatus::Remote => attendance_table::AttendanceStatus::Office,
                             };
                        }
                    }
                }
                _ => {}
            }
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let date_str = self.current_date.format("%B %Y").to_string();
        let employee_count = self.attendance_table.len();
        let top_bar = TopBar::view(
            date_str,
            format!("Monthly Attendance Overview • {} Active Employees", employee_count),
            &self.search_query,
            self.is_dark,
        )
        .map(Message::TopBar);

        let action_bar = ActionBar::view(self.is_dark, self.attendance_table.selected_employee.is_some()).map(Message::ActionBar);
        let attendance_table = self.attendance_table.view(self.is_dark).map(Message::AttendanceTable);

        let content = container(
            column![
                top_bar,
                container(action_bar).padding(Padding::from([24, 32])),
                container(attendance_table).padding(Padding {
                    top: 0.0,
                    right: 32.0,
                    bottom: 32.0,
                    left: 32.0,
                })
            ]
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(palette.background.into()),
                    ..Default::default()
                }
            });

        let modal = modals::view(&self.modal, self.is_dark).map(Message::Modal);

        stack![
            content,
            modal
        ].into()
    }
}
