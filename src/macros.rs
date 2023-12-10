//! Module for all the Macros

/// macro for logging with a specific [`Level`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::log;
///
///  log!(Level::INFO,"This is an information");
///  // [INFO]: This is an information
///  let num = 1;
///  log!(Level::DEBUG,"{} + {}","This is a debugging message",num);
///  // [DEBUG]: This is a debugging message + 1
/// ```
///
/// [`Level`]: crate::Level
#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {
        $crate::log(format_args!($($arg)+),$lvl)
    };
}
/// macro for logging sensitive data with a specific [`Level`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::{log, log_sensitive};
///
///  log_sensitive!(Level::INFO,"This is a sensitive information");
///  // [INFO]: This is an information
///  // change env variable "SAFE_LOGGING" to true
///  log_sensitive!(Level::INFO,"This is a sensitive information");
///  // [INFO]: [REDACTED]
///  log_sensitive!(Level::INFO,"{} - {}","Info 1","Info 2");
///  // [INFO]: [REDACTED]
/// ```
///
/// [`Level`]: crate::Level
#[macro_export]
macro_rules! log_sensitive {
    ($lvl:expr, $($arg:tt)+) => {
        $crate::log_sensitive(format_args!($($arg)+),$lvl)
    };
}
/// macro for logging with Level [`INFO`]
///
/// # Example
///
/// ```
/// use aul::info;
///
///  info!("This is an information");
///  // [INFO]: This is an information
/// ```
///
/// [`INFO`]: crate::Level::INFO
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        use aul::log;
        use aul::level::Level;
        log!(Level::INFO,$($arg)+)
    };
}
/// macro for logging with Level [`ERROR`]
///
/// # Example
///
/// ```
///  use aul::error;
///
///  error!("This is an error");
///  // [ERROR]: This is an error
/// ```
///
/// [`ERROR`]: crate::Level::ERROR
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        use aul::log;
        use aul::level::Level;
        log!(Level::ERROR,$($arg)+)
    };
}
/// macro for logging with Level [`TRACE`]
///
/// # Example
///
/// ```
///  use aul::trace;
///
///  trace!("calling method add");
///  // [TRACE]: calling method add
/// ```
///
/// [`TRACE`]: crate::Level::TRACE
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        use aul::log;
        use aul::level::Level;
        log!(Level::TRACE,$($arg)+)
    };
}
/// macro for logging with Level [`DEBUG`]
///
/// # Example
///
/// ```
///  use aul::debug;///
///  debug!("debugging info");
///  // [DEBUG]: debugging info
/// ```
///
/// [`DEBUG`]: crate::Level::DEBUG
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        use aul::log;
        use aul::level::Level;
        log!(Level::DEBUG,$($arg)+)
    };
}
/// macro for logging with Level [`WARN`]
///
/// # Example
///
/// ```
///  use aul::warn;///
///  warn!("WARNING");
///  log!(Level::INFO,"hello")
///  // [WARN]: WARNING
/// ```
///
/// [`WARN`]: crate::Level::WARN
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        use aul::log;
        use aul::level::Level;
        log!(Level::WARN,$($arg)+)
    };
}

