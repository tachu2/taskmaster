use chrono::Local;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            logger::DEBUG => Some(LogLevel::DEBUG),
            logger::INFO => Some(LogLevel::INFO),
            logger::WARN => Some(LogLevel::WARN),
            logger::ERROR => Some(LogLevel::ERROR),
            logger::CRITICAL => Some(LogLevel::CRITICAL),
            _ => None,
        }
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    fn log(&self, level: LogLevel, message: &str) {
        let now = Local::now();
        let timestamp = now.format("%Y/%m/%d %H:%M:%S").to_string();
        println!("[{}] [{}] {}", timestamp, level, message);
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
}

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        level: LogLevel::DEBUG
    });
}

///
///INFO: never panic as long as it runs on single thread or does not panic when locking
pub fn get_logger() -> std::sync::MutexGuard<'static, Logger> {
    LOGGER.lock().expect("Failed to lock LOGGER")
}

mod logger {
    pub const DEBUG: &str = "DEBUG";
    pub const INFO: &str = "INFO";
    pub const WARN: &str = "WARN";
    pub const ERROR: &str = "ERROR";
    pub const CRITICAL: &str = "CRITICAL";
}
