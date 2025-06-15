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
        #[derive(Debug, PartialEq)]
        enum State {
            Start,
            Key,
            Value,
            QuoteedValue,
            End,
        }

        let mut list: LinkedList<String> = LinkedList::new();
        let chars: Vec<char> = environment.chars().collect();
        let mut i = 0;
        let mut cur_key: String = String::new();
        let mut cur_value: String = String::new();
        let mut cur_state = State::Start;
        let mut delimiter: char = '\0';
        while i < chars.len() {
            let c = chars[i];
            match cur_state {
                State::Start => {
                    if c.is_whitespace() {
                        i += 1;
                        continue;
                    }
                    cur_state = State::Key;
                }
                State::Key => {
                    if c == '=' {
                        Self::parse_enviroment_key(&cur_key)?;
                        cur_state = State::Value;
                    } else {
                        cur_key.push(c);
                    }
                    i += 1;
                }
                State::Value => {
                    match c {
                        '\'' | '"' => {
                            cur_state = State::QuoteedValue;
                            delimiter = c;
                        }
                        c if c.is_whitespace() || c == ',' => {
                            list.push_back(format!("{}={}", cur_key, cur_value));
                            cur_state = State::End;
                        }
                        c => cur_value.push(c),
                    }
                    i += 1;
                }
                State::QuoteedValue => {
                    if c == delimiter {
                        delimiter = '\0';
                        cur_state = State::Value;
                    } else {
                        cur_value.push(c);
                    }
                    i += 1;
                }
                State::End => {
                    cur_key.clear();
                    cur_value.clear();
                    delimiter = '\0';
                    cur_state = State::Start;
                }
            }
        }
        if !cur_value.is_empty() {
            list.push_back(format!("{}={}", cur_key, cur_value));
            cur_state = State::End;
        }
        if cur_state != State::Start && cur_state != State::End {
            Err(ConfigParseError::UnexpectedValue(environment.to_string()))?;
        }
        Ok(list)
    }

    fn parse_enviroment_key(key: &str) -> Result<(), ConfigParseError> {
        if key.is_empty() {
            return Err(ConfigParseError::UnexpectedValue(key.to_string()));
        }
        Ok(())
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

    mod parse_autostart_tests {
        use super::*;

        #[test]
        fn test_parse_autostart_valid_true() {
            let autostart = "true";
            let result = ProgramParser::parse_autostart(autostart).unwrap();
            assert_eq!(result, true);
        }

        #[test]
        fn test_parse_autostart_valid_false() {
            let autostart = "false";
            let result = ProgramParser::parse_autostart(autostart).unwrap();
            assert_eq!(result, false);
        }

        #[test]
        fn test_parse_autostart_invalid() {
            let autostart = "abc";
            let result = ProgramParser::parse_autostart(autostart);
            assert!(result.is_err());
        }
    }

    mod parse_autorestart_tests {
        use super::*;

        #[test]
        fn test_parse_autorestart_valid_true() {
            let autorestart = "true";
            let result = ProgramParser::parse_autorestart(autorestart).unwrap();
            assert_eq!(result, AutoRestart::True);
        }

        #[test]
        fn test_parse_autorestart_valid_false() {
            let autorestart = "false";
            let result = ProgramParser::parse_autorestart(autorestart).unwrap();
            assert_eq!(result, AutoRestart::False);
        }

        #[test]
        fn test_parse_autorestart_valid_unexpected() {
            let autorestart = "unexpected";
            let result = ProgramParser::parse_autorestart(autorestart).unwrap();
            assert_eq!(result, AutoRestart::Unexpected);
        }

        #[test]
        fn test_parse_autorestart_invalid() {
            let autorestart = "abc";
            let result = ProgramParser::parse_autorestart(autorestart);
            assert!(result.is_err());
        }
    }

    mod parse_exitcodes_tests {
        use super::*;

        #[test]
        fn test_parse_exitcodes_valid() {
            let exitcodes = "1,2,3";
            let result = ProgramParser::parse_exitcodes(exitcodes).unwrap();
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().nth(0).unwrap(), &1);
            assert_eq!(result.iter().nth(1).unwrap(), &2);
            assert_eq!(result.iter().nth(2).unwrap(), &3);
        }

        #[test]
        fn test_parse_exitcodes_invalid() {
            let exitcodes = "1,a,3";
            let result = ProgramParser::parse_exitcodes(exitcodes);
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_exitcodes_empty() {
            let exitcodes = "";
            let result = ProgramParser::parse_exitcodes(exitcodes);
            assert!(result.is_err());
        }
    }

    mod parse_startsecs_tests {
        use super::*;

        #[test]
        fn test_parse_startsecs_valid() {
            let startsecs = "5";
            let result = ProgramParser::parse_startsecs(startsecs).unwrap();
            assert_eq!(result, 5);
        }

        #[test]
        fn test_parse_startsecs_invalid() {
            let startsecs = "abc";
            let result = ProgramParser::parse_startsecs(startsecs);
            assert!(result.is_err());
        }
    }

    mod parse_startretries_tests {
        use super::*;

        #[test]
        fn test_parse_startretries_valid() {
            let startretries = "5";
            let result = ProgramParser::parse_startretries(startretries).unwrap();
            assert_eq!(result, 5);
        }

        #[test]
        fn test_parse_startretries_invalid() {
            let startretries = "abc";
            let result = ProgramParser::parse_startretries(startretries);
            assert!(result.is_err());
        }
    }

    mod parse_stopsignal_tests {
        use super::*;

        #[test]
        fn test_parse_stopsignal_valid() {
            let stopsignal = "15";
            let result = ProgramParser::parse_stopsignal(stopsignal).unwrap();
            assert_eq!(result, 15);
        }

        #[test]
        fn test_parse_stopsignal_invalid() {
            let stopsignal = "abc";
            let result = ProgramParser::parse_stopsignal(stopsignal);
            assert!(result.is_err());
        }
    }

    mod parse_stopwaitsecs_tests {
        use super::*;

        #[test]
        fn test_parse_stopwaitsecs_valid() {
            let stopwaitsecs = "10";
            let result = ProgramParser::parse_stopwaitsecs(stopwaitsecs).unwrap();
            assert_eq!(result, 10);
        }

        #[test]
        fn test_parse_stopwaitsecs_invalid() {
            let stopwaitsecs = "abc";
            let result = ProgramParser::parse_stopwaitsecs(stopwaitsecs);
            assert!(result.is_err());
        }
    }

    mod parse_stdout_logfile_tests {
        use super::*;

        #[test]
        fn test_parse_stdout_logfile_valid() {
            let stdout_logfile = "/var/log/stdout.log";
            let result = ProgramParser::parse_stdout_logfile(stdout_logfile).unwrap();
            assert_eq!(result, "/var/log/stdout.log");
        }
    }

    mod parse_stderr_logfile_tests {
        use super::*;

        #[test]
        fn test_parse_stderr_logfile_valid() {
            let stderr_logfile = "/var/log/stderr.log";
            let result = ProgramParser::parse_stderr_logfile(stderr_logfile).unwrap();
            assert_eq!(result, "/var/log/stderr.log");
        }
    }

    mod parse_environment_tests {
        use super::*;

        #[test]
        fn test_parse_environment_without_double_quotes() {
            let environment = "key1=value1,key2=value2";
            let result = ProgramParser::parse_environment(environment).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result.iter().nth(0).unwrap(), "key1=value1");
            assert_eq!(result.iter().nth(1).unwrap(), "key2=value2");
        }

        #[test]
        fn test_parse_environment_with_double_quotes() {
            let environment = "key1=\"value1, test\",key2=\"value2\"";
            let result = ProgramParser::parse_environment(environment).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result.iter().nth(0).unwrap(), "key1=value1, test");
            assert_eq!(result.iter().nth(1).unwrap(), "key2=value2");
        }

        #[test]
        fn test_parse_environment_empty() {
            let environment = "";
            let result = ProgramParser::parse_environment(environment).unwrap();
            assert_eq!(result.len(), 0);
        }

        #[test]
        fn test_parse_environment_spaces() {
            let environment = "     ";
            let result = ProgramParser::parse_environment(environment).unwrap();
            assert_eq!(result.len(), 0);
        }
    }

    mod parse_directory_tests {
        use super::*;

        #[test]
        fn test_parse_directory_valid() {
            let directory = "/tmp";
            let result = ProgramParser::parse_directory(directory).unwrap();
            assert_eq!(result, "/tmp");
        }
    }

    mod parse_umask_tests {
        use super::*;

        #[test]
        fn test_parse_umask_valid() {
            let umask = "0777";
            let result = ProgramParser::parse_umask(umask).unwrap();
            assert_eq!(result, 777);
        }

        #[test]
        fn test_parse_umask_invalid() {
            let umask = "abc";
            let result = ProgramParser::parse_umask(umask);
            assert!(result.is_err());
        }
    }
}
