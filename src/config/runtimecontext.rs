use super::{config::Config, logger::Logger};

#[derive(Debug)]
pub struct RuntimeContext {
    pub(in crate::config) config: Config,
    pub logger: Logger,
}

impl Default for RuntimeContext {
    fn default() -> Self {
        RuntimeContext {
            config: Config::default(),
            logger: Logger::default(),
        }
    }
}
