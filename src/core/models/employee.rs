use serde::{Deserialize, Serialize};
use crate::core::models::types::{Role, Sex, Weekday};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub sex: Sex,
    pub role: Role,
    pub required_days: i32,
    pub fixed_days: Vec<Weekday>,
    pub is_mentor: bool,
    pub is_mentee: bool,
    pub mentor_id: Option<i32>,
}

impl Employee {
    pub fn new(
        name: String,
        sex: Sex,
        role: Role,
        required_days: i32,
        fixed_days: Vec<Weekday>,
        is_mentor: bool,
        is_mentee: bool,
        mentor_id: Option<i32>,
    ) -> Self {
        Self {
            id: 0,
            name,
            sex,
            role,
            required_days,
            fixed_days,
            is_mentor,
            is_mentee,
            mentor_id,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("employee name cannot be empty".to_string());
        }

        if self.required_days < 0 || self.required_days > 5 {
            return Err("required days must be between 0 and 5".to_string());
        }

        if self.fixed_days.len() > self.required_days as usize {
            return Err("fixed days cannot exceed required days".to_string());
        }

        let mut seen = HashSet::new();
        for day in &self.fixed_days {
            if !seen.insert(day) {
                return Err(format!("duplicate fixed day: {}", day));
            }
        }

        Ok(())
    }

    pub fn has_fixed_day(&self, day: Weekday) -> bool {
        self.fixed_days.contains(&day)
    }
}
