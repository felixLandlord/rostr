mod theme;
mod top_bar;
mod action_bar;
mod attendance_table;
mod modals;
mod toasts;

use attendance_table::{AttendanceTable, Message as AttendanceTableMessage, Employee};
use action_bar::{ActionBar, Message as ActionBarMessage};
use modals::{Modal, Message as ModalMessage, EmployeeForm, ReportData};
use toasts::{Toast, Status};
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
        .subscription(RostrApp::subscription)
        .run()
}

struct RostrApp {
    is_dark: bool,
    search_query: String,
    current_date: NaiveDate,
    attendance_table: AttendanceTable,
    modal: Modal,
    toasts: Vec<Toast>,
    toast_counter: u64,
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
            toasts: Vec::new(),
            toast_counter: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TopBar(TopBarMessage),
    ActionBar(ActionBarMessage),
    AttendanceTable(AttendanceTableMessage),
    Modal(ModalMessage),
    ToastTimeout(u64),
    CloseToast(u64),
    FileActionComplete,
    Tick,
}

impl RostrApp {
    fn show_toast(&mut self, title: String, body: String, status: toasts::Status) -> Task<Message> {
        self.toast_counter += 1;
        let id = self.toast_counter;
        self.toasts.push(toasts::Toast {
            id,
            title,
            body,
            status,
            created_at: std::time::Instant::now(),
        });

        Task::perform(
            async move {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                id
            },
            Message::ToastTimeout,
        )
    }

