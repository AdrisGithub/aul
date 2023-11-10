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

#[cfg(test)]
mod tests{
    use crate::level::Level;

    #[test]
    fn test_get_name(){
        assert_eq!(Level::INFO.get_name(),"INFO")
    }
    #[test]
    fn test_to_string(){
        assert_eq!(Level::DEBUG.to_string(),"DEBUG")
    }
    #[test]
    fn test_to_string_doesnt_equal_smth_else(){
        assert_ne!(Level::WARN.to_string(),"DEBUG")
    }
    #[test]
    fn test_get_name_doesnt_equal_smth_else(){
        assert_ne!(Level::DEBUG.to_string(),"INFO")
    }

}