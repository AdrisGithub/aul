use std::fmt::{Display, Formatter};

static SAFE_MODE: bool = std::env::var("SAFE_LOGGING").is_ok_and(|t| { t.eq("true") });

struct Sens<T>(T);

impl<T> Display for Sens<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if SAFE_MODE {
            "[REDACTED]".fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}