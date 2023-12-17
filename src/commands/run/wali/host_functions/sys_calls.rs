//! Module for the host functions which represent system calls made from the side of the module.
//! These are forwarded to the host OS by the runtime.

use anyhow::Result;
use libc::SYS_set_tid_address;
use wasmtime::Caller;

use tracing::info;

use crate::commands::run::wali::{memory::WasmAddress, WaliCtx};

mod writev;

pub(crate) use writev::syscall_writev;

pub(super) fn set_tid_address(mut caller: Caller<'_, WaliCtx>, ptr_wasm: i32) -> i64 {
    info!("module has executed the 'set_tid_address' host function");
    let ptr_wasm = WasmAddress::new(ptr_wasm, &mut caller);
    match set_tid_address_impl(caller, ptr_wasm) {
        Ok(r) => r,
        Err(e) => {
            println!("error when calling set_tid_address: {e}");
            -1
        }
    }
}

fn set_tid_address_impl(mut caller: Caller<'_, WaliCtx>, ptr_wasm: WasmAddress) -> Result<i64> {
    let host_address: i64 = ptr_wasm.to_host_address(&mut caller).into();
    let sys_call_result = unsafe { libc::syscall(SYS_set_tid_address, host_address) };
    Ok(sys_call_result)
}

pub(super) fn ioctl(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: i32) -> i64 {
    info!("module has executed the 'ioctl' host function");
    let ptr_wasm = WasmAddress::new(a3, &mut caller);
    ioctl_impl(caller, a1, a2, ptr_wasm)
}

fn ioctl_impl(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: WasmAddress) -> i64 {
    let a3 = a3.to_host_address(&mut caller);
    let sys_call_result = unsafe { libc::ioctl(a1, a2 as u64, a3) };
    sys_call_result as i64
}
