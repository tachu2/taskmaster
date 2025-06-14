use std::collections::LinkedList;

use crate::errors::ConfigParseError;

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
}
