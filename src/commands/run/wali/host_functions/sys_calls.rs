//! Module for the host functions which represent system calls made from the side of the module.
//! These are forwarded to the host OS by the runtime.

use anyhow::Result;
use libc::SYS_set_tid_address;
use wasmtime::Caller;

use tracing::info;

use crate::commands::run::wali::{
    memory::{address::WasmAddress, AsMemory},
    WaliCtx,
};

mod mmap;
mod munmap;
mod writev;

pub(crate) use mmap::syscall_mmap;
pub(crate) use munmap::syscall_munmap;
pub(crate) use writev::syscall_writev;

pub(super) fn set_tid_address(mut caller: Caller<'_, WaliCtx>, ptr_wasm: i32) -> i64 {
    info!("module has executed the 'set_tid_address' host function");
    let ptr_wasm = WasmAddress::new(ptr_wasm, &caller.as_memory());
    match set_tid_address_impl(caller, ptr_wasm) {
        Ok(r) => r,
        Err(e) => {
            println!("error when calling set_tid_address: {e}");
            -1
        }
    }
}

fn set_tid_address_impl(mut caller: Caller<'_, WaliCtx>, ptr_wasm: WasmAddress) -> Result<i64> {
    let host_address: i64 = ptr_wasm.to_host_address(&caller.as_memory()).into();
    let sys_call_result = unsafe { libc::syscall(SYS_set_tid_address, host_address) };
    Ok(sys_call_result)
}

pub(super) fn ioctl(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: i32) -> i64 {
    info!("module has executed the 'ioctl' host function");
    let ptr_wasm = WasmAddress::new(a3, &caller.as_memory());
    ioctl_impl(caller, a1, a2, ptr_wasm)
}

fn ioctl_impl(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: WasmAddress) -> i64 {
    let a3 = a3.to_host_address(&caller.as_memory());
    let sys_call_result = unsafe { libc::ioctl(a1, a2 as u64, a3) };
    sys_call_result as i64
}

pub(super) fn syscall_clock_gettime(
    mut caller: Caller<'_, WaliCtx>,
    a1: i32,
    wasm_offset: i32,
) -> i64 {
    info!("module trying to get the system time");

    let wasm_address = WasmAddress::new(wasm_offset, &caller.as_memory());
    let host_address = wasm_address.to_host_address(&caller.as_memory());

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

    let memory = caller.as_memory();
    let host_address = WasmAddress::new(time_spec_offset, &memory).to_host_address(&memory);
    let host_address_remain =
        WasmAddress::new(timespec_remain_offset, &memory).to_host_address(&memory);

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

    let wasm_address = WasmAddress::new(wasm_offset, &caller.as_memory());
    let host_address = wasm_address.to_host_address(&caller.as_memory());

    unsafe { libc::syscall(libc::SYS_write, a1, host_address, a3) }
}

pub(super) fn syscall_brk(_a1: i32) -> i64 {
    info!(
        "module has executed the 'brk' host function. In WASM context, this corresponds to a NOP"
    );
    0
}

pub(super) fn syscall_mprotect(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: i32) -> i64 {
    info!("module has executed the 'mprotect' host function");
    let memory = caller.as_memory();
    let host_address = WasmAddress::new(a1, &memory).to_host_address(&memory);
    let sys_call_result = unsafe { libc::mprotect(host_address.as_void_ptr(), a2 as usize, a3) };
    sys_call_result as i64
}

pub(super) fn syscall_access(
    mut caller: Caller<'_, WaliCtx>,
    path_name_addr: i32,
    mode: i32,
) -> i64 {
    info!("syscall access: a1: {path_name_addr}; a2: {mode}.");

    let memory = caller.as_memory();
    let host_address = WasmAddress::new(path_name_addr, &memory)
        .to_host_address(&memory)
        .as_i64_ptr();

    #[cfg(target_arch = "x86_64")]
    {
        unsafe { libc::syscall(libc::SYS_access, host_address, mode) }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        panic!("syscall access not implemented for architectures other than x86")
    }
}

pub(super) fn syscall_open(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32, a3: i32) -> i64 {
    let memory = caller.as_memory();
    let host_address = WasmAddress::new(a1, &memory)
        .to_host_address(&memory)
        .as_i64_ptr();

    #[cfg(target_arch = "x86_64")]
    {
        unsafe { libc::syscall(libc::SYS_open, host_address, a2, a3) }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        panic!("syscall access not implemented for architectures other than x86")
    }
}

pub(super) fn syscall_nanosleep(mut caller: Caller<'_, WaliCtx>, a1: i32, a2: i32) -> i64 {
    let memory = caller.as_memory();
    let host_address_1 = WasmAddress::new(a1, &memory)
        .to_host_address(&memory)
        .as_i64_ptr();
    let host_address_2 = WasmAddress::new(a2, &memory)
        .to_host_address(&memory)
        .as_i64_ptr();
    unsafe { libc::syscall(libc::SYS_nanosleep, host_address_1, host_address_2) }
}

pub(super) fn syscall_uname(mut caller: Caller<'_, WaliCtx>, a1: i32) -> i64 {
    let memory = caller.as_memory();
    let host_address = WasmAddress::new(a1, &memory)
        .to_host_address(&memory)
        .as_i64_ptr();
    unsafe { libc::syscall(libc::SYS_uname, host_address) }
}
