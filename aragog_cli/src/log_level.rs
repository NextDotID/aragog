use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Verbose,
}

impl From<LogLevel> for u64 {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Info => 0,
            LogLevel::Debug => 1,
            LogLevel::Verbose => 2,
        }
    }
}

impl From<u8> for LogLevel {
    fn from(val: u8) -> Self {
        match val {
            v if v == 0 => Self::Info,
            v if v == 1 => Self::Debug,
            _ => Self::Verbose,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            LogLevel::Info => "",
            LogLevel::Debug => "[Debug]",
            LogLevel::Verbose => "[Trace]",
        };
        write!(f, "{}", format!("[Aragog]{}", str))
    }
}
