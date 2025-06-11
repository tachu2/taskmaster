use std::collections::{HashSet, LinkedList};

#[derive(Debug)]
pub struct Program {
    pub(in crate::config) programname: String, // unique identifier for the program
    pub(in crate::config) command: LinkedList<String>,
    pub(in crate::config) numprocs: u8, // number of processes to start
    pub(in crate::config) autostart: bool, // whether to start the program automatically
    pub(in crate::config) autorestart: program::AutoRestart, // whether to restart the program automatically
    pub(in crate::config) exitcodes: LinkedList<i32>,        // exit codes to consider successful
    pub(in crate::config) startsecs: u8, // seconds to wait before considering the program started
    pub(in crate::config) startretries: u8, // number of retries to start the program
    pub(in crate::config) stopsignal: i32, // signal to send to stop the program
    pub(in crate::config) stopwaitsecs: u32, // seconds to wait for the program to stop
    pub(in crate::config) stdout_logfile: String,
    pub(in crate::config) stderr_logfile: String,
    pub(in crate::config) enviroment: Option<LinkedList<String>>, // environment variables to set for the program
    pub(in crate::config) directory: Option<String>, // working directory for the program
    pub(in crate::config) umask: Option<u16>,        // working directory for the program
    pub(in crate::config) processnames: HashSet<String>,
}

impl Program {
    pub fn new(
        programname: String,
        command: LinkedList<String>,
        numprocs: Option<u8>,
        autostart: Option<bool>,
        autorestart: Option<program::AutoRestart>,
        exitcodes: Option<LinkedList<i32>>,
        startsecs: Option<u8>,
        startretries: Option<u8>,
        stopsignal: Option<i32>,
        stopwaitsecs: Option<u32>,
        stdout_logfile: Option<String>,
        stderr_logfile: Option<String>,
        environment: Option<LinkedList<String>>,
        directory: Option<String>,
        umask: Option<u16>,
    ) -> Self {
        let numprocs = numprocs.unwrap_or(1);
        let processnames = (1..numprocs)
            .map(|i| format!("{}{}", programname, i))
            .collect();
        let stdout_logfile = stdout_logfile.unwrap_or_else(|| format!("{}.log", programname));
        let stderr_logfile = stderr_logfile.unwrap_or_else(|| format!("{}_err.log", programname));

        Program {
            programname,
            command: command,
            numprocs: numprocs,
            autostart: autostart.unwrap_or(true),
            autorestart: autorestart.unwrap_or(program::AutoRestart::Unexpected),
            exitcodes: exitcodes.unwrap_or(LinkedList::from([0])),
            startsecs: startsecs.unwrap_or(1),
            startretries: startretries.unwrap_or(1),
            stopsignal: stopsignal.unwrap_or(1),
            stopwaitsecs: stopwaitsecs.unwrap_or(10),
            stdout_logfile: stdout_logfile,
            stderr_logfile: stderr_logfile,
            enviroment: environment,
            directory: directory,
            umask: umask,
            processnames: processnames,
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.command.push_back(command);
    }

    pub fn add_processname(&mut self, processname: String) {
        self.processnames.insert(processname);
    }
}

pub enum ProgramSection {
    Command,
    NumProcs,
    AutoStart,
    AutoRestart,
    ExitCodes,
    StartSecs,
    StartRetries,
    StopSignal,
    StopWaitSecs,
    StdoutLogfile,
    StderrLogfile,
    Environment,
    Directory,
    Umask,
}

impl ProgramSection {
    pub fn as_str(&self) -> &str {
        match self {
            ProgramSection::Command => program::COMMAND,
            ProgramSection::NumProcs => program::NUMPROCS,
            ProgramSection::AutoStart => program::AUTOSTART,
            ProgramSection::AutoRestart => program::AUTORESTART,
            ProgramSection::ExitCodes => program::EXITCODES,
            ProgramSection::StartSecs => program::STARTSECS,
            ProgramSection::StartRetries => program::STARTRETRIES,
            ProgramSection::StopSignal => program::STOPSIGNAL,
            ProgramSection::StopWaitSecs => program::STOPWAITSECS,
            ProgramSection::StdoutLogfile => program::STDOUTLOGFILE,
            ProgramSection::StderrLogfile => program::STDERRLOGFILE,
            ProgramSection::Environment => program::ENVIRONMENT,
            ProgramSection::Directory => program::DIRECTORY,
            ProgramSection::Umask => program::UMASK,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            program::COMMAND => Some(ProgramSection::Command),
            program::NUMPROCS => Some(ProgramSection::NumProcs),
            program::AUTOSTART => Some(ProgramSection::AutoStart),
            program::AUTORESTART => Some(ProgramSection::AutoRestart),
            program::EXITCODES => Some(ProgramSection::ExitCodes),
            program::STARTSECS => Some(ProgramSection::StartSecs),
            program::STARTRETRIES => Some(ProgramSection::StartRetries),
            program::STOPSIGNAL => Some(ProgramSection::StopSignal),
            program::STOPWAITSECS => Some(ProgramSection::StopWaitSecs),
            program::STDOUTLOGFILE => Some(ProgramSection::StdoutLogfile),
            program::STDERRLOGFILE => Some(ProgramSection::StderrLogfile),
            program::ENVIRONMENT => Some(ProgramSection::Environment),
            program::DIRECTORY => Some(ProgramSection::Directory),
            program::UMASK => Some(ProgramSection::Umask),
            _ => None,
        }
    }
}

mod program {
    pub const COMMAND: &str = "command";
    pub const NUMPROCS: &str = "numprocs";
    pub const AUTOSTART: &str = "autostart";
    pub const AUTORESTART: &str = "autorestart";
    pub const EXITCODES: &str = "exitcodes";
    pub const STARTSECS: &str = "startsecs";
    pub const STARTRETRIES: &str = "startretries";
    pub const STOPSIGNAL: &str = "stopsignal";
    pub const STOPWAITSECS: &str = "stopwaitsecs";
    pub const STDOUTLOGFILE: &str = "stdout_logfile";
    pub const STDERRLOGFILE: &str = "stderr_logfile";
    pub const ENVIRONMENT: &str = "environment";
    pub const DIRECTORY: &str = "directory";
    pub const UMASK: &str = "umask";

    #[derive(Debug, PartialEq, Eq)]
    pub enum AutoRestart {
        Unexpected,
        True,
        False,
    }
}
