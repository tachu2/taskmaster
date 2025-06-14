pub struct Adapter;

use crate::config::program::program;
use crate::config::{
    parser::ProgramParser,
    taskmasterd::{taskmasterd, TaskmasterdSection},
};
use crate::{config::config::Config, errors::ConfigParseError};
use ini::{Ini, Properties};

use super::program::{Program, ProgramSection};
use super::{
    logger::{LogLevel, Logger},
    taskmasterd::Taskmasterd,
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
            match sec {
                Some(taskmasterd::TASKMASTERD) => {
                    Self::parse_taskmasterd(config, prop)?;
                }
                Some(s) if s.starts_with(program::PROGRAM) => {
                    Self::parse_program(config, s, prop)?;
                }
                Some(_) => {}
                None => {
                    // return Err(ConfigParseError::UnexpectedValue(
                    //     sec.unwrap_or_default().to_string(),
                    // ))
                }
            }
        }
        Ok(())
    }

    fn parse_taskmasterd(config: &mut Config, prop: &Properties) -> Result<(), ConfigParseError> {
        for (key, value) in prop.iter() {
            let section_value = TaskmasterdSection::from_str(key)
                .ok_or_else(|| ConfigParseError::UnexpectedValue(key.to_string()))?;
            match section_value {
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

    fn parse_program(
        config: &mut Config,
        sec: &str,
        prop: &Properties,
    ) -> Result<(), ConfigParseError> {
        let parts: Vec<&str> = sec.split(':').collect();
        if parts.len() != 2 {
            return Err(ConfigParseError::UnexpectedValue(sec.to_string()));
        }
        let program_name = parts[1].to_string();
        if config.programs.contains_key(&program_name) {
            return Err(ConfigParseError::DuplicatedValue(program_name));
        }
        let mut builder = Program::builder();
        builder = builder.programname(program_name);
        for (key, value) in prop.iter() {
            let section_value = ProgramSection::from_str(key)
                .ok_or_else(|| ConfigParseError::UnexpectedValue(key.to_string()))?;
            match section_value {
                ProgramSection::Command => {
                    builder.command(ProgramParser::parse_command(value)?);
                }
                _ => {
                    // return Err(ConfigParseError::UnexpectedValue(key.to_string()));
                }
            }
        }
        let program = builder
            .build()
            .map_err(|_| ConfigParseError::MissingCommand(program_name.clone()))?;
        config.programs.insert(program_name, program);
        Ok(())
    }
}
