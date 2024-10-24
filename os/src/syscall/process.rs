//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
/// dummy
pub struct TimeVal {
    /// dummy
    pub sec: usize,
    /// dummy
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
    /// Start time of task
    pub start_time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    // use crate::syscall::{SYSCALL_WRITE, SYSCALL_EXIT, SYSCALL_YIELD, SYSCALL_GET_TIME, SYSCALL_TASK_INFO};
    // let ti = unsafe { &mut *_ti };
    // println!("current task status: {:?}, running time: {:?}", ti.status, ti.time);
    // println!("SYSCALL_WRITE: {:?}, SYSCALL_EXIT: {:?}, SYSCALL_YIELD: {:?}, SYSCALL_GET_TIME: {:?}, SYSCALL_TASK_INFO: {:?}",
    //     ti.syscall_times[SYSCALL_WRITE], ti.syscall_times[SYSCALL_EXIT], ti.syscall_times[SYSCALL_YIELD], ti.syscall_times[SYSCALL_GET_TIME], ti.syscall_times[SYSCALL_TASK_INFO]
    // );

    let task_info = crate::task::get_current_task_info();
    // match task_info.status {
    //     TaskStatus::Running => {
    //         println!("current task status: {:?}, running time: {:?}", task_info.status, task_info.time);
    //     },
    //     _ => {
    //         println!("not running");
    //     }
    // }
    let ti = unsafe { &mut *_ti };
    ti.status = task_info.status;
    ti.syscall_times = task_info.syscall_times;
    ti.time = task_info.time;

    0
}
