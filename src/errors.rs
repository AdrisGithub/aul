//! Error Module for [ParseLogLevelError]
use std::error::Error;
use std::fmt::{Display, Formatter};

static ERROR_MESSAGE: &str = "failed to convert a string that doesn't match a log level";

/// Error struct used and thrown when calling [`from_str`] or [`try_from`] on a String that doesn't contain a valid [`Level`]
///
/// [`Level`]: crate::Level
/// [`from_str`]: std::str::FromStr::from_str
/// [`try_from`]: TryFrom::try_from
#[derive(Debug)]
pub struct ParseLogLevelError(());

impl ParseLogLevelError {
    /// Constructs a new Instance of ParseLogLevelError
    ///
    pub const fn new() -> Self {
        ParseLogLevelError(())
    }
}

impl Error for ParseLogLevelError {}

impl Display for ParseLogLevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(ERROR_MESSAGE)
    }
}