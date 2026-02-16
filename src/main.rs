mod core;
mod ui;

use ui::attendance_table::{self, AttendanceTable, Message as AttendanceTableMessage, Employee as UIEmployee, AttendanceStatus};
use ui::action_bar::{ActionBar, Message as ActionBarMessage};
use ui::modals::{self, Modal, Message as ModalMessage, EmployeeForm, ReportData};
use ui::toasts::{self, Toast, Status};
use chrono::{Datelike, Local, NaiveDate};
use iced::task::Task;
use iced::widget::{column, container, stack};
use iced::{Element, Length, Padding, Theme};
use lucide_icons::LUCIDE_FONT_BYTES;
use ui::top_bar::{Message as TopBarMessage, TopBar};

use core::storage::{Database, EmployeeRepository, ScheduleRepository};
use core::engine::Engine;
use core::models::{Employee as CoreEmployee, MonthlySchedule, Role, Sex, Weekday};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    FileActionComplete,
    ToastTimeout(u64),
    CloseToast(u64),
    ActionBar(ActionBarMessage),
    AttendanceTable(AttendanceTableMessage),
    TopBar(TopBarMessage),
    Modal(ModalMessage),
}

pub fn main() -> iced::Result {
    iced::application(RostrApp::new, RostrApp::update, RostrApp::view)
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
    database: Option<Database>, // Option because it might fail to init
    current_schedule: Option<MonthlySchedule>,
    is_locked: bool,
}

impl RostrApp {
    fn new() -> (Self, Task<Message>) {
        let now = Local::now();
        let current_date = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
        
        let mut app = Self {
            is_dark: false,
            search_query: String::new(),
            current_date,
            attendance_table: AttendanceTable::new(),
            modal: Modal::None,
            toasts: Vec::new(),
            toast_counter: 0,
            database: None,
            current_schedule: None,
            is_locked: false,
        };

        let mut commands = Vec::new();

        // Initialize Database
        match Database::new() {
            Ok(db) => {
                app.database = Some(db);
                // Load schedule and employees for the current month
                app.load_schedule_for_date();
            }
            Err(e) => {
                commands.push(app.show_toast("Database Error".to_string(), e.to_string(), Status::Error));
            }
        }

        (app, Task::batch(commands))
    }

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

    fn refresh_employees(&mut self) {
        if let Some(db) = &self.database {
            if let Ok(conn) = db.get_connection() {
                // If search query is active
                let result = if self.search_query.is_empty() {
                    EmployeeRepository::find_all(&conn)
                } else {
                    EmployeeRepository::search(&conn, &self.search_query)
                };

                if let Ok(employees) = result {
                    // We need to preserve current attendance if possible? 
                    // Or re-apply schedule?
                    // If we just reloaded, we lose the "Generate" state if we haven't saved it.
                    // But "Generate" updates `current_schedule`. 
                    // So we should re-map `current_schedule` to the new list.
                    
                    let mut ui_employees: Vec<UIEmployee> = employees.into_iter().map(core_to_ui_employee).collect();
                    
                    if let Some(schedule) = &self.current_schedule {
                        apply_schedule_to_ui(&mut ui_employees, schedule);
                    }

                    self.attendance_table.employees = ui_employees;
                }
            }
        }
    }

