use std::sync::MutexGuard;

use anyhow::{bail, Result};
use libc::MAP_FIXED;
use wasmtime::Caller;

use tracing::{error, info, trace};

use crate::commands::run::wali::{
    memory::address::{HostAddress, WasmAddress},
    store::{InnerCtx, MMapData},
    WaliCtx,
};

pub fn syscall_mmap(
    caller: Caller<'_, WaliCtx>,
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i64,
) -> i64 {
    info!("module has executed the 'mmap' host function");
    log_arguments(a1, a2, a3, a4, a5, a6);
    match syscall_mmap_impl(caller, a1, a2, a3, a4, a5, a6) {
        Ok(r) => r,
        Err(e) => {
            error!("error when calling mmap: {e}");
            -1
        }
    }
}

fn syscall_mmap_impl(
    caller: Caller<'_, WaliCtx>,
    _a1: i32,
    length: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i64,
) -> Result<i64> {
    let mut ctx_inner = caller.data().lock()?;
    grow_wasm_memory(&mut ctx_inner, length)?;
    let mmap_addr = call_mmap_syscall(&mut ctx_inner, length, a3, a4, a5, a6)?;
    let wasm_addr = udpate_n_mmap_pages(&mut ctx_inner, mmap_addr, length)?;
    Ok(wasm_addr.offset() as i64)
}

fn udpate_n_mmap_pages(
    ctx_inner: &mut MutexGuard<'_, InnerCtx>,
    mmap_addr: HostAddress,
    length: i32,
) -> Result<WasmAddress> {
    let memory = ctx_inner.get_memory()?;
    let host_addr: HostAddress = mmap_addr.into();
    let wasm_addr = host_addr.to_wasm_address(memory)?;
    let mmap_data = ctx_inner.mmap_data();
    let n_mmap_pages = n_native_pages_for_size(&mmap_data, length as usize);

    mmap_data.add_mmap_pages(n_mmap_pages);
    trace!("overall num of mmap pages: {}", mmap_data.n_mmap_pages);
    Ok(wasm_addr)
}

fn call_mmap_syscall(
    ctx_inner: &mut MutexGuard<'_, InnerCtx>,
    length: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i64,
) -> Result<HostAddress> {
    let memory = ctx_inner.get_memory()?;
    let memory_start = WasmAddress::new(0, memory)
        .to_host_address(memory)
        .as_usize();
    let mmap_data = ctx_inner.mmap_data();
    let aligned_address_memory_end = mmap_data.memory_end_aligned(memory_start)?;
    let mmap_addr = unsafe {
        libc::mmap(
            aligned_address_memory_end as *mut libc::c_void,
            length as usize,
            a3,
            MAP_FIXED | a4,
            a5,
            a6,
        )
    };

    if mmap_addr == libc::MAP_FAILED {
        error!("mmap failed");
        bail!("mmap failed");
    }
    Ok(mmap_addr.into())
}

fn grow_wasm_memory(ctx_inner: &mut MutexGuard<'_, InnerCtx>, length: i32) -> Result<()> {
    trace!("Mmap length: {length:x}");
    let memory = ctx_inner.get_memory()?;
    let memory_size = memory.data_size();
    let mmap_data = ctx_inner.mmap_data();

    mmap_data.init_base_size(memory_size);

    let n_additional_wasm_pages =
        n_additional_wasm_pages(memory_size, &mmap_data, length as usize)?;
    trace!("Growing wasm memory by {} pages", n_additional_wasm_pages);

    let memory = ctx_inner.get_memory()?;
    memory.grow(n_additional_wasm_pages as u64)?;
    Ok(())
}

fn n_additional_wasm_pages(memory_size: usize, mmap_data: &MMapData, size: usize) -> Result<usize> {
    let page_size_native = mmap_data.page_size_native;
    let base_memory_size = mmap_data.base_size()?;
    let mmap_memory_size = mmap_data.n_mmap_pages * page_size_native;

    let available_size = memory_size - (base_memory_size + mmap_memory_size);
    if available_size >= size {
        Ok(0)
    } else {
        let missing_size = size - available_size;
        Ok(n_wasm_pages_for_size(mmap_data, missing_size))
    }
}

fn n_wasm_pages_for_size(mmap_data: &MMapData, size: usize) -> usize {
    let page_size_wasm = mmap_data.page_size_wasm;
    let n_pages = size / page_size_wasm;
    if size % page_size_wasm != 0 {
        n_pages + 1
    } else {
        n_pages
    }
}

fn n_native_pages_for_size(mmap_data: &MMapData, size: usize) -> usize {
    let page_size_native = mmap_data.page_size_native;
    let n_pages = size / page_size_native;
    if size % page_size_native != 0 {
        n_pages + 1
    } else {
        n_pages
    }
}

fn log_arguments(a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i64) {
    trace!("mmap arguments:");
    trace!("a1: {a1}");
    trace!("a2: {a2}");
    trace!("a3: {a3}");
    trace!("a4: {a4}");
    trace!("a5: {a5}");
    trace!("a6: {a6}");
}
