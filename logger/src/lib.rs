use std::fmt;

pub enum LogStep {
    EquationStructure,
}

impl fmt::Display for LogStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogStep::EquationStructure => "EquationStructure",
        };
        write!(f, "[{s}]")
    }
}

fn _log(step: LogStep, message: &str) {
    println!("{step} {message}");
}

pub fn log(step: LogStep, _message: &str) {
    match step {
        #[cfg(feature = "debug-equation-structure")]
        LogStep::EquationStructure => _log(step, _message),
        #[allow(unreachable_patterns)]
        _ => {},
    }
}