    fn load_schedule_for_date(&mut self) {
        // Reset schedule first
        self.current_schedule = None;
        self.is_locked = false;

        if let Some(db) = &self.database {
            if let Ok(conn) = db.get_connection() {
                 if let Ok(Some(schedule)) = ScheduleRepository::find_by_year_month(
                    &conn, 
                    self.current_date.year(), 
                    self.current_date.month()
                ) {
                    self.current_schedule = Some(schedule);
                }

                // Check for future schedules to lock past/current months
                if let Ok(has_future) = ScheduleRepository::has_future_schedule(
                    &conn,
                    self.current_date.year(),
                    self.current_date.month()
                ) {
                    self.is_locked = has_future;
                }
            }
        }
        
        // Refresh employees to apply schedule (or clear it if None)
        self.refresh_employees();
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
                    attendance_table::AttendanceStatus::NA => {}
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
                             let mut employee_clone = employee.clone();
                             let mut report_date_str = self.current_date.format("%B %Y").to_string();
                             
                             if let Some(db) = &self.database {
                                 if let Ok(conn) = db.get_connection() {
                                     // Fetch last 12 months
                                     if let Ok(schedules) = ScheduleRepository::find_recent_schedules(&conn, 12) {
                                         // 1. Identify the latest schedule (most recent month in DB)
                                         // 2. Identify past schedules
                                         
                                         let mut past_attendance = Vec::new();
                                         let weekdays = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday];
                                         
                                         if let Some(latest_schedule) = schedules.first() {
                                            // Update current schedule info to be the latest from DB
                                            report_date_str = NaiveDate::from_ymd_opt(latest_schedule.year, latest_schedule.month, 1)
                                                 .unwrap_or_default()
                                                 .format("%B %Y")
                                                 .to_string();

                                            let is_included = latest_schedule.included_employees.contains(&employee_clone.id);
                                            let has_any_day = latest_schedule.schedules.values().any(|emps| emps.iter().any(|e| e.id == employee_clone.id));

                                            if is_included || has_any_day {
                                                for (i, day) in weekdays.iter().enumerate() {
                                                     let employees_on_day = latest_schedule.get_employees_for_day(*day);
                                                     let is_scheduled = employees_on_day.iter().any(|e| e.id == employee_clone.id);
                                                     
                                                     if is_scheduled {
                                                         employee_clone.attendance[i] = AttendanceStatus::Office;
                                                     } else if is_included {
                                                         employee_clone.attendance[i] = AttendanceStatus::Remote;
                                                     } else {
                                                         employee_clone.attendance[i] = AttendanceStatus::NA;
                                                     }
                                                }
                                            } else {
                                                // If latest schedule exists but employee not in it -> N/A
                                                 employee_clone.attendance = [AttendanceStatus::NA; 5];
                                            }
                                         } else {
                                             // No schedules in DB at all
                                             employee_clone.attendance = [AttendanceStatus::NA; 5];
                                         }

                                         // Process past schedules (skipping the first one which is latest)
                                         for schedule in schedules.iter().skip(1) {
                                             let is_included = schedule.included_employees.contains(&employee_clone.id);
                                             let has_any_day = schedule.schedules.values().any(|emps| emps.iter().any(|e| e.id == employee_clone.id));

                                             if is_included || has_any_day {
                                                 let date_label = NaiveDate::from_ymd_opt(schedule.year, schedule.month, 1)
                                                     .unwrap_or_default()
                                                     .format("%B %Y")
                                                     .to_string();
                                                     
                                                 let mut attendance = [AttendanceStatus::NA; 5];
                                                 
                                                 for (i, day) in weekdays.iter().enumerate() {
                                                     let employees_on_day = schedule.get_employees_for_day(*day);
                                                     let is_scheduled = employees_on_day.iter().any(|e| e.id == employee_clone.id);
                                                     
                                                     if is_scheduled {
                                                         attendance[i] = AttendanceStatus::Office;
                                                     } else if is_included {
                                                         attendance[i] = AttendanceStatus::Remote;
                                                     } else {
                                                         attendance[i] = AttendanceStatus::NA;
                                                     }
                                                 }
                                                 past_attendance.push((date_label, attendance));
                                             }
                                         }
                                         employee_clone.past_attendance = past_attendance;
                                     }
                                 }
                             }

