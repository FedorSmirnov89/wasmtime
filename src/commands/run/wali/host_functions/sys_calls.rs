//! Module for the host functions which represent system calls made from the side of the module.
//! These are forwarded to the host OS by the runtime.

use anyhow::Result;
use libc::SYS_set_tid_address;
use wasmtime::Caller;

use tracing::info;

use crate::commands::run::wali::{memory::address::WasmAddress, WaliCtx};

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

pub(super) fn syscall_clock_gettime(
    mut caller: Caller<'_, WaliCtx>,
    a1: i32,
    wasm_offset: i32,
) -> i64 {
    info!("module trying to get the system time");

    let wasm_address = WasmAddress::new(wasm_offset, &mut caller);
    let host_address = wasm_address.to_host_address(&mut caller);

    let sys_call_result = unsafe { libc::syscall(libc::SYS_clock_gettime, a1, host_address) };
    sys_call_result
}

pub(super) fn syscall_clock_nanosleep(
    mut caller: Caller<'_, WaliCtx>,
    clock_id: i32,
    flags: i32,
    time_spec_offset: i32,
    timespec_remain_offset: i32,
) -> i64 {
    info!("module has executed the 'write' host function");

    let host_address = WasmAddress::new(time_spec_offset, &mut caller).to_host_address(&mut caller);
    let host_address_remain =
        WasmAddress::new(timespec_remain_offset, &mut caller).to_host_address(&mut caller);

    unsafe {
        libc::syscall(
            libc::SYS_clock_nanosleep,
            clock_id,
            flags,
            host_address,
            host_address_remain,
        )
    }
}

pub(super) fn syscall_write(
    mut caller: Caller<'_, WaliCtx>,
    a1: i32,
    wasm_offset: i32,
    a3: i32,
) -> i64 {
    info!("module has executed the 'write' host function");

    let wasm_address = WasmAddress::new(wasm_offset, &mut caller);
    let host_address = wasm_address.to_host_address(&mut caller);

    unsafe { libc::syscall(libc::SYS_write, a1, host_address, a3) }
}

pub(super) fn syscall_brk(_a1: i32) -> i64 {
    info!(
        "module has executed the 'brk' host function.\nIn WASM context, this corresponds to a NOP"
    );
    0
}
