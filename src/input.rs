use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::{
    editor::{Editor, EditorRow},
    modes::Modes,
};

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
            handle_keypress(editor, key_event.code);
            false
        }
        _ => false,
    }
}

/// #Available keybindings
///
/// ##All modes
/// - arrow keys to move through text
/// - ESC to return to normal mode
/// - Ctrl-q to quit
///
/// ##Normal Mode
///
/// ###Movement
/// - hkjl to move through text
/// - Backspace to move back one char
/// - Enter to move one row down
/// - $ or gl to move to last char in row
/// - 0 to move to start of the row
/// - gh / _ to move to first char in row
/// - gg to move to first row
/// - G to move to last row
/// - M to move to middle line of screen
///
/// ###Text Insertion
/// - a to enter insert mode to the right of the cursor
/// - A to enter insert mode at the end of the row
/// - i to enter insert mode to the left of the cursor
/// - I to enter insert mode to the left of the first char in the row
///
/// ###Modes
/// - r / R to enter replace mode ( does nothing currently )
/// - v / V to enter visual mode ( does nothing currently )
///
fn handle_keypress(editor: &mut Editor, key: KeyCode) {
    // get a local state of pending_g so we don't have to reset it on every branch
    let pending_g = editor.pending_g;
    editor.pending_g = false;

    match key {
        // ----------------------
        // ALL MODES
        // ----------------------

        // Arrow keys
        KeyCode::Left => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Right => {
            editor.cursor_x = editor.cursor_x.saturating_add(1);
        }
        KeyCode::Up => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Down if editor.cursor_y + 1 < editor.text_rows.len() => {
            editor.cursor_y += 1;
        }

        KeyCode::Esc => editor.mode = Modes::Normal,

        // ----------------------
        // NORMAL MODE
        // ----------------------

        // Vim insertions
        KeyCode::Char('a') if editor.mode == Modes::Normal => {
            editor.cursor_x += 1;
            editor.mode = Modes::Insert;
        }
        KeyCode::Char('A') if editor.mode == Modes::Normal => {
            editor.cursor_x = editor.current_row_len();
            editor.mode = Modes::Insert;
        }
        KeyCode::Char('i') if editor.mode == Modes::Normal => {
            editor.mode = Modes::Insert;
        }
        KeyCode::Char('I') if editor.mode == Modes::Normal => {
            editor.cursor_x = editor
                .text_rows
                .get(editor.cursor_y)
                .unwrap_or(&EditorRow::new(String::new()))
                .chars
                .chars()
                .position(|ch| !ch.is_whitespace())
                .unwrap_or(0);
            editor.mode = Modes::Insert;
        }

        // Modes
        KeyCode::Char('r') if editor.mode == Modes::Normal => editor.mode = Modes::Replace,
        KeyCode::Char('R') if editor.mode == Modes::Normal => editor.mode = Modes::Replace,
        KeyCode::Char('v') if editor.mode == Modes::Normal => editor.mode = Modes::Visual,
        KeyCode::Char('V') if editor.mode == Modes::Normal => editor.mode = Modes::Visual,

        // Vim movement keys
        KeyCode::Backspace if editor.mode == Modes::Normal => {
            editor.cursor_x = editor.cursor_x.saturating_sub(1)
        }
        KeyCode::Enter if editor.mode == Modes::Normal => {
            editor.cursor_y = editor.cursor_y.saturating_add(1)
        }
        KeyCode::Char('0') if editor.mode == Modes::Normal => editor.cursor_x = 0,
        KeyCode::Char('h') if pending_g && editor.mode == Modes::Normal => {
            editor.cursor_x = editor
                .text_rows
                .get(editor.cursor_y)
                .unwrap_or(&EditorRow::new(String::new()))
                .chars
                .chars()
                .position(|ch| !ch.is_whitespace())
                .unwrap_or(0);
        }
        KeyCode::Char('_') if editor.mode == Modes::Normal => {
            editor.cursor_x = editor
                .text_rows
                .get(editor.cursor_y)
                .unwrap_or(&EditorRow::new(String::new()))
                .chars
                .chars()
                .position(|ch| !ch.is_whitespace())
                .unwrap_or(0);
        }
        KeyCode::Char('g') if pending_g && editor.mode == Modes::Normal => editor.cursor_y = 0,
        KeyCode::Char('g') if editor.mode == Modes::Normal => editor.pending_g = true,
        KeyCode::Char('$') if editor.mode == Modes::Normal => {
            editor.cursor_x = editor.current_row_len().saturating_sub(1)
        }
        KeyCode::Char('l') if pending_g && editor.mode == Modes::Normal => {
            editor.cursor_x = editor.current_row_len().saturating_sub(1)
        }
        KeyCode::Char('G') if editor.mode == Modes::Normal => {
            editor.cursor_y = editor.text_rows.len().saturating_sub(1)
        }
        KeyCode::Char('M') if editor.mode == Modes::Normal => {
            editor.cursor_y = editor.row_offset + editor.rows as usize / 2;
        }
        // o and O
        KeyCode::Char('h') if editor.mode == Modes::Normal => {
            editor.cursor_x = editor.cursor_x.saturating_sub(1)
        }
        KeyCode::Char('l')
            if editor.cursor_x + 1 < editor.current_row_len() && editor.mode == Modes::Normal =>
        {
            editor.cursor_x = editor.cursor_x.saturating_add(1);
        }
        KeyCode::Char('k') if editor.mode == Modes::Normal => {
            editor.cursor_y = editor.cursor_y.saturating_sub(1)
        }
        KeyCode::Char('j')
            if editor.cursor_y + 1 < editor.text_rows.len() && editor.mode == Modes::Normal =>
        {
            editor.cursor_y += 1;
        }
        // KeyCode::Char('b')
        // KeyCode::Char('B')
        // KeyCode::Char('w')
        // KeyCode::Char('W')
        // KeyCode::Char('e')
        // KeyCode::Char('E')

        // ----------------------
        // INSERT MODE
        // ----------------------
        KeyCode::Char(char) if editor.mode == Modes::Insert => editor.insert_char(char),
        // TODO: this currently only works if the file is empty
        // if this is done on a file with text cursor moves down and adds lines to end of file
        KeyCode::Enter if editor.mode == Modes::Insert => {
            editor.text_rows.push(EditorRow::new(String::new()));
            editor.cursor_y += 1;
        }
        _ => {}
    }

    let line_length = if editor.mode != Modes::Insert {
        editor.current_row_len().saturating_sub(1)
    } else {
        editor.current_row_len()
    };
    editor.cursor_x = editor.cursor_x.min(line_length);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::EditorRow, modes::Modes};
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
    fn change_modes() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();

        handle_keypress(&mut editor, KeyCode::Char('a'));
        assert_eq!(editor.mode, Modes::Insert);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('A'));
        assert_eq!(editor.mode, Modes::Insert);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('i'));
        assert_eq!(editor.mode, Modes::Insert);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('I'));
        assert_eq!(editor.mode, Modes::Insert);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('v'));
        assert_eq!(editor.mode, Modes::Visual);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('V'));
        assert_eq!(editor.mode, Modes::Visual);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('r'));
        assert_eq!(editor.mode, Modes::Replace);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('R'));
        assert_eq!(editor.mode, Modes::Replace);
    }

    #[test]
    fn test_movement_vim() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.mode, Modes::Normal);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('l'));
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('j'));
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Char('k'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('k'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn test_movement_arrows_normal() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.mode, Modes::Normal);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Right);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Down);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn test_movement_arrows_insert() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();

        editor.mode = Modes::Insert;
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Right);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Down);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn test_movement_arrows_visual() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();

        editor.mode = Modes::Visual;
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Right);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Down);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Left);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Up);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn capital_g_goes_to_end() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('G'));
        assert_eq!(editor.cursor_y, editor.text_rows.len() - 1);

        handle_keypress(&mut editor, KeyCode::Char('g'));
        assert_eq!(editor.cursor_y, editor.text_rows.len() - 1);
    }

    #[test]
    fn gg_goes_to_top() {
        let mut editor = build_app();
        editor.cursor_y = 10;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        handle_keypress(&mut editor, KeyCode::Char('g'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        handle_keypress(&mut editor, KeyCode::Char('g'));
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

        handle_keypress(&mut editor, KeyCode::Char('g'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        handle_keypress(&mut editor, KeyCode::Char('u'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);

        handle_keypress(&mut editor, KeyCode::Char('g'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 10);
    }

    #[test]
    fn goes_to_first_char() {
        let mut editor = build_app();
        editor.text_rows = vec![EditorRow::new("Hello".to_string())];
        editor.cursor_x = 3;

        editor.pending_g = true;
        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        editor.text_rows[0] = EditorRow::new("   Hello".to_string());
        editor.cursor_x = 3;

        editor.pending_g = true;
        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.cursor_x, 3);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn goes_to_start_of_line() {
        let mut editor = build_app();
        editor.text_rows = vec![EditorRow::new("Hello".to_string())];
        editor.cursor_x = 3;

        editor.pending_g = true;
        handle_keypress(&mut editor, KeyCode::Char('0'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        editor.text_rows[0] = EditorRow::new("   Hello".to_string());
        editor.cursor_x = 3;

        editor.pending_g = true;
        handle_keypress(&mut editor, KeyCode::Char('0'));
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn goes_to_last_char() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('$'));
        assert_eq!(editor.cursor_x, editor.text_rows[0].chars.len() - 1);
        assert_eq!(editor.cursor_y, 0);

        editor.pending_g = true;
        handle_keypress(&mut editor, KeyCode::Char('l'));
        assert_eq!(editor.cursor_x, editor.text_rows[0].chars.len() - 1);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn m_to_middle_of_screen() {
        let mut editor = build_app();
        editor.rows = 10;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('M'));
        assert_eq!(editor.cursor_y, 5);

        editor.cursor_x = 10;
        handle_keypress(&mut editor, KeyCode::Char('M'));
        assert_eq!(editor.cursor_y, 5);
    }

    #[test]
    fn text_insertion_with_vim_motions() {
        let mut editor = build_app();
        editor.text_rows = vec![EditorRow::new("Hello".to_string())];
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Char('i'));
        assert_eq!(editor.mode, Modes::Insert);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('a'));
        assert_eq!(editor.mode, Modes::Insert);
        assert_eq!(editor.cursor_x, 1);
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('A'));
        assert_eq!(editor.mode, Modes::Insert);
        assert_eq!(editor.cursor_x, editor.text_rows[0].chars.len());
        assert_eq!(editor.cursor_y, 0);

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('I'));
        assert_eq!(editor.mode, Modes::Insert);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn text_insertion() {
        let mut editor = build_app();
        handle_keypress(&mut editor, KeyCode::Char('i'));
        assert_eq!(editor.mode, Modes::Insert);
        handle_keypress(&mut editor, KeyCode::Char('a'));
        assert!(!editor.text_rows.is_empty());
        assert_eq!(editor.text_rows.len(), 1);
        assert_eq!(editor.text_rows[0].chars, "a");

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Char('h'));
        handle_keypress(&mut editor, KeyCode::Char('i'));
        assert_eq!(editor.mode, Modes::Insert);
        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.text_rows.len(), 1);
        assert_eq!(editor.text_rows[0].chars, "ha");

        handle_keypress(&mut editor, KeyCode::Esc);
        assert_eq!(editor.mode, Modes::Normal);
        handle_keypress(&mut editor, KeyCode::Right);
        handle_keypress(&mut editor, KeyCode::Char('a'));
        assert_eq!(editor.mode, Modes::Insert);
        handle_keypress(&mut editor, KeyCode::Char('h'));
        assert_eq!(editor.text_rows.len(), 1);
        assert_eq!(editor.text_rows[0].chars, "hah");
    }
}
