#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taskmasterd {
    Logfile,
    Loglevel,
}

impl Taskmasterd {
    pub fn as_str(&self) -> &'static str {
        match self {
            Taskmasterd::Logfile => section::LOGFILE,
            Taskmasterd::Loglevel => section::LOGLEVEL,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            section::LOGFILE => Some(Taskmasterd::Logfile),
            section::LOGLEVEL => Some(Taskmasterd::Loglevel),
            _ => None,
        }
    }
}

mod section {
    pub const LOGFILE: &str = "logfile";
    pub const LOGLEVEL: &str = "loglevel";
}
