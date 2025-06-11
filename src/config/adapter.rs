pub struct Adapter;

use crate::{config::config::Config, errors::ConfigParseError};
use ini::{Ini, Properties};

use super::{
    logger::{LogLevel, Logger},
    taskmasterd::{Taskmasterd, TaskmasterdSection},
};

impl Adapter {
    pub fn parse_config(
        config: &mut Config,
        file_path: Option<&String>,
    ) -> Result<(), ConfigParseError> {
        let file_path = match file_path {
            Some(path) => path.to_string(),
            None => Config::find_config()?,
        };
        let ini = Ini::load_from_file(file_path)?;

        for (sec, prop) in ini.iter() {
            println!("Section: {:?}", sec);
            for (key, value) in prop.iter() {
                // match key {
                //     _ => todo!(),
                // }
                println!("{}:{}", key, value);
            }
        }
        Ok(())
    }

    fn parse_taskmasterd(config: &mut Config, prop: Properties) -> Result<(), ConfigParseError> {
        for (key, value) in prop.iter() {
            let section = TaskmasterdSection::from_str(key)
                .ok_or_else(|| ConfigParseError::UnexpectedValue(key.to_string()))?;
            match section {
                TaskmasterdSection::Logfile => {
                    config.taskmasterd.logfile = value.to_string();
                }
                TaskmasterdSection::Loglevel => {
                    let value = LogLevel::from_str(value)
                        .ok_or_else(|| ConfigParseError::UnexpectedValue(value.to_string()))?;
                    match value {
                        LogLevel::DEBUG => config.taskmasterd.loglevel = LogLevel::DEBUG,
                        LogLevel::INFO => config.taskmasterd.loglevel = LogLevel::INFO,
                        LogLevel::WARN => config.taskmasterd.loglevel = LogLevel::WARN,
                        LogLevel::ERROR => config.taskmasterd.loglevel = LogLevel::ERROR,
                        LogLevel::CRITICAL => config.taskmasterd.loglevel = LogLevel::CRITICAL,
                    }
                }
            }
        }

        Ok(())
    }
}
