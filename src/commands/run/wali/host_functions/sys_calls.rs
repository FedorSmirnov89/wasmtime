//! Module for the host functions which represent system calls made from the side of the module.
//! These are forwarded to the host OS by the runtime.

use tracing::info;

mod execve;
mod exit_group;
mod fwd;
mod mmap;
mod munmap;
mod writev;

pub(crate) use execve::execve;
pub(crate) use exit_group::exit_group;
pub(crate) use fwd::*;
pub(crate) use mmap::syscall_mmap;
pub(crate) use munmap::syscall_munmap;
pub(crate) use writev::syscall_writev;

pub(super) fn getpid() -> i64 {
    info!("module has executed the 'getpid' host function.");
    let sys_call_result = unsafe { libc::getpid() };
    sys_call_result as i64
}

pub(super) fn brk(_a1: i32) -> i64 {
    info!(
        "module has executed the 'brk' host function. In WASM context, this corresponds to a NOP"
    );
    0
}
