use std::collections::HashMap;

use crate::exec::process::Process;

#[derive(Debug)]
pub struct ProcessGroup {
    programname: String,
    processes: HashMap<String, Process>,
}
