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
            match level {
                LogLevel::ERROR | LogLevel::CRITICAL => {
                    eprintln!("[{:?}] {}", level, message);
                }
                _ => {
                    println!("[{:?}] {}", level, message);
                }
            }
        }
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
