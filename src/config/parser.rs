use std::collections::LinkedList;

use crate::errors::ConfigParseError;

use super::program::program::AutoRestart;

pub struct ProgramParser;

impl ProgramParser {
    pub fn parse_command(command: &str) -> Result<LinkedList<String>, ConfigParseError> {
        let list = command
            .split(|c| c == ' ' || c == '\t')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<LinkedList<_>>();
        if list.is_empty() {
            Err(ConfigParseError::UnexpectedValue(command.to_string()))?;
        }
        Ok(list)
    }

    pub fn parse_numprocs(numprocs: &str) -> Result<u8, ConfigParseError> {
        numprocs
            .parse::<u8>()
            .map_err(|_| ConfigParseError::UnexpectedValue(numprocs.to_string()))
    }

    pub fn parse_autostart(auto_start: &str) -> Result<bool, ConfigParseError> {
        match auto_start.to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(ConfigParseError::UnexpectedValue(auto_start.to_string())),
        }
    }

    pub fn parse_autorestart(autorestart: &str) -> Result<AutoRestart, ConfigParseError> {
        match autorestart.to_lowercase().as_str() {
            "true" => Ok(AutoRestart::True),
            "false" => Ok(AutoRestart::False),
            "unexpected" => Ok(AutoRestart::Unexpected),
            _ => Err(ConfigParseError::UnexpectedValue(autorestart.to_string())),
        }
    }
}
