//! Module for the host functions which the runtime offers to the Wasm modules using the WALI interface.

use anyhow::Result;
use wasmtime::Linker;

use crate::commands::{
    run::wali::host_functions::{
        arguments::{cl_copy_argv, cl_get_argc, cl_get_argv_len},
        sys_calls::{
            accept, access, bind, brk, clock_gettime, clock_nanosleep, close, connect, flock,
            getdents64, getpid, kill, listen, lseek, lstat, mprotect, nanosleep, open, pipe, read,
            sendto, setpgid, setsockopt, shutdown, socket, stat, syscall_mmap, syscall_munmap,
            syscall_writev, uname, utimensat, write,
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
        linker.func_wrap("wali", "SYS_setpgid", setpgid)?;
        linker.func_wrap("wali", "SYS_accept", accept)?;
        linker.func_wrap("wali", "SYS_access", access)?;
        linker.func_wrap("wali", "SYS_bind", bind)?;
        linker.func_wrap("wali", "SYS_brk", brk)?;
        linker.func_wrap("wali", "SYS_clock_gettime", clock_gettime)?;
        linker.func_wrap("wali", "SYS_clock_nanosleep", clock_nanosleep)?;
        linker.func_wrap("wali", "SYS_close", close)?;
        linker.func_wrap("wali", "SYS_connect", connect)?;
        linker.func_wrap("wali", "SYS_flock", flock)?;
        linker.func_wrap("wali", "SYS_getdents64", getdents64)?;
        linker.func_wrap("wali", "SYS_getpid", getpid)?;
        linker.func_wrap("wali", "SYS_ioctl", ioctl)?;
        linker.func_wrap("wali", "SYS_kill", kill)?;
        linker.func_wrap("wali", "SYS_listen", listen)?;
        linker.func_wrap("wali", "SYS_lseek", lseek)?;
        linker.func_wrap("wali", "SYS_lstat", lstat)?;
        linker.func_wrap("wali", "SYS_pipe", pipe)?;
        linker.func_wrap("wali", "SYS_read", read)?;
        linker.func_wrap("wali", "SYS_sendto", sendto)?;
        linker.func_wrap("wali", "SYS_setsockopt", setsockopt)?;
        linker.func_wrap("wali", "SYS_shutdown", shutdown)?;
        linker.func_wrap("wali", "SYS_socket", socket)?;
        linker.func_wrap("wali", "SYS_stat", stat)?;
        linker.func_wrap("wali", "SYS_mmap", syscall_mmap)?;
        linker.func_wrap("wali", "SYS_mprotect", mprotect)?;
        linker.func_wrap("wali", "SYS_munmap", syscall_munmap)?;
        linker.func_wrap("wali", "SYS_nanosleep", nanosleep)?;
        linker.func_wrap("wali", "SYS_open", open)?;
        linker.func_wrap("wali", "SYS_set_tid_address", set_tid_address)?;
        linker.func_wrap("wali", "SYS_uname", uname)?;
        linker.func_wrap("wali", "SYS_utimensat", utimensat)?;
        linker.func_wrap("wali", "SYS_write", write)?;
        linker.func_wrap("wali", "SYS_writev", syscall_writev)?;

        Ok(())
    }
}
