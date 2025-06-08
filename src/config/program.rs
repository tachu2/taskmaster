use std::collections::{HashSet, LinkedList};

#[derive(Debug)]
pub struct Program {
    pub programname: String,
    pub command: LinkedList<String>,
    pub processnames: HashSet<String>,
}

impl Program {
    pub fn new(programname: String) -> Self {
        Program {
            programname,
            command: LinkedList::new(),
            processnames: HashSet::new(),
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
}
