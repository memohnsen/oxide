use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io::Result;

use crate::editor::Editor;

pub fn process_keypress(editor: &mut Editor) -> Result<bool> {
    let char = event::read()?;
    match char {
        Event::Key(key_event)
            if key_event.code == KeyCode::Char('q')
                && key_event.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            Ok(true)
        }
        Event::Key(key_event) => {
            move_cursor(editor, key_event.code);
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn move_cursor(editor: &mut Editor, key: KeyCode) {
    // get a local state of pending_g so we don't have to reset it on every branch
    let pending_g = editor.pending_g;
    editor.pending_g = false;

    match key {
        KeyCode::Left | KeyCode::Char('h') => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Right | KeyCode::Char('l') if editor.cursor_x + 1 < editor.cols => {
            editor.cursor_x = editor.cursor_x.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') if editor.cursor_y + 1 < editor.text_rows.len() => {
            editor.cursor_y += 1;
        }
        KeyCode::Char('G') => editor.cursor_y = editor.text_rows.len().saturating_sub(1),
        KeyCode::Char('g') if pending_g => editor.cursor_y = 0,
        KeyCode::Char('g') => editor.pending_g = true,
        KeyCode::Char('0') => editor.cursor_x = 0,
        KeyCode::Char('$') => editor.cursor_x = editor.cols.saturating_sub(1),

        _ => {}
    }
}
