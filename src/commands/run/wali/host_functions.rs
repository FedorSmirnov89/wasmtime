//! Module for the host functions which the runtime offers to the Wasm modules using the WALI interface.

use anyhow::Result;
use wasmtime::Linker;

use crate::commands::{run::wali::host_functions::sys_calls::syscall_writev, RunCommand};

use tracing::debug;

use self::{
    env_vars::get_init_envfile,
    sys_calls::{ioctl, set_tid_address},
    wali_specific::{call_ctors, call_dtors, proc_exit},
};

use super::WaliCtx;
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

        // sys calls
        linker.func_wrap("wali", "SYS_set_tid_address", set_tid_address)?;
        linker.func_wrap("wali", "SYS_ioctl", ioctl)?;
        linker.func_wrap("wali", "SYS_writev", syscall_writev)?;

        Ok(())
    }
}
