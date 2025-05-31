#[derive(Debug)]
pub enum ProcessState {
    STOPPED,
    STARTING,
    BACKOFF,
    STOPPING,
    EXITED,
    FATAL,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Process {
    pid: u32,
    name: String,
    state: ProcessState,
}
