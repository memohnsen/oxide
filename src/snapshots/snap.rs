#[cfg(test)]
mod tests {
    use crate::{
        editor::Editor,
        modes::Modes,
        ui::{draw_rows, show_status_bar},
    };
    use insta::assert_snapshot;
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

    fn show_all_rows(screen: &Editor) -> String {
        let mut output = String::new();
        output.push_str(&draw_rows(screen));
        output.push_str(&show_status_bar(screen));

        output
    }

    #[test]
    fn app_renders_home_screen() {
        let editor = build_app();
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_narrow() {
        let mut editor = build_app();
        editor.cols = 40;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_super_narrow() {
        let mut editor = build_app();
        editor.cols = 2;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
        assert_eq!(show_status_bar(&editor).len(), editor.cols as usize);
    }

    #[test]
    fn app_renders_down_screen() {
        let mut editor = build_app();
        editor.row_offset = 20;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_cols_row_loc() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.cursor_x = 8;
        editor.cursor_y = 10;
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn insert_mode() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.mode = Modes::Insert;
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn visual_mode() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.mode = Modes::Visual;
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn replace_mode() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.mode = Modes::Replace;
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn command_mode() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        editor.mode = Modes::Command;
        let output = show_all_rows(&editor);

        assert_snapshot!(output);
    }
}
