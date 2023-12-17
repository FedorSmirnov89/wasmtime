//! Module for the structs and functions used for translating/reading/writing between the module and the host memory

use std::sync::atomic::AtomicU8;

use wasmtime::{Caller, SharedMemory};

use super::WaliCtx;

pub(crate) mod address;
pub(crate) mod writing;

pub(crate) trait AddressCalculation {
    fn address_of_offset(&self, offset: usize) -> usize;
    fn memory_size(&self) -> usize;
}

impl AddressCalculation for &SharedMemory {
    fn address_of_offset(&self, offset: usize) -> usize {
        self.data().get(offset).expect("memory bound violation") as *const _ as usize
    }

    fn memory_size(&self) -> usize {
        self.data().len()
    }
}

pub(crate) trait AsMemory {
    fn as_memory(&mut self) -> SharedMemory;
}

impl AsMemory for Caller<'_, WaliCtx> {
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

pub(crate) trait PageAlignment {
    fn page_aligned(&self, page_size: usize) -> usize;
}

impl PageAlignment for usize {
    fn page_aligned(&self, page_size: usize) -> usize {
        let offset = *self & (page_size - 1);
        if offset == 0 {
            *self
        } else {
            *self + page_size - offset
        }
    }
}
