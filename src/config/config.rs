use std::collections::HashMap;

use crate::config::program_section::ProgramSection;

#[derive(Debug)]
pub struct Config {
    pub(crate) programs: HashMap<String, ProgramSection>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            programs: HashMap::new(),
        }
    }

    pub fn get_program(&self, name: &String) -> Option<&ProgramSection> {
        self.programs.get(name)
    }

    pub fn programs(&self) -> &HashMap<String, ProgramSection> {
        &self.programs
    }
}
