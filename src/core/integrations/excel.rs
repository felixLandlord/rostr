use std::error::Error;
use std::path::Path;
use rust_xlsxwriter::{Workbook, Format, Color, FormatAlign, FormatBorder};
use crate::core::models::{Employee, MonthlySchedule, Weekday};

pub fn export_schedule_excel<P: AsRef<Path>>(path: P, schedule: &MonthlySchedule) -> Result<(), Box<dyn Error>> {
    let mut workbook = Workbook::new();
    
    let sheet_name = format!("{} {}", schedule.month, schedule.year); // Assuming simple month/year display
    let worksheet = workbook.add_worksheet().set_name(&sheet_name)?;

    // Formats
    let header_format = Format::new()
        .set_bold()
        .set_font_size(12)
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x4F81BD))
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin);

    let count_format = Format::new()
        .set_bold()
        .set_font_size(11)
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x4F81BD))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let name_format = Format::new()
        .set_bold()
        .set_font_size(11)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let x_format = Format::new()
        .set_bold()
        .set_font_size(12)
        .set_font_color(Color::RGB(0x7A52A3))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);
    
    let cell_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    // Set column widths
    worksheet.set_column_width(0, 20)?; // A
    for i in 1..=5 {
        worksheet.set_column_width(i, 12)?;
    }

    // Write Headers
    worksheet.write_string_with_format(0, 0, "Name", &header_format)?;
    
    for (i, day) in Weekday::all().iter().enumerate() {
        worksheet.write_string_with_format(0, (i + 1) as u16, day.to_string(), &header_format)?;
    }

    // Write Counts Row
    worksheet.write_blank(1, 0, &count_format)?;
    
    let day_counts = schedule.day_count();
    for (i, day) in Weekday::all().iter().enumerate() {
        let count = day_counts.get(day).unwrap_or(&0);
        worksheet.write_number_with_format(1, (i + 1) as u16, *count as f64, &count_format)?;
    }

    // Collect employees
    let mut employees: Vec<Employee> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    for emps in schedule.schedules.values() {
        for emp in emps {
            if seen.insert(emp.id) {
                employees.push(emp.clone());
            }
        }
    }
    employees.sort_by(|a, b| a.name.cmp(&b.name));

    // Write Employee Rows
    for (row_idx, emp) in employees.iter().enumerate() {
        let row = (row_idx + 2) as u32;
        worksheet.write_string_with_format(row, 0, &emp.name, &name_format)?;

        for (col_idx, day) in Weekday::all().iter().enumerate() {
            let col = (col_idx + 1) as u16;
            let is_assigned = schedule.get_employees_for_day(*day).iter().any(|e| e.id == emp.id);
            
            if is_assigned {
                worksheet.write_string_with_format(row, col, "X", &x_format)?;
            } else {
                worksheet.write_blank(row, col, &cell_format)?;
            }
        }
    }

    workbook.save(path)?;
    Ok(())
}
