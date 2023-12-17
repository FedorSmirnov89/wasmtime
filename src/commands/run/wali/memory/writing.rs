use std::{ffi::CString, sync::atomic::Ordering};

use anyhow::Result;

use super::{address::WasmAddress, AsMemorySlice};

///
/// Writes the given c string into the given module memory at the
/// specified address. Returns the number of bytes written.
///
pub(crate) fn write_c_string_into_module_memory(
    memory: impl AsMemorySlice,
    wasm_addr: WasmAddress,
    s: CString,
) -> Result<usize> {
    let bytes = s.as_bytes();
    write_into_memory(memory, wasm_addr, bytes)
}

fn write_into_memory(
    mut memory: impl AsMemorySlice,
    wasm_addr: WasmAddress,
    bytes: &[u8],
) -> Result<usize> {
    let atomic_slice = memory.as_memory_slice();
    for idx in 0..bytes.len() {
        let mem_idx = wasm_addr.offset() + idx;
        atomic_slice[mem_idx].store(bytes[idx], Ordering::Release);
    }
    Ok(bytes.len())
}