    fn calculate_report(&self) -> ReportData {
        let employees = &self.attendance_table.employees;
        let total_employees = employees.len();
        let mut total_males = 0;
        let mut total_females = 0;
        let mut males_per_day = [0; 5];
        let mut females_per_day = [0; 5];
        let mut roles_per_day: [std::collections::HashMap<String, usize>; 5] = Default::default();
        let mut daily_attendance = [0; 5];
        let mut daily_remote = [0; 5];

        for employee in employees {
            if employee.sex == "Male" {
                total_males += 1;
            } else if employee.sex == "Female" {
                total_females += 1;
            }

            for (day, status) in employee.attendance.iter().enumerate() {
                match status {
                    attendance_table::AttendanceStatus::Office => {
                        daily_attendance[day] += 1;

                        if employee.sex == "Male" {
                            males_per_day[day] += 1;
                        } else if employee.sex == "Female" {
                            females_per_day[day] += 1;
                        }

                        *roles_per_day[day].entry(employee.role.clone()).or_insert(0) += 1;
                    }
                    attendance_table::AttendanceStatus::Remote => {
                        daily_remote[day] += 1;
                    }
                }
            }
        }

        let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
        let mut max_day_idx = 0;
        let mut min_day_idx = 0;
        for i in 1..5 {
            if daily_attendance[i] > daily_attendance[max_day_idx] {
                max_day_idx = i;
            }
            if daily_attendance[i] < daily_attendance[min_day_idx] {
                min_day_idx = i;
            }
        }

        let mut daily_office_percentage = [0.0; 5];
        let mut daily_remote_percentage = [0.0; 5];
        
        for i in 0..5 {
            let total_for_day = daily_attendance[i] + daily_remote[i];
            if total_for_day > 0 {
                daily_office_percentage[i] = (daily_attendance[i] as f32 / total_for_day as f32) * 100.0;
                daily_remote_percentage[i] = (daily_remote[i] as f32 / total_for_day as f32) * 100.0;
            }
        }

        let total_seats = 100.0;
        let mut office_utilization_per_day = [0.0; 5];
        for i in 0..5 {
             office_utilization_per_day[i] = (daily_attendance[i] as f32 / total_seats) * 100.0;
        }

        ReportData {
            date: self.current_date.format("%B %Y").to_string(),
            total_employees,
            total_males,
            total_females,
            males_per_day,
            females_per_day,
            roles_per_day,
            day_most_attendance: days[max_day_idx].to_string(),
            day_least_attendance: days[min_day_idx].to_string(),
            daily_remote_percentage,
            daily_office_percentage,
            office_utilization_per_day,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {},
            Message::FileActionComplete => {},
            Message::ToastTimeout(id) => {
                self.toasts.retain(|t| t.id != id);
            }
            Message::CloseToast(id) => {
                self.toasts.retain(|t| t.id != id);
            }
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
                ActionBarMessage::Report => {
                    if let Some(idx) = self.attendance_table.selected_employee {
                        if let Some(employee) = self.attendance_table.employees.get(idx) {
                             self.modal = Modal::EmployeeReport(employee.clone(), self.current_date.format("%B %Y").to_string());
                        }
                    } else {
                        let data = self.calculate_report();
                        self.modal = Modal::GeneralReport(data);
                    }
                }
                ActionBarMessage::Import => {
                    return Task::perform(async {
                        let _ = rfd::AsyncFileDialog::new().pick_file().await;
                    }, |_| Message::FileActionComplete);
                }
                ActionBarMessage::Export => {
                    return Task::perform(async {
                        let _ = rfd::AsyncFileDialog::new().save_file().await;
                    }, |_| Message::FileActionComplete);
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
                ModalMessage::Close => {
                    self.modal = Modal::None;
                }
                ModalMessage::DownloadPdf => {
                    // Placeholder for download PDF logic
                    return self.show_toast("Download PDF".to_string(), "PDF Download started...".to_string(), Status::Success);
                }
                ModalMessage::SubmitAdd => {
                    let new_employee = if let Modal::AddEmployee(form) = &self.modal {
                        Some(Employee {
                            name: if form.name.is_empty() { "New Employee".to_string() } else { form.name.clone() },
                            role: form.role.clone().unwrap_or("Role".to_string()),
                            sex: form.sex.clone().unwrap_or("Male".to_string()),
                            days_per_week: form.days_per_week.unwrap_or(0),
                            mentee: form.mentee.clone().filter(|s| s != "None"),
                        mentor: form.mentor.clone().filter(|s| s != "None"),
                        attendance: form.attendance,
                        past_attendance: vec![],
                    })
                } else {
                    None
                };

                    if let Some(employee) = new_employee {
                        let name = employee.name.clone();
                        self.attendance_table.employees.push(employee);
                        self.modal = Modal::None;
                        return self.show_toast("Employee Added".to_string(), format!("{} has been successfully added.", name), Status::Success);
                    }
                }
                ModalMessage::SubmitEdit(idx) => {
                    let update_data = if let Modal::EditEmployee(_, form) = &self.modal {
                         Some((
                            form.name.clone(),
                            form.role.clone().unwrap_or_default(),
                            form.sex.clone().unwrap_or_default(),
                            form.days_per_week.unwrap_or(0),
                            form.mentee.clone().filter(|s| s != "None"),
                            form.mentor.clone().filter(|s| s != "None"),
                            form.attendance,
                        ))
                    } else {
                        None
                    };

                    if let Some((name, role, sex, days, mentee, mentor, attendance)) = update_data {
                         if let Some(employee) = self.attendance_table.employees.get_mut(idx) {
                            employee.name = name.clone();
                            employee.role = role;
                            employee.sex = sex;
                            employee.days_per_week = days;
                            employee.mentee = mentee;
                            employee.mentor = mentor;
                            employee.attendance = attendance;
                            
                            self.modal = Modal::None;
                            return self.show_toast("Employee Updated".to_string(), format!("{}'s details have been updated.", name), Status::Success);
                        }
                    }
                    self.modal = Modal::None;
                }
                ModalMessage::ConfirmDelete(idx) => {
                    let mut deleted_name = String::new();
                    if idx < self.attendance_table.employees.len() {
                        deleted_name = self.attendance_table.employees[idx].name.clone();
                        self.attendance_table.employees.remove(idx);
                        // Adjust selection if needed or clear it
                        self.attendance_table.selected_employee = None;
                    }
                    self.modal = Modal::None;
                    if !deleted_name.is_empty() {
                        return self.show_toast("Employee Deleted".to_string(), format!("{} has been removed.", deleted_name), Status::Success);
                    }
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

    fn subscription(&self) -> iced::Subscription<Message> {
        if !self.toasts.is_empty() {
            iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::Tick)
        } else {
            iced::Subscription::none()
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
        let toasts = toasts::view(&self.toasts, Message::CloseToast);

        stack![
            content,
            modal,
            toasts
        ].into()
    }
}
