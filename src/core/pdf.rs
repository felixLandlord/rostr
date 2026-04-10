use printpdf::*;
use std::io::{BufWriter, Cursor};

pub fn generate_pdf_data(markdown: &str) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) =
        PdfDocument::new("Attendance Report", Mm(210.0), Mm(297.0), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to load font: {}", e))?;
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to load bold font: {}", e))?;

    let mut y_position = 280.0;
    let line_height = 6.0;
    let margin_left = 20.0;
    let page_width = 170.0;

    for line in markdown.lines() {
        if y_position < 20.0 {
            // TODO: Add new page
            break;
        }

        let (text, size, use_bold) = if line.starts_with("# ") {
            (&line[2..], 18.0, true)
        } else if line.starts_with("## ") {
            (&line[3..], 14.0, true)
        } else if line.starts_with("### ") {
            (&line[4..], 12.0, true)
        } else if line.starts_with("- ") || line.starts_with("* ") {
            (line, 10.0, false)
        } else if line.trim().is_empty() {
            (" ", 10.0, false)
        } else {
            (line, 10.0, false)
        };

        let selected_font = if use_bold { &font_bold } else { &font };

        let wrapped_lines = wrap_text(text, page_width, size);

        for wrapped_line in wrapped_lines {
            if y_position < 20.0 {
                break;
            }

            current_layer.use_text(
                wrapped_line,
                size,
                Mm(margin_left),
                Mm(y_position),
                selected_font,
            );

            y_position -= line_height;
        }

        if line.starts_with("# ") || line.starts_with("## ") {
            y_position -= line_height;
        }
    }

    let mut buffer = Vec::new();
    doc.save(&mut BufWriter::new(Cursor::new(&mut buffer)))
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(buffer)
}

pub async fn save_pdf_with_dialog(
    suggested_filename: String,
    pdf_data: Vec<u8>,
) -> Result<(), String> {
    let file_handle = rfd::AsyncFileDialog::new()
        .add_filter("PDF Document", &["pdf"])
        .set_file_name(&suggested_filename)
        .set_title("Save Attendance Report as PDF")
        .save_file()
        .await;

    match file_handle {
        Some(handle) => {
            match handle.write(&pdf_data).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write PDF file: {}", e)),
            }
        }
        None => Ok(()), // User cancellation
    }
}

pub fn generate_pdf_from_markdown(markdown: &str, output_path: &str) -> Result<(), String> {
    let pdf_data = generate_pdf_data(markdown)?;
    std::fs::write(output_path, pdf_data).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let test_width = text_width(&test_line, font_size);

        if test_width > max_width && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

fn text_width(text: &str, font_size: f32) -> f32 {
    text.len() as f32 * font_size * 0.5
}
