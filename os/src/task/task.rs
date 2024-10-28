//! Types related to task management

use super::TaskContext;

// My code
use crate::task::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub status: TaskStatus,
    /// The task context
    pub context: TaskContext,
    /// The task syscall times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The task running time
    pub running_time: usize,
    /// Start time of task
    pub start_time: usize,
}

/// The status of a task
#[derive(Debug)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
