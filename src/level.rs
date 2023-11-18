//! Module for different logging [Level]s
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::ParseLogLevelError;

static NAMES: [&str; 5] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"];

/// ### Used for specifying the Logging Level
///
/// Example:
/// ```
/// use aul::log;
/// use aul::level::Level;
///
/// log!(Level::INFO,"Hello")
/// // [INFO]: Hello
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Level {
    /// ### Designated for low priority, verbose information
    ///
    /// For Example: Tracing method calls
    TRACE,
    /// ### Designated for lower priority information
    ///
    /// For Example: Value of a specific variable
    DEBUG,
    /// ### Designated for useful information
    ///
    /// For Example: User connected with your server
    INFO,
    /// ### Designated for "hazardous" information
    ///
    /// For Example: Server didn't find setup file and use defaults
    WARN,
    /// ### Designated for very serious errors
    ///
    /// For Example: Subprocess exited with abnormal exit code
    ERROR,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.get_name())
    }
}

impl FromStr for Level {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(s))
            .map(Level::try_from)
            .unwrap()
    }
}

impl TryFrom<usize> for Level {
    type Error = ParseLogLevelError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Some(Level::TRACE),
            1 => Some(Level::DEBUG),
            2 => Some(Level::INFO),
            3 => Some(Level::WARN),
            4 => Some(Level::ERROR),
            _ => None,
        }.ok_or(ParseLogLevelError::new())
    }
}


impl Level {
    fn get_name(&self) -> &'static str {
        NAMES[*self as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::level::Level;

    #[test]
    fn test_get_name() {
        assert_eq!(Level::INFO.get_name(), "INFO")
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Level::DEBUG.to_string(), "DEBUG")
    }

    #[test]
    fn test_to_string_doesnt_equal_something_else() {
        assert_ne!(Level::WARN.to_string(), "DEBUG")
    }

    #[test]
    fn test_get_name_doesnt_equal_something_else() {
        assert_ne!(Level::DEBUG.to_string(), "INFO")
    }
}