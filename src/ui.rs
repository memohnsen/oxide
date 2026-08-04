use crossterm::{cursor, execute, style, terminal};
use std::io::{Result, stdout};

use crate::editor::Editor;

const WELCOME_MESSAGE: &str = "Oxide Editor -- Version 0.1.0";

pub fn draw_rows(screen: &Editor) -> String {
    let mut buffer = String::new();
    let rows = screen.rows;
    let cols = screen.cols;

    for row in 0..rows {
        let file_row = usize::from(row) + screen.row_offset;
        if let Some(text_row) = screen.text_rows.get(file_row) {
            buffer.extend(
                text_row
                    .render
                    .chars()
                    .skip(screen.col_offset)
                    .take(cols as usize),
            );
        } else if screen.text_rows.is_empty() && row == rows / 3 {
            show_home_screen(cols, &mut buffer);
        } else {
            buffer.push('~');
        }

        if row + 1 < rows {
            buffer.push_str("\r\n");
        }
    }
    buffer.push_str("\r\n");

    buffer
}

pub fn refresh_screen(screen: &Editor) -> Result<()> {
    let buffer_rows = draw_rows(screen);
    let status_bar = show_status_bar(screen);

    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print(buffer_rows),
        style::SetAttribute(style::Attribute::Reverse),
        style::Print(status_bar),
        style::SetAttribute(style::Attribute::Reset),
        cursor::MoveTo(
            (screen.cursor_x - screen.col_offset) as u16,
            (screen.cursor_y - screen.row_offset) as u16
        ),
        cursor::Show,
    )?;

    Ok(())
}

pub fn show_status_bar(screen: &Editor) -> String {
    let mut output = String::new();

    let text = format!(
        " {} | {} | {} lines",
        screen.mode,
        screen.filename.clone().unwrap_or("NO FILE".to_string()),
        screen.text_rows.len(),
    );
    output.push_str(text.as_str());

    output.push_str(&" ".repeat((screen.cols as usize) - text.len() - 5));

    let location = format!("{:2}:{:2}", screen.cursor_y, screen.cursor_x);
    output.push_str(&location);

    output
}

fn show_home_screen(cols: u16, buffer: &mut String) {
    let message_len = WELCOME_MESSAGE.len().min(cols as usize);
    let mut padding = (cols as usize - message_len) / 2;

    if padding > 0 {
        buffer.push('~');
        padding -= 1;
    }

    for _ in 0..padding {
        buffer.push(' ');
    }

    buffer.push_str(&WELCOME_MESSAGE[..message_len]);
}
