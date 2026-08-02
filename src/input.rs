use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::editor::Editor;

pub fn process_keypress(editor: &mut Editor) -> bool {
    let char = event::read().expect("Error: Unable to read the keypresses from your device");
    match char {
        Event::Key(key_event)
            if key_event.code == KeyCode::Char('q')
                && key_event.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            true
        }
        Event::Key(key_event) => {
            move_cursor(editor, key_event.code);
            false
        }
        _ => false,
    }
}

fn move_cursor(editor: &mut Editor, key: KeyCode) {
    // get a local state of pending_g so we don't have to reset it on every branch
    let pending_g = editor.pending_g;
    editor.pending_g = false;

    match key {
        // Vim keybindings to move through the file
        KeyCode::Left | KeyCode::Char('h') => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Right | KeyCode::Char('l') if editor.cursor_x + 1 < editor.current_row_len() => {
            editor.cursor_x = editor.cursor_x.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') if editor.cursor_y + 1 < editor.text_rows.len() => {
            editor.cursor_y += 1;
        }
        // Move to last row in file
        KeyCode::Char('G') => editor.cursor_y = editor.text_rows.len().saturating_sub(1),
        // Move to first row in file
        KeyCode::Char('g') if pending_g => editor.cursor_y = 0,
        KeyCode::Char('g') => editor.pending_g = true,
        // Move to first char in row
        KeyCode::Char('0') => editor.cursor_x = 0,
        // Move to last char in row
        KeyCode::Char('$') => editor.cursor_x = editor.current_row_len().saturating_sub(1),

        // KeyCode::Char('b')
        // KeyCode::Char('B')
        // KeyCode::Char('w')
        // KeyCode::Char('W')
        // KeyCode::Char('e')
        // KeyCode::Char('E')
        _ => {}
    }

    let line_length = editor.current_row_len().saturating_sub(1);
    editor.cursor_x = editor.cursor_x.min(line_length);
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
            pending_g: false,
            text_rows: Vec::new(),
            row_offset: 0,
            col_offset: 0,
        }
    }

    #[test]
    fn test_movement_vim() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        let mut key = KeyCode::Char('l');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Char('j');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        key = KeyCode::Char('h');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        key = KeyCode::Char('k');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Char('h');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Char('k');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn test_movement_arrows() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        let mut key = KeyCode::Right;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Down;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        key = KeyCode::Left;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        key = KeyCode::Up;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Left;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Up;
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn capital_g_goes_to_end() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        let mut key = KeyCode::Char('G');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_y, editor.text_rows.len() - 1);

        key = KeyCode::Char('G');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_y, editor.text_rows.len() - 1);
    }

    #[test]
    fn gg_goes_to_top() {
        let mut editor = build_app();
        editor.cursor_y = 10;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        let mut key = KeyCode::Char('g');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        key = KeyCode::Char('g');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn interrupt_g_doesnt_go_to_top() {
        let mut editor = build_app();
        editor.cursor_y = 10;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        let mut key = KeyCode::Char('g');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        key = KeyCode::Char('h');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        key = KeyCode::Char('g');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);
    }

    #[test]
    fn zero_goes_to_first_char() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.cursor_x = 5;
        assert_eq!(editor.cursor_x, 5);
        assert_eq!(editor.cursor_y, 0);

        let mut key = KeyCode::Char('0');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Char('0');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn dollarsign_goes_to_last_char() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        let mut key = KeyCode::Char('$');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, editor.text_rows[0].chars.len() - 1);
        assert_eq!(editor.cursor_y, 0);

        key = KeyCode::Char('$');
        move_cursor(&mut editor, key);
        assert_eq!(editor.cursor_x, editor.text_rows[0].chars.len() - 1);
        assert_eq!(editor.cursor_y, 0);
    }
}
