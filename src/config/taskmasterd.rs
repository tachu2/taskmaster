use super::logger::LogLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Taskmasterd {
    pub(in crate::config) logfile: String,
    pub(in crate::config) loglevel: LogLevel,
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

mod taskmasterd {
    pub const LOGFILE: &str = "logfile";
    pub const LOGLEVEL: &str = "loglevel";
}
