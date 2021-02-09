use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Verbose,
}

impl Into<u64> for LogLevel {
    fn into(self) -> u64 {
        match self {
            LogLevel::Info => 0,
            LogLevel::Debug => 1,
            LogLevel::Verbose => 2,
        }
    }
}

impl From<u64> for LogLevel {
    fn from(val: u64) -> Self {
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