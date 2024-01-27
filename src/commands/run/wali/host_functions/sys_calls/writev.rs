use anyhow::Result;
use wasmtime::{Caller, SharedMemory};

use crate::commands::run::wali::{memory::address::WasmAddress, WaliCtx};

use tracing::{error, info};

pub(crate) fn syscall_writev(
    caller: Caller<'_, WaliCtx>,
    fd: i32,
    iov_offset: i32,
    iov_cnt: i32,
) -> i64 {
    let tid = unsafe { libc::pthread_self() };
    info!("module has executed the 'writev' host function from thread {tid}.");
    let ctx_guard = caller.data().lock().expect("could not lock ctx");
    let memory = ctx_guard.get_memory().expect("memory not set");
    let iov_addr_wasm = WasmAddress::new(iov_offset, memory);
    match syscall_writev_impl(memory, fd, iov_addr_wasm, iov_cnt) {
        Ok(r) => r,
        Err(e) => {
            error!("error when calling writev: {e}");
            -1
        }
    }
}

fn syscall_writev_impl(
    memory: &SharedMemory,
    fd: i32,
    iov_addr_wasm: WasmAddress,
    iov_cnt: i32,
) -> Result<i64> {
    let iov_addr_host = iov_addr_wasm.to_host_address(memory);

    let iov_ptr = iov_addr_host.as_i64_ptr() as *const IoVecWasm;
    let iov_slice = unsafe { std::slice::from_raw_parts(iov_ptr, iov_cnt as usize) };

    let mut iovs_host = vec![];
    for iov in iov_slice {
        let size = iov.iov_len as usize;
        let base_wasm = WasmAddress::new(iov.iov_base, memory);
        let base_host = base_wasm.to_host_address(memory).as_void_ptr();
        let iov_host = libc::iovec {
            iov_base: base_host,
            iov_len: size,
        };
        iovs_host.push(iov_host);
    }

    let sys_call_result = unsafe { libc::writev(fd, iovs_host.as_ptr(), iovs_host.len() as i32) };
    Ok(sys_call_result as i64)
}

///
/// Represents an `iovec` struct in the module memory. It is necessary to use this struct instead of the
/// `libc::iovec` struct because the latter uses `usize` for the `iov_base` field, which could be (depending
/// on the platform where the runtime is run) not compatible with the `i32` type used in the module memory.
///
#[repr(C)]
struct IoVecWasm {
    pub iov_base: i32,
    pub iov_len: i32,
}
