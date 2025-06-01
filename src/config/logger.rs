#[derive(Debug)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

#[derive(Debug)]
pub struct Logger {
    level: LogLevel,
    instance: Option<&'static Self>,
}

impl Logger {
    fn new(level: LogLevel) -> Self {
        Logger {
            level,
            instance: None,
        }
    }

    pub fn get() -> &'static Logger {
        static LOGGER: std::sync::OnceLock<Logger> = std::sync::OnceLock::new();
        LOGGER.get_or_init(|| Logger::new(LogLevel::INFO))
    }

    pub fn change_level(level: LogLevel) {
        let logger = Logger::get();
        // This is not how you modify a static.  It's also not thread safe.
        // logger.level = level;
        println!("Attempting to change log level to: {:?}", level);
    }
}
