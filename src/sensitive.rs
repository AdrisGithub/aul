//! Module for Sensitive wrapping with the [Sens] struct
use std::fmt::{Display, Formatter};

const ENV_KEY: &str = "SAFE_LOGGING";
const ENV_VALUE: &str = "true";
const SAFE_PRINT: &str = "[REDACTED]";

/// ### Used to censor specific information in your logs. <br>
/// Can be activated and deactivated (at any point at runtime!) by setting the environment variable `SAFE_LOGGING` to `true`
///
/// When activated it will only print `[REDACTED]` <br>
///
/// Example:
/// ```
///     use aul::sensitive::Sens;
///
///     println!("{}",Sens("Hello"));  // Hello
///     //change env variable "SAFE_LOGGING" to true
///     println!("{}",Sens("Hello")) // "[REDACTED]"
/// ```
/// ### Warning:
/// It will look for the env key every call which has a runtime cost
///
/// [`SAFE_LOGGING`]: ENV_KEY
/// [`true`]: ENV_VALUE
/// [`REDACTED`]: SAFE_PRINT
pub struct Sens<T>(pub T);

impl<T> Display for Sens<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_safe() {
            SAFE_PRINT.fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}


impl<T> Sens<T> {
    fn is_safe(&self) -> bool {
        std::env::var(ENV_KEY).is_ok_and(|t| { t.eq(ENV_VALUE) })
    }
}


#[cfg(test)]
mod tests {
    use crate::sensitive::{ENV_KEY, ENV_VALUE, SAFE_PRINT, Sens};

    #[test]
    fn test_sens_mode() {
        std::env::set_var(ENV_KEY, "");
        assert_eq!(Sens(1).to_string(), "1");
    }

    #[test]
    fn test_sens_mode_redacted() {
        std::env::set_var(ENV_KEY, ENV_VALUE);
        assert_eq!(Sens(1).to_string(), SAFE_PRINT);
    }
}
