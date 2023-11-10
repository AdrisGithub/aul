extern crate core;

use std::fmt::Arguments;

use crate::level::Level;

pub mod level;
pub mod sensitive;
pub mod macros;


fn log(args: Arguments, level: Level) {
    println!("[{}]: {}", level, args)
}

#[cfg(test)]
mod tests {
    use crate::level::Level;
    use crate::log;

    #[test]
    fn test_log_macro() {
        log!(Level::INFO,"{}","Hello")
    }
}



