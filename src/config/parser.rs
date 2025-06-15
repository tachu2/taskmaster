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

    pub fn parse_exitcodes(exitcodes: &str) -> Result<LinkedList<i32>, ConfigParseError> {
        let list = exitcodes
            .split(|c| c == ',' || c == ' ' || c == '\t')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|_| ConfigParseError::UnexpectedValue(s.to_string()))
            })
            .collect::<Result<LinkedList<_>, _>>()?;
        if list.is_empty() {
            Err(ConfigParseError::UnexpectedValue(exitcodes.to_string()))?;
        }
        Ok(list)
    }

    pub fn parse_startsecs(startsecs: &str) -> Result<u8, ConfigParseError> {
        startsecs
            .parse::<u8>()
            .map_err(|_| ConfigParseError::UnexpectedValue(startsecs.to_string()))
    }

    pub fn parse_startretries(startretries: &str) -> Result<u8, ConfigParseError> {
        startretries
            .parse::<u8>()
            .map_err(|_| ConfigParseError::UnexpectedValue(startretries.to_string()))
    }

    pub fn parse_stopsignal(stopsignal: &str) -> Result<i32, ConfigParseError> {
        stopsignal
            .parse::<i32>()
            .map_err(|_| ConfigParseError::UnexpectedValue(stopsignal.to_string()))
    }

    pub fn parse_stopwaitsecs(stopwaitsecs: &str) -> Result<u32, ConfigParseError> {
        stopwaitsecs
            .parse::<u32>()
            .map_err(|_| ConfigParseError::UnexpectedValue(stopwaitsecs.to_string()))
    }

    pub fn parse_stdout_logfile(stdout_logfile: &str) -> Result<String, ConfigParseError> {
        Ok(stdout_logfile.to_string())
    }

    pub fn parse_stderr_logfile(stderr_logfile: &str) -> Result<String, ConfigParseError> {
        Ok(stderr_logfile.to_string())
    }

    pub fn parse_environment(environment: &str) -> Result<LinkedList<String>, ConfigParseError> {
        let list = environment
            .split(|c| c == ',' || c == ' ' || c == '\t')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<LinkedList<_>>();
        if list.is_empty() {
            Err(ConfigParseError::UnexpectedValue(environment.to_string()))?;
        }
        Ok(list)
    }

    pub fn parse_directory(directory: &str) -> Result<String, ConfigParseError> {
        Ok(directory.to_string())
    }

    pub fn parse_umask(umask: &str) -> Result<u16, ConfigParseError> {
        umask
            .parse::<u16>()
            .map_err(|_| ConfigParseError::UnexpectedValue(umask.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_command_tests {
        use super::*;

        #[test]
        fn test_parse_command_valid_with_arguments() {
            let command = "ls -l /tmp";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
            assert_eq!(result.iter().nth(1).unwrap(), "-l");
            assert_eq!(result.iter().nth(2).unwrap(), "/tmp");
        }

        #[test]
        fn test_parse_command_empty() {
            let command = "";
            let result = ProgramParser::parse_command(command);
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_command_with_leading_and_trailing_spaces() {
            let command = "   ls -l /tmp   ";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
            assert_eq!(result.iter().nth(1).unwrap(), "-l");
            assert_eq!(result.iter().nth(2).unwrap(), "/tmp");
        }

        #[test]
        fn test_parse_command_with_multiple_spaces() {
            let command = "ls  -l   /tmp";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
            assert_eq!(result.iter().nth(1).unwrap(), "-l");
            assert_eq!(result.iter().nth(2).unwrap(), "/tmp");
        }

        #[test]
        fn test_parse_command_no_arguments() {
            let command = "ls";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
        }

        #[test]
        fn test_parse_command_single_argument() {
            let command = "ls /tmp";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
            assert_eq!(result.iter().nth(1).unwrap(), "/tmp");
        }

        #[test]
        fn test_parse_command_argument_with_spaces() {
            let command = "ls /tmp/with spaces";
            let result = ProgramParser::parse_command(command).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().nth(0).unwrap(), "ls");
            assert_eq!(result.iter().nth(1).unwrap(), "/tmp/with");
            assert_eq!(result.iter().nth(2).unwrap(), "spaces");
        }
    }

    mod parse_numprocs_tests {
        use super::*;

        #[test]
        fn test_parse_numprocs_valid() {
            let numprocs = "5";
            let result = ProgramParser::parse_numprocs(numprocs).unwrap();
            assert_eq!(result, 5);
        }

        #[test]
        fn test_parse_numprocs_invalid() {
            let numprocs = "abc";
            let result = ProgramParser::parse_numprocs(numprocs);
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_numprocs_zero() {
            let numprocs = "0";
            let result = ProgramParser::parse_numprocs(numprocs).unwrap();
            assert_eq!(result, 0);
        }

        #[test]
        fn test_parse_numprocs_max() {
            let numprocs = "255";
            let result = ProgramParser::parse_numprocs(numprocs).unwrap();
            assert_eq!(result, 255);
        }

        #[test]
        fn test_parse_numprocs_out_of_range() {
            let numprocs = "256";
            let result = ProgramParser::parse_numprocs(numprocs);
            assert!(result.is_err());
        }
    }
}
