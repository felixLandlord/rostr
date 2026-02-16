use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::core::models::{MonthlySchedule, PastSchedules, Employee, Weekday};
use serde_json;
use std::collections::HashMap;

pub struct ScheduleRepository;

impl ScheduleRepository {
    pub fn save(conn: &Connection, schedule: &MonthlySchedule) -> Result<()> {
        let schedule_json = serde_json::to_string(schedule).unwrap();

        conn.execute(
            "INSERT INTO schedules (year, month, schedule_data)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(year, month) DO UPDATE SET
                schedule_data = excluded.schedule_data,
                updated_at = CURRENT_TIMESTAMP",
            params![
                schedule.year,
                schedule.month,
                schedule_json
            ],
        )?;
        Ok(())
    }

    pub fn find_by_year_month(conn: &Connection, year: i32, month: u32) -> Result<Option<MonthlySchedule>> {
        let mut stmt = conn.prepare(
            "SELECT schedule_data FROM schedules
             WHERE year = ?1 AND month = ?2",
        )?;

        let schedule = stmt.query_row(params![year, month], |row| {
            let schedule_json: String = row.get(0)?;
            let schedule: MonthlySchedule = serde_json::from_str(&schedule_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            Ok(schedule)
        }).optional()?;

        Ok(schedule)
    }

    pub fn find_past_schedules(conn: &Connection, year: i32, month: u32, lookback: i32) -> Result<PastSchedules> {
        let mut stmt = conn.prepare(
            "SELECT schedule_data FROM schedules
             WHERE (year < ?1 OR (year = ?1 AND month < ?2))
             ORDER BY year DESC, month DESC
             LIMIT ?3",
        )?;

        let rows = stmt.query_map(params![year, month, lookback], |row| {
            let schedule_json: String = row.get(0)?;
            let schedule: MonthlySchedule = serde_json::from_str(&schedule_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            Ok(schedule)
        })?;

        let mut past_schedules: PastSchedules = HashMap::new();

        for schedule_result in rows {
            if let Ok(schedule) = schedule_result {
                // Collect all unique employees in this schedule
                let mut employees_in_schedule = HashMap::new();
                for emps in schedule.schedules.values() {
                    for emp in emps {
                        employees_in_schedule.insert(emp.id, emp);
                    }
                }

                for emp_id in employees_in_schedule.keys() {
                    let entry = past_schedules.entry(*emp_id).or_insert_with(Vec::new);
                    
                    let mut month_schedule = HashMap::new();
                    for (day, emps) in &schedule.schedules {
                        if emps.iter().any(|e| e.id == *emp_id) {
                            month_schedule.insert(*day, true);
                        }
                    }
                    entry.push(month_schedule);
                }
            }
        }

        Ok(past_schedules)
    }

    pub fn find_recent_schedules(conn: &Connection, limit: i32) -> Result<Vec<MonthlySchedule>> {
        let mut stmt = conn.prepare(
            "SELECT schedule_data FROM schedules
             ORDER BY year DESC, month DESC
             LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            let schedule_json: String = row.get(0)?;
            let schedule: MonthlySchedule = serde_json::from_str(&schedule_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            Ok(schedule)
        })?;

        let mut schedules = Vec::new();
        for row in rows {
            schedules.push(row?);
        }

        Ok(schedules)
    }

    pub fn delete(conn: &Connection, year: i32, month: u32) -> Result<()> {
        conn.execute(
            "DELETE FROM schedules WHERE year = ?1 AND month = ?2",
            params![year, month],
        )?;
        Ok(())
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<MonthlySchedule>> {
        let mut stmt = conn.prepare(
            "SELECT schedule_data FROM schedules
             ORDER BY year DESC, month DESC",
        )?;

        let schedule_iter = stmt.query_map([], |row| {
            let schedule_json: String = row.get(0)?;
            let schedule: MonthlySchedule = serde_json::from_str(&schedule_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            Ok(schedule)
        })?;

        let mut schedules = Vec::new();
        for s in schedule_iter {
            schedules.push(s?);
        }
        Ok(schedules)
    }

    pub fn has_future_schedule(conn: &Connection, current_year: i32, current_month: u32) -> Result<bool> {
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM schedules WHERE year > ?1 OR (year = ?1 AND month > ?2)"
        )?;
        
        let count: i32 = stmt.query_row(params![current_year, current_month], |row| row.get(0))?;
        Ok(count > 0)
    }
}
