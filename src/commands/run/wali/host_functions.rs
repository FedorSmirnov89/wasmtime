//! Module for the host functions which the runtime offers to the Wasm modules using the WALI interface.

use anyhow::Result;
use wasmtime::Linker;

use crate::commands::{
    run::wali::host_functions::{
        arguments::{cl_copy_argv, cl_get_argc, cl_get_argv_len},
        sys_calls::{
            access, brk, clock_gettime, clock_nanosleep, mprotect, nanosleep, open, syscall_mmap,
            syscall_munmap, syscall_writev, uname, write,
        },
    },
    RunCommand,
};

use tracing::debug;

use self::{
    env_vars::get_init_envfile,
    sys_calls::{ioctl, set_tid_address},
    wali_specific::{call_ctors, call_dtors, proc_exit},
};

use super::WaliCtx;
pub(crate) mod arguments;
pub(crate) mod env_vars;
pub(crate) mod sys_calls;
pub(crate) mod wali_specific;

impl RunCommand {
    pub(crate) fn link_wali_host_functions(&self, linker: &mut Linker<WaliCtx>) -> Result<()> {
        debug!("linking host functions");

        // wali-specific
        linker.func_wrap("wali", "__call_ctors", call_ctors)?;
        linker.func_wrap("wali", "__call_dtors", call_dtors)?;
        linker.func_wrap("wali", "__proc_exit", proc_exit)?;

        // env vars
        linker.func_wrap("wali", "__get_init_envfile", get_init_envfile)?;

        // arguments
        linker.func_wrap("wali", "__cl_get_argc", cl_get_argc)?;
        linker.func_wrap("wali", "__cl_get_argv_len", cl_get_argv_len)?;
        linker.func_wrap("wali", "__cl_copy_argv", cl_copy_argv)?;

        // sys calls
        linker.func_wrap("wali", "SYS_access", access)?;
        linker.func_wrap("wali", "SYS_brk", brk)?;
        linker.func_wrap("wali", "SYS_clock_gettime", clock_gettime)?;
        linker.func_wrap("wali", "SYS_clock_nanosleep", clock_nanosleep)?;
        linker.func_wrap("wali", "SYS_ioctl", ioctl)?;
        linker.func_wrap("wali", "SYS_mmap", syscall_mmap)?;
        linker.func_wrap("wali", "SYS_mprotect", mprotect)?;
        linker.func_wrap("wali", "SYS_munmap", syscall_munmap)?;
        linker.func_wrap("wali", "SYS_nanosleep", nanosleep)?;
        linker.func_wrap("wali", "SYS_open", open)?;
        linker.func_wrap("wali", "SYS_set_tid_address", set_tid_address)?;
        linker.func_wrap("wali", "SYS_uname", uname)?;
        linker.func_wrap("wali", "SYS_write", write)?;
        linker.func_wrap("wali", "SYS_writev", syscall_writev)?;

        Ok(())
    }
}
