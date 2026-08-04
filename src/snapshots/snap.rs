#[cfg(test)]
mod tests {
    use crate::{editor::Editor, modes::Modes, ui::draw_rows};
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
        }
    }

    #[test]
    fn app_renders_home_screen() {
        let editor = build_app();
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders() {
        let mut editor = build_app();
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_narrow() {
        let mut editor = build_app();
        editor.cols = 40;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_down_screen() {
        let mut editor = build_app();
        editor.row_offset = 20;
        editor.open_file(path::Path::new("test.txt")).unwrap();
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }
}
