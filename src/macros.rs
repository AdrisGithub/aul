use crate::level::Level;
#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {
        $crate::log(format_args!($($arg)+),$lvl)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        log!(Level::INFO,$($arg)+)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        log!(Level::ERROR,$($arg)+)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        log!(Level::TRACE,$($arg)+)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        log!(Level::DEBUG,$($arg)+)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        log!(Level::WARN,$($arg)+)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        log!(Level::ERROR,$($arg)+)
    };
}