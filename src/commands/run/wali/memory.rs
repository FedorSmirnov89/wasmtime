//! Module for the structs and functions used for translating/reading/writing between the module and the host memory

use std::sync::atomic::AtomicU8;

use wasmtime::{Caller, SharedMemory};

use super::WaliCtx;

pub(crate) mod address;
pub(crate) mod writing;

pub(crate) trait AsMemory {
    fn as_memory(&mut self) -> SharedMemory;
}

impl AsMemory for &mut Caller<'_, WaliCtx> {
    fn as_memory(&mut self) -> SharedMemory {
        let memory_export = self
            .get_export("memory")
            .expect("module does not export memory");
        match memory_export {
            wasmtime::Extern::SharedMemory(s) => s,
            _ => unreachable!("WALI modules always have shared memory"),
        }
    }
}

pub(crate) trait AsMemorySlice {
    fn as_memory_slice(&mut self) -> &mut [AtomicU8];
}

impl AsMemorySlice for &mut Caller<'_, WaliCtx> {
    fn as_memory_slice(&mut self) -> &mut [AtomicU8] {
        let memory = self.as_memory();
        unsafe {
            std::slice::from_raw_parts_mut(
                memory.data().as_ptr() as *mut AtomicU8,
                memory.data().len(),
            )
        }
    }
}
