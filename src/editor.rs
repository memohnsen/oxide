use std::{fs, io::Result, path::Path, time::Instant};

use crossterm::terminal;

use crate::modes::Modes;

const TAB_WIDTH: usize = 4;

#[derive(Clone)]
pub struct StatusMessage {
    pub text: String,
    pub created_at: Instant,
}

pub struct Editor {
    pub cols: u16,
    pub rows: u16,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub render_x: usize,
    // tracks whether gg to go top of file is going to be pressed
    pub pending_g: bool,
    pub text_rows: Vec<EditorRow>,
    // which file line is at the top and edge of the screen
    pub row_offset: usize,
    pub col_offset: usize,
    pub mode: Modes,
    pub filename: Option<String>,
    pub status_message: Option<StatusMessage>,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (screen_cols, screen_rows) = terminal::size()?;
        Ok(Self {
            cols: screen_cols,
            rows: screen_rows.saturating_sub(2),
            cursor_x: 0,
            cursor_y: 0,
            render_x: 0,
            pending_g: false,
            text_rows: Vec::new(),
            row_offset: 0,
            col_offset: 0,
            mode: Modes::Normal,
            filename: None,
            status_message: Some(StatusMessage {
                text: "Ctrl-q to quit".to_string(),
                created_at: Instant::now(),
            }),
        })
    }

    pub fn open_file(&mut self, filename: &Path) -> Result<()> {
        let contents = fs::read_to_string(filename)?;
        self.filename = Some(filename.to_str().unwrap_or("").to_owned());

        self.text_rows = contents
            .lines()
            .map(|line| EditorRow::new(line.to_owned()))
            .collect();

        Ok(())
    }

    pub fn current_row_len(&self) -> usize {
        self.text_rows
            .get(self.cursor_y)
            .map_or(0, |row| row.chars.chars().count())
    }

    pub fn scroll(&mut self) {
        // VERTICAL SCROLL
        let screen_rows = usize::from(self.rows);

        if screen_rows == 0 {
            return;
        }

        // TODO: this should be defined in a config file
        let v_margin = 5.min(screen_rows.saturating_sub(1) / 2);

        // if cursor is above the offset scroll up
        if self.cursor_y < self.row_offset + v_margin {
            self.row_offset = self.cursor_y.saturating_sub(v_margin);
        }

        // if cursor is below the offset scroll down
        if self.cursor_y >= self.row_offset + screen_rows - v_margin {
            self.row_offset = self.cursor_y + v_margin + 1 - screen_rows;
        }

        // HORIZONTAL SCROLL
        let screen_cols = usize::from(self.cols);
        let h_margin = 5.min(screen_cols.saturating_sub(1) / 2);

        if self.cursor_x < self.col_offset + h_margin {
            self.col_offset = self.cursor_x.saturating_sub(h_margin);
        }

        if self.cursor_x >= self.col_offset + screen_cols - h_margin {
            self.col_offset = self
                .cursor_x
                .saturating_add(h_margin)
                .saturating_add(1)
                .saturating_sub(screen_cols);
        }
    }
}

pub struct EditorRow {
    pub chars: String,
    pub render: String,
}

impl EditorRow {
    pub fn new(chars: String) -> Self {
        let mut render = String::new();

        for char in chars.chars() {
            if char == '\t' {
                render.push_str(&" ".repeat(TAB_WIDTH));
            } else {
                render.push(char);
            }
        }

        Self { chars, render }
    }

    // pub fn render_x(&self, cursor_x: usize) -> usize {
    //     let pos = 0;
    //     for char in self.chars {
    //
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

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
            status_message: None,
        }
    }

    #[test]
    fn test_line_length() {
        let mut editor = build_app();
        editor.text_rows = vec![EditorRow::new("Hello, this is a test.".to_string())];
        assert_eq!(editor.current_row_len(), 22);

        editor.text_rows = vec![EditorRow::new(String::new())];
        assert_eq!(editor.current_row_len(), 0);
    }

    #[test]
    fn file_opens() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let first_line = "use crossterm::{cursor, execute, style, terminal};".to_string();
        assert_eq!(editor.text_rows[0].chars, first_line);

        let second_line = "use std::io::{Result, stdout};".to_string();
        assert_eq!(editor.text_rows[1].chars, second_line);

        let third_line = String::new();
        assert_eq!(editor.text_rows[2].chars, third_line);
    }

    #[test]
    fn render_tabs() {
        let mut editor = build_app();
        let line = "\tabc".to_string();
        editor.text_rows = vec![EditorRow::new(line)];
        assert_eq!(editor.text_rows[0].render, "    abc");

        let line = "\t\tabc".to_string();
        editor.text_rows = vec![EditorRow::new(line)];
        assert_eq!(editor.text_rows[0].render, "        abc");

        let line = "\ta\tbc".to_string();
        editor.text_rows = vec![EditorRow::new(line)];
        assert_eq!(editor.text_rows[0].render, "    a    bc");
    }
}
