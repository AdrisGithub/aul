use std::fmt::{Display, Formatter};

static NAMES: [&str; 5] = ["TRACE", "INFO", "WARN", "DEBUG", "ERROR"];

#[derive(Debug, Copy, Clone)]
pub enum Level {
    TRACE,
    INFO,
    WARN,
    DEBUG,
    ERROR,
}
impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.get_name())
    }
}

impl Level {
    fn get_name(&self) -> &'static str {
        NAMES[*self as usize]
    }
}