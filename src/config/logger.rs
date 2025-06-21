use chrono::Local;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    #[default]
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::TRACE => write!(f, "TRACE"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::CRITICAL => write!(f, "CRITICAL"),
        }
    }
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            logger::TRACE => Some(LogLevel::TRACE),
            logger::DEBUG => Some(LogLevel::DEBUG),
            logger::INFO => Some(LogLevel::INFO),
            logger::WARN => Some(LogLevel::WARN),
            logger::ERROR => Some(LogLevel::ERROR),
            logger::CRITICAL => Some(LogLevel::CRITICAL),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Logger {
    level: LogLevel,
    enabled: bool,
}

impl Logger {
    pub fn default() -> Self {
        Logger {
            level: LogLevel::INFO,
            enabled: false,
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if !self.enabled {
            return;
        }
        let now = Local::now();
        let timestamp = now.format("%Y/%m/%d %H:%M:%S").to_string();
        println!("[{}] [{}] {}", timestamp, level, message);
    }

    pub fn trace(&self, message: &str) {
        if self.level > LogLevel::TRACE {
            return;
        }
        self.log(LogLevel::TRACE, message);
    }

    pub fn debug(&self, message: &str) {
        if self.level > LogLevel::DEBUG {
            return;
        }
        self.log(LogLevel::DEBUG, message);
    }

    pub fn info(&self, message: &str) {
        if self.level > LogLevel::INFO {
            return;
        }
        self.log(LogLevel::INFO, message);
    }

    pub fn warn(&self, message: &str) {
        if self.level > LogLevel::WARN {
            return;
        }
        self.log(LogLevel::WARN, message);
    }

    pub fn error(&self, message: &str) {
        if self.level > LogLevel::ERROR {
            return;
        }
        self.log(LogLevel::ERROR, message);
    }

    pub fn critical(&self, message: &str) {
        if self.level > LogLevel::CRITICAL {
            return;
        }
        self.log(LogLevel::CRITICAL, message);
    }

    pub fn change_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}

mod logger {
    pub const TRACE: &str = "TRACE";
    pub const DEBUG: &str = "DEBUG";
    pub const INFO: &str = "INFO";
    pub const WARN: &str = "WARN";
    pub const ERROR: &str = "ERROR";
    pub const CRITICAL: &str = "CRITICAL";
}
