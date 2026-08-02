#[cfg(test)]
mod tests {
    use std::path;

    use crate::{editor::Editor, ui::draw_rows};
    use insta::assert_snapshot;

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

    fn open_file(editor: &mut Editor) {
        let filename = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.txt");
        editor.open_file(path::Path::new(&filename)).unwrap();
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
        open_file(&mut editor);
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_narrow() {
        let mut editor = build_app();
        editor.cols = 40;
        open_file(&mut editor);
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }

    #[test]
    fn app_renders_down_screen() {
        let mut editor = build_app();
        editor.row_offset = 20;
        open_file(&mut editor);
        let output = draw_rows(&editor);

        assert_snapshot!(output);
    }
}
