use std::{fs, io::Result, path::Path};

use crossterm::terminal;

pub struct Editor {
    pub cols: u16,
    pub rows: u16,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub pending_g: bool,
    pub text_rows: Vec<EditorRow>,
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
}

pub struct EditorRow {
    pub chars: String,
}
