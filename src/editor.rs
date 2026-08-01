use std::{fs, io::Result, path::Path};

use crossterm::terminal;

pub struct Editor {
    pub cols: u16,
    pub rows: u16,
    pub cursor_x: u16,
    pub cursor_y: usize,
    pub pending_g: bool,
    pub text_rows: Vec<EditorRow>,
    // which file line is at the top of the screen
    pub row_offset: usize,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (screen_cols, screen_rows) = terminal::size()?;
        Ok(Self {
            cols: screen_cols,
            rows: screen_rows,
            cursor_x: 0,
            cursor_y: 0,
            pending_g: false,
            text_rows: Vec::new(),
            row_offset: 0,
        })
    }

    pub fn open_file(&mut self, filename: &Path) -> Result<()> {
        let contents = fs::read_to_string(filename)?;

        self.text_rows = contents
            .lines()
            .map(|line| EditorRow {
                chars: line.to_owned(),
            })
            .collect();

        Ok(())
    }

    pub fn scroll(&mut self) {
        let screen_rows = usize::from(self.rows);

        if screen_rows == 0 {
            return;
        }

        // TODO: this should be defined in a config file
        let margin = 5.min(screen_rows.saturating_sub(1) / 2);

        // if cursor is above the offset scroll up
        if self.cursor_y < self.row_offset + margin {
            self.row_offset = self.cursor_y.saturating_sub(margin);
        }

        // if cursor is below the offset scroll down
        if self.cursor_y >= self.row_offset + screen_rows - margin {
            self.row_offset = self.cursor_y + margin + 1 - screen_rows;
        }
    }
}

pub struct EditorRow {
    pub chars: String,
}
