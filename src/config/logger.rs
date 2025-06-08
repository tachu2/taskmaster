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

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.level {
            println!("[{:?}] {}", level, message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.level > LogLevel::DEBUG {
            return;
        }
        println!("[{:?}] {}", LogLevel::DEBUG, message);
    }

    pub fn info(&self, message: &str) {
        if self.level > LogLevel::INFO {
            return;
        }
        println!("[{:?}] {}", LogLevel::INFO, message);
    }

    pub fn warn(&self, message: &str) {
        if self.level > LogLevel::WARN {
            return;
        }
        println!("[{:?}] {}", LogLevel::WARN, message);
    }

    pub fn error(&self, message: &str) {
        if self.level > LogLevel::ERROR {
            return;
        }
        println!("[{:?}] {}", LogLevel::ERROR, message);
    }

    pub fn critical(&self, message: &str) {
        if self.level > LogLevel::CRITICAL {
            return;
        }
        println!("[{:?}] {}", LogLevel::CRITICAL, message);
    }

    pub fn change_level(&mut self, level: LogLevel) {
        self.level = level;
    }
}

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        level: LogLevel::WARN
    });
}

///
///INFO: never panic as long as it runs on single thread or does not panic when locking
pub fn get_logger() -> std::sync::MutexGuard<'static, Logger> {
    LOGGER.lock().expect("Failed to lock LOGGER")
}
