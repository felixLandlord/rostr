use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::core::models::{Employee, Sex, Role, Weekday};
use std::str::FromStr;
use serde_json;

pub struct EmployeeRepository {
    // We create a new connection per operation or pass it in. 
    // Ideally, we pass a connection or a pool. 
    // Since we are using sqlite (file based), opening a connection is cheap but better to share if possible.
    // But `rusqlite::Connection` is not thread-safe (Send but not Sync).
    // For simplicity in this "core" lib, we will accept a Connection reference or create one.
    // The Go code had `db *Database` which held `*sql.DB` (pool).
    // Rusqlite doesn't have a built-in pool.
    // I will pass `&Connection` to methods.
}

impl EmployeeRepository {
    pub fn create(conn: &Connection, emp: &mut Employee) -> Result<()> {
        let fixed_days_json = serde_json::to_string(&emp.fixed_days).unwrap(); // Should handle error properly but it's a Vec<enum> so it should be fine.
        
        conn.execute(
            "INSERT INTO employees (name, sex, role, required_days, fixed_days, is_mentor, is_mentee, mentor_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                emp.name,
                emp.sex.to_string(),
                emp.role.to_string(),
                emp.required_days,
                fixed_days_json,
                emp.is_mentor,
                emp.is_mentee,
                emp.mentor_id
            ],
        )?;

        emp.id = conn.last_insert_rowid() as i32;
        Ok(())
    }

    pub fn update(conn: &Connection, emp: &Employee) -> Result<()> {
        let fixed_days_json = serde_json::to_string(&emp.fixed_days).unwrap();

        conn.execute(
            "UPDATE employees
             SET name = ?1, sex = ?2, role = ?3, required_days = ?4, fixed_days = ?5, is_mentor = ?6, is_mentee = ?7, mentor_id = ?8, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?9",
            params![
                emp.name,
                emp.sex.to_string(),
                emp.role.to_string(),
                emp.required_days,
                fixed_days_json,
                emp.is_mentor,
                emp.is_mentee,
                emp.mentor_id,
                emp.id
            ],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM employees WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_all(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM employees", [])?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: i32) -> Result<Option<Employee>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, sex, role, required_days, fixed_days, is_mentor, is_mentee, mentor_id
             FROM employees WHERE id = ?1",
        )?;

        let employee = stmt.query_row(params![id], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let sex_str: String = row.get(2)?;
            let role_str: String = row.get(3)?;
            let required_days: i32 = row.get(4)?;
            let fixed_days_json: String = row.get(5)?;
            let is_mentor: bool = row.get(6)?;
            let is_mentee: bool = row.get(7)?;
            let mentor_id: Option<i32> = row.get(8)?;

            let sex = Sex::from_str(&sex_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let role = Role::from_str(&role_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let fixed_days: Vec<Weekday> = serde_json::from_str(&fixed_days_json).unwrap_or_default();

            Ok(Employee {
                id,
                name,
                sex,
                role,
                required_days,
                fixed_days,
                is_mentor,
                is_mentee,
                mentor_id,
            })
        }).optional()?;

        Ok(employee)
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<Employee>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, sex, role, required_days, fixed_days, is_mentor, is_mentee, mentor_id
             FROM employees ORDER BY name",
        )?;

        let employee_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let sex_str: String = row.get(2)?;
            let role_str: String = row.get(3)?;
            let required_days: i32 = row.get(4)?;
            let fixed_days_json: String = row.get(5)?;
            let is_mentor: bool = row.get(6)?;
            let is_mentee: bool = row.get(7)?;
            let mentor_id: Option<i32> = row.get(8)?;

            let sex = Sex::from_str(&sex_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let role = Role::from_str(&role_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let fixed_days: Vec<Weekday> = serde_json::from_str(&fixed_days_json).unwrap_or_default();

            Ok(Employee {
                id,
                name,
                sex,
                role,
                required_days,
                fixed_days,
                is_mentor,
                is_mentee,
                mentor_id,
            })
        })?;

        let mut employees = Vec::new();
        for emp in employee_iter {
            employees.push(emp?);
        }
        Ok(employees)
    }

    pub fn search(conn: &Connection, query: &str) -> Result<Vec<Employee>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, sex, role, required_days, fixed_days, is_mentor, is_mentee, mentor_id
             FROM employees
             WHERE LOWER(name) LIKE ?1
             ORDER BY name",
        )?;

        let search_query = format!("%{}%", query.to_lowercase());
        
        let employee_iter = stmt.query_map(params![search_query], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let sex_str: String = row.get(2)?;
            let role_str: String = row.get(3)?;
            let required_days: i32 = row.get(4)?;
            let fixed_days_json: String = row.get(5)?;
            let is_mentor: bool = row.get(6)?;
            let is_mentee: bool = row.get(7)?;
            let mentor_id: Option<i32> = row.get(8)?;

            let sex = Sex::from_str(&sex_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let role = Role::from_str(&role_str).map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            let fixed_days: Vec<Weekday> = serde_json::from_str(&fixed_days_json).unwrap_or_default();

            Ok(Employee {
                id,
                name,
                sex,
                role,
                required_days,
                fixed_days,
                is_mentor,
                is_mentee,
                mentor_id,
            })
        })?;

        let mut employees = Vec::new();
        for emp in employee_iter {
            employees.push(emp?);
        }
        Ok(employees)
    }
}
