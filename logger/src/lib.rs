use std::fmt;

pub enum LogStep {
    Compilation,

}

impl fmt::Display for LogStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogStep::Compilation => "Compilation",
        };
        write!(f, "[{s}]")
    }
}

fn _log(step: LogStep, message: &str) {
    println!("{step} {message}");
}

pub fn log(step: LogStep, _message: &str) {
    match step {
        #[cfg(feature = "debug-equation-compilation")]
        LogStep::Compilation => _log(step, _message),
        _ => {},
    }
}