use std::collections::LinkedList;

use crate::errors::ConfigParseError;

pub struct CommandParser;

impl CommandParser {
    pub fn parse_command(command: &String) -> Result<LinkedList<String>, ConfigParseError> {
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
}
