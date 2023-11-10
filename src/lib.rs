extern crate core;

use std::fmt::Arguments;

use crate::level::Level;
use crate::sensitive::Sens;

pub mod level;
pub mod sensitive;
pub mod macros;
pub mod errors;

#[allow(dead_code)]
fn log(args: Arguments, level: Level) {
    println!("[{}]: {}", level, args)
}

#[allow(dead_code)]
fn log_sensitive(args: Arguments, level: Level) {
    println!("[{}]: {}", level, Sens(args))
}

#[cfg(test)]
mod tests {
    use crate::{log, log_sensitive};
    use crate::level::Level;
    use crate::sensitive::Sens;

    #[test]
    fn test_log_macro() {
        log!(Level::INFO,"{}","Hello");
        log_sensitive!(Level::INFO,"{}","Hello");
        std::env::set_var("SAFE_LOGGING", "true");
        log_sensitive!(Level::INFO,"{}","Hello");
        log!(Level::WARN,"{} {}",Sens("Hello"),"Hello")
    }
}



