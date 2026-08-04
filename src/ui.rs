use crossterm::{cursor, execute, style, terminal};
use std::{
    io::{Result, stdout},
    time::Duration,
};

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
    let command_row = show_command_row(screen);

    execute!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print(buffer_rows),
        style::SetAttribute(style::Attribute::Reverse),
        style::Print(status_bar),
        style::SetAttribute(style::Attribute::Reset),
        style::Print(command_row),
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

    let mut left_text = format!(
        " {} | {} | {} lines",
        screen.mode,
        screen.filename.clone().unwrap_or("NO FILE".to_string()),
        screen.text_rows.len(),
    );
    let mut right_text = format!("{}:{}", screen.cursor_y + 1, screen.cursor_x + 1);

    right_text.truncate(screen.cols as usize);
    let left_space_available = screen.cols.saturating_sub(right_text.len() as u16);
    left_text.truncate(left_space_available as usize);

    let filler_len = screen
        .cols
        .saturating_sub(left_text.len() as u16 + right_text.len() as u16);
    let filler = " ".repeat(filler_len as usize);

    output.push_str(left_text.as_str());
    output.push_str(filler.as_str());
    output.push_str(right_text.as_str());

    output
}

pub fn show_command_row(screen: &Editor) -> String {
    if let Some(status_msg) = &screen.status_message {
        if status_msg.created_at.elapsed() > Duration::from_secs(5) {
            String::new()
        } else {
            status_msg.text.chars().take(screen.cols as usize).collect()
        }
    } else {
        String::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::StatusMessage, modes::Modes};
    use std::time::Instant;

    fn build_app() -> Editor {
        Editor {
            cols: 100,
            rows: 40,
            cursor_y: 0,
            cursor_x: 0,
            render_x: 0,
            pending_g: false,
            text_rows: Vec::new(),
            row_offset: 0,
            col_offset: 0,
            mode: Modes::Normal,
            filename: None,
            status_message: Some(StatusMessage {
                text: String::from("Ctrl-q to quit"),
                created_at: Instant::now(),
            }),
        }
    }

    #[test]
    fn status_bar_string() {
        let mut editor = build_app();
        editor.cols = 31;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, " NORMAL | NO FILE | 0 lines 1:1");

        editor.cols = 40;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, " NORMAL | NO FILE | 0 lines          1:1");

        editor.cols = 30;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, " NORMAL | NO FILE | 0 lines1:1");

        editor.cols = 20;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, " NORMAL | NO FILE1:1");

        editor.cols = 10;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, " NORMAL1:1");

        editor.cols = 2;
        let status_bar = show_status_bar(&editor);
        assert_eq!(status_bar, "1:");
    }

    #[test]
    fn command_row_string() {
        let mut editor = build_app();
        let status_msg = show_command_row(&editor);
        assert_eq!(status_msg, "Ctrl-q to quit");

        editor.status_message = Some(StatusMessage {
            text: "Ctrl-q to quit".to_string(),
            created_at: Instant::now() - Duration::from_secs(6),
        });
        let status_msg = show_command_row(&editor);
        assert_eq!(status_msg, String::new());

        editor.status_message = None;
        assert_eq!(show_command_row(&editor), String::new());

        editor.status_message = Some(StatusMessage {
            text: "Ctrl-q to quit".to_string(),
            created_at: Instant::now(),
        });
        editor.cols = 4;
        let status_msg = show_command_row(&editor);
        assert_eq!(status_msg, "Ctrl");
    }
}
