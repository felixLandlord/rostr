use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::core::models::{Employee, MonthlySchedule};

pub fn import_employees_json<P: AsRef<Path>>(path: P) -> Result<Vec<Employee>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let employees = serde_json::from_reader(reader)?;
    Ok(employees)
}

pub fn export_employees_json<P: AsRef<Path>>(path: P, employees: &[Employee]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, employees)?;
    Ok(())
}

pub fn export_schedule_json<P: AsRef<Path>>(path: P, schedule: &MonthlySchedule) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, schedule)?;
    Ok(())
}
