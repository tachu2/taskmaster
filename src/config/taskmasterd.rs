use std::fmt::Display;

use super::logger::LogLevel;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Taskmasterd {
    pub(in crate::config) logfile: String,
    pub(in crate::config) loglevel: LogLevel,
}

impl Taskmasterd {
    pub fn default() -> Self {
        Taskmasterd {
            logfile: String::new(),
            loglevel: LogLevel::INFO,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskmasterdSection {
    Logfile,
    Loglevel,
}

impl TaskmasterdSection {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskmasterdSection::Logfile => taskmasterd::LOGFILE,
            TaskmasterdSection::Loglevel => taskmasterd::LOGLEVEL,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            taskmasterd::LOGFILE => Some(TaskmasterdSection::Logfile),
            taskmasterd::LOGLEVEL => Some(TaskmasterdSection::Loglevel),
            _ => None,
        }
    }
}

pub mod taskmasterd {
    pub const TASKMASTERD: &str = "taskmasterd";
    pub const LOGFILE: &str = "logfile";
    pub const LOGLEVEL: &str = "loglevel";
}
