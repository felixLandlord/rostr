use crate::core::models::types::Weekday;
use crate::core::models::employee::Employee;
use chrono::{DateTime, Utc, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySchedule {
    pub year: i32,
    pub month: u32,
    pub schedules: HashMap<Weekday, Vec<Employee>>,
    #[serde(default)]
    pub included_employees: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MonthlySchedule {
    pub fn new(year: i32, month: u32) -> Self {
        let mut schedules = HashMap::new();
        for day in Weekday::all() {
            schedules.insert(day, Vec::new());
        }

        Self {
            year,
            month,
            schedules,
            included_employees: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_employee(&mut self, day: Weekday, employee: Employee) {
        self.schedules.entry(day).or_default().push(employee);
        self.updated_at = Utc::now();
    }

    pub fn remove_employee(&mut self, day: Weekday, employee_id: i32) {
        if let Some(employees) = self.schedules.get_mut(&day) {
            if let Some(pos) = employees.iter().position(|e| e.id == employee_id) {
                employees.remove(pos);
                self.updated_at = Utc::now();
            }
        }
    }

    pub fn get_employees_for_day(&self, day: Weekday) -> Vec<Employee> {
        self.schedules.get(&day).cloned().unwrap_or_default()
    }

    pub fn get_days_for_employee(&self, employee_id: i32) -> Vec<Weekday> {
        let mut days = Vec::new();
        for (day, employees) in &self.schedules {
            if employees.iter().any(|e| e.id == employee_id) {
                days.push(*day);
            }
        }
        days
    }

    pub fn day_count(&self) -> HashMap<Weekday, usize> {
        let mut counts = HashMap::new();
        for (day, employees) in &self.schedules {
            counts.insert(*day, employees.len());
        }
        counts
    }

    pub fn total_employees(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for employees in self.schedules.values() {
            for emp in employees {
                seen.insert(emp.id);
            }
        }
        seen.len()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.year < 2000 || self.year > 2100 {
            return Err(format!("invalid year: {}", self.year));
        }
        if self.month < 1 || self.month > 12 {
            return Err(format!("invalid month: {}", self.month));
        }
        Ok(())
    }

    pub fn is_same_content(&self, other: &Self) -> bool {
        // Compare year and month
        if self.year != other.year || self.month != other.month {
            return false;
        }

        // Compare schedule content (ignore timestamps)
        // We need to compare the map of employees per day
        if self.schedules.len() != other.schedules.len() {
            return false;
        }

        for (day, employees) in &self.schedules {
            match other.schedules.get(day) {
                Some(other_employees) => {
                    if employees.len() != other_employees.len() {
                        return false;
                    }
                    
                    // We need to check if the sets of employee IDs are the same for this day
                    let mut self_ids: Vec<i32> = employees.iter().map(|e| e.id).collect();
                    let mut other_ids: Vec<i32> = other_employees.iter().map(|e| e.id).collect();
                    
                    self_ids.sort();
                    other_ids.sort();
                    
                    if self_ids != other_ids {
                        return false;
                    }
                },
                None => return false,
            }
        }

        true
    }
}

pub type PastSchedules = HashMap<i32, Vec<HashMap<Weekday, bool>>>;

pub trait PastSchedulesExt {
    fn get_employee_past_schedules(&self, employee_id: i32, limit: usize) -> Vec<HashMap<Weekday, bool>>;
}

impl PastSchedulesExt for PastSchedules {
    fn get_employee_past_schedules(&self, employee_id: i32, limit: usize) -> Vec<HashMap<Weekday, bool>> {
        if let Some(schedules) = self.get(&employee_id) {
            if schedules.len() <= limit {
                schedules.clone()
            } else {
                schedules[schedules.len() - limit..].to_vec()
            }
        } else {
            Vec::new()
        }
    }
}
