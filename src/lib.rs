//! # Another useless Logger
//!
//! This crate can be used for logging to [`stdout`] and censoring private information using [Sens]
//!
//! ## Usage
//! This library provides multiple [macros] for example [`log!`]
//! ```
//!     use aul::level::Level;
//!     use aul::log;
//!
//!     let a = 1;
//!     let b = 2;
//!
//!     log!(Level::TRACE,"Calling method add with params {} and {}",a,b);
//!
//!     let c = add(a,b);
//!
//!     log!(Level::TRACE,"Called method add");
//!     log!(Level::INFO,"Result: {}",c);
//!
//!     fn add(a : i32, b : i32)-> i32 { a + b }
//! ```
//! [`stdout`]: https://en.wikipedia.org/wiki/Standard_streams

extern crate core;

use std::fmt::Arguments;

use colored::Colorize;

use crate::level::Level;
use crate::sensitive::Sens;

pub mod level;
pub mod sensitive;
pub mod macros;
pub mod errors;

/// ### Warning used by the macro to log. Not intended for personal usage
pub fn log(args: Arguments, level: Level) {
    println!("[{}]: {}", paint_level(level), args)
}

/// ### Warning used by the macro to log. Not intended for personal usage
pub fn log_sensitive(args: Arguments, level: Level) {
    println!("[{}]: {}", paint_level(level), Sens(args))
}

#[cfg(not(feature = "color"))]
fn paint_level(level: Level) -> String {
    level.to_string()
}

#[cfg(feature = "color")]
pub fn paint_level(level: Level) -> String {
    match level {
        Level::TRACE => level.to_string().blue(),
        Level::DEBUG => level.to_string().blue(),
        Level::INFO => level.to_string().blue(),
        Level::WARN => level.to_string().blue(),
        Level::ERROR => level.to_string().blue(),
    }.to_string()
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::{log, log_info, log_sensitive};
    use crate::level::Level;
    use crate::Sens;

    #[test]
    fn test_log_macro() {
        log!(Level::WARN,"Hello");
        log_sensitive!(Level::INFO,"{}","Hello");
        std::env::set_var("SAFE_LOGGING", "true");
        log_sensitive!(Level::INFO,"{}","Hello");
        log!(Level::WARN,"{} {}",Sens("Hello"),"Hello");
        log_info!("HEllo");
        print!("{}", "Hello".blue())
    }
}



