use std::collections::{HashSet, LinkedList};

#[derive(Debug)]
pub struct ProgramSection {
    pub programname: String,
    pub command: LinkedList<String>,
    pub processnames: HashSet<String>,
}

impl ProgramSection {
    pub fn new(programname: String) -> Self {
        ProgramSection {
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
