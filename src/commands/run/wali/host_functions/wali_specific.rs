//! Module for the host functions which are Wali-specific, i.e., are introduced during the compilation to Wasm
//! or used internally by the runtime

use tracing::info;

pub(crate) fn call_ctors() {
    info!("module has executed the '__call_ctors' host function");
}

pub(crate) fn call_dtors() {
    info!("module has executed the '__call_dtors' host function");
}

pub(crate) fn proc_exit(exit_code: i32) {
    info!("module has executed the 'exit' host function");
    info!("module executed successfully; exiting process");
    std::process::exit(exit_code);
}
