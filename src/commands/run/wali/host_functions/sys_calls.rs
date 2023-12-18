//! Module for the host functions which represent system calls made from the side of the module.
//! These are forwarded to the host OS by the runtime.

use tracing::info;

mod fwd;
mod mmap;
mod munmap;
mod writev;

pub(crate) use fwd::*;
pub(crate) use mmap::syscall_mmap;
pub(crate) use munmap::syscall_munmap;
pub(crate) use writev::syscall_writev;

pub(super) fn brk(_a1: i32) -> i64 {
    info!(
        "module has executed the 'brk' host function. In WASM context, this corresponds to a NOP"
    );
    0
}
