use std::{env, io::Result, path};

use crossterm::terminal;
use oxide_rs::{CleanUp, editor::Editor, input::process_keypress, ui::refresh_screen};

fn main() -> Result<()> {
    // Uses crossterm fn to enable raw mode without having to manually change all the flags
    terminal::enable_raw_mode()?;
    let _clean_up = CleanUp;
    let mut editor = Editor::new()?;

    if let Some(filename) = env::args_os().nth(1) {
        editor.open_file(path::Path::new(&filename))?;
    }

    loop {
        refresh_screen(&editor)?;

        if process_keypress(&mut editor)? {
            break;
        }
    }

    Ok(())
}
