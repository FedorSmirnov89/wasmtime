//! Module for the host functions providing the input arguments to the wasm module.

use anyhow::Result;
use tracing::{error, info};
use wasmtime::Caller;

use crate::commands::run::wali::{
    memory::{address::WasmAddress, writing::write_c_string_into_module_memory, AsMemory},
    WaliCtx,
};

///
/// Returns the number of arguments that the module was started with
///
pub(super) fn cl_get_argc(caller: Caller<'_, WaliCtx>) -> i32 {
    let arg_c = caller.data().arg_len();
    info!("module requested number of arguments; Number of arguments is {arg_c}");
    arg_c as i32
}

///
/// Returns the length (number of bytes it will occupy in module memory) of the argument
/// at the provided idx.
///
pub(super) fn cl_get_argv_len(caller: Caller<'_, WaliCtx>, arg_idx: i32) -> i32 {
    info!("module requesting length of arg at idx {arg_idx}");
    match get_arg_len(&caller, arg_idx as usize) {
        Ok(arg_len) => arg_len as i32,
        Err(e) => {
            error!("error getting arg len: {e}");
            -1
        }
    }
}

fn get_arg_len(caller: &Caller<'_, WaliCtx>, arg_idx: usize) -> Result<usize> {
    caller.data().arg_byte_len(arg_idx)
}

///
/// Copies the argument at the provided idx to the module memory address specified by the given offset
///
pub(super) fn cl_copy_argv(mut caller: Caller<'_, WaliCtx>, argv_addr: i32, arg_idx: i32) -> i32 {
    info!("module trying to copy argument at idx {arg_idx} into memory at position {argv_addr}");
    let addr_wasm = WasmAddress::new(argv_addr, &caller.as_memory());
    match copy_arg_into_module(&mut caller, addr_wasm, arg_idx as usize) {
        Ok(n_written) => n_written as i32,
        Err(e) => {
            error!("error when copying argument into module memory: {e}");
            -1
        }
    }
}

fn copy_arg_into_module(
    caller: &mut Caller<'_, WaliCtx>,
    addr_wasm: WasmAddress,
    arg_idx: usize,
) -> Result<usize> {
    let c_string = caller.data().arg_as_c_string(arg_idx)?;
    write_c_string_into_module_memory(caller, addr_wasm, c_string)
}
