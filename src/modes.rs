use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Modes {
    Normal,
    Insert,
    Visual,
    Replace,
    Command,
}

impl Modes {
    pub fn as_str(self) -> &'static str {
        match self {
            Modes::Normal => "NORMAL",
            Modes::Insert => "INSERT",
            Modes::Replace => "REPLACE",
            Modes::Visual => "VISUAL",
            Modes::Command => "COMMAND",
        }
    }
}

impl Display for Modes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
