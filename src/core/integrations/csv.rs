use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::{ReaderBuilder, WriterBuilder};
use crate::core::models::{Employee, Sex, Role, Weekday};
use std::str::FromStr;

pub fn import_employees_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Employee>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut employees = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.len() < 8 {
            return Err("Invalid record length".into());
        }

        let name = record[0].to_string();
        let sex = Sex::from_str(&record[1])?;
        let role = Role::from_str(&record[2])?;
        let required_days: i32 = record[3].parse()?;
        
        let fixed_days_str = &record[4];
        let mut fixed_days = Vec::new();
        if !fixed_days_str.is_empty() {
            for day_str in fixed_days_str.split(';') {
                if !day_str.is_empty() {
                    fixed_days.push(Weekday::from_str(day_str)?);
                }
            }
        }

        let is_mentor: bool = record[5].parse()?;
        let is_mentee: bool = record[6].parse()?;
        
        let mentor_id_str = &record[7];
        let mentor_id = if !mentor_id_str.is_empty() {
            Some(mentor_id_str.parse()?)
        } else {
            None
        };

        let employee = Employee::new(
            name,
            sex,
            role,
            required_days,
            fixed_days,
            is_mentor,
            is_mentee,
            mentor_id,
        );
        
        employees.push(employee);
    }

    Ok(employees)
}

pub fn export_employees_csv<P: AsRef<Path>>(path: P, employees: &[Employee]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    wtr.write_record(&["Name", "Sex", "Role", "Required Days", "Fixed Days", "Is Mentor", "Is Mentee", "Mentor ID"])?;

    for emp in employees {
        let fixed_days_str: Vec<String> = emp.fixed_days.iter().map(|d| d.to_string()).collect();
        let mentor_id_str = emp.mentor_id.map(|id| id.to_string()).unwrap_or_default();

        wtr.write_record(&[
            &emp.name,
            &emp.sex.to_string(),
            &emp.role.to_string(),
            &emp.required_days.to_string(),
            &fixed_days_str.join(";"),
            &emp.is_mentor.to_string(),
            &emp.is_mentee.to_string(),
            &mentor_id_str,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
