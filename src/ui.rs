use crossterm::{cursor, execute, style, terminal};
use std::io::{Result, stdout};

use crate::editor::Editor;

const WELCOME_MESSAGE: &str = "Oxide Editor -- Version 0.1.0";

pub fn draw_rows(screen: &Editor) -> String {
    let mut buffer = String::new();
    let rows = screen.rows;
    let cols = screen.cols;

    for row in 0..rows {
        if let Some(text_row) = screen.text_rows.get(row as usize) {
            buffer.extend(text_row.chars.chars().take(cols as usize));
        } else if screen.text_rows.is_empty() && row == rows / 3 {
            show_home_screen(cols, &mut buffer);
        } else {
            buffer.push('~');
        }

        if row + 1 < rows {
            buffer.push_str("\r\n");
        }
    }

    buffer
}

/// Clear the screen and put the cursor a top left
pub fn refresh_screen(screen: &Editor) -> Result<()> {
    let buffer_rows = draw_rows(screen);
    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print(buffer_rows),
        cursor::MoveTo(screen.cursor_x, screen.cursor_y),
        cursor::Show
    )?;

    Ok(())
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