                             self.modal = Modal::EmployeeReport(employee_clone, report_date_str);
                        }
                    } else {
                        let data = self.calculate_report();
                        self.modal = Modal::GeneralReport(data);
                    }
                }
                ActionBarMessage::Generate => {
                    // Logic:
                    // 1. Fetch all employees from DB (Core types)
                    // 2. Fetch past schedules (TODO: Implement in Core)
                    // 3. Run Engine
                    // 4. Update UI
                    
                    if self.attendance_table.employees.is_empty() {
                         return self.show_toast("No Employees".to_string(), "Please add employees before generating a schedule.".to_string(), Status::Error);
                    }

                    if let Some(db) = &self.database {
                        if let Ok(conn) = db.get_connection() {
                            if let Ok(core_employees) = EmployeeRepository::find_all(&conn) {
                                let engine = Engine::new();
                                // TODO: Pass past schedules
                                let past_schedules = std::collections::HashMap::new(); 
                                
                                let schedule = engine.generate_schedule(
                                    &core_employees,
                                    &past_schedules,
                                    self.current_date.year(),
                                    self.current_date.month(),
                                );
                                
                                self.current_schedule = Some(schedule.clone());
                                
                                // Update UI
                                apply_schedule_to_ui(&mut self.attendance_table.employees, &schedule);
                                
                                return self.show_toast("Schedule Generated".to_string(), "New schedule has been generated (not saved yet).".to_string(), Status::Success);
                            }
                        }
                    }
                }
                ActionBarMessage::Save => {
                    if let Some(schedule) = &self.current_schedule {
                        if let Some(db) = &self.database {
                            if let Ok(conn) = db.get_connection() {
                                if let Ok(Some(existing_schedule)) = ScheduleRepository::find_by_year_month(&conn, schedule.year, schedule.month) {
                                     // Check if the content is exactly the same
                                     if schedule.is_same_content(&existing_schedule) {
                                         return self.show_toast("Schedule Exists".to_string(), "This exact schedule is already saved.".to_string(), Status::Info);
                                     }
                                     // If different, we proceed to save (overwrite)
                                }

                                match ScheduleRepository::save(&conn, schedule) {
                                    Ok(_) => return self.show_toast("Schedule Saved".to_string(), "Schedule successfully saved to database.".to_string(), Status::Success),
                                    Err(e) => return self.show_toast("Save Failed".to_string(), e.to_string(), Status::Error),
                                }
                            }
                        }
                    } else {
                         return self.show_toast("No Schedule".to_string(), "Please generate a schedule first.".to_string(), Status::Info);
                    }
                }
                ActionBarMessage::Undo => {
                    // Logic:
                    // 1. Check if there is a saved schedule for the current month
                    // 2. If yes, revert to it (update current_schedule and UI)
                    // 3. If no, clear current_schedule and reset UI to N/A (which happens automatically if current_schedule is None)
                    
                    self.load_schedule_for_date();
                    
                    if self.current_schedule.is_some() {
                        return self.show_toast("Changes Undone".to_string(), "Reverted to the last saved schedule.".to_string(), Status::Success);
                    } else {
                        return self.show_toast("Changes Undone".to_string(), "Reverted to empty state (no saved schedule).".to_string(), Status::Info);
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
                TopBarMessage::SearchChanged(query) => {
                    self.search_query = query;
                    self.refresh_employees();
                },
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
                    self.load_schedule_for_date();
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
                    self.load_schedule_for_date();
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
                    return self.show_toast("Download PDF".to_string(), "PDF Download started...".to_string(), Status::Success);
                }
                ModalMessage::SubmitAdd => {
                    let form_data = if let Modal::AddEmployee(form) = &self.modal {
                        Some(form.clone())
                    } else {
                        None
                    };

                    if let Some(form) = form_data {
                        // Check for duplicate name (case-insensitive)
                        if self.attendance_table.employees.iter().any(|e| e.name.trim().eq_ignore_ascii_case(form.name.trim())) {
                            return self.show_toast("Duplicate Name".to_string(), "An employee with this name already exists.".to_string(), Status::Error);
                        }

                        if let Some(db) = &self.database {
                            if let Ok(conn) = db.get_connection() {
                                // Convert form to CoreEmployee
                                let sex = Sex::from_str(&form.sex.unwrap_or("Male".to_string())).unwrap_or(Sex::Male);
                                let role = Role::from_str(&form.role.unwrap_or("Full-stack Engineer".to_string())).unwrap_or(Role::FullStackEngineer);
                                let mut fixed_days = Vec::new();
                                for (i, status) in form.attendance.iter().enumerate() {
                                    if *status == AttendanceStatus::Office {
                                        // Fixed days
                                        match i {
                                            0 => fixed_days.push(Weekday::Monday),
                                            1 => fixed_days.push(Weekday::Tuesday),
                                            2 => fixed_days.push(Weekday::Wednesday),
                                            3 => fixed_days.push(Weekday::Thursday),
                                            4 => fixed_days.push(Weekday::Friday),
                                            _ => {}
                                        }
                                    }
                                }

                                let mut core_emp = CoreEmployee::new(
                                    form.name.clone(),
                                    sex,
                                    role,
                                    form.days_per_week.unwrap_or(0) as i32,
                                    fixed_days,
                                    form.mentor.as_deref() == Some("Yes"), // UI logic for mentor/mentee is "None" or Name. 
                                    // Wait, UI form has "Mentee" and "Mentor" dropdowns which select NAMES. 
                                    // But Core has `is_mentor` (bool) and `is_mentee` (bool) and `mentor_id` (Option<int>).
                                    // The UI currently just selects strings. 
                                    // Simplified assumption: If Mentee field has a value != "None", is_mentee = true.
                                    // If Mentor field has a value != "None", is_mentor = true? 
                                    // Actually, usually "Mentor" field means "Who is my mentor?". So if I select someone, I AM a Mentee.
                                    // "Mentee" field? If I select someone, does it mean I AM a Mentor to them?
                                    // Let's assume:
                                    // - "Mentee" dropdown: "None" or Name. If Name selected -> I am Mentor to [Name]. (Logic might be complex here without IDs).
                                    // - "Mentor" dropdown: "None" or Name. If Name selected -> I am Mentee of [Name].
                                    
                                    // For now, let's just save simple flags if possible, or ignore relationship linking by ID until better UI.
                                    // We'll set defaults for now to avoid errors.
                                    false,
                                    None
                                );
                                
                                // Try to resolve mentor ID if possible (needs lookup). skipping for now.

                                match EmployeeRepository::create(&conn, &mut core_emp) {
                                    Ok(_) => {
                                        // Do not invalidate current schedule, just refresh list
                                        // self.current_schedule = None;

                                        self.refresh_employees();
                                        self.modal = Modal::None;
                                        return self.show_toast("Employee Added".to_string(), format!("{} has been successfully added.", form.name), Status::Success);
                                    }
                                    Err(e) => {
                                        return self.show_toast("Error".to_string(), e.to_string(), Status::Error);
                                    }
                                }
                            }
                        }
                    }
                }
                ModalMessage::SubmitEdit(idx) => {
                    let form_data = if let Modal::EditEmployee(_, form) = &self.modal {
                        Some(form.clone())
                    } else {
                        None
                    };

                    if let Some(form) = form_data {
                         if let Some(employee) = self.attendance_table.employees.get(idx) {
                            // Check for duplicate name (excluding current employee)
                            if self.attendance_table.employees.iter().any(|e| e.id != employee.id && e.name.trim().eq_ignore_ascii_case(form.name.trim())) {
                                return self.show_toast("Duplicate Name".to_string(), "An employee with this name already exists.".to_string(), Status::Error);
                            }

                            if let Some(db) = &self.database {
                                if let Ok(conn) = db.get_connection() {
                                    // Fetch original to keep ID
                                    if let Ok(Some(mut core_emp)) = EmployeeRepository::find_by_id(&conn, employee.id) {
                                        core_emp.name = form.name.clone();
                                        core_emp.sex = Sex::from_str(&form.sex.unwrap_or_default()).unwrap_or(Sex::Male);
                                        core_emp.role = Role::from_str(&form.role.unwrap_or_default()).unwrap_or(Role::FullStackEngineer);
                                        core_emp.required_days = form.days_per_week.unwrap_or(0) as i32;
                                        
                                        let mut fixed_days = Vec::new();
                                        for (i, status) in form.attendance.iter().enumerate() {
                                            if *status == AttendanceStatus::Office {
                                                match i {
                                                    0 => fixed_days.push(Weekday::Monday),
                                                    1 => fixed_days.push(Weekday::Tuesday),
                                                    2 => fixed_days.push(Weekday::Wednesday),
                                                    3 => fixed_days.push(Weekday::Thursday),
                                                    4 => fixed_days.push(Weekday::Friday),
                                                    _ => {}
                                                }
                                            }
                                        }
                                        core_emp.fixed_days = fixed_days;

                                        match EmployeeRepository::update(&conn, &core_emp) {
                                            Ok(_) => {
                                                // Do not invalidate current schedule
                                                // self.current_schedule = None;

                                                self.refresh_employees();
                                                self.modal = Modal::None;
                                                return self.show_toast("Employee Updated".to_string(), format!("{}'s details have been updated.", core_emp.name), Status::Success);
                                            }
                                            Err(e) => {
                                                return self.show_toast("Error".to_string(), e.to_string(), Status::Error);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.modal = Modal::None;
                }
                ModalMessage::ConfirmDelete(idx) => {
                    if let Some(employee) = self.attendance_table.employees.get(idx) {
                        let name = employee.name.clone(); // Clone name before potential deletion logic
                        if let Some(db) = &self.database {
                            if let Ok(conn) = db.get_connection() {
                                match EmployeeRepository::delete(&conn, employee.id) {
                                    Ok(_) => {
                                        // Invalidate current schedule as employee list changed
                                        self.current_schedule = None;
                                        
                                        self.refresh_employees();
                                        self.attendance_table.selected_employee = None;
                                        self.modal = Modal::None;
                                        return self.show_toast("Employee Deleted".to_string(), format!("{} has been removed.", name), Status::Success);
                                    }
                                    Err(e) => {
                                        return self.show_toast("Error".to_string(), e.to_string(), Status::Error);
                                    }
                                }
                            }
                        }
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
                                 attendance_table::AttendanceStatus::NA => attendance_table::AttendanceStatus::Office, // Allow setting fixed days even if NA
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

        let action_bar = ActionBar::view(self.is_dark, self.attendance_table.selected_employee.is_some(), self.is_locked).map(Message::ActionBar);
        let attendance_table = self.attendance_table.view(self.is_dark, self.is_locked).map(Message::AttendanceTable);

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

// Helpers

fn core_to_ui_employee(e: CoreEmployee) -> UIEmployee {
    UIEmployee {
        id: e.id,
        name: e.name,
        role: e.role.to_string(),
        sex: e.sex.to_string(),
        days_per_week: e.required_days as u8,
        fixed_days: e.fixed_days,
        mentee: None, // TODO: Map relationships
        mentor: None, // TODO: Map relationships
        attendance: [AttendanceStatus::NA; 5], // Default to NA
        past_attendance: vec![],
    }
}

fn apply_schedule_to_ui(ui_employees: &mut [UIEmployee], schedule: &MonthlySchedule) {
    let weekdays = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday];
    
    for emp in ui_employees.iter_mut() {
        let is_included = schedule.included_employees.contains(&emp.id);

        for (i, day) in weekdays.iter().enumerate() {
            let employees_on_day = schedule.get_employees_for_day(*day);
            let is_scheduled = employees_on_day.iter().any(|e| e.id == emp.id);
            
            if is_scheduled {
                emp.attendance[i] = AttendanceStatus::Office;
            } else if is_included {
                // If they are included in the schedule but not in Office, they are Remote
                emp.attendance[i] = AttendanceStatus::Remote;
            } else {
                // If not included (new employee or legacy schedule without record), they are N/A
                emp.attendance[i] = AttendanceStatus::NA;
            }
        }
    }
}
