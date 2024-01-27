use anyhow::Result;
use tracing::{debug, error, info};
use wasmtime::{Caller, SharedMemory};

use crate::commands::run::wali::{memory::address::WasmAddress, WaliCtx};

pub(crate) fn execve(caller: Caller<'_, WaliCtx>, path: i32, argv: i32, envp: i32) -> i64 {
    info!("module has executed the 'execve' host function.");
    match execve_imp(caller, path, argv, envp) {
        Ok(r) => r,
        Err(e) => {
            error!("error when calling 'execve': {e}");
            -1
        }
    }
}

fn execve_imp(caller: Caller<'_, WaliCtx>, path: i32, argv_wasm: i32, _envp: i32) -> Result<i64> {
    let ctx_guard = caller.data().lock()?;
    let memory = ctx_guard.get_memory()?;
    let arr_ptr: *const i32 = WasmAddress::new(argv_wasm, memory)
        .to_host_address(memory)
        .as_i32_ptr();
    let mut cur_ptr = WasmAddress::new(argv_wasm, memory)
        .to_host_address(memory)
        .as_i32_ptr();
    let mut arg_c: usize = 0;

    // count the arguments
    loop {
        let address = unsafe { *cur_ptr };
        if address == 0 {
            break;
        }
        arg_c += 1;
        cur_ptr = unsafe { cur_ptr.offset(1) };
    }
    debug!("counted {arg_c} arguments");

    // create a vector of the arguments on the host side
    let arg_vec = null_terminated_address_vec(arr_ptr, arg_c, &memory);
    let arg_vec_ptr = arg_vec.as_ptr() as *const i64;

    let path_str = WasmAddress::new(path, memory)
        .to_host_address(memory)
        .as_usize();

    debug!("ignoring the env vars for now");
    let env_vec = vec![];
    let env_vec_ptr = env_vec.as_ptr() as *const i64;

    let syscall_result = unsafe { libc::syscall(59, path_str, arg_vec_ptr, env_vec_ptr) };
    Ok(syscall_result)
}

fn null_terminated_address_vec(
    array_ptr: *const i32,
    n_args: usize,
    memory: &SharedMemory,
) -> Vec<usize> {
    let arg_addr_slice = unsafe { std::slice::from_raw_parts(array_ptr, n_args) };
    let mut arg_addr_vec = Vec::new();

    for arg_addr_wasm in arg_addr_slice {
        let offset = *arg_addr_wasm;
        let host_address = WasmAddress::new(offset, memory)
            .to_host_address(memory)
            .as_usize();
        arg_addr_vec.push(host_address);
    }
    let null_entry = std::ptr::null::<i32>() as usize;
    arg_addr_vec.push(null_entry);
    arg_addr_vec
}
