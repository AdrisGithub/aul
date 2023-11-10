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