use crossterm::{cursor, execute, terminal};
use std::io::stdout;

pub mod editor;
pub mod input;
pub mod snapshots;
pub mod ui;

pub struct CleanUp;
// If a fn errors and exits the program before disable is called then the term will stay broken
// Drop now will run this func when it goes out of scope or when a panic in scope
impl Drop for CleanUp {
    fn drop(&mut self) {
        let _ = execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Show
        );
        let _ = terminal::disable_raw_mode();
    }
}
