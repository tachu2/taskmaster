use std::collections::HashMap;

use crate::exec::process_group::ProcessGroup;

#[derive(Debug)]
pub struct ProcessManager {
    process_groups: HashMap<String, ProcessGroup>,
}
