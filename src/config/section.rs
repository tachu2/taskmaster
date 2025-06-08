#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Taskmasterd,
    Program,
}

impl Section {
    pub fn as_str(&self) -> &'static str {
        match self {
            Section::Taskmasterd => section::TASKMASTERD,
            Section::Program => section::PROGRAM,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            section::TASKMASTERD => Some(Section::Taskmasterd),
            section::PROGRAM => Some(Section::Program),
            _ => None,
        }
    }
}

mod section {
    pub const TASKMASTERD: &str = "taskmasterd";
    pub const PROGRAM: &str = "program";
}
