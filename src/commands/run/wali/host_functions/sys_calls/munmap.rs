use anyhow::Result;
use wasmtime::Caller;

use tracing::{error, info, trace};

use crate::commands::run::wali::{memory::address::WasmAddress, WaliCtx};

pub(crate) fn syscall_munmap(caller: Caller<'_, WaliCtx>, address: i32, size: i32) -> i64 {
    info!("module has executed the 'munmap' host function");
    match syscall_munmap_impl(caller, address, size) {
        Ok(r) => r,
        Err(e) => {
            error!("error when calling munmap: {e}");
            -1
        }
    }
}

fn syscall_munmap_impl(caller: Caller<'_, WaliCtx>, address: i32, size: i32) -> Result<i64> {
    let mut ctx_inner = caller.data().lock()?;
    let memory = ctx_inner.get_memory()?;

    let munmap_start = WasmAddress::new(address, memory)
        .to_host_address(memory)
        .as_usize();
    let munmap_end = munmap_start + size as usize;

    let memory_start = WasmAddress::new(0, memory)
        .to_host_address(memory)
        .as_usize();
    let mmap_data = ctx_inner.mmap_data();
    let native_page_size = mmap_data.page_size_native;
    let memory_end = mmap_data.memory_end_aligned(memory_start)?;

    if memory_end == munmap_end {
        let n_unmapped_pages = (size as usize + native_page_size - 1) / native_page_size;
        trace!("Unmapping {n_unmapped_pages} from the end");
        mmap_data.unmap_pages_from_end(n_unmapped_pages);
    }

    let sys_call_result = unsafe { libc::munmap(munmap_start as *mut _, size as usize) };
    Ok(sys_call_result as i64)
}
