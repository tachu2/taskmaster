pub struct Adapter;

use crate::config::program::program;
use crate::config::{
    parser::ProgramParser,
    taskmasterd::{taskmasterd, TaskmasterdSection},
};
use crate::errors::ProgramBuilderError;
use crate::{config::config::Config, errors::ConfigParseError};
use ini::{Ini, Properties};

use super::logger::{self, get_logger};
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
        let logger = get_logger();
        logger.info("Parsing configuration file.");
        let file_path = match file_path {
            Some(path) => path.to_string(),
            None => Config::find_config()?,
        };
        let ini = Ini::load_from_file(file_path)?;

        for (sec, prop) in ini.iter() {
            match sec {
                Some(taskmasterd::TASKMASTERD) => {
                    logger.debug("Parsing taskmasterd section.");
                    Self::parse_taskmasterd(config, prop)?;
                }
                Some(s) if s.starts_with(program::PROGRAM) => {
                    logger.debug(&format!("Parsing program section: {}", s));
                    Self::parse_program(config, s, prop)?;
                }
                Some(s) => {
                    logger.error(&format!("Parsing unknown section: {}", s));
                    return Err(ConfigParseError::UnexpectedValue(s.to_string()));
                }
                // the first section is always None so don't do anything
                None => {
                    logger.debug("Skipping empty section.");
                }
            }
        }
        Ok(())
    }

    fn parse_taskmasterd(config: &mut Config, prop: &Properties) -> Result<(), ConfigParseError> {
        let logger = get_logger();
        for (key, value) in prop.iter() {
            let section_value = TaskmasterdSection::from_str(key)
                .ok_or_else(|| ConfigParseError::UnexpectedValue(key.to_string()))?;
            match section_value {
                TaskmasterdSection::Logfile => {
                    logger.debug(&format!("logfile: {}", value));
                    config.taskmasterd.logfile = value.to_string();
                }
                TaskmasterdSection::Loglevel => {
                    let value = LogLevel::from_str(value)
                        .ok_or_else(|| ConfigParseError::UnexpectedValue(value.to_string()))?;
                    logger.debug(&format!("loglevel: {}", value));
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
        builder.programname(program_name.clone());
        for (key, value) in prop.iter() {
            let section_value = ProgramSection::from_str(key)
                .ok_or_else(|| ConfigParseError::UnexpectedValue(key.to_string()))?;
            match section_value {
                ProgramSection::Command => {
                    builder.command(ProgramParser::parse_command(value)?);
                }
                ProgramSection::NumProcs => {
                    builder.numprocs(ProgramParser::parse_numprocs(value)?);
                }
                ProgramSection::AutoStart => {
                    builder.autostart(ProgramParser::parse_autostart(value)?);
                }
                ProgramSection::AutoRestart => {
                    builder.autorestart(ProgramParser::parse_autorestart(value)?);
                }
                ProgramSection::ExitCodes => {
                    builder.exitcodes(ProgramParser::parse_exitcodes(value)?);
                }
                ProgramSection::StartSecs => {
                    builder.startsecs(ProgramParser::parse_startsecs(value)?);
                }
                ProgramSection::StartRetries => {
                    builder.startretries(ProgramParser::parse_startretries(value)?);
                }
                ProgramSection::StopSignal => {
                    builder.stopsignal(ProgramParser::parse_stopsignal(value)?);
                }
                ProgramSection::StopWaitSecs => {
                    builder.stopwaitsecs(ProgramParser::parse_stopwaitsecs(value)?);
                }
                ProgramSection::StdoutLogfile => {
                    builder.stdout_logfile(ProgramParser::parse_stdout_logfile(value)?);
                }
                ProgramSection::StderrLogfile => {
                    builder.stderr_logfile(ProgramParser::parse_stderr_logfile(value)?);
                }
                ProgramSection::Environment => {
                    let env = ProgramParser::parse_environment(value)?;
                    builder.environment(env);
                }
                ProgramSection::Directory => {
                    builder.directory(ProgramParser::parse_directory(value)?);
                }
                ProgramSection::Umask => {
                    builder.umask(ProgramParser::parse_umask(value)?);
                }
            }
        }
        let program = builder.build().map_err(|e| match e {
            ProgramBuilderError::MissingCommand => {
                ConfigParseError::MissingCommand(program_name.clone())
            }
            ProgramBuilderError::MissingProgramName => {
                ConfigParseError::Critical(ProgramBuilderError::MissingProgramName.to_string())
            }
        })?;
        config.programs.insert(program_name, program);
        Ok(())
    }
}
