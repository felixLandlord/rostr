use crate::core::models::{Employee, Weekday};
use std::collections::HashMap;

pub struct MentorshipValidator<'a> {
    employees: HashMap<i32, &'a Employee>,
}

impl<'a> MentorshipValidator<'a> {
    pub fn new(employees: &'a [Employee]) -> Self {
        let mut emp_map = HashMap::new();
        for emp in employees {
            emp_map.insert(emp.id, emp);
        }
        Self { employees: emp_map }
    }

    pub fn validate_mentorship(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        for emp in self.employees.values() {
            if emp.is_mentee {
                if let Some(mentor_id) = emp.mentor_id {
                    if let Some(mentor) = self.employees.get(&mentor_id) {
                        if mentor.required_days == 0 {
                            warnings.push(format!(
                                "Warning: {}'s mentor ({}) works fully remote (0 office days)",
                                emp.name, mentor.name
                            ));
                        }
                    } else {
                        warnings.push(format!("Mentee {} has invalid mentor ID", emp.name));
                    }
                }
            }
        }

        warnings
    }
}
