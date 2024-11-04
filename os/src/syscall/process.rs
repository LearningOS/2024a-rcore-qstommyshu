//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_buffer,
    task::{
        change_program_brk,
        current_task_info,
        current_task_mmap,
        current_task_munmap,
        current_user_token,
        exit_current_and_run_next,
        suspend_current_and_run_next,
        TaskStatus,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

// copy a data type from current space to a space specified by "token"
// takes in *mut T because we need to modify dst data type
fn copy_to_space<T>(token: usize, src: &T, dst: *mut T) {
    let src_ptr: *const u8 = unsafe { core::mem::transmute(src) };
    let dst_ptr = unsafe { core::mem::transmute(dst) };

    let size = core::mem::size_of::<T>();

    let dst_bytes = translated_byte_buffer(token, dst_ptr, size);

    let mut bytes_read = 0;
    for dst_byte in dst_bytes {
        let src_byte = unsafe {
            core::slice::from_raw_parts(src_ptr.add(bytes_read), dst_byte.len())
        };
        dst_byte.copy_from_slice(src_byte);
        bytes_read += dst_byte.len();
    }
}

// Syscall fold should all be kernel mode
/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // user mode wants to call sys_get_time
    // call get_time_us() in supervisor mode, make it into bytes,
    // then copy it to user mode (we got user data pointer here)
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    // copy to user space
    copy_to_space(current_user_token(), &time_val, ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let task_info = current_task_info();
    copy_to_space(current_user_token(), &task_info, ti);
    0
}

// YOUR JOB: Implement mmap.
// start is the starting address of the vaddress
// len is byte length that we need to map
// port is bitflags
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    current_task_mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    current_task_munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
