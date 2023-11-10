use std::fmt::{Display, Formatter};

const ENV_KEY: &'static str = "SAFE_LOGGING";
const ENV_VALUE: &'static str = "true";
const SAFE_PRINT: &'static str = "[REDACTED]";

pub struct Sens<T>(T);

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

