use std::error::Error;
use std::path::Path;
use rust_xlsxwriter::{Workbook, Format, Color, FormatAlign, FormatBorder};
use crate::core::models::{Employee, MonthlySchedule, Weekday};
use chrono::Month;
use std::collections::{HashMap, HashSet};

// Helper to get month name safely
fn get_month_name(month: u32) -> String {
    Month::try_from(month as u8)
        .map(|m| m.name().to_string())
        .unwrap_or_else(|_| format!("Month_{}", month)) // Fallback for invalid month
}

pub fn generate_xlsx_data(
    schedule: &MonthlySchedule,
) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    let month_name = get_month_name(schedule.month);
    let filename = format!("office_schedule_{}_{}.xlsx", month_name, schedule.year);

    let weekdays = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
    ];

    // Create a new workbook
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet().set_name("Schedule")?;

    // Define formats
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x4F81BD))
        .set_font_color(Color::White)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_size(14);

    let count_format = Format::new()
        .set_italic()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0x4F81BD))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin)
        .set_font_size(13);

    let name_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_size(11);

    let data_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let x_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_bold()
        .set_font_color(Color::RGB(0x7A52A3))
        .set_border(FormatBorder::Thin)
        .set_font_size(12);

    // Set column widths
    worksheet.set_column_width(0, 17.0)?; // Name column
    for i in 1..=5 {
        worksheet.set_column_width(i as u16, 12.0)?; // Weekday columns
    }

    // --- Header Row 1 ---
    worksheet.write_string_with_format(0, 0, "Name", &header_format)?;
    for (i, day) in weekdays.iter().enumerate() {
        worksheet.write_string_with_format(0, (i + 1) as u16, &day.to_string(), &header_format)?;
    }

    // --- Header Row 2 (Counts) ---
    worksheet.write_string_with_format(1, 0, "", &count_format)?; // Empty cell for name column
    
    let day_counts = schedule.day_count();
    for (i, day) in weekdays.iter().enumerate() {
        let count = day_counts.get(day).unwrap_or(&0);
        worksheet.write_string_with_format(
            1,
            (i + 1) as u16,
            &format!("{}", count),
            &count_format,
        )?;
    }

    // --- Data Rows ---
    // 1. Collect all unique employees and sort them by name
    let mut all_employees: Vec<Employee> = Vec::new();
    let mut seen = HashSet::new();
    
    // Check included employees (even if not assigned to any day)
    // Rostr's MonthlySchedule has included_employees IDs, but we might not have the full objects here easily
    // unless we fetch them or they are in the schedules.
    // However, the current structure of MonthlySchedule stores Vec<Employee> in schedules.
    // Let's iterate over schedules values.
    
    for emps in schedule.schedules.values() {
        for emp in emps {
            if seen.insert(emp.id) {
                all_employees.push(emp.clone());
            }
        }
    }
    
    all_employees.sort_by(|a, b| a.name.cmp(&b.name));

    // 2. Create assignment lookup
    let assignments: HashMap<Weekday, HashSet<i32>> = schedule.schedules
        .iter()
        .map(|(day, emps)| {
            let ids = emps.iter().map(|e| e.id).collect::<HashSet<_>>();
            (day.clone(), ids)
        })
        .collect();

    // 3. Generate data rows
    for (row_idx, emp) in all_employees.iter().enumerate() {
        let excel_row = (row_idx + 2) as u32; // Start from row 2 (0-indexed, after headers)

        // Employee name (bold)
        worksheet.write_string_with_format(excel_row, 0, &emp.name, &name_format)?;

        // Assignment status for each day
        for (col_idx, day) in weekdays.iter().enumerate() {
            let excel_col = (col_idx + 1) as u16;
            let is_assigned = assignments
                .get(day)
                .map_or(false, |ids_on_day| ids_on_day.contains(&emp.id));

            if is_assigned {
                worksheet.write_string_with_format(excel_row, excel_col, "X", &x_format)?;
            } else {
                worksheet.write_string_with_format(excel_row, excel_col, "", &data_format)?;
            }
        }
    }

    // Convert workbook to bytes
    let xlsx_data = workbook.save_to_buffer()?;

    Ok((filename, xlsx_data))
}

pub async fn save_xlsx_with_dialog(
    suggested_filename: String,
    xlsx_data: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let file_handle = rfd::AsyncFileDialog::new()
        .add_filter("Excel", &["xlsx"])
        .set_file_name(&suggested_filename)
        .set_title("Save Schedule as Excel")
        .save_file()
        .await;

    match file_handle {
        Some(handle) => {
            // rfd's FileHandle provides an async write method that works on both native and wasm
            match handle.write(&xlsx_data).await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        }
        None => Ok(()), // User cancellation is not an error
    }
}

// Keep the old function for compatibility if needed, but it's not really needed anymore
pub fn export_schedule_excel<P: AsRef<Path>>(path: P, schedule: &MonthlySchedule) -> Result<(), Box<dyn Error>> {
    let (_, data) = generate_xlsx_data(schedule)?;
    std::fs::write(path, data)?;
    Ok(())
}
