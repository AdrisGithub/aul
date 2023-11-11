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
/// use aul::level::Level;
/// use aul::{log,log_info};
///
///  log_info!("This is an information");
///  // [INFO]: This is an information
/// ```
///
/// [`INFO`]: crate::Level::INFO
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        log!(Level::INFO,$($arg)+)
    };
}
/// macro for logging with Level [`ERROR`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::{log,log_error};
///
///  log_error!("This is an error");
///  // [ERROR]: This is an error
/// ```
///
/// [`ERROR`]: crate::Level::ERROR
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        log!(Level::ERROR,$($arg)+)
    };
}
/// macro for logging with Level [`TRACE`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::{log,log_trace};
///
///  log_trace!("calling method add");
///  // [TRACE]: calling method add
/// ```
///
/// [`TRACE`]: crate::Level::TRACE
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        log!(Level::TRACE,$($arg)+)
    };
}
/// macro for logging with Level [`DEBUG`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::{log,log_debug};
///
///  log_debug!("debugging info");
///  // [DEBUG]: debugging info
/// ```
///
/// [`DEBUG`]: crate::Level::DEBUG
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        log!(Level::DEBUG,$($arg)+)
    };
}
/// macro for logging with Level [`WARN`]
///
/// # Example
///
/// ```
///  use aul::level::Level;
///  use aul::{log,log_warn};
///  log_warn!("WARNING");
///  // [WARN]: WARNING
/// ```
///
/// [`WARN`]: crate::Level::WARN
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        log!(Level::WARN,$($arg)+)
    };
}

