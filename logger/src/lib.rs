use std::fmt;

pub enum LogStep {
    Structure,
    Setup,
    Compute,
}

impl fmt::Display for LogStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogStep::Structure => "Structure",
            LogStep::Setup => "Setup",
            LogStep::Compute => "Compute",
        };
        write!(f, "[{s}]")
    }
}

fn _log(step: LogStep, message: &str) {
    println!("{step} {message}");
}

pub fn log(step: LogStep, _message: &str) {
    match step {
        #[cfg(feature = "debug-structure")]
        LogStep::Structure => _log(step, _message),

        #[cfg(feature = "debug-setup")]
        LogStep::Setup => _log(step, _message),

        #[cfg(feature = "debug-compute")]
        LogStep::Compute => _log(step, _message),

        #[allow(unreachable_patterns)]
        _ => {},
    }
}