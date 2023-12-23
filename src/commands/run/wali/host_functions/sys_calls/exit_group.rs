use wasmtime::Caller;

use tracing::info;

use crate::commands::run::wali::{host_functions::wali_specific::proc_exit, WaliCtx};

pub(crate) fn exit_group(_caller: Caller<'_, WaliCtx>, a1: i32) -> i64 {
    info!("module has executed the 'exit_group' host function.");
    proc_exit(a1);
    -1
}
