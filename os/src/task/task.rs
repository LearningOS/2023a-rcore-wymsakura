//! Types related to task management


use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info inner
    pub task_info_inner: TaskInfoInner,
}

/// The status of a task
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

/// The info of a task
#[derive(Copy, Clone)]
pub struct TaskInfoInner {
    /// The task status in it's lifecycle
    pub status: TaskStatus,
    /// The times of syscall by bucket
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The task start_time
    pub start_time: usize,
    /// The task end_time
    pub end_time: usize,
}

impl TaskInfoInner {
    /// Create a new empty task info
    pub fn init() -> Self {
        Self {
            status: TaskStatus::Running,
            syscall_times: [0; MAX_SYSCALL_NUM],
            start_time: 0,
            end_time: 0,
        }
    }
}